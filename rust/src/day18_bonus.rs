use std::collections::VecDeque;

mod utils;

#[derive(Debug)]
enum Token {
    Number(i64),
    ParenthesisOpen,
    ParenthesisClose,
    OperatorPlus,
    OperatorTimes,
}

fn main() {
    let lines = utils::read_input();
    let lines_tokens = parse_input(&lines);

    let result = lines_tokens
        .iter()
        .map(|tokens| to_polish_notation(tokens))
        .map(|tokens_in_polish_notation| evaluate_expression(&tokens_in_polish_notation))
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
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::ParenthesisOpen),
            ')' => tokens.push(Token::ParenthesisClose),
            '+' => tokens.push(Token::OperatorPlus),
            '*' => tokens.push(Token::OperatorTimes),
            digit_char if c.is_ascii_digit() => {
                let mut number = digit_char.to_digit(10).unwrap() as i64;
                
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_ascii_digit() {
                        number = number * 10 + next_c.to_digit(10).unwrap() as i64;
                        chars.next(); // Consume the char
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Number(number));
            },
            _ if c.is_whitespace() => {},
            _ => panic!("Got unexpected character {}", c)
        }
    }

    tokens
}

// Using shunting yard algorithm.
fn to_polish_notation(tokens: &Vec<Token>) -> VecDeque<&Token> {
    let mut output = VecDeque::new();
    let mut operators = VecDeque::<&Token>::new();

    tokens
        .iter()
        .for_each(|token| {
            match token {
                Token::Number(_) => output.push_back(token),
                Token::OperatorPlus => operators.push_back(token),
                // The asignment specifies that '+' has higher precedence than '*'
                Token::OperatorTimes => {
                    while let Some(last_operator) = operators.back() {
                        match last_operator {
                            Token::OperatorPlus => output.push_back(operators.pop_back().unwrap()),
                            _ => break
                        }
                    }
                    operators.push_back(token);
                },
                Token::ParenthesisOpen => operators.push_back(token),
                Token::ParenthesisClose => {
                    while let Some(operator) = operators.pop_back() {
                        match operator {
                            Token::ParenthesisOpen => break,
                            _ => output.push_back(operator),
                        }
                    }
                }
            }
        });

    while let Some(operator) = operators.pop_back() {
        output.push_back(operator);
    }

    output
}

fn evaluate_expression(tokens_in_polish_notation: &VecDeque<&Token>) -> i64 {
    let mut stack: VecDeque<i64> = VecDeque::<i64>::new();

    tokens_in_polish_notation
        .iter()
        .for_each(|token| {
            let new_num = match token {
                Token::Number(num) => *num,
                Token::OperatorPlus => stack.pop_back().unwrap() + stack.pop_back().unwrap(),
                Token::OperatorTimes => stack.pop_back().unwrap() * stack.pop_back().unwrap(),
                _ => panic!("Expected only number or operation tokens, but got: {:?}", token)
            };
            stack.push_back(new_num);
        });

    stack[0]
}