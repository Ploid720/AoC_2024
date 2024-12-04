
use std::{fs, collections::HashSet};

fn solve_part_1(input: &String) -> usize
{
    let search = "XMAS";

    let search_vec: Vec<char> = search.chars().collect();
    let key_len = search_vec.len();

    let mx: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mx_ref = &mx;
    let max_inner_len = mx.iter().map(|line| line.len()).max().unwrap_or(0);
    let max_side = mx.len().max(max_inner_len);

    let right = mx.clone();
    let left: Vec<Vec<char>> = right.iter()
        .map(|line| line.iter().rev().copied().collect())
        .collect();
    let up: Vec<Vec<char>> = transpose(&mx)
        .map(|line| line.copied().collect())
        .collect();
    let down: Vec<Vec<char>> = up.iter()
        .map(|line| line.iter().rev().copied().collect())
        .collect();

    let down_right: Vec<Vec<char>> = ((1 - (max_side as isize))..(max_side as isize))
        .map(|y| (0..max_side)
            .map(move |x| (x, y + x as isize))
            .filter_map(|(x, y)| match usize::try_from(y) {
                Ok(uy) => Some((x, uy)),
                _ => None
            })
            .filter(|(x, y)| *y < mx_ref.len() && *x < mx_ref[*y].len())
            .map(|(x, y)| mx_ref[y][x])
            .collect())
        .filter(|v: &Vec<_>| !v.is_empty())
        .collect();

    let up_right: Vec<Vec<char>> = (0..(2 * max_side as isize))
        .map(|y| (0..max_side)
            .map(move |x| (x, y as isize - x as isize))
            .filter_map(|(x, y)| match usize::try_from(y) {
                Ok(uy) => Some((x, uy)),
                _ => None
            })
            .filter(|(x, y)| *y < mx_ref.len() && *x < mx_ref[*y].len())
            .map(|(x, y)| mx_ref[y][x])
            .collect())
        .filter(|v: &Vec<_>| !v.is_empty())
        .collect();

    let up_left: Vec<Vec<char>> = down_right.iter()
        .map(|line| line.iter().rev().copied().collect())
        .collect();
    let down_left: Vec<Vec<char>> = up_right.iter()
        .map(|line| line.iter().rev().copied().collect())
        .collect();

    return [up, down, left, right, up_left, up_right, down_left, down_right]
        .iter()
        .map(|v| v.iter()
            .map(|line| line.windows(key_len)
                .filter(|window| window == &search_vec)
                .count())
            .sum::<usize>())
        .sum();
}

fn solve_part_2(input: &String) -> usize
{
    let search = "MAS";

    let key_len = search.len();

    let mx: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mx_ref = &mx;
    let max_inner_len = mx.iter().map(|line| line.len()).max().unwrap_or(0);
    let max_side = mx.len().max(max_inner_len);

    let down_right: Vec<Vec<_>> = ((1 - (max_side as isize))..(max_side as isize))
        .map(|y| (0..max_side)
            .map(move |x| (x, y + x as isize))
            .filter_map(|(x, y)| match usize::try_from(y) {
                Ok(uy) => Some((x, uy)),
                _ => None
            })
            .filter(|(x, y)| *y < mx_ref.len() && *x < mx_ref[*y].len())
            .map(|(x, y)| ((x, y), mx_ref[y][x]))
            .collect())
        .filter(|v: &Vec<_>| !v.is_empty())
        .collect();

    let up_right: Vec<Vec<_>> = (0..(2 * max_side as isize))
        .map(|y| (0..max_side)
            .map(move |x| (x, y as isize - x as isize))
            .filter_map(|(x, y)| match usize::try_from(y) {
                Ok(uy) => Some((x, uy)),
                _ => None
            })
            .filter(|(x, y)| *y < mx_ref.len() && *x < mx_ref[*y].len())
            .map(|(x, y)| ((x, y), mx_ref[y][x]))
            .collect())
        .filter(|v: &Vec<_>| !v.is_empty())
        .collect();

    let up_left: Vec<Vec<_>> = down_right.iter()
        .map(|line| line.iter().rev().copied().collect())
        .collect();
    let down_left: Vec<Vec<_>> = up_right.iter()
        .map(|line| line.iter().rev().copied().collect())
        .collect();

    let up_right_coords: HashSet<_> = up_right
        .iter()
        .flat_map(|line| line.windows(key_len)
            .filter(|window| window
                .iter()
                .zip(search.chars())
                .all(|((_, a), b)| *a == b)
            )
            .map(|window| window[0].0))
        .collect();

    let down_left_coords: HashSet<_> = down_left
        .iter()
        .flat_map(|line| line.windows(key_len)
            .filter(|window| window
                .iter()
                .zip(search.chars())
                .all(|((_, a), b)| *a == b)
            )
            .map(|window| window[0].0))
        .collect();

    let dr_res = down_right
        .iter()
        .flat_map(|line| line.windows(key_len)
            .filter(|window| {
                let (x, y) = window[0].0;
                return window
                    .iter()
                    .zip(search.chars())
                    .all(|((_, a), b)| *a == b)
                    && (up_right_coords.contains(&(x, y + key_len - 1))
                        || down_left_coords.contains(&(x + key_len - 1, y)))
                })
            .map(|window| window[0].0))
        .count();
    let ul_res = up_left
        .iter()
        .flat_map(|line| line.windows(key_len)
            .filter(|window| {
                let (x, y) = window[0].0;
                return window
                    .iter()
                    .zip(search.chars())
                    .all(|((_, a), b)| *a == b)
                    && (up_right_coords.contains(&(x + 1 - key_len, y))
                        || down_left_coords.contains(&(x, y + 1 - key_len)))
                })
            .map(|window| window[0].0))
        .count();

    return dr_res + ul_res;
}

fn transpose<'iter, Column, Item: 'iter>(columns: &'iter [Column]) 
    -> impl Iterator<Item = impl Iterator<Item = &'iter Item>>
    where &'iter Column: IntoIterator<Item = &'iter Item>
{
    return (0 ..)
        .scan((), move |&mut (), row_idx| {
            let mut col_iter = columns.iter();
            let first_col = col_iter.next()?;
            let first: &'iter Item = first_col
                .into_iter()
                .nth(row_idx)?;
            return Some(Iterator::chain(
                std::iter::once(first),
                col_iter
                    .filter_map(move |column| {
                        column
                            .into_iter()
                            .nth(row_idx)
                    })
            ));
        })
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
