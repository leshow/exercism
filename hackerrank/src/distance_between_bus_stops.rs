// 1184. Distance Between Bus Stops
// Easy

// A bus has n stops numbered from 0 to n - 1 that form a circle. We know the
// distance between all pairs of neighboring stops where distance[i] is the
// distance between the stops number i and (i + 1) % n.

// The bus goes along both directions i.e. clockwise and counterclockwise.

// Return the shortest distance between the given start and destination stops.
pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    use std::{cmp, mem};
    let mut start = start as usize;
    let mut destination = destination as usize;
    if start > destination {
        mem::swap(&mut start, &mut destination);
    }    let sum1 = distance[start..destination].iter().sum();

    let sum2 = distance[..start].iter().sum::<i32>() + distance[destination..].iter().sum::<i32>();
    cmp::min(sum1, sum2)
}
