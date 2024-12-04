use std::fs;

static FILE_PATH: &str = "input";

type Report = Vec<usize>;

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let reports: Vec<Report> = contents
        .lines()
        .into_iter()
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| level.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    part_one(&reports);
    part_two(&reports);
}

fn part_one(reports: &Vec<Report>) {
    let mut safe = 0;
    for report in reports {
        if is_safe(&report) {
            safe = safe + 1;
        }
    }
    println!("Number of safe reports in part one: {}", safe);
}

fn part_two(reports: &Vec<Report>) {
    let mut safe = 0;
    for report in reports {
        if num_unsafe(&report) <= 1 {
            safe = safe + 1;
        }
    }
    println!("Number of safe reports in part two: {}", safe);
}

fn is_safe(report: &Report) -> bool {
    return num_unsafe(report) == 0;
}

fn num_unsafe(report: &Report) -> usize {
    // true is increasing, false is decreasing, but the only thing that
    // really matters is just that it stays consistent
    let mut dir: Option<bool> = None;
    let mut iter = report.iter().peekable();
    let mut n_unsafe = 0;
    loop {
        let level = iter.next();
        match level {
            None => break,
            Some(level) => {
                let next_level = iter.peek();
                match next_level {
                    None => break,
                    Some(next_level) => match dir {
                        None => {
                            if *level != **next_level {
                                dir = Some(*next_level > level);
                            }
                            if !compare_adjacent(*level, **next_level) {
                                n_unsafe = n_unsafe + 1;
                            }
                        }
                        Some(incr_or_decr) => {
                            if !((*next_level > level) == incr_or_decr) {
                                n_unsafe = n_unsafe + 1;
                            } else if !compare_adjacent(*level, **next_level) {
                                n_unsafe = n_unsafe + 1;
                            }
                        }
                    },
                }
            }
        }
    }
    return n_unsafe;
}

fn compare_adjacent(level: usize, next_level: usize) -> bool {
    if level == next_level {
        return false;
    }
    let diff = (next_level as isize - level as isize).abs();
    if diff > 3 {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_num_unsafe() {
        assert_eq!(num_unsafe(&vec![7, 6, 4, 2, 1]), 0);
        assert_eq!(num_unsafe(&vec![1, 2, 7, 8, 9]), 1);
        assert_eq!(num_unsafe(&vec![9, 7, 6, 2, 1]), 1);
        assert_eq!(num_unsafe(&vec![1, 3, 2, 4, 5]), 1);
        assert_eq!(num_unsafe(&vec![8, 6, 4, 4, 1]), 1);
        assert_eq!(num_unsafe(&vec![1, 3, 6, 7, 9]), 0);
        assert_eq!(num_unsafe(&vec![1, 1, 6, 7, 9]), 2);
    }

    #[test]
    fn test_is_safe() {
        assert_eq!(is_safe(&vec![]), true);
        assert_eq!(is_safe(&vec![1]), true);
        assert_eq!(is_safe(&vec![1, 2]), true);
        assert_eq!(is_safe(&vec![2, 1]), true);
        assert_eq!(is_safe(&vec![1, 1]), false);
        assert_eq!(is_safe(&vec![1, 5]), false);
        assert_eq!(is_safe(&vec![5, 1]), false);
        assert_eq!(is_safe(&vec![89, 92, 95, 96, 97]), true);
        assert_eq!(is_safe(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_safe(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(is_safe(&vec![1, 3, 6, 7, 9]), true);
        assert_eq!(is_safe(&vec![1, 1, 6, 7, 9]), false);
    }
}

