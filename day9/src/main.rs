
use std::{fs, iter, time::Instant};

enum Data {
    File{id: usize, position: usize},
    FreeSpace{position: usize}
}

fn solve_part_1(input: &String) -> usize
{
    let (mut disk, files, free_space) = input.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .scan((true, 0, 0), |(is_file, curr_id, curr_pos), size| {
            let ret = (if *is_file {Data::File{id: *curr_id, position: *curr_pos}} else {Data::FreeSpace{position: *curr_pos}}, size);
            if *is_file {
                *curr_id += 1;
            }
            *curr_pos += size;
            *is_file = !*is_file;
            return Some(ret);
        })
        .fold((vec!(), vec!(), vec!()), |(mut disk, mut files, mut free_space), (data, size)| match data {
            Data::File{id, position} => {
                for _ in 0..size {
                    disk.push(Some(id));
                }
                files.push((id, position, size));
                return (disk, files, free_space);
            },
            Data::FreeSpace{position} => {
                for _ in 0..size {
                    disk.push(None);
                }
                free_space.push((position, size));
                return (disk, files, free_space);
            }
        });

    let file_iter = files
        .iter()
        .rev()
        .flat_map(|(id, pos, size)| iter::repeat_n((id, pos + size - 1), *size as usize)
            .scan(0, |offset, (id, pos)| {
                let ret = (*id, pos - *offset);
                *offset += 1;
                return Some(ret);
            }));
    let free_space_iter = free_space
        .iter()
        .flat_map(|(pos, size)| iter::repeat_n(pos, *size as usize)
            .scan(0, |offset, pos| {
                let ret = *pos + *offset;
                *offset += 1;
                return Some(ret);
            }));

    let mut limit = 0;
    for ((id, file_pos), free_space_pos) in file_iter.zip(free_space_iter) {
        if file_pos < free_space_pos {
            limit = free_space_pos;
            break;
        }
        disk[free_space_pos] = Some(id);
        disk[file_pos] = None;
    }

    return disk.iter()
        .take(limit)
        .filter_map(|x| *x)
        .enumerate()
        .map(|(pos, id)| pos * id)
        .sum();
}

fn solve_part_2(input: &String) -> usize
{
    let (mut disk, files, mut free_space) = input.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .scan((true, 0, 0), |(is_file, curr_id, curr_pos), size| {
            let ret = (if *is_file {Data::File{id: *curr_id, position: *curr_pos}} else {Data::FreeSpace{position: *curr_pos}}, size);
            if *is_file {
                *curr_id += 1;
            }
            *curr_pos += size;
            *is_file = !*is_file;
            return Some(ret);
        })
        .fold((vec!(), vec!(), vec!()), |(mut disk, mut files, mut free_space), (data, size)| match data {
            Data::File{id, position} => {
                for _ in 0..size {
                    disk.push(Some(id));
                }
                files.push((id, position, size));
                return (disk, files, free_space);
            },
            Data::FreeSpace{position} => {
                for _ in 0..size {
                    disk.push(None);
                }
                free_space.push((position, size));
                return (disk, files, free_space);
            }
        });

    for (id, file_pos, file_size) in files.iter().rev() {
        let slot = free_space
            .iter_mut()
            .take_while(|(pos, _)| file_pos > pos)
            .find(|(_, size)| size >= file_size);
        let (slot_pos, slot_size) = match slot {
            Some(slot) => slot,
            None => continue
        };

        for i in 0..*file_size {
            disk[*slot_pos + i] = Some(*id);
            disk[file_pos + i] = None;
        }
        *slot_pos = *slot_pos + *file_size;
        *slot_size = *slot_size - *file_size;
    }

    return disk.iter()
        .enumerate()
        .filter_map(|(pos, val)| match val {
            Some(id) => Some(pos * id),
            None => None
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
