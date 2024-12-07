
use std::{collections::{HashMap, HashSet}, fs, time::Instant};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn solve_part_1(input: &String) -> usize
{
    let (map, start) = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .map(move |(x, c)| ((x, y), c)))
        .fold((HashMap::new(), None), |(mut map, start), ((x, y), c)| {
            let pos = (x as isize, y as isize);
            map.insert(pos, c);
            match c {
                '^' => (map, Some(pos)),
                _ => (map, start)
            }
        });

    type Dir = Direction;
    fn next_direction(dir: Dir) -> Dir {
        match dir {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up
        }
    }
    fn next_position(pos: &(isize, isize), dir: &Dir) -> (isize, isize) {
        let (x, y) = pos;
        match dir {
            Dir::Up => (*x, y - 1),
            Dir::Right => (x + 1, *y),
            Dir::Down => (*x, y + 1),
            Dir::Left => (x - 1, *y)
        }
    }

    let mut pos = start.expect("Input should include starting position");
    let mut dir = Dir::Up;

    let mut visited = HashSet::new();

    loop {
        visited.insert(pos);
        let next_pos = next_position(&pos, &dir);
        match map.get(&next_pos) {
            Some('#') => dir = next_direction(dir),
            Some(_) => pos = next_pos,
            None => break
        }
    }

    return visited.len();
}

fn solve_part_2(input: &String) -> usize
{
    let (map, start) = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .map(move |(x, c)| ((x, y), c)))
        .fold((HashMap::new(), None), |(mut map, start), ((x, y), c)| {
            let pos = (x as isize, y as isize);
            map.insert(pos, c);
            match c {
                '^' => (map, Some(pos)),
                _ => (map, start)
            }
        });

    type Dir = Direction;
    fn next_direction(dir: Dir) -> Dir {
        match dir {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up
        }
    }
    fn next_position(pos: &(isize, isize), dir: &Dir) -> (isize, isize) {
        let (x, y) = pos;
        match dir {
            Dir::Up => (*x, y - 1),
            Dir::Right => (x + 1, *y),
            Dir::Down => (*x, y + 1),
            Dir::Left => (x - 1, *y)
        }
    }

    let start_pos = start.expect("Input should include starting position");

    let mut pos = start_pos;
    let mut dir = Dir::Up;
    let mut base_path = HashSet::new();

    loop {
        base_path.insert(pos);
        let next_pos = next_position(&pos, &dir);
        match map.get(&next_pos) {
            Some('#') => dir = next_direction(dir),
            Some(_) => pos = next_pos,
            None => break
        }
    }

    let mut res = 0;

    for obstr_pos in map.keys()
    {
        if *obstr_pos == start_pos
            || !base_path.contains(obstr_pos) {
            continue;
        }

        let mut pos_fast = start_pos;
        let mut dir_fast = Dir::Up;
        let mut pos_slow = start_pos;
        let mut dir_slow = Dir::Up;

        let mut found_loop = false;
        let mut update_slow = false;

        loop {
            let next_pos_fast = next_position(&pos_fast, &dir_fast);
            match map.get(&next_pos_fast) {
                Some(c) if *c == '#' || next_pos_fast == *obstr_pos => dir_fast = next_direction(dir_fast),
                Some(_) => pos_fast = next_pos_fast,
                None => break
            }

            if update_slow {
                let next_pos_slow = next_position(&pos_slow, &dir_slow);
                match map.get(&next_pos_slow) {
                    Some(c) if *c == '#' || next_pos_slow == *obstr_pos => dir_slow = next_direction(dir_slow),
                    Some(_) => pos_slow = next_pos_slow,
                    None => break
                }

                if (pos_fast, dir_fast) == (pos_slow, dir_slow) {
                    found_loop = true;
                    break;
                }
            }
            update_slow = !update_slow;
        }

        if found_loop {
            res += 1;
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
    let inst2 = Instant::now();
    let part_2_res = solve_part_2(&content);
    let t2 = inst2.elapsed();

    println!("Part 1 result: {}, solved in {} ms", part_1_res, t1.as_secs_f64() * 1000.0);
    println!("Part 2 result: {}, solved in {} ms", part_2_res, t2.as_secs_f64() * 1000.0);
}
