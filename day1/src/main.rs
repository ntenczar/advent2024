use std::fs;

static FILE_PATH: &str = "input";

fn main() {
    println!("In file {FILE_PATH}");

    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in contents.lines().into_iter() {
        let split: Vec<&str> = line.split("   ").collect();
        left_list.push(split[0].parse::<i32>().unwrap());
        right_list.push(split[1].parse::<i32>().unwrap());
    }
    left_list.sort();
    right_list.sort();

    part_one(&left_list, &right_list);
    part_two(&left_list, &right_list);
}

fn part_one(left_list: &Vec<i32>, right_list: &Vec<i32>) {
    let mut diff = 0;
    for i in 0..left_list.len() {
        let current_diff: i32 = (left_list[i] - right_list[i]).abs();
        diff = diff + current_diff;
    }
    println!("Part one diffs: {}", diff);
}

fn part_two(left_list: &Vec<i32>, right_list: &Vec<i32>) {
    let mut similarity = 0;
    for v in left_list {
        let appearances = right_list
            .into_iter()
            .clone()
            .filter(|&rl_v| rl_v == v)
            .count();
        let diff = v * appearances as i32;
        similarity += diff;
    }
    println!("Part two similarity: {}", similarity);
}
