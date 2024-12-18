use std::{collections::{HashSet, VecDeque}, fs, time::Instant};

fn solve_part_1(input: &String) -> Option<usize>
{
    let w = 71;
    let h = 71;

    let blocked: HashSet<_> = input.lines()
        .map(|line| line.split(","))
        .filter_map(|mut comma_split| match (comma_split.next(), comma_split.next()) {
            (Some(x), Some(y)) => Some((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())),
            _ => None
        })
        .take(1024)
        .collect();

    let target = (w - 1, h - 1);
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(((0, 0), 0));
    while let Some((pos, step)) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        if pos == target {
            return Some(step);
        }
        visited.insert(pos);

        let (x, y) = pos;
        [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1)
        ].into_iter()
            .filter(|(x, y)| *x >= 0 && *x < w && *y >= 0 && *y < h)
            .filter(|pos| !visited.contains(pos))
            .filter(|pos| !blocked.contains(pos))
            .for_each(|pos| queue.push_back((pos, step + 1)));
    }

    return None;
}

fn solve_part_2(input: &String) -> Option<(i32, i32)>
{
    let w = 71;
    let h = 71;

    let mut obstacles: Vec<_> = input.lines()
        .map(|line| line.split(","))
        .filter_map(|mut comma_split| match (comma_split.next(), comma_split.next()) {
            (Some(x), Some(y)) => Some((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())),
            _ => None
        })
        .rev()
        .collect();

    let target = (w - 1, h - 1);
    let mut blocked = HashSet::new();
    let mut last_added_obstacle = None;

    loop {
        let mut best_path = None;

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(((0, 0), vec!()));
        while let Some((pos, mut path)) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            path.push(pos);
            if pos == target {
                best_path = Some(path);
                break;
            }
            visited.insert(pos);

            let (x, y) = pos;
            [
                (x + 1, y),
                (x - 1, y),
                (x, y + 1),
                (x, y - 1)
            ].into_iter()
                .filter(|(x, y)| *x >= 0 && *x < w && *y >= 0 && *y < h)
                .filter(|pos| !visited.contains(pos))
                .filter(|pos| !blocked.contains(pos))
                .for_each(|pos| queue.push_back((pos, path.clone())));
        }

        match best_path {
            Some(path) => {
                if obstacles.is_empty() {
                    return None;
                }

                let path_tiles: HashSet<_> = path.into_iter().collect();

                while let Some(obstacle) = obstacles.pop() {
                    blocked.insert(obstacle);
                    last_added_obstacle = Some(obstacle);
                    if path_tiles.contains(&obstacle) {
                        break;
                    }
                }
            }
            None => {
                return last_added_obstacle;
            }
        }
    }
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let inst1 = Instant::now();
    let part_1_res = solve_part_1(&content).map(|n| n.to_string()).unwrap_or("No solution".to_owned());
    let t1 = inst1.elapsed();
    let inst2 = Instant::now();
    let part_2_res = solve_part_2(&content).map(|(x, y)| format!("{x},{y}")).unwrap_or("No solution".to_owned());
    let t2 = inst2.elapsed();

    println!("Part 1 result: {}, solved in {} ms", part_1_res, t1.as_secs_f64() * 1000.0);
    println!("Part 2 result: {}, solved in {} ms", part_2_res, t2.as_secs_f64() * 1000.0);
}
