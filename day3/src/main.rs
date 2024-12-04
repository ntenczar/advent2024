use regex::Regex;
use std::fs;

static FILE_PATH: &str = "input";

fn main() {
    println!("In file {FILE_PATH}");

    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    println!("Part one sum: {}", part_one(&contents));
    println!("Part two sum: {}", part_two(&contents));
}

fn part_one(input: &str) -> usize {
    let input_re = Regex::new(r"(mul\([0-9]*,[0-9]*\))").unwrap();
    let match_nums_re = Regex::new(r"([0-9])+").unwrap();
    let matches: Vec<_> = input_re.find_iter(input).map(|m| m.as_str()).collect();
    let mut sum = 0;
    for m in matches.iter() {
        let vals: Vec<usize> = match_nums_re
            .find_iter(m)
            .map(|m| m.as_str().parse::<usize>().unwrap())
            .collect();
        sum = sum + (vals[0] * vals[1]);
    }
    return sum;
}

fn part_two(input: &str) -> usize {
    let input_re = Regex::new(r"(mul\([0-9]*,[0-9]*\))|(don't)|(do)").unwrap();
    let match_nums_re = Regex::new(r"([0-9])+").unwrap();
    let matches: Vec<&str> = input_re.find_iter(input).map(|m| m.as_str()).collect();
    let mut sum = 0;
    let mut should_mult = true;
    for m in matches.iter() {
        match *m {
            "do" => should_mult = true,
            "don't" => should_mult = false,
            _ => {
                if should_mult {
                    let vals: Vec<usize> = match_nums_re
                        .find_iter(m)
                        .map(|m| m.as_str().parse::<usize>().unwrap())
                        .collect();
                    sum = sum + (vals[0] * vals[1]);
                }
            }
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("mul(1,1)"), 1);
        assert_eq!(part_one("mul(1,1)mul(2,2)"), 5);
        assert_eq!(part_one("mul(1,1)mul(2,2)asdmfasd"), 5);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mul(1,1)"), 1);
        assert_eq!(part_two("don'tmul(1,1)"), 0);
        assert_eq!(part_two("don'tmul(1,1)domul(1,1)"), 1);
        assert_eq!(part_two("don'tmul(1,1)domul(1,1)don'tmul(2,2)"), 1);
    }
}
