
use std::{collections::HashSet, collections::HashMap, fs, time::Instant};

fn solve_part_1(input: &String) -> usize
{
    let h = input.lines().count();
    let (w, antennas) = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter_map(move |(x, c)| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Some((c, (x, y))),
                _ => None
            })
            .map(|antenna| (line.len(), antenna)))
        .fold((0, HashMap::<char, Vec<_>>::new()), |(max_width, mut antennas), (width, antenna)| {
            let (freq, (x, y)) = antenna;
            let pos = (x as isize, y as isize);
            match antennas.get_mut(&freq) {
                Some(pos_vec) => pos_vec.push(pos),
                None => {antennas.insert(freq, vec!(pos));}
            }
            return (max_width.max(width), antennas);
        });

    let iw = w as isize;
    let ih = h as isize;
    let antinodes: HashSet<_> = antennas.iter()
        .flat_map(|(_, pos_vec)| pos_vec.iter()
            .flat_map(|pos1| pos_vec.iter().map(move |pos2| (pos1, pos2)))
            .filter(|(pos1, pos2)| pos1 != pos2)
            .flat_map(|((x1, y1), (x2, y2))| {
                let dx = x2 - x1;
                let dy = y2 - y1;
                return [(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)];
            })
            .filter(|(x, y)| (*x >= 0) && (*y >= 0) && (*x < iw) && (*y < ih)))
        .collect();

    return antinodes.len();
}

fn solve_part_2(input: &String) -> usize
{
    let h = input.lines().count();
    let (w, antennas) = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter_map(move |(x, c)| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Some((c, (x, y))),
                _ => None
            })
            .map(|antenna| (line.len(), antenna)))
        .fold((0, HashMap::<char, Vec<_>>::new()), |(max_width, mut antennas), (width, antenna)| {
            let (freq, (x, y)) = antenna;
            let pos = (x as isize, y as isize);
            match antennas.get_mut(&freq) {
                Some(pos_vec) => pos_vec.push(pos),
                None => {antennas.insert(freq, vec!(pos));}
            }
            return (max_width.max(width), antennas);
        });

    let iw = w as isize;
    let ih = h as isize;
    let is = w.max(h) as isize;
    let antinodes: HashSet<_> = antennas.iter()
        .flat_map(|(_, pos_vec)| pos_vec.iter()
            .flat_map(|pos1| pos_vec.iter().map(move |pos2| (pos1, pos2)))
            .filter(|(pos1, pos2)| pos1 != pos2)
            .flat_map(|((x1, y1), (x2, y2))| {
                let dx = x2 - x1;
                let dy = y2 - y1;
                return ((1 - is)..is).map(move |k| (x1 + (k * dx), y1 + (k * dy)));
            })
            .filter(|(x, y)| (*x >= 0) && (*y >= 0) && (*x < iw) && (*y < ih)))
        .collect();

    return antinodes.len();
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
