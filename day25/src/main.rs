use std::{fs, time::Instant};

fn solve_part_1(input: &String) -> usize
{
    let input: Vec<Vec<Vec<_>>> = input
        .replace("\r\n", "\n")
        .replace("\r", "\n")
        .split("\n\n")
        .map(|sch| sch.lines()
            .map(|line| line.chars().collect())
            .collect())
        .collect();

    let height = input[0].len();

    let input: Vec<(bool, Vec<_>)> = input
        .iter()
        .map(|map| (map[0].iter().all(|c| *c == '#'), map))
        .map(|(is_lock, map)| (is_lock, (0..map[0].len())
            .map(|x| (0..map.len())
                .filter(|y| map[*y][x] == '#')
                .count())
            .collect()))
        .collect();

    let locks: Vec<_> = input.iter()
        .filter(|(is_lock, _)| *is_lock)
        .map(|(_, lengths)| lengths)
        .collect();

    let keys: Vec<_> = input.iter()
        .filter(|(is_lock, _)| !*is_lock)
        .map(|(_, lengths)| lengths)
        .collect();

    let mut res = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.iter()
                .enumerate()
                .all(|(x, v)| v + key[x] <= height) {
                res += 1;
            }
        }
    }

    return res;
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let inst1 = Instant::now();
    let part_1_res = solve_part_1(&content);
    let t1 = inst1.elapsed();

    println!("Part 1 result: {}, solved in {} ms", part_1_res, t1.as_secs_f64() * 1000.0);
}
