
use std::{fs, collections::HashMap};

fn solve_part_1(input: &String) -> u32
{
    let unsorted: Vec<_> = input.lines()
        .map(|line| line.split_ascii_whitespace())
        .filter_map(|mut x| match (x.next(), x.next()) {
            (Some(a), Some(b)) => Some((a.parse::<i32>(), b.parse::<i32>())),
            _ => None
        })
        .filter_map(|x| match x {
            (Ok(a), Ok(b)) => Some((a, b)),
            _ => None
        })
        .collect();

    let mut left: Vec<_> = unsorted
        .iter()
        .map(|(l, _)| *l)
        .collect();
    left.sort();

    let mut right: Vec<_> = unsorted
        .iter()
        .map(|(_, r)| *r)
        .collect();
    right.sort();

    return left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .reduce(|a, b| a + b)
        .unwrap_or(0);
        
    
}

fn solve_part_2(input: &String) -> i32
{
    let original: Vec<_> = input.lines()
        .map(|line| line.split_ascii_whitespace())
        .filter_map(|mut x| match (x.next(), x.next()) {
            (Some(a), Some(b)) => Some((a.parse::<i32>(), b.parse::<i32>())),
            _ => None
        })
        .filter_map(|x| match x {
            (Ok(a), Ok(b)) => Some((a, b)),
            _ => None
        })
        .collect();

    let freq = original
        .iter()
        .map(|(_, r)| r)
        .fold(HashMap::new(), |mut map, v|{
            map.entry(v)
               .and_modify(|freq| *freq += 1)
               .or_insert(1);
            return map;
        });

    return original
        .iter()
        .map(|(l, _)| l)
        .map(|v| v * freq.get(v).unwrap_or(&0))
        .reduce(|a, b| a + b)
        .unwrap_or(0);
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
