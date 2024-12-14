use std::{collections::HashSet, fs, time::Instant};

fn solve_part_1(input: &String) -> i32
{
    let t = 100;
    let (w, h) = (101, 103);

    let w_half = w / 2;
    let h_half = h / 2;

    let (q1, q2, q3, q4) = input.lines()
        .map(|line| {
            let mut comma_split = line.split(",");
            let (s1, s2_3, s4) = (
                comma_split.next().unwrap(),
                comma_split.next().unwrap(),
                comma_split.next().unwrap()
            );
            let mut space_split = s2_3.split_ascii_whitespace();
            let (s2, s3) = (
                space_split.next().unwrap(),
                space_split.next().unwrap()
            );
            let px = s1[2..].parse::<i32>().unwrap();
            let py = s2.parse::<i32>().unwrap();
            let vx = s3[2..].parse::<i32>().unwrap();
            let vy = s4.parse::<i32>().unwrap();

            let x = (px + (vx * t)).rem_euclid(w);
            let y = (py + (vy * t)).rem_euclid(h);
            
            return (x, y);
        })
        .fold((0, 0, 0, 0), |(mut q1, mut q2, mut q3, mut q4), (x, y)| {
            if x < w_half && y < h_half {
                q1 += 1;
            }
            else if x > w_half && y < h_half {
                q2 += 1;
            }
            else if x < w_half && y > h_half {
                q3 += 1;
            }
            else if x > w_half && y > h_half {
                q4 += 1;
            }
            return (q1, q2, q3, q4);
        });

    return q1 * q2 * q3 * q4;
}

fn solve_part_2(input: &String) -> i32
{
    let (w, h) = (101, 103);

    let robots: Vec<_> = input.lines()
        .map(|line| {
            let mut comma_split = line.split(",");
            let (s1, s2_3, s4) = (
                comma_split.next().unwrap(),
                comma_split.next().unwrap(),
                comma_split.next().unwrap()
            );
            let mut space_split = s2_3.split_ascii_whitespace();
            let (s2, s3) = (
                space_split.next().unwrap(),
                space_split.next().unwrap()
            );
            let px = s1[2..].parse::<i32>().unwrap();
            let py = s2.parse::<i32>().unwrap();
            let vx = s3[2..].parse::<i32>().unwrap();
            let vy = s4.parse::<i32>().unwrap();

            return ((px, py), (vx, vy));
        })
        .collect();

    let mut steps = 0;
    loop {
        let mut unvisited: HashSet<_> = robots.iter()
            .map(|((px, py), (vx, vy))| {
                let x = (px + (vx * steps)).rem_euclid(w);
                let y = (py + (vy * steps)).rem_euclid(h);
                return (x, y)
            })
            .collect();
        
        let total = unvisited.len();
        let total_half = total / 2;
        while let Some(&pos) = unvisited.iter().next() {
            let mut count = 0;
            let mut stack = vec!(pos);
            while let Some(pos) = stack.pop() {
                unvisited.remove(&pos);
                count += 1;

                let (x, y) = pos;
                [
                    (x + 1, y),
                    (x - 1, y),
                    (x, y + 1),
                    (x, y - 1)
                ].into_iter()
                    .filter(|pos| unvisited.contains(pos))
                    .for_each(|pos| stack.push(pos));
            }

            if count > total_half {
                return steps;
            }
        }

        steps += 1;
    }
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
