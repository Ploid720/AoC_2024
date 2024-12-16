use std::{collections::{HashSet, HashMap}, fs, time::Instant};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn solve_part_1(input: &String) -> isize
{
    let input = input
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    let mut inputs = input
        .split("\n\n");

    type Dir = Direction;
    
    let map: HashMap<_, _> = inputs.next()
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .map(move |(x, c)| ((x as isize, y as isize), c)))
        .collect();

    let moves: Vec<_> = inputs.next()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Dir::Up),
            'v' => Some(Dir::Down),
            '<' => Some(Dir::Left),
            '>' => Some(Dir::Right),
            _ => None
        })
        .collect();
    
    let walls: HashSet<_> = map.iter()
        .filter_map(|(pos, c)| match c {
            '#' => Some(*pos),
            _ => None
        })
        .collect();
    let mut boxes: HashSet<_> = map.iter()
        .filter_map(|(pos, c)| match c {
            'O' => Some(*pos),
            _ => None
        })
        .collect();
    let start = *map.iter()
        .find_map(|(pos, c)| match c {
            '@' => Some(pos),
            _ => None
        })
        .unwrap();
    
    fn offset(pos: (isize, isize), dir: Dir) -> (isize, isize) {
        let (x, y) = pos;
        return match dir {
            Dir::Up    => (x, y - 1),
            Dir::Down  => (x, y + 1),
            Dir::Left  => (x - 1, y),
            Dir::Right => (x + 1, y)
        }
    }

    let mut robot_pos = start;
    for dir in moves {
        let next_robot_pos = offset(robot_pos, dir);
        let mut offset_pos = next_robot_pos;
        let mut boxes_to_push = vec!();
        while boxes.contains(&offset_pos) {
            boxes_to_push.push(offset_pos);
            offset_pos = offset(offset_pos, dir);
        }
        if walls.contains(&offset_pos) {
            continue;
        }

        for box_pos in &boxes_to_push {
            boxes.remove(box_pos);
        }
        for box_pos in boxes_to_push {
            boxes.insert(offset(box_pos, dir));
        }
        robot_pos = next_robot_pos;
    }

    return boxes.iter()
        .map(|(x, y)| 100 * y + x)
        .sum();
}

fn solve_part_2(input: &String) -> isize
{
    let input = input
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    let mut inputs = input
        .split("\n\n");

    type Dir = Direction;
    
    let map: HashMap<_, _> = inputs.next()
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .map(move |(x, c)| ((x as isize, y as isize), c)))
        .collect();

    let moves: Vec<_> = inputs.next()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Dir::Up),
            'v' => Some(Dir::Down),
            '<' => Some(Dir::Left),
            '>' => Some(Dir::Right),
            _ => None
        })
        .collect();
    
    let walls: HashSet<_> = map.iter()
        .filter_map(|((x, y), c)| match c {
            '#' => Some(((2 * x, 2 * x + 1), *y)),
            _ => None
        })
        .flat_map(|((x1, x2), y)| [
            (x1, y),
            (x2, y)
        ])
        .collect();
    let mut boxes: HashMap<_, _> = map.iter()
        .filter_map(|((x, y), c)| match c {
            'O' => Some(((2 * x, 2 * x + 1), *y)),
            _ => None
        })
        .flat_map(|((x1, x2), y)| [
            ((x1, y), (x2, y)),
            ((x2, y), (x1, y))
        ])
        .collect();
    let start = map.iter()
        .find_map(|((x, y), c)| match c {
            '@' => Some((2 * x, *y)),
            _ => None
        })
        .unwrap();
    
    fn offset(pos: (isize, isize), dir: Dir) -> (isize, isize) {
        let (x, y) = pos;
        return match dir {
            Dir::Up    => (x, y - 1),
            Dir::Down  => (x, y + 1),
            Dir::Left  => (x - 1, y),
            Dir::Right => (x + 1, y)
        }
    }

    let mut robot_pos = start;

    'outer: for dir in moves {
        let next_robot_pos = offset(robot_pos, dir);
        if walls.contains(&next_robot_pos) {
            continue;
        }

        let mut boxes_to_push = HashSet::new();
        let mut layer = vec!(next_robot_pos);
        let mut visited = HashSet::new();
        while !layer.is_empty() {
            let mut next_layer = vec!();
            for offset_pos in layer {
                if visited.contains(&offset_pos) {
                    continue;
                }
                visited.insert(offset_pos);

                if walls.contains(&offset_pos) {
                    continue 'outer;
                }

                if let Some(&(bx, by)) = boxes.get(&offset_pos) {
                    let (ox, oy) = offset_pos;
                    let boxl = (ox.min(bx), oy.min(by));
                    let boxr = (ox.max(bx), oy.max(by));
                    visited.insert(boxl);
                    visited.insert(boxr);
                    boxes_to_push.insert((boxl, boxr));
                    next_layer.push(offset(boxl, dir));
                    next_layer.push(offset(boxr, dir));
                }
            }
            layer = next_layer;
        }
        
        for (boxl, boxr) in &boxes_to_push {
            boxes.remove(boxl);
            boxes.remove(boxr);
        }
        for (boxl, boxr) in boxes_to_push {
            let boxlo = offset(boxl, dir);
            let boxro = offset(boxr, dir);
            boxes.insert(boxlo, boxro);
            boxes.insert(boxro, boxlo);
        }
        robot_pos = next_robot_pos;
    }

    let boxes_small: HashSet<_> = boxes.iter()
        .map(|((x1, y1), (x2, y2))| (x1.min(x2), y1.min(y2)))
        .collect();

    return boxes_small.iter()
        .map(|(x, y)| 100 * *y + *x)
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
