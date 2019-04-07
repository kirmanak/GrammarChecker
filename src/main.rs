use std::io::{Read, stdin};
use std::str::Chars;

const BUF_SIZE: usize = 256;

fn main() {
    let mut stack: Vec<Symbol> = Vec::new();
    stack.push(Symbol::hash);

    let mut buffer = [0; BUF_SIZE];
    let stdin = stdin();
    let mut reader = stdin.lock();

    let mut check_result = false;
    loop {
        let result = reader.read(&mut buffer).expect("Failed to read from stdin");
        if result > 0 {
            let vec = buffer[0..result].to_vec();
            let string = String::from_utf8(vec)
                .expect("Failed to parse stdin contents")
                .trim()
                .to_string();
            check_result = check_grammar(&mut stack, string.chars());
        }
        if result < BUF_SIZE || !check_result {
            break; // Since we've read less than requested the input is over
        }
    }
    check_result = check_result && stack == vec![Symbol::hash, Symbol::S];
    let verb = if check_result { "is" } else { "is not" };
    println!("The grammar {} correct", verb);
}

/**
 * Accepts current stack state and a bunch of new characters
 * Returns true if another bunch should be read
 */
fn check_grammar(stack: &mut Vec<Symbol>, chars: Chars) -> bool {
    for character in chars {
        let symbol = Symbol::from(character);
        loop {
            let stack_head = stack.last().expect("Stack is empty!");
            match order(stack_head, &symbol) {
                Order::Equal | Order::Less => {
                    stack.push(symbol);
                    break;
                }
                Order::Greater => {
                    if !reduce(stack) {
                        return false;
                    }
                }
                Order::NoRule => return false,
            }
        }
    }

    true
}

fn reduce(stack: &mut Vec<Symbol>) -> bool {
    let mut base: Vec<Symbol> = Vec::with_capacity(stack.len());

    loop {
        let last_symbol = stack.pop().unwrap();
        base.push(last_symbol);
        let last_symbol = base.last().unwrap();
        let stack_head = stack.last().unwrap();
        if order(stack_head, last_symbol) == Order::Less {
            break;
        }
    }

    let new_symbol = if base == vec![Symbol::b, Symbol::B] || base == vec![Symbol::a, Symbol::A] {
        Symbol::S
    } else if base == vec![Symbol::a, Symbol::S]
        || base == vec![Symbol::a, Symbol::E, Symbol::A]
        || base == vec![Symbol::c]
    {
        Symbol::A
    } else if base == vec![Symbol::b, Symbol::S]
        || base == vec![Symbol::b, Symbol::F, Symbol::B]
        || base == vec![Symbol::d]
    {
        Symbol::B
    } else if base == vec![Symbol::B] {
        Symbol::E
    } else if base == vec![Symbol::A] {
        Symbol::F
    } else {
        return false;
    };
    stack.push(new_symbol);
    true
}

#[derive(Debug, Eq, PartialEq)]
enum Order {
    Greater,
    Equal,
    Less,
    NoRule,
}

#[derive(Debug, Eq, PartialEq)]
enum Symbol {
    S,
    B,
    A,
    E,
    F,
    a,
    b,
    c,
    d,
    hash,
}

fn order(left: &Symbol, right: &Symbol) -> Order {
    match left {
        Symbol::S => match right {
            Symbol::a | Symbol::b => Order::Equal,
            Symbol::hash => Order::Greater,
            _ => Order::NoRule,
        },
        Symbol::B => match right {
            Symbol::S | Symbol::B | Symbol::A | Symbol::c | Symbol::d => Order::Less,
            Symbol::F | Symbol::b => Order::Equal,
            Symbol::a | Symbol::hash => Order::Greater,
            _ => Order::NoRule,
        },
        Symbol::A => match right {
            Symbol::S | Symbol::B | Symbol::A | Symbol::c | Symbol::d => Order::Less,
            Symbol::E | Symbol::a => Order::Equal,
            Symbol::b | Symbol::hash => Order::Greater,
            _ => Order::NoRule,
        },
        Symbol::E => match right {
            Symbol::a => Order::Equal,
            Symbol::hash => Order::Greater,
            _ => Order::NoRule,
        },
        Symbol::F => match right {
            Symbol::b => Order::Equal,
            Symbol::hash => Order::Greater,
            _ => Order::NoRule,
        },
        Symbol::b | Symbol::d => match right {
            Symbol::E => Order::NoRule,
            _ => Order::Greater,
        },
        Symbol::a | Symbol::c => match right {
            Symbol::F => Order::NoRule,
            _ => Order::Greater,
        },
        Symbol::hash => Order::Less,
    }
}

impl From<char> for Symbol {
    fn from(character: char) -> Symbol {
        match character {
            'a' => Symbol::a,
            'b' => Symbol::b,
            'c' => Symbol::c,
            'd' => Symbol::d,
            _ => panic!("{} is not allowed", character),
        }
    }
}

mod tests {
    use super::*;

    fn check(string: &str) -> bool {
        check_grammar(&mut vec![Symbol::hash], string.chars())
    }

    #[test]
    fn empty() {
        assert_eq!(check(""), true);
    }

    #[test]
    fn db() {
        assert_eq!(check("db"), true);
    }

    #[test]
    fn bd() {
        assert_eq!(check("bd"), false);
    }

    #[test]
    fn dbaaaabcbb() {
        assert_eq!(check("dbaaaabcbb"), true)
    }

    #[test]
    #[should_panic(expected = "h is not allowed")]
    fn panics() {
        check("h");
    }

    #[test]
    fn caaaaa() {
        assert_eq!(check("caaaaa"), true);
    }

    #[test]
    fn aaaaa() {
        assert_eq!(check("aaaaa"), false);
    }
}
