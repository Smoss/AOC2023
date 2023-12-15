use std::cmp::Ordering;
use std::default;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use rayon::prelude::*;
use rayon::prelude::*;
fn get_input(file_name: &str) -> Vec<String> {
    // let digits = Regex::new(r"\d").unwrap();
    // println!("In file {}", file_name);

    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    return contents.clone().split('\n').map(|line|line.trim().to_string()).collect();
}
fn get_adjacencies(point: char) -> HashSet<(i64, i64)> {
        match point {
            '|' => HashSet::from([(0, 1), (0, -1)]),
            '-' => HashSet::from([(1, 0), (-1, 0)]),
            'L' => HashSet::from([(1, 0), (0, -1)]),
            'J' => HashSet::from([(-1, 0), (0, -1)]),
            '7' => HashSet::from([(-1, 0), (0, 1)]),
            'F' => HashSet::from([(1, 0), (0, 1)]),
            'S' => HashSet::from([
                // (1, 0),
                (-1, 0),
                (0, 1),
                // (0, -1),
            ]),
            _ => HashSet::new()
        }
}
fn find_start(lines: &Vec<String>) -> (i64, i64) {
    let mut start: (i64, i64) = (-1, -1);
    for (y, line) in lines.iter().enumerate() {
        for (x, point) in line.chars().collect::<Vec<_>>().iter().enumerate() {
            if *point == 'S' {
                start = (x as i64, y as i64);
            }
        }
        if start != (-1, -1) {
            break;
        }
    }
    return start;
}

fn paint_not_enclosed(lines: &Vec<String>) -> i64 {
    // println!("BREAK");
    let mut all_dirs:Vec<(i64, i64)> = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            if x != 0 || y != 0 {
                all_dirs.push((x as i64, y as i64));
            }
        }
    }
    let unvisited = HashSet::from(['*', ' ']);
    let mut curr_locs: HashSet<(i64, i64)> = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        if unvisited.contains(line.chars().collect::<Vec<char>>().first().unwrap()) {
            curr_locs.insert((0, i as i64));
        }
        if unvisited.contains(line.chars().collect::<Vec<char>>().last().unwrap()) {
            curr_locs.insert(((line.len() - 1) as i64, i as i64));
        }
    }
    for i in 0..lines[0].len(){
        let first_line = lines.first().unwrap();
        let last_line = lines.last().unwrap();
        if  unvisited.contains(&first_line.chars().collect::<Vec<char>>()[i]) {
            curr_locs.insert((i as i64, 0));
        }
        if unvisited.contains(&last_line.chars().collect::<Vec<char>>()[i]) {
            curr_locs.insert((i as i64, (lines.len() - 1) as i64));
        }
    }
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut new_lines = lines.clone();
    while curr_locs.len() > 0 {
        visited.extend(&curr_locs);

        curr_locs = curr_locs.iter().flat_map(|(curr_x, curr_y)| {
            all_dirs.iter().map(|(add_x, add_y)| {(curr_x + add_x, curr_y + add_y)}).collect::<HashSet<(i64,i64)>>()
        }).filter(|(new_x, new_y)| {
            !visited.contains(&(*new_x, *new_y)) && *new_x >= 0 && *new_x < lines[0].len() as i64 && *new_y >= 0 && *new_y < lines.len() as i64
        }).filter(
            |(new_x, new_y)| {
                new_lines[*new_y as usize].chars().collect::<Vec<char>>()[*new_x as usize] == '*'
                || new_lines[*new_y as usize].chars().collect::<Vec<char>>()[*new_x as usize] == ' '
            }
        ).collect();
    }
    let mut enclosed: i64 = 0;
    for y in 0..lines.len() {
        for x in 0..lines.first().unwrap().len(){
            if visited.contains(&(x as i64, y as i64)) {
                let old_line = new_lines[y].clone();
                let mut new_line = old_line.chars().collect::<Vec<char>>();
                let point = new_line[x];
                if point != ' '{
                    new_line[x] = '0';
                    new_lines[y] = new_line.iter().collect::<String>();
                }
            }
        }
    }
    for line in new_lines{
        for point in line.chars() {
            if point == '*' {
                enclosed += 1;
            }
        }
    }
    return enclosed;
}

fn map_pipes_2(old_lines: &Vec<String>) -> i64  {
    let mut check_lines: Vec<String> = old_lines.iter().flat_map(|line| {
        [
            match line.chars().flat_map(|val|{[val, ' ']}).collect::<Vec<char>>().as_slice() {
                [chars @ .., _] => chars.iter().collect::<String>(),
                [] => "".to_string()
            },
            match line.chars().flat_map(|_| {[' ', ' ']}).collect::<Vec<char>>().as_slice() {
                [chars @ .., _] => chars.iter().collect::<String>(),
                [] => "".to_string()
            }
        ]
    }).collect();
    let start = find_start(&check_lines);
    let mut curr_locs: HashSet<(i64, i64)> = HashSet::from([start]);
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    while curr_locs.len() > 0 {
        visited.extend(&curr_locs);
        curr_locs = curr_locs.iter().flat_map(|(curr_x, curr_y)| {
            get_adjacencies(check_lines[*curr_y as usize].chars().collect::<Vec<char>>()[*curr_x as usize]).iter().map(|(add_x, add_y)| {(curr_x + add_x, curr_y + add_y)}).collect::<HashSet<(i64,i64)>>()
        }).filter(|(new_x, new_y)| {!visited.contains(&(*new_x, *new_y)) && *new_x >= 0 && *new_x < check_lines[0].len() as i64 && *new_y >= 0 && *new_y < check_lines.len() as i64}).collect();

        for (x, y) in &curr_locs {
            let old_line = check_lines[*y as usize].clone();
            let mut new_line = old_line.chars().collect::<Vec<char>>();
            let point = new_line[*x as usize];
            if point == ' ' {
                new_line[*x as usize] = if y % 2 == 0 {'-'} else {'|'};
                check_lines[*y as usize] = new_line.iter().collect::<String>();
            }
        }
    }

    for y in 0..check_lines.len() {
        for x in 0..check_lines.first().unwrap().len(){
            if !visited.contains(&(x as i64, y as i64)) {
                let old_line = check_lines[y].clone();
                let mut new_line = old_line.chars().collect::<Vec<char>>();
                let point = new_line[x];
                if point != ' ' {
                    new_line[x] = '*';
                } else {
                    new_line[x] = ' ';
                }
                check_lines[y] = new_line.iter().collect::<String>();
            }
        }
    }
    paint_not_enclosed(&check_lines)
}

fn map_pipes(lines: &Vec<String>) -> i64  {
    let start = find_start(lines);
    let mut steps = -1;
    let mut curr_locs: HashSet<(i64, i64)> = HashSet::from([start]);
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    while curr_locs.len() > 0 {
        visited.extend(&curr_locs);
        curr_locs = curr_locs.iter().flat_map(|(curr_x, curr_y)| {
            get_adjacencies(lines[*curr_y as usize].chars().collect::<Vec<char>>()[*curr_x as usize]).iter().map(|(add_x, add_y)| {(curr_x + add_x, curr_y + add_y)}).collect::<HashSet<(i64,i64)>>()
        }).filter(|(new_x, new_y)| {!visited.contains(&(*new_x, *new_y)) && *new_x >= 0 && *new_x < lines[0].len() as i64 && *new_y >= 0 && *new_y < lines.len() as i64}).collect();
        steps += 1;
    }
    steps
}

fn main() {
    let lines = get_input("src\\input.txt");
    let val_1 = map_pipes(&lines);
    let val_2 = map_pipes_2(&lines);
    println!("First part {}", val_1);
    
    println!("Second part {}", val_2);
}
