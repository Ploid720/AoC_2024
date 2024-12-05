
use std::{fs, collections::{HashSet, HashMap}};

fn solve_part_1(input: &String) -> u32
{
    let inputs = input
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    let mut inputs = inputs.split("\n\n");

    let rules = inputs.next()
        .expect("No ordering definition in input")
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| match line.find("|") {
            Some(ind) => {
                let page = line[..ind].parse::<u32>().unwrap();
                let target = line[(ind + 1)..].parse::<u32>().unwrap();
                return Some((page, target));
            }
            None => None
        })
        .fold(HashMap::new(), |mut map: HashMap<u32, Vec<u32>>, (page, target)| {
            match map.get_mut(&page) {
                Some(list) => list.push(target),
                None => {map.insert(page, vec!(target));()}
            }
            return map;
        });

    let empty = vec!();

    return inputs.next()
        .expect("No data definition in input")
        .lines()
        .filter_map(|line| {
            let mut prev_pages = HashSet::new();
            let pages: Vec<_> = line.split(",")
                .filter_map(|page| page.parse::<u32>().ok())
                .collect();
            if pages.is_empty() {
                return None;
            }

            for page in &pages {
                if rules.get(&page)
                    .unwrap_or(&empty)
                    .iter()
                    .any(|target| prev_pages.contains(target)) {
                    return None;
                }
                prev_pages.insert(page);
            }

            return Some(pages[pages.len() / 2]);
        })
        .sum();
}

fn solve_part_2(input: &String) -> u32
{
    let inputs = input
        .replace("\r\n", "\n")
        .replace("\r", "\n");
    let mut inputs = inputs.split("\n\n");

    let rules = inputs.next()
        .expect("No ordering definition in input")
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| match line.find("|") {
            Some(ind) => {
                let page = line[..ind].parse::<u32>().unwrap();
                let target = line[(ind + 1)..].parse::<u32>().unwrap();
                return Some((page, target));
            }
            None => None
        })
        .fold(HashMap::new(), |mut map: HashMap<u32, Vec<u32>>, (page, target)| {
            match map.get_mut(&page) {
                Some(list) => list.push(target),
                None => {map.insert(page, vec!(target));()}
            }
            return map;
        });

    let empty = vec!();

    return inputs.next()
        .expect("No data definition in input")
        .lines()
        .filter_map(|line| {
            let mut prev_pages = HashSet::new();
            let pages: Vec<_> = line.split(",")
                .filter_map(|page| page.parse::<u32>().ok())
                .collect();
            if pages.is_empty() {
                return None;
            }

            for page in &pages {
                if rules.get(&page)
                    .unwrap_or(&empty)
                    .iter()
                    .any(|target| prev_pages.contains(target)) {
                    return Some(pages);
                }
                prev_pages.insert(page);
            }

            return None;
        })
        .map(|mut pages| {
            let mut prev_pages = HashSet::new();

            'L0: loop {
                for i in 0..pages.len() {
                    let page = pages[i];
                    let target = rules.get(&page)
                        .unwrap_or(&empty)
                        .iter()
                        .find(|target| prev_pages.contains(*target));
                    match target {
                        Some(target) => {
                            let ti = pages
                                .iter()
                                .position(| page| page == target)
                                .unwrap();
                            pages[i] = *target;
                            pages[ti] = page;
                            prev_pages.clear();
                            continue 'L0;
                        }
                        None => {
                            prev_pages.insert(page);
                        }
                    }
                }

                return pages[pages.len() / 2];
            }
        })
        .sum();
}

fn main() {
    let file_path = "input/input.txt";
    let content = fs::read_to_string(file_path)
        .expect(format!("file {} should be present", file_path).as_str());

    let part_1_res = solve_part_1(&content);
    let part_2_res = solve_part_2(&content);

    println!("Part 1 result: {}", part_1_res);
    println!("Part 2 result: {}", part_2_res);
}
