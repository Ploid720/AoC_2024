use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}, fs, time::Instant};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    position: (isize, isize),
    direction: Direction,
    score: usize
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
            .then_with(|| self.position.cmp(&other.position))
    }
}

fn solve_part_1(input: &String) -> Option<usize>
{
    let map: HashMap<_, _> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .map(move |(x, c)| ((x as isize, y as isize), c)))
        .collect();

    type Dir = Direction;

    let start = map.iter()
        .find_map(|(pos, c)| if *c == 'S' {Some(*pos)} else {None})
        .unwrap();
    let start_dir = Dir::East;

    let end = map.iter()
        .find_map(|(pos, c)| if *c == 'E' {Some(*pos)} else {None})
        .unwrap();
    let tiles: HashSet<_> = map.iter()
        .filter_map(|(pos, c)| if *c != '#' {Some(*pos)} else {None})
        .collect();

    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert((start, start_dir), 0);
    heap.push(State{position: start, direction: start_dir, score: 0});

    fn rotate_clockwise(dir: Dir) -> Dir {
        match dir {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North
        }
    }
    fn rotate_anticlockwise(dir: Dir) -> Dir {
        match dir {
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
            Dir::East => Dir::North
        }
    }
    fn offset(pos: (isize, isize), dir: Dir) -> (isize, isize) {
        let (x, y) = pos;
        match dir {
            Dir::North => (x, y - 1),
            Dir::East => (x + 1, y),
            Dir::South => (x, y + 1),
            Dir::West => (x - 1, y)
        }
    }

    while let Some(State{position: pos, direction: dir, score}) = heap.pop() {
        if pos == end {
            return Some(score);
        }

        if dist.get(&(pos, dir))
            .map(|curr| score > *curr)
            .unwrap_or(false) {
            continue
        }

        for (next_pos, next_dir, score_incr) in [
            (pos, rotate_clockwise(dir), 1000),
            (offset(pos, dir), dir, 1),
            (pos, rotate_anticlockwise(dir), 1000)
        ] {
            if !tiles.contains(&next_pos) {
                continue;
            }

            let next = (next_pos, next_dir);
            let next_score = score + score_incr;
            if dist.get(&next)
                .map(|sc| next_score < *sc)
                .unwrap_or(true) {
                heap.push(State{position: next_pos, direction: next_dir, score: next_score});
                dist.insert(next, next_score);
            }
        }
    }

    return None;
}

fn solve_part_2(input: &String) -> usize
{
    let map: HashMap<_, _> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .map(move |(x, c)| ((x as isize, y as isize), c)))
        .collect();

    type Dir = Direction;

    let start = map.iter()
        .find_map(|(pos, c)| if *c == 'S' {Some(*pos)} else {None})
        .unwrap();
    let start_dir = Dir::East;

    let end = map.iter()
        .find_map(|(pos, c)| if *c == 'E' {Some(*pos)} else {None})
        .unwrap();
    let tiles: HashSet<_> = map.iter()
        .filter_map(|(pos, c)| if *c != '#' {Some(*pos)} else {None})
        .collect();

    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert((start, start_dir), 0);
    heap.push(State{position: start, direction: start_dir, score: 0});

    fn rotate_clockwise(dir: Dir) -> Dir {
        match dir {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North
        }
    }
    fn rotate_anticlockwise(dir: Dir) -> Dir {
        match dir {
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
            Dir::East => Dir::North
        }
    }
    fn offset(pos: (isize, isize), dir: Dir) -> (isize, isize) {
        let (x, y) = pos;
        match dir {
            Dir::North => (x, y - 1),
            Dir::East => (x + 1, y),
            Dir::South => (x, y + 1),
            Dir::West => (x - 1, y)
        }
    }

    let mut paths: HashMap<_, HashSet<_>> = HashMap::new();
    let mut end_dir = Dir::North;

    while let Some(State{position: pos, direction: dir, score}) = heap.pop() {
        if pos == end {
            end_dir = dir;
            break;
        }

        if dist.get(&(pos, dir))
            .map(|curr| score > *curr)
            .unwrap_or(false) {
            continue
        }

        for (next_pos, next_dir, score_incr) in [
            (pos, rotate_clockwise(dir), 1000),
            (offset(pos, dir), dir, 1),
            (pos, rotate_anticlockwise(dir), 1000)
        ] {
            if !tiles.contains(&next_pos) {
                continue;
            }

            let next = (next_pos, next_dir);
            let next_score = score + score_incr;

            let better = dist.get(&next)
                .map(|sc| next_score < *sc)
                .unwrap_or(true);
            let better_or_eq= dist.get(&next)
                .map(|sc| next_score <= *sc)
                .unwrap_or(true);

            if better {
                heap.push(State{position: next_pos, direction: next_dir, score: next_score});
                dist.insert(next, next_score);

                let mut path = HashSet::new();
                path.insert((pos, dir));
                paths.insert(next, path);
            }
            else if better_or_eq {
                match paths.get_mut(&next) {
                    Some(path) => {
                        path.insert((pos, dir));
                    }
                    None => {
                        let mut path = HashSet::new();
                        path.insert((pos, dir));
                        paths.insert(next, path);
                    }
                }
            }
        }
    }

    let empty = HashSet::new();
    let mut visited = HashSet::new();
    
    let mut stack = vec!((end, end_dir));
    while let Some(curr) = stack.pop() {
        if visited.contains(&curr) {
            continue;
        }
        visited.insert(curr);

        paths.get(&curr)
            .unwrap_or(&empty)
            .iter()
            .for_each(|next| stack.push(*next));
    }

    let tiles_on_best_paths: HashSet<_> = visited.iter().map(|(pos, _)| pos).collect();
    return tiles_on_best_paths.len();
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let inst1 = Instant::now();
    let part_1_res = solve_part_1(&content).map(|v| v.to_string()).unwrap_or("No path found".to_owned());
    let t1 = inst1.elapsed();
    let inst2 = Instant::now();
    let part_2_res = solve_part_2(&content);
    let t2 = inst2.elapsed();

    println!("Part 1 result: {}, solved in {} ms", part_1_res, t1.as_secs_f64() * 1000.0);
    println!("Part 2 result: {}, solved in {} ms", part_2_res, t2.as_secs_f64() * 1000.0);
}
