use std::collections::HashMap;
use std::fs;
use std::collections::HashSet;
fn locate_parts(lines: Vec<&str>) -> HashSet<(i32, i32)> {
    let mut ret: HashSet<(i32, i32)> = HashSet::new();
    
    for line_num in 0..lines.len() {
        let line = lines[line_num].trim();
        for x in 0..line.len() {
            match line.chars().nth(x).unwrap() {
                '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0'|'.' => false,
                _ => { 
                    ret.insert((line_num as i32 + 1, (x as i32) + 1))
                },
            };
        }
    }
    return ret;
}
fn locate_gears(lines: Vec<&str>) -> HashSet<(i32, i32)> {
    let mut ret: HashSet<(i32, i32)> = HashSet::new();
    
    for line_num in 0..lines.len() {
        let line = lines[line_num].trim();
        for x in 0..line.len() {
            match line.chars().nth(x).unwrap() {
                '*' => { 
                    ret.insert((line_num as i32 + 1, (x as i32) + 1))
                },
                _ => false
            };
        }
    }
    return ret;
}
struct PartNumber {
    number: i32,
    x_end: i32,
    y: i32
}
impl PartNumber {
    fn clone(&self) -> Self {
        PartNumber {
            number: self.number.clone(),
            x_end: self.x_end.clone(),
            y: self.y.clone()
        }
    }
}
fn locate_part_numbers(pair: (usize, &&str)) -> Vec<PartNumber> {
    let (line_num, line) = pair;

    let mut parts: Vec<PartNumber> = Vec::new();
    let mut curr_part: String = "".to_string();
    for x in 0..line.len() {
        // println!("{}", x);
        let curr_char = line.chars().nth(x).unwrap();
        match curr_char {
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' => {
                curr_part.push(curr_char);
            },
            _ => if curr_part.len() > 0 {
                        parts.push(PartNumber {
                            number: curr_part.clone().parse::<i32>().unwrap(),
                            x_end: (x) as i32,
                            y: line_num as i32 + 1
                        });
                        curr_part = String::new();
                },
        };
    }
    if curr_part.len() > 0 {
        parts.push(PartNumber {
            number: curr_part.clone().parse::<i32>().unwrap(),
            x_end: (line.len()) as i32,
            y: line_num as i32 + 1
        });
    }
    return parts;
}
fn check_part_is_valid(part: &PartNumber, part_locations: &HashSet<(i32, i32)>) -> bool {
    let x_begin = part.x_end - part.number.to_string().len() as i32;
    let x_end = part.x_end + 1;
    let y_begin = part.y - 1;
    let y_end = part.y + 1;
    for x in x_begin..=x_end {
        for y in y_begin..=y_end {
            if part_locations.contains(&(y, x)) {
                return true
            }
        }
    }
    false
}
fn check_gears_near_part(part: &PartNumber, gear_locs: &HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut nearby_gears:Vec<(i32, i32)> = Vec::new();
    let x_begin = part.x_end - part.number.to_string().len() as i32;
    let x_end = part.x_end + 1;
    let y_begin = part.y - 1;
    let y_end = part.y + 1;
    for x in x_begin..=x_end {
        for y in y_begin..=y_end {
            if gear_locs.contains(&(y, x)) {
                nearby_gears.push((y, x));
            }
        }
    }
    nearby_gears
}
fn check_gears_count(adj: &Vec<PartNumber>) -> i32 {
    match adj.as_slice() {
        [part_a, part_b] => part_a.number * part_b.number,
        [..] => 0
    }
}
fn main() {
    let file_path = "src\\input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();


    let part_numbers: Vec<_> = lines.iter().enumerate().flat_map(locate_part_numbers).collect();
    let part_locations: HashSet<_> = locate_parts(lines.clone());
    let gear_locs: HashSet<_> = locate_gears(lines.clone());

    let mut gear_locs_map: HashMap<(i32, i32), Vec<PartNumber>> = HashMap::new();
    for gear_loc in gear_locs.iter() {
        gear_locs_map.insert(*gear_loc, Vec::new());
    }
    let check_part_is_valid_partial: &dyn Fn(&&PartNumber) -> bool = &(|part: &&PartNumber| -> bool {check_part_is_valid(part, &part_locations)});
    println!("******");
    let total_value: i32 = part_numbers.iter().filter(check_part_is_valid_partial).map(|part| part.number).sum();
    for part_number in part_numbers {
        for gear_loc in check_gears_near_part(&part_number, &gear_locs) {
            gear_locs_map.get_mut(&gear_loc).unwrap().push(part_number.clone());
        }
    }
    let value_2: i32 = gear_locs_map.values().map(check_gears_count).sum();
    println!("{}", total_value);
    println!("{}", value_2)
}
