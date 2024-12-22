use std::{collections::{HashMap, HashSet, VecDeque}, fs, time::Instant};

fn solve_part_1(input: &String) -> usize
{
    let targets: Vec<_> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let numpad: HashMap<_, _> = [
        ((0, 0), '7'),
        ((1, 0), '8'),
        ((2, 0), '9'),
        ((0, 1), '4'),
        ((1, 1), '5'),
        ((2, 1), '6'),
        ((0, 2), '1'),
        ((1, 2), '2'),
        ((2, 2), '3'),
        ((1, 3), '0'),
        ((2, 3), 'A')
    ].into_iter().collect();
    let dirpad: HashMap<_, _> = [
        ((1, 0), '^'),
        ((2, 0), 'A'),
        ((0, 1), '<'),
        ((1, 1), 'v'),
        ((2, 1), '>'),
    ].into_iter().collect();

    fn to_movement_map(map: &HashMap<(i32, i32), char>) -> HashMap<(char, char), Vec<Vec<char>>> {
        return map.iter()
            .flat_map(|(start_pos, start_char)| map
                .iter()
                .map(move |(end_pos, end_char)| {
                    let mut visited = HashSet::new();
                    let mut queue = VecDeque::new();
                    let mut paths = vec!();
                    let mut target_len = None;
                    queue.push_back((*start_pos, vec!()));
                    while let Some((pos, mut path)) = queue.pop_front() {
                        path.push(pos);
                        let state = (pos, path.clone());
                        if visited.contains(&state) {
                            continue;
                        }
                        visited.insert(state);

                        if target_len
                            .map(|target_len| target_len < path.len())
                            .unwrap_or(false) {
                            continue;
                        }

                        let (x, y) = pos;
                        [
                            (x + 1, y),
                            (x - 1, y),
                            (x, y + 1),
                            (x, y - 1)
                        ].into_iter()
                            .filter(|next| map.contains_key(next))
                            .for_each(|next| queue.push_back((next, path.clone())));

                        if pos == *end_pos {
                            target_len = Some(path.len());
                            paths.push(path);
                        }
                    }
                    return ((*start_char, *end_char), paths);
                })
                .map(|(k, v)| (k, v.into_iter()
                    .map(|path| path.windows(2)
                        .map(|pos| match pos {
                            [(x1, y1), (x2, y2)] => 
                                if x1 < x2 {'>'}
                                else if x1 > x2 {'<'}
                                else if y1 < y2 {'v'}
                                else if y1 > y2 {'^'}
                                else {unreachable!()},
                            _ => unreachable!()
                        })
                        .collect())
                    .collect()))
            )
            .collect();
    }

    let numpad_moves: HashMap<_, _> = to_movement_map(&numpad);
    let dirpad_moves: HashMap<_, _> = to_movement_map(&dirpad);

    fn get_path_len(target: &str, moves: &HashMap<(char, char), Vec<Vec<char>>>) -> usize {
        return target.chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|cs|  match cs {
                [start_c, end_c] => moves.get(&(*start_c, *end_c)).unwrap(),
                _ => unreachable!()
            })
            .map(|paths| paths.first().unwrap().len() + 1)
            .sum();
    }
    fn get_paths(target: &str, moves: &HashMap<(char, char), Vec<Vec<char>>>) -> Vec<Vec<char>> {
        return target.chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|cs|  match cs {
                [start_c, end_c] => moves.get(&(*start_c, *end_c)).unwrap(),
                _ => unreachable!()
            })
            .fold(vec!(vec!()), |paths: Vec<Vec<char>>, curr| {
                return paths.into_iter()
                    .flat_map(|path| curr.iter()
                        .map(move |curr_path| {
                            let mut next = path.clone();
                            next.extend(curr_path);
                            next.push('A');
                            return next;
                        }))
                    .collect();
            });
    }

    let mut res = 0;
    for target in targets {
        let first_order_paths = get_paths(&("A".to_owned() + target), &numpad_moves);

        let (second_order_paths, _) = first_order_paths.iter()
            .fold((vec!(), None), |(mut acc, best_len), path| {
                let target = "A".to_owned() + &path.iter().collect::<String>();

                let next_len = get_path_len(&target, &dirpad_moves);
                let best_len = best_len.unwrap_or(next_len);
                
                if best_len < next_len {
                    return (acc, Some(best_len));
                }
                if best_len > next_len {
                    return (get_paths(&target, &dirpad_moves), Some(next_len));
                }
                acc.extend(get_paths(&target, &dirpad_moves));
                return (acc, Some(best_len));
            });

        let (third_order_paths, _) = second_order_paths.iter()
            .fold((vec!(), None), |(mut acc, best_len), path| {
                let target = "A".to_owned() + &path.iter().collect::<String>();

                let next_len = get_path_len(&target, &dirpad_moves);
                let best_len = best_len.unwrap_or(next_len);
                
                if best_len < next_len {
                    return (acc, Some(best_len));
                }
                if best_len > next_len {
                    return (get_paths(&target, &dirpad_moves), Some(next_len));
                }
                acc.extend(get_paths(&target, &dirpad_moves));
                return (acc, Some(best_len));
            });

        let code = target.chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        res += code * third_order_paths.first().unwrap().len()
    }

    return res;
}

fn solve_part_2(input: &String) -> usize
{
    let depth = 25;

    let targets: Vec<_> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let numpad: HashMap<_, _> = [
        ((0, 0), '7'),
        ((1, 0), '8'),
        ((2, 0), '9'),
        ((0, 1), '4'),
        ((1, 1), '5'),
        ((2, 1), '6'),
        ((0, 2), '1'),
        ((1, 2), '2'),
        ((2, 2), '3'),
        ((1, 3), '0'),
        ((2, 3), 'A')
    ].into_iter().collect();
    let dirpad: HashMap<_, _> = [
        ((1, 0), '^'),
        ((2, 0), 'A'),
        ((0, 1), '<'),
        ((1, 1), 'v'),
        ((2, 1), '>'),
    ].into_iter().collect();

    fn to_movement_map(map: &HashMap<(i32, i32), char>) -> HashMap<(char, char), Vec<Vec<char>>> {
        return map.iter()
            .flat_map(|(start_pos, start_char)| map
                .iter()
                .map(move |(end_pos, end_char)| {
                    let mut visited = HashSet::new();
                    let mut queue = VecDeque::new();
                    let mut paths = vec!();
                    let mut target_len = None;
                    queue.push_back((*start_pos, vec!()));
                    while let Some((pos, mut path)) = queue.pop_front() {
                        path.push(pos);
                        let state = (pos, path.clone());
                        if visited.contains(&state) {
                            continue;
                        }
                        visited.insert(state);

                        if target_len
                            .map(|target_len| target_len < path.len())
                            .unwrap_or(false) {
                            continue;
                        }

                        let (x, y) = pos;
                        [
                            (x + 1, y),
                            (x - 1, y),
                            (x, y + 1),
                            (x, y - 1)
                        ].into_iter()
                            .filter(|next| map.contains_key(next))
                            .for_each(|next| queue.push_back((next, path.clone())));

                        if pos == *end_pos {
                            target_len = Some(path.len());
                            paths.push(path);
                        }
                    }
                    return ((*start_char, *end_char), paths);
                })
                .map(|(k, v)| (k, v.into_iter()
                    .map(|path| path.windows(2)
                        .map(|pos| match pos {
                            [(x1, y1), (x2, y2)] => 
                                if x1 < x2 {'>'}
                                else if x1 > x2 {'<'}
                                else if y1 < y2 {'v'}
                                else if y1 > y2 {'^'}
                                else {unreachable!()},
                            _ => unreachable!()
                        })
                        .chain(['A'].into_iter())
                        .collect())
                    .collect()))
            )
            .collect();
    }

    let numpad_moves: HashMap<_, _> = to_movement_map(&numpad);
    let dirpad_moves: HashMap<_, _> = to_movement_map(&dirpad);

    fn get_paths(target: &str, moves: &HashMap<(char, char), Vec<Vec<char>>>) -> Vec<Vec<char>> {
        return target.chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|cs|  match cs {
                [start_c, end_c] => moves.get(&(*start_c, *end_c)).unwrap(),
                _ => unreachable!()
            })
            .fold(vec!(vec!()), |paths: Vec<Vec<char>>, curr| {
                return paths.into_iter()
                    .flat_map(|path| curr.iter()
                        .map(move |curr_path| {
                            let mut next = path.clone();
                            next.extend(curr_path);
                            return next;
                        }))
                    .collect();
            });
    }

    let mut res = 0;
    for target in targets {
        let paths = get_paths(&("A".to_owned() + target), &numpad_moves);

        fn solve(start_c: char, end_c: char, steps_left: usize, 
            moves: &HashMap<(char, char), Vec<Vec<char>>>,
            cache: &mut HashMap<(char, char, usize), usize>) -> usize {

            if steps_left == 0 {
                return moves.get(&(start_c, end_c))
                    .unwrap()
                    .first()
                    .unwrap()
                    .len();
            }
            if let Some(res) = cache.get(&(start_c, end_c, steps_left)) {
                return *res;
            }

            let mut best_len: Option<usize> = None;
            for path in moves.get(&(start_c, end_c)).unwrap() {
                let path: Vec<_> = ['A'].iter()
                    .chain(path.iter())
                    .map(|c| *c)
                    .collect();
                let curr_len = path.windows(2)
                    .map(|w| match w {
                        [c1, c2] => solve(*c1, *c2, steps_left - 1, moves, cache),
                        _ => unreachable!()
                    })
                    .sum();
                best_len = Some(best_len.map(|bl| bl.min(curr_len)).unwrap_or(curr_len));
            }

            let res = best_len.unwrap();
            cache.insert((start_c, end_c, steps_left), res);
            return res;
        }

        let best_len = paths.iter()
            .map(|path| ['A'].iter().chain(path.iter())
                .map(|c| *c)
                .collect::<Vec<char>>()
                .windows(2)
                .map(|w| match w {
                    [c1, c2] => solve(*c1, *c2, depth - 1, &dirpad_moves, &mut HashMap::new()),
                    _ => unreachable!()
                })
                .sum::<usize>())
            .min()
            .unwrap();

        let code = target.chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        res += code * best_len
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
