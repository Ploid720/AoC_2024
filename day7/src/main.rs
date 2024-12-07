
use std::{collections::HashSet, fs, time::Instant};

fn solve_part_1(input: &String) -> i64
{
    return input.lines()
        .map(|line| line.split(":"))
        .filter_map(|mut line| match (line.next(), line.next()) {
            (Some(target), Some(nums)) => Some((target.parse::<i64>(), nums
                .split_ascii_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect::<Vec<_>>())),
            _ => None
        })
        .filter_map(|(target, nums)| match target {
            Ok(target) => Some((target, nums)),
            _ => None
        })
        .filter(|(target, nums)| nums.iter()
            .fold(HashSet::new(), |mut curr_vals, num| {
                if curr_vals.is_empty() {
                    curr_vals.insert(*num);
                    return curr_vals;
                }
                let next_vals: Vec<_> = curr_vals.iter()
                    .flat_map(|val| vec!(*val + num, *val * num))
                    .filter(|val| val <= target)
                    .collect();
                curr_vals.clear();
                next_vals.iter().for_each(|val| {curr_vals.insert(*val);});
                return curr_vals;
            })
            .iter()
            .any(|num| num == target))
        .map(|(target, _)| target)
        .sum();
}

fn solve_part_2(input: &String) -> i64
{
    return input.lines()
        .map(|line| line.split(":"))
        .filter_map(|mut line| match (line.next(), line.next()) {
            (Some(target), Some(nums)) => Some((target.parse::<i64>(), nums
                .split_ascii_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect::<Vec<_>>())),
            _ => None
        })
        .filter_map(|(target, nums)| match target {
            Ok(target) => Some((target, nums)),
            _ => None
        })
        .filter(|(target, nums)| nums.iter()
            .fold(HashSet::new(), |mut curr_vals, num| {
                if curr_vals.is_empty() {
                    curr_vals.insert(*num);
                    return curr_vals;
                }
                let next_vals: Vec<_> = curr_vals.iter()
                    .flat_map(|val| vec!(*val + num, *val * num, *val * (10_i64.pow(num.checked_ilog10().unwrap_or(0) + 1)) + *num))
                    .filter(|val| val <= target)
                    .collect();
                curr_vals.clear();
                next_vals.iter().for_each(|val| {curr_vals.insert(*val);});
                return curr_vals;
            })
            .iter()
            .any(|num| num == target))
        .map(|(target, _)| target)
        .sum();
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let inst1 = Instant::now();
    let part_1_res = solve_part_1(&content);
    let t1 = inst1.elapsed();
    let inst2 = Instant::now();
    let part_2_res = solve_part_2(&content);
    let t2 = inst2.elapsed();

    println!("Part 1 result: {}, solved in {} ms", part_1_res, t1.as_secs_f64() * 1000.0);
    println!("Part 2 result: {}, solved in {} ms", part_2_res, t2.as_secs_f64() * 1000.0);
}
