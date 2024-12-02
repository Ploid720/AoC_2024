
use std::{fs, iter};

fn solve_part_1(input: &String) -> usize
{
    return input.lines()
        .map(|line| line.split_ascii_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>())
        .filter_map(|arr| arr.iter()
            .zip(arr.iter().skip(1))
            .map(|(a, b)| a - b)
            .fold(None, |curr, diff| match curr {
                None => {
                    let dist = diff.abs();
                    let dir = diff.signum();
                    if (dist >= 1) && (dist <= 3) {
                        return Some((dir, true));
                    }
                    return Some((dir, false));
                },
                Some((_, false)) => Some((0, false)),
                Some((last_dir, true)) => {
                    let dist = diff.abs();
                    let dir = diff.signum();
                    if (dist >= 1) && (dist <= 3) && (dir == last_dir) {
                        return Some((dir, true));
                    }
                    return Some((dir, false));
                }
            }))
        .filter(|(_, safe)| *safe)
        .count();
}

fn solve_part_2(input: &String) -> usize
{
    return input.lines()
        .map(|line| line.split_ascii_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>())
        .filter(|base_arr| (0..base_arr.len())
            .map(|ind_to_remove| base_arr
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != ind_to_remove)
                .map(|(_, x)| *x)
                .collect())
            .chain(iter::once(base_arr.clone()))
            .any(|arr| arr.iter()
                .zip(arr.iter().skip(1))
                .map(|(a, b)| a - b)
                .fold(None, |curr, diff| match curr {
                    None => {
                        let dist = diff.abs();
                        let dir = diff.signum();
                        if (dist >= 1) && (dist <= 3) {
                            return Some((dir, true));
                        }
                        return Some((dir, false));
                    },
                    Some((_, false)) => Some((0, false)),
                    Some((last_dir, true)) => {
                        let dist = diff.abs();
                        let dir = diff.signum();
                        if (dist >= 1) && (dist <= 3) && (dir == last_dir) {
                            return Some((dir, true));
                        }
                        return Some((dir, false));
                    }
                })
                .map(|(_, x)| x)
                .unwrap_or(false)))
        .count();
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let part_1_res = solve_part_1(&content);
    let part_2_res = solve_part_2(&content);

    println!("Part 1 result: {}", part_1_res);
    println!("Part 2 result: {}", part_2_res);
}
