// 812. Largest Triangle Area
// Easy

// You have a list of points in the plane. Return the area of the largest
// triangle that can be formed by any 3 of the points.

// Example:
// Input: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
// Output: 2
// Explanation:
// The five points are show in the figure below. The red triangle is the
// largest.

pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    use std::f64;
    let mut largest: f64 = 0.;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            for m in (j + 1)..points.len() {
                largest = f64::max(largest, area(&points[i], &points[j], &points[m]));
            }
        }
    }
    largest
}
fn area(a: &[i32], b: &[i32], c: &[i32]) -> f64 {
    let r =
        (a[0] * b[1] + b[0] * c[1] + c[0] * a[1] - a[1] * b[0] - b[1] * c[0] - c[1] * a[0]).abs();
    r as f64 / 2.
}
