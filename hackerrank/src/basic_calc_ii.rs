// https://leetcode.com/problems/basic-calculator-ii/
pub fn calculate(s: String) -> i32 {
    let (expr, _) = parse_all(&lex(&s).expect("Lexing failed")).unwrap();
    expr.eval()
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

impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::Num(n) => *n,
            Expr::Mul(e1, e2) => e1.eval() * e2.eval(),
            Expr::Div(e1, e2) => e1.eval() / e2.eval(),
            Expr::Sub(e1, e2) => e1.eval() - e2.eval(),
            Expr::Add(e1, e2) => e1.eval() + e2.eval(),
        }
    }
}

fn parse_num(tokens: &[Token]) -> Option<(Expr, &[Token])> {
    let (head, tail) = tokens.split_at(1);
    match head {
        [Token::Num(t)] => Some((Expr::Num(*t), tail)),
        _ => None,
    }
}

fn parse_add(tokens: &[Token]) -> Option<(Expr, &[Token])> {
    let (expr1, tokens) = parse_num(tokens)?;
    dbg!(&expr1, tokens);
    if tokens.is_empty() {
        return Some((expr1, tokens));
    }
    let (head, tail) = tokens.split_at(1);
    if let [Token::Add] = head {
        let (expr2, rest) = parse_add(tail)?;
        return Some((Expr::Add(Box::new(expr1), Box::new(expr2)), rest));
    }
    Some((expr1, tokens))
}

fn parse_sub(tokens: &[Token]) -> Option<(Expr, &[Token])> {
    let (expr1, tokens) = parse_add(tokens)?;
    dbg!(&expr1, tokens);
    if tokens.is_empty() {
        return Some((expr1, tokens));
    }
    let (head, tail) = tokens.split_at(1);
    if let [Token::Sub] = head {
        let (expr2, rest) = parse_sub(tail)?;
        return Some((Expr::Sub(Box::new(expr1), Box::new(expr2)), rest));
    }
    Some((expr1, tokens))
}

fn parse_div(tokens: &[Token]) -> Option<(Expr, &[Token])> {
    let (expr1, tokens) = parse_sub(tokens)?;
    dbg!(&expr1, tokens);
    if tokens.is_empty() {
        return Some((expr1, tokens));
    }
    let (head, tail) = tokens.split_at(1);
    if let [Token::Div] = head {
        let (expr2, rest) = parse_div(tail)?;
        return Some((Expr::Div(Box::new(expr1), Box::new(expr2)), rest));
    }
    Some((expr1, tokens))
}

fn parse_mul(tokens: &[Token]) -> Option<(Expr, &[Token])> {
    let (expr1, tokens) = parse_div(tokens)?;
    dbg!(&expr1, tokens);
    if tokens.is_empty() {
        return Some((expr1, tokens));
    }
    let (head, tail) = tokens.split_at(1);
    if let [Token::Mul] = head {
        let (expr2, rest) = parse_mul(tail)?;
        return Some((Expr::Mul(Box::new(expr1), Box::new(expr2)), rest));
    }
    Some((expr1, tokens))
}

fn parse_all(tokens: &[Token]) -> Option<(Expr, &[Token])> {
    let (expr, tokens) = parse_mul(tokens)?;
    Some((expr, tokens))
}

#[test]
fn test_parse() {
    let tokens = lex("3+2*1-4*5").expect("failed");
    println!("{:?}", tokens);
    println!("{:?}", parse_all(&tokens));
}

#[test]
fn test_calc() {
    assert_eq!(calculate("3+2*2".into()), 7);
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
