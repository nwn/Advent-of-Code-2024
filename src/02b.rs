use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let mut num_safe = 0;
    for line in lines {
        let levels: Vec<i32> = line.split_whitespace().map(|word| word.parse().unwrap()).collect();
        if is_safe_dampened(levels) {
            num_safe += 1;
        }
    }
    println!("{num_safe}");
}

fn is_safe_dampened(mut levels: Vec<i32>) -> bool {
    // There are two cases where exactly one level may render a record unsafe:
    // - The level is more than 3 away from either of its neighbours. In this case, it will be
    //   reported as either the index returned by `is_safe`, or the one thereafter.
    // - The level, with either of its neighbours, is not monotonic to the rest of the record. In
    //   this case, either:
    //   - The original "signum" calculated from the first pair is correct and the later level at
    //     the returned index is incorrect.
    //   - The original "signum" is _incorrect_, in which case the first level must be incorrect.
    //
    // We therefore try three cases when a record is not trivially safe:
    // - Removing the first level
    // - Removing the level at the returned index
    // - Removing the level immediately following the returned index

    // Try as is.
    if let Err(idx) = is_safe(&levels) {
        // Try without the first element.
        if is_safe(&levels[1..]).is_ok() {
            return true;
        }

        // Try without the n'th element.
        let removed = levels.remove(idx);
        if is_safe(&levels).is_ok() {
            return true;
        }

        // Try without the (n+1)'th element.
        levels[idx] = removed;
        is_safe(&levels).is_ok()
    } else {
        true
    }
}

fn is_safe(levels: &'_[i32]) -> Result<(), usize> {
    let mut signum = 0;
    for (idx, (lhs, rhs)) in adjacent_pairs(levels).enumerate() {
        let diff = rhs - lhs;
        if signum == 0 { signum = diff.signum() }
        if diff.signum() != signum || (diff.abs() < 1) || (diff.abs() > 3) {
            return Err(idx);
        }
    }
    Ok(())
}

fn adjacent_pairs(list: &[i32]) -> impl Iterator<Item = (i32, i32)> + use<'_> {
    list.windows(2).map(|pair| (pair[0], pair[1]))
}
