use std::cmp::Ordering;
use std::default;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use rayon::prelude::*;
use num::integer::lcm;
use num::integer;
#[derive(Debug)]
struct Node {
    loc: String,
    left: String,
    right: String
}
fn get_input(file_name: &str) -> Vec<String> {
    // let digits = Regex::new(r"\d").unwrap();
    println!("In file {}", file_name);

    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    return contents.clone().split('\n').map(|line|line.trim().to_string()).collect();
}
fn get_directions(lines: &Vec<String>) -> Vec<&String> {
    match lines.as_slice() {
        [_, _, dirs @ ..] => dirs.iter().collect(),
        [..] => Vec::new(),
    }
}
fn get_l_r(line: &String) -> (String, String, String) {
    match line.split(" = ").collect::<Vec<&str>>().as_slice() {
        [source, dests] => match dests.split(", ").collect::<Vec<&str>>().as_slice() {
            [l, r] => (source.to_string(), l.replace("(", ""), r.replace(")", "")),
            [..] => ("".to_string(), "".to_string(), "".to_string())
        },
        [..] => ("".to_string(), "".to_string(), "".to_string())
    }
}
fn solve_1(lines: &Vec<String>) -> i64 {
    let dirs: Vec<char> = lines.get(0).unwrap().chars().collect();
    let nodes_raw = get_directions(&lines);
    let mut nodes: HashMap<String, Node>= HashMap::new();
    for dir in nodes_raw {
        match get_l_r(dir) {
            (source, l, r) => {
                nodes.insert(source.clone(), Node {
                    loc: source,
                    left: l,
                    right: r
                });
            }
        }
    }
    let mut curr_node = "AAA";
    let mut steps: i64 = 0;
    while curr_node != "ZZZ" {
        curr_node = match dirs.get((steps % dirs.len() as i64) as usize).unwrap() {
            'L' => nodes.get(curr_node).unwrap().left.as_str(),
            _ => nodes.get(curr_node).unwrap().right.as_str(),
        };
        steps += 1;
        // println!("{} {}", curr_node, dirs.get((steps % dirs.len() as i64) as usize).unwrap());
    }
    return steps;
}
fn solve_2(lines: &Vec<String>) -> i64 {
    let dirs: Vec<char> = lines.get(0).unwrap().chars().collect();
    let nodes_raw = get_directions(&lines);
    let mut nodes: HashMap<String, Node>= HashMap::new();
    for dir in nodes_raw {
        match get_l_r(dir) {
            (source, l, r) => {
                nodes.insert(source.clone(), Node {
                    loc: source,
                    left: l,
                    right: r
                });
            }
        }
    }
    let mut curr_nodes: Vec<&String> = nodes.keys().filter(|loc| {loc.ends_with('A')}).collect();
    let mut node_index_rate: HashMap<usize, i64> = HashMap::new();
    // for i in 0..curr_nodes.len() {
    //     node_index_rate
    // }
    let mut steps: i64 = 0;
    // let expected_size = curr_nodes.len();
    while curr_nodes.iter().find(|loc| {!loc.ends_with('Z')}) != None {
        let curr_dir = dirs.get((steps % dirs.len() as i64) as usize).unwrap();
        curr_nodes = curr_nodes.iter().map(|curr_node| {match curr_dir {
            'L' => &nodes.get(*curr_node).unwrap().left,
            _ => &nodes.get(*curr_node).unwrap().right,
        }}).collect();
        steps += 1;
        if steps % 1_000_000 == 0 {
            println!("{}", steps);
            // println!("{}", curr_nodes.len());
            println!("{:?}", node_index_rate);
        }
        for pair in curr_nodes.iter().enumerate() {
            match pair {
                (idx, loc) => {if loc.ends_with('Z') {node_index_rate.entry(idx).or_insert(steps);}}
            }
        }
        if node_index_rate.len() >= curr_nodes.len() {
            break;
        }
        // println!("{} {}", curr_node, dirs.get((steps % dirs.len() as i64) as usize).unwrap());
    }
    let mut tot = 1;
    for i in node_index_rate.values() {
        tot = lcm(tot, *i);
    }
    return tot;
}
fn main() {
    let lines = get_input("src\\input.txt");
    let val_1 = solve_1(&lines);
    let val_2 = solve_2(&lines);
    println!("First Solution: {}", val_1);
    println!("Second Solution: {}", val_2);
}
