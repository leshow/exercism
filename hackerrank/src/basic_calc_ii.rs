// https://leetcode.com/problems/basic-calculator-ii/
pub fn calculate(s: String) -> i32 {
    unimplemented!();
}

use std::iter::Peekable;

fn lex(i: &str) -> Result<Vec<Token>, String> {
    let mut res = Vec::new();
    let mut it = i.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '+' => {
                res.push(Token::Add);
                it.next();
            }
            '-' => {
                res.push(Token::Sub);
                it.next();
            }
            '*' => {
                res.push(Token::Mul);
                it.next();
            }
            '/' => {
                res.push(Token::Div);
                it.next();
            }
            ' ' => {
                it.next();
            }
            '0'..='9' => {
                res.push(lex_num(c, &mut it)?);
            }
            _ => return Err(format!("Can't parse token {}", c)),
        }
    }
    Ok(res)
}

fn lex_num<T: Iterator<Item = char>>(c: char, it: &mut Peekable<T>) -> Result<Token, String> {
    // let mut n = c.to_digit(10).ok_or_else(|| "Cant convert to digit")?;
    let mut n = 0;
    while let Some(p) = it.peek().and_then(|p| p.to_digit(10)) {
        n = n * 10 + p;
        it.next();
    }
    Ok(Token::Num(n as i32))
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Num(i32),
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Expr {
    Num(i32),
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn parse(tokens: &[Token]) -> Result<(Expr, usize), String> {
    unimplemented!()
}

#[test]
fn test_lex() {
    let v = lex("2 + 2 * 4").expect("failed to unwrap");
    let res = vec![
        Token::Num(2),
        Token::Add,
        Token::Num(2),
        Token::Mul,
        Token::Num(4),
    ];
    for (token, r) in v.into_iter().zip(res.into_iter()) {
        assert_eq!(token, r);
    }
}
