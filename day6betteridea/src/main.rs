
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

    let w = map.keys().map(|(x, _)| x).max().unwrap_or(&0) + 1;
    let h = map.keys().map(|(_, y)| y).max().unwrap_or(&0) + 1;

    let map_ref = &map;
    let left: HashMap<_, _> = (0..h)
        .flat_map(|y| (0..w).scan(None, move |curr, x| {
            let pos = (x, y);
            if map_ref.get(&pos) == Some(&'#') {
                *curr = Some(pos);
            }
            return Some(((x, y), curr.map(|(cx, cy)| (cx + 1, cy))));
        }))
        .collect();
    let right: HashMap<_, _> = (0..h)
        .flat_map(|y| (0..w).rev().scan(None, move |curr, x| {
            let pos = (x, y);
            if map_ref.get(&pos) == Some(&'#') {
                *curr = Some(pos);
            }
            return Some(((x, y), curr.map(|(cx, cy)| (cx - 1, cy))));
        }))
        .collect();
    let up: HashMap<_, _> = (0..w)
        .flat_map(|x| (0..h).scan(None, move |curr, y| {
            let pos = (x, y);
            if map_ref.get(&pos) == Some(&'#') {
                *curr = Some(pos);
            }
            return Some(((x, y), curr.map(|(cx, cy)| (cx, cy + 1))));
        }))
        .collect();
    let down: HashMap<_, _> = (0..w)
        .flat_map(|x| (0..h).rev().scan(None, move |curr, y| {
            let pos = (x, y);
            if map_ref.get(&pos) == Some(&'#') {
                *curr = Some(pos);
            }
            return Some(((x, y), curr.map(|(cx, cy)| (cx, cy - 1))));
        }))
        .collect();

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
    let mut res = 0;

    let mut pos = start.expect("Input should include starting position");
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

    for obstr_pos in map.keys()
    {
        if *obstr_pos == start_pos
            || !base_path.contains(obstr_pos) {
            continue;
        }
        let (ox, oy) = *obstr_pos;

        let ob_left: HashSet<_> = ((ox + 1)..w)
            .filter(|x| match left.get(&(*x, oy)) {
                Some(Some((tx, _))) => *tx <= ox,
                _ => true
            })
            .map(|x|  (x, oy))
            .collect();
        let ob_right: HashSet<_> = (0..ox)
            .filter(|x| match right.get(&(*x, oy)) {
                Some(Some((tx, _))) => *tx >= ox,
                _ => true
            })
            .map(|x|  (x, oy))
            .collect();
        let ob_up: HashSet<_> = ((oy + 1)..h)
            .filter(|y| match up.get(&(ox, *y)) {
                Some(Some((_, ty))) => *ty <= oy,
                _ => true
            })
            .map(|y|  (ox, y))
            .collect();
        let ob_down: HashSet<_> = (0..oy)
            .filter(|y| match down.get(&(ox, *y)) {
                Some(Some((_, ty))) => *ty >= oy,
                _ => true
            })
            .map(|y|  (ox, y))
            .collect();

        let mut pos_fast = start_pos;
        let mut dir_fast = Dir::Up;
        let mut pos_slow = start_pos;
        let mut dir_slow = Dir::Up;

        let mut found_loop = false;
        let mut update_slow = false;

        loop {
            let in_ob_path_fast = (match dir_fast {
                Dir::Up    => &ob_up,
                Dir::Down  => &ob_down,
                Dir::Left  => &ob_left,
                Dir::Right => &ob_right,
            }).contains(&pos_fast);
            let next_pos_fast = if in_ob_path_fast {
                match dir_fast {
                    Dir::Up    => Some((ox, oy + 1)),
                    Dir::Down  => Some((ox, oy - 1)),
                    Dir::Left  => Some((ox + 1, oy)),
                    Dir::Right => Some((ox - 1, oy)),
                }
            }
            else {
                *(match dir_fast {
                    Dir::Up    => up.get(&pos_fast),
                    Dir::Down  => down.get(&pos_fast),
                    Dir::Left  => left.get(&pos_fast),
                    Dir::Right => right.get(&pos_fast),
                }).unwrap_or(&None)
            };

            match next_pos_fast {
                Some(next_pos_fast) => {
                    pos_fast = next_pos_fast;
                    dir_fast = next_direction(dir_fast)
                },
                None => break
            }

            if update_slow {
                let in_ob_path_slow = (match dir_slow {
                    Dir::Up    => &ob_up,
                    Dir::Down  => &ob_down,
                    Dir::Left  => &ob_left,
                    Dir::Right => &ob_right,
                }).contains(&pos_slow);
                let next_pos_slow = if in_ob_path_slow {
                    match dir_slow {
                        Dir::Up    => Some((ox, oy + 1)),
                        Dir::Down  => Some((ox, oy - 1)),
                        Dir::Left  => Some((ox + 1, oy)),
                        Dir::Right => Some((ox - 1, oy)),
                    }
                }
                else {
                    *(match dir_slow {
                        Dir::Up    => up.get(&pos_slow),
                        Dir::Down  => down.get(&pos_slow),
                        Dir::Left  => left.get(&pos_slow),
                        Dir::Right => right.get(&pos_slow),
                    }).unwrap_or(&None)
                };
    
                match next_pos_slow {
                    Some(next_pos_slow) => {
                        pos_slow = next_pos_slow;
                        dir_slow = next_direction(dir_slow)
                    },
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
