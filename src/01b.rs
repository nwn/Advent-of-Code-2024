use std::io::{self, prelude::*};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let mut left = vec![];
    let mut right = HashMap::<i32, i32>::new();
    for line in lines {
        let mut words = line.split_whitespace();
        let lhs = words.next().unwrap().parse::<i32>().unwrap();
        let rhs = words.next().unwrap().parse::<i32>().unwrap();
        left.push(lhs);
        *right.entry(rhs).or_insert(0) += 1;
    }

    let sim_score: i32 = left.iter()
        .map(|id| id * right.get(id).copied().unwrap_or(0))
        .sum();
    println!("{sim_score}");
}
