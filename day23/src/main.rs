use std::{collections::{HashMap, HashSet}, fs, time::Instant};

fn solve_part_1(input: &String) -> usize
{
    let connections = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split("-"))
        .filter_map(|mut spl| match (spl.next(), spl.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None
        })
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .fold(HashMap::new(), |mut map: HashMap<_, HashSet<_>>, (a, b)| {
            match map.get_mut(&a) {
                Some(con_set) => {
                    con_set.insert(b);
                },
                None => {
                    map.insert(a, HashSet::from([b]));
                }
            }
            return map;
        });

    let mut unvisited: Vec<_> = connections
        .keys()
        .copied()
        .collect();
    let mut triples = HashSet::new();
    while let Some(start) = unvisited.pop() {
        let start_adj_set = connections.get(&start).unwrap();
        start_adj_set.iter()
            .flat_map(|a| start_adj_set.iter()
                .map(|b| (*a, *b))
                .filter(|(a, b)| a != b))
            .filter(|(a, b)| connections.get(a).unwrap().contains(b))
            .for_each(|(a, b)| {
                let mut arr = [start, a, b];
                arr.sort();
                triples.insert(arr);
            });
    }

    return triples.iter()
        .filter(|arr| arr.iter().any(|comp| comp.starts_with("t")))
        .count();
}

fn solve_part_2(input: &String) -> String
{
    let connections = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split("-"))
        .filter_map(|mut spl| match (spl.next(), spl.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None
        })
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .fold(HashMap::new(), |mut map: HashMap<_, HashSet<_>>, (a, b)| {
            match map.get_mut(&a) {
                Some(con_set) => {
                    con_set.insert(b);
                },
                None => {
                    map.insert(a, HashSet::from([b]));
                }
            }
            return map;
        });

    
    fn bk<'a>(r: HashSet<&'a str>, p: HashSet<&'a str>, x: HashSet<&'a str>, connections: &'a HashMap<&'a str, HashSet<&'a str>>) -> HashSet<&'a str> {
        let mut best = HashSet::new();
        if p.is_empty() && x.is_empty() {
            best = r.clone();
        }

        let mut p = p;
        let mut x = x;
        while let Some(&v) = p.iter().next() {
            let nv = connections.get(&v).unwrap();
            let res = bk(
                r.union(&HashSet::from([v])).copied().collect(), 
                p.intersection(nv).copied().collect(), 
                x.intersection(nv).copied().collect(),
                connections);
            if res.len() > best.len() {
                best = res;
            }

            p.remove(v);
            x.insert(v);
        }
        return best; 
    }

    let computers: HashSet<_> = connections
        .keys()
        .copied()
        .collect();

    let mut best: Vec<_> = bk(HashSet::new(), computers, HashSet::new(), &connections)
        .into_iter()
        .collect();
    best.sort();

    return best.into_iter()
        .map(|comp| comp.to_owned())
        .reduce(|a, b| a + "," + &b)
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
