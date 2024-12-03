use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let mut left = vec![];
    let mut right = vec![];
    for line in lines {
        let mut words = line.split_whitespace();
        let lhs = words.next().unwrap().parse::<i32>().unwrap();
        let rhs = words.next().unwrap().parse::<i32>().unwrap();
        left.push(lhs);
        right.push(rhs);
    }
    left.sort();
    right.sort();

    let total_dist: i32 = left.into_iter().zip(right)
        .map(|(lhs, rhs)| (lhs - rhs).abs())
        .sum();
    println!("{total_dist}");
}
