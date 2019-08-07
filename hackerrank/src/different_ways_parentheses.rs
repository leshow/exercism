/// leetcode
/// 241. Different Ways to Add Parentheses
/// Given a string of numbers and operators, return all possible results from
/// computing all the different possible ways to group numbers and operators.
/// The valid operators are +, - and *.
///
/// Example 1:
///
/// Input: "2-1-1"
/// Output: [0, 2]
/// Explanation:
/// ((2-1)-1) = 0
/// (2-(1-1)) = 2
///
/// Example 2:
/// Input: "2*3-4*5"
/// Output: [-34, -14, -10, -10, 10]
/// Explanation:
/// (2*(3-(4*5))) = -34
/// ((2*3)-(4*5)) = -14
/// ((2*(3-4))*5) = -10
/// (2*((3-4)*5)) = -10
/// (((2*3)-4)*5) = 10
pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
    let chars: Vec<char> = input.chars().collect::<Vec<_>>();
    parse(&chars).into_iter().map(|tree| tree.eval()).collect()
}

fn parse(input: &[char]) -> Vec<AST> {
    let mut res = vec![];
    if input.is_empty() {
        return res;
    }
    for i in 0..input.len() {
        if input[i].is_digit(10) {
            continue;
        } else {
            let op = input[i];
            let left = parse(&input[0..i]);
            let right = parse(&input[(i + 1)..]);
            for a in &left {
                for b in &right {
                    let val = AST::op(op, a, b);
                    res.push(val);
                }
            }
        }
    }
    if res.is_empty() {
        res.push(AST::Num(
            input.iter().collect::<String>().parse::<i32>().unwrap(),
        ));
    }
    res
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum AST {
    Num(i32),
    Add(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),
    Mul(Box<AST>, Box<AST>),
    Null,
}

impl AST {
    pub fn num(c: i32) -> AST {
        AST::Num(c)
    }

    pub fn get_num(&self) -> Option<i32> {
        match *self {
            AST::Num(i) => Some(i),
            _ => None,
        }
    }

    pub fn op(c: char, a: &AST, b: &AST) -> AST {
        match c {
            '+' => AST::Add(Box::new(a.clone()), Box::new(b.clone())),
            '-' => AST::Sub(Box::new(a.clone()), Box::new(b.clone())),
            '*' => AST::Mul(Box::new(a.clone()), Box::new(b.clone())),
            _ => AST::Null,
        }
    }

    pub fn eval(self) -> i32 {
        match self {
            AST::Num(i) => i,
            AST::Add(a, b) => a.eval() + b.eval(),
            AST::Sub(a, b) => a.eval() - b.eval(),
            AST::Mul(a, b) => a.eval() * b.eval(),
            AST::Null => panic!("Found magic variant"),
        }
    }
}

#[test]
fn test_compute() {
    let ans = diff_ways_to_compute("2*3-4*5".to_string());
    assert_eq!(&ans[..], &[-34, -10, -14, -10, 10]);
}

// Alternate solution w/o AST
fn diff_ways_2(input: String) -> Vec<i32> {
    let chars: Vec<char> = input.chars().collect::<Vec<_>>();
    parse_2(&chars)
}

fn parse_2(input: &[char]) -> Vec<i32> {
    let mut r = Vec::new();
    if input.is_empty() {
        return r;
    }
    for i in 0..input.len() {
        if input[i].is_digit(10) {
            continue;
        } else {
            let op = input[i];
            let left = parse_2(&input[0..i]);
            let right = parse_2(&input[(i + 1)..]);
            for a in &left {
                for b in &right {
                    r.push(eval(op, *a, *b));
                }
            }
        }
    }
    if r.is_empty() {
        r.push(input.iter().collect::<String>().parse::<i32>().unwrap());
    }
    r
}

fn eval(op: char, a: i32, b: i32) -> i32 {
    match op {
        '-' => a - b,
        '*' => a * b,
        '+' => a + b,
        _ => panic!(),
    }
}
