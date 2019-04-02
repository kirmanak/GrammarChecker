use std::io::{stdin, Read};
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
        println!("Result: {}", result);
        if result > 0 {
            let vec = buffer.to_vec();
            let string = String::from_utf8(vec).expect("Failed to parse stdin contents");
            let string = string.trim().to_string();
            check_result = check_grammar(&mut stack, string.chars());
        }
        if result < BUF_SIZE || !check_result {
            break; // Since we've read less than requested the input is over
        }
    }
    println!("{}", check_result);
}

/**
 * Accepts current stack state and a bunch of new characters
 */
fn check_grammar(stack: &mut Vec<Symbol>, chars: Chars) -> bool {
    for character in chars {
        let symbol = Symbol::from(character);
        let stack_head = &stack[stack.len() - 1];
        match order(stack_head, &symbol) {
            Order::Equal | Order::Less => stack.push(symbol),
            Order::Greater => {}
            Order::NoRule => return false,
        }
    }

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

fn order(stack_head: &Symbol, new_symbol: &Symbol) -> Order {
    if *stack_head == Symbol::hash || *new_symbol == Symbol::hash {
        panic!("{:?}, {:?}", stack_head, new_symbol);
    }
    match stack_head {
        Symbol::S => match new_symbol {
            Symbol::a | Symbol::b => Order::Equal,
            _ => Order::NoRule,
        },
        Symbol::B => match new_symbol {
            Symbol::S | Symbol::B | Symbol::A | Symbol::c | Symbol::d => Order::Less,
            Symbol::F | Symbol::b => Order::Equal,
            Symbol::a => Order::Greater,
            _ => Order::NoRule,
        },
        Symbol::A => match new_symbol {
            Symbol::S | Symbol::B | Symbol::A | Symbol::c | Symbol::d => Order::Less,
            Symbol::E | Symbol::a => Order::Equal,
            Symbol::b => Order::Greater,
            _ => Order::NoRule,
        },
        Symbol::E => match new_symbol {
            Symbol::a => Order::Equal,
            _ => Order::NoRule,
        },
        Symbol::F => match new_symbol {
            Symbol::b => Order::Equal,
            _ => Order::NoRule,
        },
        Symbol::b | Symbol::d => match new_symbol {
            Symbol::E => Order::NoRule,
            _ => Order::Greater,
        },
        Symbol::a | Symbol::c => match new_symbol {
            Symbol::F => Order::NoRule,
            _ => Order::Greater,
        },
        _ => Order::NoRule,
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
        assert_eq!(check_grammar(&mut vec![Symbol::hash], "a".chars()), true);
    }

    #[test]
    #[should_panic]
    fn panics() {
        check_grammar(&mut vec![Symbol::hash], "h".chars());
    }
}
