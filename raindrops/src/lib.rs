pub fn raindrops(n: i32) -> String {
    let mut out = String::new();
    if n % 3 == 0 {
        out.push_str("Pling");
    }
    if n % 5 == 0 {
        out.push_str("Plang");
    }
    if n % 7 == 0 {
        out.push_str("Plong");
    }
    if out.len() == 0 {
        out.push_str(&n.to_string());
    }
    out
}
