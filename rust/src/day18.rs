use std::{iter::Peekable, slice::Iter};


mod utils;

struct TokenError {
    message: String,
}

#[derive(Debug)]
enum Token {
    Number(i32),
    ParenthesisOpen,
    ParenthesisClose,
    OperatorPlus,
    OperatorTimes,
}

#[derive(Debug)]
enum Operation {
    Plus,
    Times,
}

fn main() {
    let lines = utils::read_input();
    let lines_tokens = parse_input(&lines);

    let result = lines_tokens
        .iter()
        .map(|tokens| {
            let mut iter = tokens.iter().peekable();
            evaluate_expression(&mut iter)
        })
        .sum::<i64>();

    println!("{}", result);
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<Token>> {
    lines
        .iter()
        .map(|line| parse_tokens(line))
        .collect()
}

fn parse_tokens(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = line.chars();

    let mut number_builder: Option<i32> = None;
    
    while let Some(c) = chars.next() {
        if !c.is_numeric() {
            if let Some(number) = number_builder {
                tokens.push(Token::Number(number));
                number_builder = None;
            }
        }

        match c {
            '(' => tokens.push(Token::ParenthesisOpen),
            ')' => tokens.push(Token::ParenthesisClose),
            '+' => tokens.push(Token::OperatorPlus),
            '*' => tokens.push(Token::OperatorTimes),
            digit_char if c.is_numeric() => {
                let digit = digit_char.to_digit(10).unwrap() as i32;

                match number_builder {
                    Some(number) => {
                        number_builder = Some(number * 10 + digit)
                    },
                    None => {
                        number_builder = Some(digit)
                    }
                }
            },
            _ if c.is_whitespace() => {},
            _ => panic!("Got unexpected character {}", c)
        }
    }

    if let Some(number) = number_builder {
        tokens.push(Token::Number(number));
    }

    tokens
}

fn evaluate_expression(iter: &mut Peekable<Iter<Token>>) -> i64 {
    let mut result = get_number(iter);

    while get_operation(iter.peek().map(|&token| token)).is_ok() {
        result = apply_operation(result, iter);                
    }

    result
}

// Read and apply "operation, num" pair to the first num from the parameters.
fn apply_operation(first_number: i64, iter: &mut Peekable<Iter<Token>>) -> i64 {
    match get_operation(iter.next()) {
        Ok(operation) => {
            let second_number = get_number(iter);

            match operation {
                Operation::Plus => first_number + second_number,
                Operation::Times => first_number * second_number,
            }
        },
        Err(err) => panic!("{}", err.message),
    }
}

fn get_operation(next_token: Option<&Token>) -> Result<Operation, TokenError> {
    match next_token {
        Some(token) => match token {
            Token::OperatorPlus => Ok(Operation::Plus),
            Token::OperatorTimes => Ok(Operation::Times),
            _ => Err(TokenError {
                message: format!("Unexpected token at this place. Expected operation, but got: {:?}", token)
            }),
        },
        None => Err(TokenError {
            message: String::from("Expected an operation token at this place. But got nothing.")
        })
    }
}

fn get_number(iter: &mut Peekable<Iter<Token>>) -> i64 {
    match iter.next() {
        Some(Token::Number(num)) => *num as i64,

        Some(Token::ParenthesisOpen) => {
            let num = evaluate_expression(iter);

            match iter.next() {
                Some(Token::ParenthesisClose) => {}, // Just consume it from iterator
                Some(token) => panic!("Expected closing parenthesis but got: {:?}", token),
                None => panic!("Expected closing parenthesis but got nothing"),
            };

            num
        },

        Some(token)  => panic!("Unexpected token at this place. Expected number or parenthesis, but got: {:?}", token),
        None => panic!("No token at this place. Expected number or opening parenthesis, but got nothing"),
    }
}
