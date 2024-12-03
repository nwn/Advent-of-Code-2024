use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let mut num_safe = 0;
    for line in lines {
        let levels: Vec<i32> = line.split_whitespace().map(|word| word.parse().unwrap()).collect();
        if is_safe(&levels) {
            num_safe += 1;
        }
    }
    println!("{num_safe}");
}

fn is_safe(levels: &'_[i32]) -> bool {
    (adjacent_pairs(levels).all(|(lhs, rhs)| lhs < rhs) ||
    adjacent_pairs(levels).all(|(lhs, rhs)| lhs > rhs)) &&
    adjacent_pairs(levels).all(|(lhs, rhs)| (rhs - lhs).abs() <= 3)
}

fn adjacent_pairs(list: &[i32]) -> impl Iterator<Item = (i32, i32)> + use<'_> {
    list.windows(2).map(|pair| (pair[0], pair[1]))
}
