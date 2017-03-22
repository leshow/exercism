pub fn hello(name: Option<&str>) -> String {
    match name {
        None => "Hello, World!".to_string(),
        Some(n) => format!("Hello, {}", n),
    }
}
