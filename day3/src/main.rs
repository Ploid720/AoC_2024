
use std::fs;

//Regex? A komu to potrzebne?
fn solve_part_1(input: &String) -> u32
{
    return input.chars()
        .fold((0, 0, 0, 0), |curr, c| match curr {
            (0, _, _, res) if c == 'm'                  => (1, 0, 0, res),
            (1, _, _, res) if c == 'u'                  => (2, 0, 0, res),
            (2, _, _, res) if c == 'l'                  => (3, 0, 0, res),
            (3, _, _, res) if c == '('                  => (4, 0, 0, res),
            (4, a, _, res) if c.is_digit(10) && a < 100 => (4, (a * 10) + c.to_digit(10).unwrap(), 0, res),
            (4, a, _, res) if c == ','                  => (5, a, 0, res),
            (5, a, b, res) if c.is_digit(10) && b < 100 => (5, a, (b * 10) + c.to_digit(10).unwrap(), res),
            (5, a, b, res) if c == ')'                  => (0, 0, 0, res + (a * b)),

            (_, _, _, res) => (0, 0, 0, res),
        })
        .3;
}

fn solve_part_2(input: &String) -> u32
{
    return input.chars()
        .fold(('_', true, 0, 0, 0, 0), |curr, c| match curr {
            ( _ , true, 0, _, _, res) if c == 'm'                  => ('m', true, 1, 0, 0, res),
            ('m', true, 1, _, _, res) if c == 'u'                  => ('m', true, 2, 0, 0, res),
            ('m', true, 2, _, _, res) if c == 'l'                  => ('m', true, 3, 0, 0, res),
            ('m', true, 3, _, _, res) if c == '('                  => ('m', true, 4, 0, 0, res),
            ('m', true, 4, a, _, res) if c.is_digit(10) && a < 100 => ('m', true, 4, (a * 10) + c.to_digit(10).unwrap(), 0, res),
            ('m', true, 4, a, _, res) if c == ','                  => ('m', true, 5, a, 0, res),
            ('m', true, 5, a, b, res) if c.is_digit(10) && b < 100 => ('m', true, 5, a, (b * 10) + c.to_digit(10).unwrap(), res),
            ('m', true, 5, a, b, res) if c == ')'                  => ('m', true, 0, 0, 0, res + (a * b)),
            
            ( _ , false, 0, _, _, res) if c == 'd' => ('e', false, 1, 0, 0, res),
            ('e', false, 1, _, _, res) if c == 'o' => ('e', false, 2, 0, 0, res),
            ('e', false, 2, _, _, res) if c == '(' => ('e', false, 3, 0, 0, res),
            ('e', false, 3, _, _, res) if c == ')' => ('_', true , 0, 0, 0, res),

            ( _ , true, 0, _, _, res) if c == 'd'  => ('d', true , 1, 0, 0, res),
            ('d', true, 1, _, _, res) if c == 'o'  => ('d', true , 2, 0, 0, res),
            ('d', true, 2, _, _, res) if c == 'n'  => ('d', true , 3, 0, 0, res),
            ('d', true, 3, _, _, res) if c == '\'' => ('d', true , 4, 0, 0, res),
            ('d', true, 4, _, _, res) if c == 't'  => ('d', true , 5, 0, 0, res),
            ('d', true, 5, _, _, res) if c == '('  => ('d', true , 6, 0, 0, res),
            ('d', true, 6, _, _, res) if c == ')'  => ('_', false, 0, 0, 0, res),
            
            (_, enabled, _, _, _, res) => ('_', enabled, 0, 0, 0, res),
        })
        .5;
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
