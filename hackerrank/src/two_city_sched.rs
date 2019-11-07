// 1029. Two City Scheduling
// Easy
// There are 2N people a company is planning to interview. The cost of flying
// the i-th person to city A is costs[i][0], and the cost of flying the i-th
// person to city B is costs[i][1]. Return the minimum cost to fly every person
// to a city such that exactly N people arrive in each city. Example 1:
// Input: [[10,20],[30,200],[400,50],[30,20]]
// Output: 110
// Explanation:
// The first person goes to city A for a cost of 10.
// The second person goes to city A for a cost of 30.
// The third person goes to city B for a cost of 50.
// The fourth person goes to city B for a cost of 20.
// The total minimum cost is 10 + 30 + 50 + 20 = 110 to have half the people
// interviewing in each city.

pub fn two_city_sched_rec(costs: Vec<Vec<i32>>) -> i32 {
    use std::cmp;
    pub fn two_exp(costs: &[Vec<i32>], n: usize, na: usize, nb: usize) -> i32 {
        if n == costs.len() {
            return 0;
        }
        let mut a = i32::max_value();
        let mut b = i32::max_value();

        if na > 0 {
            a = costs[n][0] + two_exp(costs, n + 1, na - 1, nb);
        }
        if nb > 0 {
            b = costs[n][1] + two_exp(costs, n + 1, na, nb - 1);
        }
        cmp::min(a, b)
    }
    let n = costs.len() >> 1;
    two_exp(&costs, 0, n, n)
}

pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let mut costs = costs;
    costs.sort_by(|a, b| (a[1] - a[0]).cmp(&(b[1] - b[0])));
    let mid = costs.len() >> 1;
    let suma: i32 = costs[..mid].iter().map(|c| c[1]).sum();
    let sumb: i32 = costs[mid..].iter().map(|c| c[0]).sum();
    suma + sumb
}
