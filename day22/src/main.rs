use std::{collections::{HashMap, HashSet, VecDeque}, fs, time::Instant};

fn solve_part_1(input: &String) -> i64
{
    let iter_count = 2000;

    let mut seeds: Vec<_> = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse::<i64>().ok())
        .collect();

    for _ in 0..iter_count {
        for seed in seeds.iter_mut() {
            let mut n = *seed;
            n ^= n * 64;
            n %= 16777216;
            n ^= n / 32;
            n %= 16777216;
            n ^= n * 2048;
            n %= 16777216;
            *seed = n;
        }
    }

    return seeds.iter().sum();
}

fn solve_part_2(input: &String) -> i64
{
    let iter_count = 2000;

    let seeds: Vec<_> = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse::<i64>().ok())
        .collect();
    let mut rng_state = seeds;

    let mut data = vec!();
    let mut last_prices = vec!();
    for _ in 0..rng_state.len() {
        data.push(HashMap::new());
        last_prices.push(VecDeque::new());
    }
    for _ in 0..iter_count {
        for (i, seed) in rng_state.iter_mut().enumerate() {
            let mut n = *seed;

            let price = n % 10;
            let last_price_buf: &mut VecDeque<i64> = last_prices.get_mut(i).unwrap();
            if last_price_buf.len() == 5 {
                let mut last_price = last_price_buf.pop_front().unwrap();
                let mut change_seq = vec!();
                for price in last_price_buf.iter() {
                    let change = *price - last_price;
                    change_seq.push(change);
                    last_price = *price;
                }

                let map = data.get_mut(i).unwrap();
                if !map.contains_key(&change_seq) {
                    map.insert(change_seq, last_price);
                }
            }
            last_price_buf.push_back(price);
            
            n ^= n * 64;
            n %= 16777216;
            n ^= n / 32;
            n %= 16777216;
            n ^= n * 2048;
            n %= 16777216;
            *seed = n;
        }
    }

    let highest: HashSet<_> = data.iter()
        .filter_map(|v| v.iter()
            .map(|(_, price)| *price)
            .max())
        .collect();

    let candidates: HashSet<_> = data.iter()
        .flat_map(|v| v)
        .filter(|(_, price)| highest.contains(price))
        .map(|(changes, _)| changes.clone())
        .collect();

    let mut best_res = 0;
    for candidate in candidates {
        let res = data.iter()
            .filter_map(|v| v.get(&candidate))
            .sum::<i64>();

        if res > best_res {
            best_res = res;
        }
    }

    return best_res;
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
