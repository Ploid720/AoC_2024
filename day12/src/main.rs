use std::{collections::{HashMap, HashSet}, fs, time::Instant};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn solve_part_1(input: &String) -> usize
{
    let map: HashMap<_, _> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .map(move |(x, c)| ((x as isize, y as isize), c)))
        .collect();

    let mut unvisited: HashSet<_> = map.keys().collect();
    let mut areas: HashMap<_, _> = HashMap::new();
    let mut perimeters: HashMap<_, _> = HashMap::new();

    let mut region_index = 0;

    while !unvisited.is_empty() {
        let mut stack = vec!(**unvisited.iter().next().unwrap());
        while !stack.is_empty() {
            let pos = stack.pop().unwrap();
            if !unvisited.contains(&pos) {
                continue;
            }
            unvisited.remove(&pos);

            let plant = map.get(&pos).unwrap();
            let (x, y) = pos;
            let nbs = [
                (x + 1, y),
                (x - 1, y),
                (x, y + 1),
                (x, y - 1)
            ];

            nbs.iter()
                .filter(|pos| map.get(pos)
                    .map(|next_plant| next_plant == plant)
                    .unwrap_or(false))
                .for_each(|pos| stack.push(*pos));

            let touch_count = nbs.iter()
                .filter(|pos| map.get(pos)
                    .map(|next_plant| next_plant == plant)
                    .unwrap_or(false))
                .count();

            areas.insert(region_index, areas.get(&region_index).unwrap_or(&0) + 1);
            perimeters.insert(region_index, perimeters.get(&region_index).unwrap_or(&0) + 4 - touch_count);
            
        }
        region_index += 1;
    }

    return areas.iter()
        .map(|(region_index, area)| area * perimeters.get(region_index).unwrap_or(&0))
        .sum();
}

fn solve_part_2(input: &String) -> usize
{
    let map: HashMap<_, _> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .map(move |(x, c)| ((x as isize, y as isize), c)))
        .collect();

    let mut unvisited: HashSet<_> = map.keys().collect();
    let mut areas: HashMap<_, _> = HashMap::new();
    let mut bounds: HashMap<_, HashSet<_>> = HashMap::new();

    type Dir = Direction;

    let mut region_index = 0;

    while !unvisited.is_empty() {
        let mut stack = vec!(**unvisited.iter().next().unwrap());
        while !stack.is_empty() {
            let pos = stack.pop().unwrap();
            if !unvisited.contains(&pos) {
                continue;
            }
            unvisited.remove(&pos);

            let plant = map.get(&pos).unwrap();
            let (x, y) = pos;

            [
                (x + 1, y),
                (x - 1, y),
                (x, y + 1),
                (x, y - 1)
            ].iter()
                .filter(|pos| map.get(pos)
                    .map(|next_plant| next_plant == plant)
                    .unwrap_or(false))
                .for_each(|pos| stack.push(*pos));

            areas.insert(region_index, areas.get(&region_index).unwrap_or(&0) + 1);
            
            [
                ((x + 1, y), Dir::Right),
                ((x - 1, y), Dir::Left),
                ((x, y + 1), Dir::Down),
                ((x, y - 1), Dir::Up)
            ].iter()
                .filter(|(pos, _)| map.get(pos)
                    .map(|next_plant| next_plant != plant)
                    .unwrap_or(true))
                .map(|(_, dir)| (pos, *dir))
                .for_each(|v| match bounds.get_mut(&region_index) {
                    Some(bounds_set) => {
                        bounds_set.insert(v);
                    },
                    None => {
                        let mut bounds_set = HashSet::new();
                        bounds_set.insert(v);
                        bounds.insert(region_index, bounds_set);
                    }
                });
        }
        region_index += 1;
    }

    return areas.iter()
        .map(|(region_index, area)| {
            let bounds_set = match bounds.get(region_index) {
                Some(bs) => bs,
                None => return 0
            };
            let mut res = 0;
            for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                let nb_offsets = match dir {
                    Dir::Up   | Dir::Down  => [(1, 0), (-1, 0)],
                    Dir::Left | Dir::Right => [(0, 1), (0, -1)]
                };

                let mut unvisited: HashSet<_> = bounds_set.iter()
                    .filter_map(|(cur_pos, cur_dir)| if *cur_dir == dir {Some(cur_pos)} else {None})
                    .collect();

                while !unvisited.is_empty() {
                    let mut stack = vec!(**unvisited.iter().next().unwrap());
                    while !stack.is_empty() {
                        let pos = stack.pop().unwrap();
                        unvisited.remove(&pos);

                        let (x, y) = pos;
                        nb_offsets.iter()
                            .map(|(x_off, y_off)| (x + x_off, y + y_off))
                            .filter(|pos| unvisited.contains(&pos))
                            .for_each(|pos| stack.push(pos));
                    }
                    res += 1;
                }
            }

            return area * res;
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
