use std::{collections::{HashMap, HashSet, VecDeque}, fs, time::Instant};

fn solve_part_1(input: &String) -> usize
{
    let map: HashMap<_, _> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .map(move |(x, c)| ((x as isize, y as isize), c)))
        .collect();

    let start = map.iter()
        .find(|(_, c)| **c == 'S')
        .map(|(pos, _)| *pos)
        .unwrap();
    let end = map.iter()
        .find(|(_, c)| **c == 'E')
        .map(|(pos, _)| *pos)
        .unwrap();

    let track: HashSet<_> = map.iter()
        .filter(|(_, c)| **c != '#')
        .map(|(pos, _)| *pos)
        .collect();

    let mut base_dist = None;
    let mut start_dist = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        start_dist.insert(pos, steps);

        if pos == end {
            base_dist = Some(steps);
        }

        let (x, y) = pos;
        [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
        ].into_iter()
            .filter(|pos| track.contains(&pos))
            .filter(|pos| !visited.contains(pos))
            .for_each(|pos| queue.push_back((pos, steps + 1)));
    }

    let mut end_dist = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((end, 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        end_dist.insert(pos, steps);

        let (x, y) = pos;
        [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
        ].into_iter()
            .filter(|pos| track.contains(&pos))
            .filter(|pos| !visited.contains(pos))
            .for_each(|pos| queue.push_back((pos, steps + 1)));
    }

    let best = base_dist.unwrap();
    let end_dist_ref = &end_dist;

    return start_dist.iter()
        .flat_map(|(start, start_step)| {
            let (x, y) = *start;
            return [
                (x - 2, y    ),
                (x - 1, y + 1),
                (x    , y + 2),
                (x + 1, y + 1),
                (x + 2, y    ),
                (x + 1, y - 1),
                (x    , y - 2),
                (x - 1, y - 1),
            ].into_iter()
                .filter(move |end| end_dist_ref.get(end)
                    .map(|end_step| (end_step + start_step) < (best - 2 - (100-1)))
                    .unwrap_or(false))
                .map(|end| (*start, end));
        })
        .collect::<HashSet<_>>()
        .len();
}

fn solve_part_2(input: &String) -> usize
{
    let map: HashMap<_, _> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .map(move |(x, c)| ((x as isize, y as isize), c)))
        .collect();

    let start = map.iter()
        .find(|(_, c)| **c == 'S')
        .map(|(pos, _)| *pos)
        .unwrap();
    let end = map.iter()
        .find(|(_, c)| **c == 'E')
        .map(|(pos, _)| *pos)
        .unwrap();

    let track: HashSet<_> = map.iter()
        .filter(|(_, c)| **c != '#')
        .map(|(pos, _)| *pos)
        .collect();

    let mut base_dist = None;
    let mut start_dist = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        start_dist.insert(pos, steps);

        if pos == end {
            base_dist = Some(steps);
        }

        let (x, y) = pos;
        [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
        ].into_iter()
            .filter(|pos| track.contains(&pos))
            .filter(|pos| !visited.contains(pos))
            .for_each(|pos| queue.push_back((pos, steps + 1)));
    }

    let mut end_dist = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((end, 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        end_dist.insert(pos, steps);

        let (x, y) = pos;
        [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
        ].into_iter()
            .filter(|pos| track.contains(&pos))
            .filter(|pos| !visited.contains(pos))
            .for_each(|pos| queue.push_back((pos, steps + 1)));
    }

    let offsets_and_dists: Vec<_> = (-20..=20)
        .flat_map(|x| (-20..=20)
            .map(move |y| (x as isize, y as isize))
            .map(|(x, y)| ((x, y), (x.abs() + y.abs())))
            .filter(|(_, dist)| (*dist <= 20) && (*dist > 1)))
        .collect();

    let best = base_dist.unwrap();
    let end_dist_ref = &end_dist;

    return start_dist.iter()
        .flat_map(|(start, start_step)| {
            let (x, y) = *start;
            return offsets_and_dists
                .iter()
                .map(move |((x_off, y_off), dist)| ((x + *x_off, y + *y_off), dist))
                .filter(move |(end, dist)| end_dist_ref.get(end)
                    .map(|end_step| (end_step + start_step) < (best - *dist - (100-1)))
                    .unwrap_or(false))
                .map(|end| (*start, end));
        })
        .collect::<HashSet<_>>()
        .len();
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
