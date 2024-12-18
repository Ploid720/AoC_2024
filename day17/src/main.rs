use std::{collections::HashMap, fs, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instructions {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv
}

fn solve_part_1(input: &String) -> String
{
    let input = input
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    let mut inputs = input
        .split("\n\n");

    let registers: HashMap<_, _> = inputs.next()
        .expect("Register definitions should be present in input")
        .lines()
        .map(|line| {
            let reg = &line[9..(line.chars().position(|c| c == ':').unwrap())];
            let val = line[12..].parse::<i32>().unwrap();

            return (reg, val);
        })
        .collect();

    let nums: Vec<_> = inputs.next()
        .expect("Instruction definitions should be present in input")
        [9..]
        .split(",")
        .map(|instr| instr.trim())
        .filter(|instr| !instr.is_empty())
        .map(|instr| instr.parse::<u8>().unwrap())
        .collect();

    type Instr = Instructions;

    let instructions: Vec<_> = nums.chunks(2)
        .map(|nums| match nums {
            [0, operand] => (Instr::Adv, *operand),
            [1, operand] => (Instr::Bxl, *operand),
            [2, operand] => (Instr::Bst, *operand),
            [3, operand] => (Instr::Jnz, *operand),
            [4, operand] => (Instr::Bxc, *operand),
            [5, operand] => (Instr::Out, *operand),
            [6, operand] => (Instr::Bdv, *operand),
            [7, operand] => (Instr::Cdv, *operand),
            _ => panic!("Invalid instruction")
        })
        .collect();

    fn resolve_combo(combo_operand: u8, a: i32, b: i32, c: i32) -> i32 {
        return match combo_operand {
            0..=3 => combo_operand as i32,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Invalid combo operand")
        }
    }

    let prog_size = instructions.len();
    let mut pc = 0;
    let mut a = *registers.get("A").unwrap();
    let mut b = *registers.get("B").unwrap();
    let mut c = *registers.get("C").unwrap();
    let mut out = vec!();
    loop {
        let (instr, operand) = instructions[pc];
        match instr {
            Instr::Adv => {
                let num = a;
                let den = 2i32.pow(resolve_combo(operand, a, b, c).try_into().unwrap());
                a = num / den;
            },
            Instr::Bxl => {
                b ^= operand as i32;
            },
            Instr::Bst => {
                b = resolve_combo(operand, a, b, c) & 7;
            },
            Instr::Jnz => {
                if a != 0 {
                    pc = operand as usize / 2;
                    continue;
                }
            },
            Instr::Bxc => {
                b ^= c;
            },
            Instr::Out => {
                out.push(resolve_combo(operand, a, b, c) & 7);
            },
            Instr::Bdv => {
                let num = a;
                let den = 2i32.pow(resolve_combo(operand, a, b, c).try_into().unwrap());
                b = num / den;
            },
            Instr::Cdv => {
                let num = a;
                let den = 2i32.pow(resolve_combo(operand, a, b, c).try_into().unwrap());
                c = num / den;
            },
        }
        pc += 1;
        if pc >= prog_size {
            break;
        }
    }

    return out.iter()
        .map(|n| n.to_string())
        .reduce(|a, b| a + "," + &b)
        .unwrap_or("".to_owned());
}

fn solve_part_2(input: &String) -> Option<isize>
{
    let input = input
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    let mut inputs = input
        .split("\n\n");

    let registers: HashMap<_, _> = inputs.next()
        .expect("Register definitions should be present in input")
        .lines()
        .map(|line| {
            let reg = &line[9..(line.chars().position(|c| c == ':').unwrap())];
            let val = line[12..].parse::<isize>().unwrap();

            return (reg, val);
        })
        .collect();

    let nums: Vec<_> = inputs.next()
        .expect("Instruction definitions should be present in input")
        [9..]
        .split(",")
        .map(|instr| instr.trim())
        .filter(|instr| !instr.is_empty())
        .map(|instr| instr.parse::<u8>().unwrap())
        .collect();

    type Instr = Instructions;

    let instructions: Vec<_> = nums.chunks(2)
        .map(|nums| match nums {
            [0, operand] => (Instr::Adv, *operand),
            [1, operand] => (Instr::Bxl, *operand),
            [2, operand] => (Instr::Bst, *operand),
            [3, operand] => (Instr::Jnz, *operand),
            [4, operand] => (Instr::Bxc, *operand),
            [5, operand] => (Instr::Out, *operand),
            [6, operand] => (Instr::Bdv, *operand),
            [7, operand] => (Instr::Cdv, *operand),
            _ => panic!("Invalid instruction")
        })
        .collect();

    fn resolve_combo(combo_operand: u8, a: isize, b: isize, c: isize) -> isize {
        return match combo_operand {
            0..=3 => combo_operand as isize,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Invalid combo operand")
        }
    }

    fn emulate(instructions: &Vec<(Instructions, u8)>, a: isize, b: isize, c: isize) -> Vec<isize> {
        let prog_size = instructions.len();
        let mut pc = 0;
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut out = vec!();
        loop {
            let (instr, operand) = instructions[pc];
            match instr {
                Instr::Adv => {
                    let num = a;
                    let den = 2isize.pow(resolve_combo(operand, a, b, c).try_into().unwrap());
                    a = num / den;
                },
                Instr::Bxl => {
                    b ^= operand as isize;
                },
                Instr::Bst => {
                    b = resolve_combo(operand, a, b, c) & 7;
                },
                Instr::Jnz => {
                    if a != 0 {
                        pc = operand as usize / 2;
                        continue;
                    }
                },
                Instr::Bxc => {
                    b ^= c;
                },
                Instr::Out => {
                    out.push(resolve_combo(operand, a, b, c) & 7);
                },
                Instr::Bdv => {
                    let num = a;
                    let den = 2isize.pow(resolve_combo(operand, a, b, c).try_into().unwrap());
                    b = num / den;
                },
                Instr::Cdv => {
                    let num = a;
                    let den = 2isize.pow(resolve_combo(operand, a, b, c).try_into().unwrap());
                    c = num / den;
                },
            }
            pc += 1;
            if pc >= prog_size {
                return out;
            }
        }
    }

    let nums_is: Vec<_> = nums.iter()
        .map(|num| *num as isize)
        .collect();

    //In this configuration:
    // - Instructions are executed top to bottom, then repeat
    // - A is shifted 3 bits down once per iteration
    // - At the end of last iteration A was 0
    // - therefore at the start of last iteration A was < 8 (and last output value can be extracted)
    // - at the start of n-to-last iteration A was < 8^n (and n-to-last output value can be extracted)
    assert_eq!(instructions[instructions.len() - 1], (Instructions::Jnz, 0));
    assert_eq!(instructions[instructions.len() - 2].0, Instructions::Out);
    assert!(instructions.iter().any(|op| *op == (Instructions::Adv, 3)));
    assert!(instructions.iter().filter(|(instr, _)| *instr == Instructions::Jnz).count() == 1);
    assert!(instructions.iter().filter(|(instr, _)| *instr == Instructions::Out).count() == 1);
    assert!(instructions.iter().filter(|(instr, _)| *instr == Instructions::Adv).count() == 1);

    let mut best_a: Option<isize> = None;
    let b = *registers.get("B").unwrap();
    let c = *registers.get("C").unwrap();

    let mut stack = vec!((0, nums.len() - 1));
    while let Some((last_a, depth)) = stack.pop() {
        for curr_oct in 0..8 {
            let a = last_a << 3 | curr_oct;
            let out = emulate(&instructions, a, b, c);

            if out == nums_is[depth..] {
                if depth == 0 {
                    best_a = Some(best_a.map(|curr_a| curr_a.min(a)).unwrap_or(a));
                }
                else {
                    stack.push((a, depth - 1));
                }
            }
        }
    }

    return best_a;
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let inst1 = Instant::now();
    let part_1_res = solve_part_1(&content);
    let t1 = inst1.elapsed();
    let inst2 = Instant::now();
    let part_2_res = solve_part_2(&content).map(|res| res.to_string()).unwrap_or("No solution".to_owned());
    let t2 = inst2.elapsed();

    println!("Part 1 result: {}, solved in {} ms", part_1_res, t1.as_secs_f64() * 1000.0);
    println!("Part 2 result: {}, solved in {} ms", part_2_res, t2.as_secs_f64() * 1000.0);
}
