use std::{collections::{HashMap, HashSet}, fs, time::Instant};

fn solve_part_1(input: &String) -> usize
{
    let input = input
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    let mut inputs = input
        .split("\n\n");

    let mut available: Vec<_> = inputs.next()
        .unwrap()
        .split(",")
        .map(|pattern| pattern.trim())
        .collect();
    available.sort_by(|a, b| a.len().cmp(&b.len()));

    return inputs.next()
        .unwrap()
        .lines()
        .map(|target| target.trim())
        .filter(|target| {
            let target_len = target.len();
            let mut visited = HashSet::new();
            let mut stack = vec!(0);
            while let Some(pos) = stack.pop() {
                if visited.contains(&pos) {
                    continue;
                }
                visited.insert(pos);

                if pos == target_len {
                    return true;
                }

                let remaining = &target[pos..];
                available.iter()
                    .filter(|pattern| remaining.starts_with(*pattern))
                    .map(|pattern| pos + pattern.len())
                    .filter(|new_pos| !visited.contains(new_pos))
                    .for_each(|new_pos| stack.push(new_pos));
            };

            return false;
        })
        .count();
}

fn solve_part_2(input: &String) -> usize
{
    let input = input
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    let mut inputs = input
        .split("\n\n");

    let mut available: Vec<_> = inputs.next()
        .unwrap()
        .split(",")
        .map(|pattern| pattern.trim())
        .filter(|pattern| !pattern.is_empty())
        .collect();
    available.sort_by(|a, b| a.len().cmp(&b.len()));

    fn solve(target: &str, patterns: &Vec<&str>, pos: usize, cache: &mut HashMap<usize, usize>) -> usize {
        if let Some(res) = cache.get(&pos) {
            return *res;
        }

        if pos == target.len() {
            return 1;
        }

        let remaining = &target[pos..];
        let res = patterns.iter()
            .filter(|pattern| remaining.starts_with(*pattern))
            .map(|pattern| solve(target, patterns, pos + pattern.len(), cache))
            .sum();
        cache.insert(pos, res);

        return res;
    }

    return inputs.next()
        .unwrap()
        .lines()
        .map(|target| target.trim())
        .map(|target| solve(target, &available, 0, &mut HashMap::new()))
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
