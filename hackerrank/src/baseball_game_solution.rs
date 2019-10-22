pub fn cal_points(ops: Vec<String>) -> i32 {
    use std::collections::VecDeque;
    let mut stack = VecDeque::new();
    let mut ops = ops.into_iter().rev().collect::<Vec<_>>();
    while !ops.is_empty() {
        match &ops.pop().unwrap()[..] {
            "+" => {
                let a = stack.pop_back().unwrap();
                let b = *stack.back().unwrap();
                stack.push_back(a);
                stack.push_back(a + b);
            }
            "D" => {
                stack.push_back(2 * (*stack.back().unwrap()));
            }
            "C" => {
                stack.pop_back();
            }
            i => {
                stack.push_back(i.parse::<i32>().unwrap());
            }
        }
    }
    stack.into_iter().sum()
}
