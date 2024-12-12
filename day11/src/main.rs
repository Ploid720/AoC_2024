use std::{fs, collections::HashMap, time::Instant};

fn solve(pebbles: Vec<u64>, blink_count: u32) -> usize
{
    let mut cache = HashMap::new();

    fn expand(pebble: u64, depth: u32, depth_limit: u32, cache: &mut HashMap<(u64, u32), usize>) -> usize {
        if let Some(res) = cache.get(&(pebble, depth)) {
            return *res;
        }
        
        if depth >= depth_limit {
            return 1;
        }

        if pebble == 0 {
            let res = expand(2024, depth + 2, depth_limit, cache);
            cache.insert((pebble, depth), res);
            return res;
        }
        let str = pebble.to_string();
        let l = str.len();
        if (l & 1) == 0 {
            let p1 = str[0..(l / 2)].parse::<u64>().unwrap_or(0);
            let p2 = str[(l / 2)..l].parse::<u64>().unwrap_or(0);
            let res = expand(p1, depth + 1, depth_limit, cache)
                + expand(p2, depth + 1, depth_limit, cache);
            cache.insert((pebble, depth), res);  
            return res;
        }

        let res = expand(pebble * 2024, depth + 1, depth_limit, cache);
        cache.insert((pebble, depth), res); 
        return res;
    }

    return pebbles.iter()
        .map(|pebble| expand(*pebble, 0, blink_count, &mut cache))
        .sum();
}

fn solve_part_1(input: &String) -> usize
{
    let pebbles: Vec<_> = input.split_ascii_whitespace()
    .flat_map(|s| s.parse::<u64>().ok())
    .collect();

    return solve(pebbles, 25);
}

fn solve_part_2(input: &String) -> usize
{
    let pebbles: Vec<_> = input.split_ascii_whitespace()
        .flat_map(|s| s.parse::<u64>().ok())
        .collect();

    return solve(pebbles, 75);
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
