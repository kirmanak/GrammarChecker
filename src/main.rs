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
            let string = String::from_utf8(vec).expect("Failed to parse stdin contents");
            let string = string.trim().to_string();
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
        let last_symbol = stack.pop().expect("Stack is empty!");
        base.push(last_symbol);
        let last_symbol = base.last().expect("Stack is empty!");
        let stack_head = stack.last();
        if let Some(head) = stack_head {
            if order(head, last_symbol) == Order::Less {
                break;
            }
        } else {
            break;
        }
    }

    let new_symbol = if base == vec![Symbol::B, Symbol::b] || base == vec![Symbol::A, Symbol::a] {
        Symbol::S
    } else if base == vec![Symbol::S, Symbol::a]
        || base == vec![Symbol::A, Symbol::E, Symbol::a]
        || base == vec![Symbol::c]
    {
        Symbol::A
    } else if base == vec![Symbol::S, Symbol::b]
        || base == vec![Symbol::B, Symbol::F, Symbol::b]
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

    #[test]
    fn empty() {
        assert_eq!(check_grammar(&mut vec![Symbol::hash], "".chars()), true);
    }

    #[test]
    fn correct() {
        assert_eq!(check_grammar(&mut vec![Symbol::hash], "db".chars()), true);
    }

    #[test]
    fn incorrect() {
        assert_eq!(check_grammar(&mut vec![Symbol::hash], "bd".chars()), false);
    }

    #[test]
    #[should_panic]
    fn panics() {
        check_grammar(&mut vec![Symbol::hash], "h".chars());
    }
}
