use Direction::*;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    arr.iter().fold(Vec::new(), |mut acc, &d| {
        if acc.is_empty() {
            acc.push(d);
            acc
        } else {
            let head = acc.last().unwrap();
            if invert(*head) == d {
                acc
            } else {
                acc.push(d);
                acc
            }
        }
    })
}

fn invert(a: Direction) -> Direction {
    match a {
        NORTH => SOUTH,
        SOUTH => NORTH,
        EAST => WEST,
        WEST => EAST,
    }
}

fn dir_reduc2(arr: &[Direction]) -> Vec<Direction> {
    let mut dir = Vec::new();
    for d in arr {
        match dir.last() {
            Some(d2) if invert(*d2) == *d => {
                dir.pop();
            }
            _ => {
                dir.push(*d);
            }
        }
    }
    dir
}
