// https://leetcode.com/problems/basic-calculator-ii/
impl Solution {
    pub fn calculate(s: String) -> i32 {
              unimplemented! 
    }
}

use std::iter::Peekable;

fn lex(i: &String) -> Result<Vec<Token>, String> {
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
            _ => return Err(format!("Can't parse token {}", c))
        }
    }
    res
}

fn lex_num(c: &char, it: &mut Peekable<T>) -> Result<Token, String> {
    let mut n = c.to_digit(10)?;
    while let Some(&p) = it.peek() {
        n = n * 10 + p.to_digit();
        iter.next();
    }
    Ok(Token::Num(n))
}


enum Token {
    Add, Sum, Mul, Div, Num(i32)
}

enum Expr {
    Num(i32), Mul(Box<Expr>, Box<Expr>), Add(Box<Expr>, Box<Expr>), Sub(Box<Expr>, Box<Expr>), 
    Div(Box<Expr>, Box<Expr>)
}

fn parse(tokens: &[Token]) -> Expr {
    match 
}