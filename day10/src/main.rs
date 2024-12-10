use std::{fs, collections::HashSet, time::Instant};

fn solve_part_1(input: &String) -> usize
{
    let map: Vec<Vec<_>> = input.lines()
        .map(|line| line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect())
        .collect();

    let h = map.len();
    let w = map.iter()
        .map(|row| row.len())
        .max()
        .unwrap_or(0);

    let (iw, ih) = (w as isize, h as isize);

    return map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter()
            .enumerate()
            .filter(|(_, v)| **v == 0)
            .map(move |(x, _)| (x as isize, y as isize)))
        .map(|trailhead| {
            let mut points = HashSet::new();
            points.insert(trailhead);
            for next_level in 1..=9 {
                points = points.iter()
                    .flat_map(|(x, y)| [
                        (x + 1, *y),
                        (x - 1, *y),
                        (*x, y + 1),
                        (*x, y - 1)
                    ])
                    .filter(|(x, y)| (*x >= 0) && (*y >= 0))
                    .filter(|(x, y)| (*x < iw) && (*y < ih))
                    .filter(|(x, y)| map[*y as usize][*x as usize] == next_level)
                    .collect();
            }
            return points.len();
        })
        .sum();
}

fn solve_part_2(input: &String) -> usize
{
    let map: Vec<Vec<_>> = input.lines()
        .map(|line| line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect())
        .collect();

    let h = map.len();
    let w = map.iter()
        .map(|row| row.len())
        .max()
        .unwrap_or(0);

    let (iw, ih) = (w as isize, h as isize);

    return map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter()
            .enumerate()
            .filter(|(_, v)| **v == 0)
            .map(move |(x, _)| (x as isize, y as isize)))
        .map(|trailhead| {
            let mut res = 0;
            let mut stack = vec!((trailhead, 0));
            while !stack.is_empty() {
                let ((x, y), level) = stack.pop().unwrap();
                if level == 9 {
                    res += 1;
                    continue;
                }
                let next_level = level + 1;
                [
                    (x + 1, y),
                    (x - 1, y),
                    (x, y + 1),
                    (x, y - 1)
                ].iter()
                    .filter(|(x, y)| (*x >= 0) && (*y >= 0))
                    .filter(|(x, y)| (*x < iw) && (*y < ih))
                    .filter(|(x, y)| map[*y as usize][*x as usize] == next_level)
                    .for_each(|p| stack.push((*p, next_level)));
            }
            return res;
        })
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
