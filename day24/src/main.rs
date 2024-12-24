use std::{collections::{HashMap, HashSet, VecDeque}, fs, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Xor,
}

fn solve_part_1(input: &String) -> usize
{
    let input = input
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    let mut inputs = input
        .split("\n\n");

    let mut values: HashMap<_, _> = inputs.next()
        .unwrap()
        .lines()
        .map(|line| line.split(":"))
        .filter_map(|mut spl| match (spl.next(), spl.next()) {
            (Some(wire), Some(v)) => Some((wire, v.trim().parse::<i8>().unwrap() != 0)),
            _ => None
        })
        .collect();

    type Op = Operation;

    let mut gates: VecDeque<_> = inputs.next()
        .unwrap()
        .lines()
        .map(|line| line.split_ascii_whitespace()
            .filter(|str| !str.is_empty()))
        .filter_map(|mut spl| match (
            spl.next(), spl.next(), spl.next(), spl.next(), spl.next()
        ) {
            (Some(in1), Some(op), Some(in2), _, Some(out)) => Some((in1, op, in2, out)),
            _ => None
        })
        .filter_map(|(in1, op, in2, out)| match op {
            "AND" => Some((in1, Op::And, in2, out)),
            "OR" => Some((in1, Op::Or, in2, out)),
            "XOR" => Some((in1, Op::Xor, in2, out)),
            _ => panic!("Invalid operation: {}", op)
        })
        .collect();

    while let Some(gate) = gates.pop_front() {
        let (in1, op, in2, out) = gate;
        match (values.get(in1), values.get(in2)) {
            (Some(&in1), Some(&in2)) => {
                let out_val = match op {
                    Op::And => in1 && in2,
                    Op::Or => in1 || in2,
                    Op::Xor => in1 ^ in2
                };
        
                values.insert(out, out_val);
            }
            _ => {
                gates.push_back(gate);
            }
        }
    }

    return values.iter()
        .filter(|(wire, _)| wire.starts_with("z"))
        .fold(0, |z, (wire, value)| {
            if !*value {
                return z;
            }
            let mask = 1 << wire[1..].parse::<u32>().unwrap();
            return z | mask;
        });
}

fn solve_part_2(input: &String) -> String
{
    let input = input
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    let mut inputs = input
        .split("\n\n");


    inputs.next().unwrap();

    type Op = Operation;

    let mut gates: HashMap<_, _> = inputs.next()
        .unwrap()
        .lines()
        .map(|line| line.split_ascii_whitespace()
            .filter(|str| !str.is_empty()))
        .filter_map(|mut spl| match (
            spl.next(), spl.next(), spl.next(), spl.next(), spl.next()
        ) {
            (Some(in1), Some(op), Some(in2), _, Some(out)) => Some((in1, op, in2, out)),
            _ => None
        })
        .filter_map(|(in1, op, in2, out)| match op {
            "AND" => Some((in1, Op::And, in2, out)),
            "OR" => Some((in1, Op::Or, in2, out)),
            "XOR" => Some((in1, Op::Xor, in2, out)),
            _ => panic!("Invalid operation: {}", op)
        })
        .map(|(in1, op, in2, out)| (out, (in1, op, in2)))
        .collect();

    let outputs: Vec<_> = gates.keys()
        .copied()
        .collect();

    for out in outputs {
        let (ln, op, rn) = *gates.get(out).unwrap();

        let left = gates.get(ln)
            .map(|gate| gate_tree_to_string(*gate, &gates))
            .unwrap_or(ln.to_owned());
        let right = gates.get(rn)
            .map(|gate| gate_tree_to_string(*gate, &gates))
            .unwrap_or(rn.to_owned());

        if right.len() < left.len() {
            gates.insert(out, (rn, op, ln));
        }
        else if right.len() > left.len() {} //already in correct order
        else if right.chars().position(|c| c == 'x').unwrap_or(usize::MAX) < left.chars().position(|c| c == 'x').unwrap_or(usize::MAX) {
            gates.insert(out, (rn, op, ln));    
        }
        else if right.chars().position(|c| c == '!').unwrap_or(usize::MAX) < left.chars().position(|c| c == '!').unwrap_or(usize::MAX) {
            gates.insert(out, (rn, op, ln));
        }
        else if right.chars().position(|c| c == 'a').unwrap_or(usize::MAX) < left.chars().position(|c| c == 'a').unwrap_or(usize::MAX) {
            gates.insert(out, (rn, op, ln));
        }
    }

    fn gate_tree_to_string(gate: (&str, Op, &str), gates: &HashMap<&str, (&str, Operation, &str)>) -> String {
        let (in1, op, in2) = gate;
        let op_str = match op {
            Op::And => "and",
            Op::Or => "or",
            Op::Xor => "!="
        };

        let left = match gates.get(in1) {
            Some(gate) => gate_tree_to_string(*gate, gates),
            None => in1.to_owned()
        };
        let right = match gates.get(in2) {
            Some(gate) => gate_tree_to_string(*gate, gates),
            None => in2.to_owned()
        };

        return format!("({left} {op_str} {right})");
    }

    fn gate_tree_to_string_limited(gate: (&str, Op, &str), gates: &HashMap<&str, (&str, Operation, &str)>, limit: usize) -> String {
        if limit == 0 {
            return "...".to_owned();
        }
        
        let (in1, op, in2) = gate;
        let op_str = match op {
            Op::And => "and",
            Op::Or => "or",
            Op::Xor => "!="
        };

        let left = match gates.get(in1) {
            Some(gate) => gate_tree_to_string_limited(*gate, gates, limit - 1),
            None => in1.to_owned()
        };
        let right = match gates.get(in2) {
            Some(gate) => gate_tree_to_string_limited(*gate, gates, limit - 1),
            None => in2.to_owned()
        };

        return format!("({left} {op_str} {right})");
    }

    let mut z_wires: Vec<_> = gates.keys()
        .filter(|out| out.starts_with("z"))
        .copied()
        .collect();

    z_wires.sort();

    fn get_inner_pattern(n: usize) -> String {
        match n {
            0 => "((x1 and y1) or ((x1 != y1) and (x0 and y0)))".to_owned(),
            _ => {
                let np1 = n + 1;
                return format!("((x{np1} and y{np1}) or ((x{np1} != y{np1}) and {}))", get_inner_pattern(n - 1));
            }
        }
    }
    fn get_pattern(n: usize) -> String {
        match n {
            0 => "(x0 != y0)".to_owned(),
            1 => "((x1 != y1) != (x0 and y0))".to_owned(),
            2 => "((x2 != y2) != ((x1 and y1) or ((x1 != y1) and (x0 and y0))))".to_owned(),
            _ => {
                let nm1 = n - 1;
                let inner_pattern = get_inner_pattern(n - 3);
                return format!("((x{n} != y{n}) != ((x{nm1} and y{nm1}) or ((x{nm1} != y{nm1}) and {inner_pattern})))");
            }
        }
    }

    fn get_limit(n: usize) -> usize {
        return 2 * (n + 3);
    }

    fn remove_all_visited(start: &str, map: &HashMap<&str, (&str, Operation, &str)>, set: &mut HashSet<&str>) {
        set.remove(start);
        if let Some((ln, _, rn)) = map.get(&start) {
            remove_all_visited(ln, map, set);
            remove_all_visited(rn, map, set);
        }
    }

    let mut possible_broken_outputs: HashSet<_> = gates.keys()
        .copied()
        .collect();

    let mut swaps = vec!();

    for z_wire in z_wires {
        let n = z_wire[1..].parse::<usize>().unwrap();
        let curr = gate_tree_to_string(*gates.get(z_wire).unwrap(), &gates)
            .replace("x0", "x")
            .replace("y0", "y");
        let target = get_pattern(n);

        if curr == target {
            remove_all_visited(z_wire, &gates, &mut possible_broken_outputs);
        }
        else {
            let mut swap = None;
            let limit = get_limit(n);
            'outer: for w1 in possible_broken_outputs.iter() {
                for w2 in possible_broken_outputs.iter() {
                    if w1 == w2 {
                        continue;
                    }

                    match (gates.get(w1), gates.get(w2)) {
                        (Some(&g1), Some(&g2)) => {
                            gates.insert(w1, g2);
                            gates.insert(w2, g1);
                            let curr = gate_tree_to_string_limited(*gates.get(z_wire).unwrap(), &gates, limit)
                                .replace("x0", "x")
                                .replace("y0", "y");
                            if curr == target {
                                swap = Some((*w1, *w2));
                                break 'outer;
                            }
                            else {
                                gates.insert(w1, g1);
                                gates.insert(w2, g2);
                            }
                        }
                        _ => {}
                    }
                }
            }

            if let Some((w1, w2)) = swap {
                remove_all_visited(z_wire, &gates, &mut possible_broken_outputs);
                swaps.push(w1);
                swaps.push(w2);
                if swaps.len() >= 8 {
                    break;
                }
            }
            else {
                break;
            }
        }
    }

    swaps.sort();

    return swaps.into_iter()
        .map(|s| s.to_owned())
        .reduce(|a, b| a.to_owned() + "," + &b)
        .unwrap_or("".to_owned());
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
