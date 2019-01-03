fn main() {
    println!("{:?}", generate_parenthesis(3));
}
// problem var n
// output [String]
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut v = Vec::new();
    gen("".into(), n, n, &mut v);
    v
}
fn gen(s: String, open: i32, close: i32, r: &mut Vec<String>) {
    if open == 0 && close == 0 {
        r.push(s);
        return;
    }
    if open > 0 && open <= close {
        gen(format!("{}(", &s), open - 1, close, r);
    }
    if close > 0 && open < close {
        gen(format!("{})", &s), open, close - 1, r);
    }
    return;
}
