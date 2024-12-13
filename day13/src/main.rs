use std::{fs, time::Instant};

fn solve_part_1(input: &String) -> i32
{
    return input
        .replace("\r\n", "\n")
        .replace("\r", "\n")
        .replace(" ", "")
        .split("\n\n")
        .map(|machine| machine.lines())
        .filter_map(|mut lines| match (lines.next(), lines.next(), lines.next()) {
            (Some(a), Some(b), Some(p)) => {
                let a_comma_pos = a.chars().position(|c| c == ',').unwrap();
                let b_comma_pos = b.chars().position(|c| c == ',').unwrap();
                let p_comma_pos = p.chars().position(|c| c == ',').unwrap();

                let ax = a[10..a_comma_pos].parse::<i32>().unwrap();
                let ay = a[(a_comma_pos + 3)..].parse::<i32>().unwrap();

                let bx = b[10..b_comma_pos].parse::<i32>().unwrap();
                let by = b[(b_comma_pos + 3)..].parse::<i32>().unwrap();

                let px = p[8..p_comma_pos].parse::<i32>().unwrap();
                let py = p[(p_comma_pos + 3)..].parse::<i32>().unwrap();

                return Some(((ax, ay), (bx, by), (px, py)));
            },
            _ => None
        })
        .filter_map(|((ax, ay), (bx, by), (px, py))| {
            let b = ((ax * py) - (px * ay)) / ((ax * by) - (bx * ay));
            let a = (px - (b * bx)) / ax;
            let solvable = (a * ax + b * bx == px) && (a * ay + b * by == py);
            if solvable {
                return Some(3 * a + b);
            }
            else {
                return None;
            }
        })
        .sum();
}

fn solve_part_2(input: &String) -> i128
{
    return input
        .replace("\r\n", "\n")
        .replace("\r", "\n")
        .replace(" ", "")
        .split("\n\n")
        .map(|machine| machine.lines())
        .filter_map(|mut lines| match (lines.next(), lines.next(), lines.next()) {
            (Some(a), Some(b), Some(p)) => {
                let a_comma_pos = a.chars().position(|c| c == ',').unwrap();
                let b_comma_pos = b.chars().position(|c| c == ',').unwrap();
                let p_comma_pos = p.chars().position(|c| c == ',').unwrap();

                let ax = a[10..a_comma_pos].parse::<i128>().unwrap();
                let ay = a[(a_comma_pos + 3)..].parse::<i128>().unwrap();

                let bx = b[10..b_comma_pos].parse::<i128>().unwrap();
                let by = b[(b_comma_pos + 3)..].parse::<i128>().unwrap();

                let px = p[8..p_comma_pos].parse::<i128>().unwrap() + 10000000000000;
                let py = p[(p_comma_pos + 3)..].parse::<i128>().unwrap() + 10000000000000;

                return Some(((ax, ay), (bx, by), (px, py)));
            },
            _ => None
        })
        .filter_map(|((ax, ay), (bx, by), (px, py))| {
            let b = ((ax * py) - (px * ay)) / ((ax * by) - (bx * ay));
            let a = (px - (b * bx)) / ax;
            let solvable = (a * ax + b * bx == px) && (a * ay + b * by == py);
            if solvable {
                return Some(3 * a + b);
            }
            else {
                return None;
            }
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
