use std::io::{stdin, Read};
use std::str::Chars;

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");
    println!("{:?}", check_grammar(buffer.chars()));
}

fn check_grammar(chars: Chars) -> bool {
    let mut stack: Vec<Symbol> = Vec::new();
    stack.push(Symbol::hash);

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
        Symbol::hash => {
            panic!("We didn't panic earlier somehow");
        }
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
    use super::check_grammar;

    #[test]
    fn empty() {
        assert_eq!(check_grammar("".chars()), true);
    }

    #[test]
    fn correct() {
        assert_eq!(check_grammar("a".chars()), true);
    }

    #[test]
    #[should_panic]
    fn panics() {
        check_grammar("h".chars());
    }
}
