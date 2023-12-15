use std::cmp::Ordering;
use std::fs;
use std::collections::HashSet;
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};

const S2S: &str = "seed-to-soil";
const S2F: &str = "soil-to-fertilizer";
const F2W: &str = "fertilizer-to-water";
const W2L: &str = "water-to-light";
const L2T: &str = "light-to-temperature";
const T2H: &str = "temperature-to-humidity";
const H2L: &str = "humidity-to-location";

#[derive(Debug)]
#[derive(Clone)]
struct Rule {
    start: i64,
    end: i64,
    dest: i64,
}
impl Rule {
    fn to_string(&self) -> String {
        return format!("start:{} end:{} dest:{}", self.start, self.end, self.dest);
    }
    fn matches(&self, val: i64) -> Ordering {
        if self.start <= val && val <= self.end {
            return Ordering::Equal;
        } else if val < self.start {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
    // fn Debug(&self) -> String {
    //     return self.to_string();
    // }
}

fn get_input(file_name: &str) -> Vec<String> {
    // let digits = Regex::new(r"\d").unwrap();
    println!("In file {}", file_name);

    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    return contents.clone().split('\n').map(|line|line.trim().to_string()).collect();
}
fn get_seeds(line: &String) -> Vec<i64>{
    match line.split(": ").collect::<Vec<&str>>().as_slice() {
        [_, seeds_raw] => seeds_raw.split(' ').map(|seed| seed.parse::<i64>().unwrap()).collect(),
        [..] => Vec::new()
    }
}
fn get_seed_ranges(line: &String) -> Vec<(i64, i64)>{
    let seeds= get_seeds(line);
    let mut seed_ranges: Vec<(i64, i64)> = Vec::new();
    for i in 0..seeds.len() / 2 {
        seed_ranges.push((seeds[i * 2], seeds[i * 2 + 1]));
    }
    return seed_ranges;
}
fn generate_rules(lines: &Vec<String>, start_index: usize) -> Vec<Rule> {
    let mut curr_idx: usize = start_index;
    let mut new_rules:Vec<Rule> = Vec::new();
    // println!("{}", start_index);
    while lines[curr_idx].len() > 0 {
        let curr_line = lines[curr_idx].trim();
        // println!("{} {:?}", curr_line, curr_line.split(' ').collect::<Vec<&str>>());
        let vals:Vec<i64> = curr_line.split(' ').map(|val| val.parse::<i64>().unwrap()).collect();
        new_rules.push(
            Rule {
                start: vals[1],
                end: vals[2] + vals[1] - 1,
                dest: vals[0],
            }
        );
        curr_idx += 1;
    }
    new_rules.sort_by(|a, b| {
        if a.start > b.start {
            Ordering::Greater
        }else {
            Ordering::Less
        }
    });
    return new_rules;
}
fn check_rules(start: i64, rules: &Vec<Rule>) -> i64 {
    // let mut ret = start.clone();
    match rules.as_slice().binary_search_by(|rule| rule.matches(start)) {
        Ok(idx) => {
            let rule = &rules[idx];
            // println!("{} {:?} {}", start, rule, (start - rule.start) + rule.dest);
            return (start - rule.start) + rule.dest;
        },
        Err(_) => start
    }
    // for rule in rules {
    //     if rule.start <= start && start <= rule.end {
    //         // println!("{} {} {}", rule.start, rule.end, rule.dest);
    //         ret = (start - rule.start) + rule.dest;
    //         break;
    //     }
    // }
    // ret
}
fn find_min(list: Vec<i64>) -> i64 {
    match list.as_slice() {
        [first, rest @ ..] => {
            let mut min = first;
            for val in rest{
                if val < min {
                    min = val;
                }
            }
            return *min;
        },
        [] => 0
    }
}
fn main() {
    // let file_name = "src\\input.txt";
    let lines = get_input("src\\input.txt");
    let seeds = get_seeds(&lines[0]);
    let seed_ranges = get_seed_ranges(&lines[0]);
    let mut curr_idx = 3;
    let S2S_rules: Vec<Rule> = generate_rules(&lines, 3);
    println!("S2S{:?}\n", S2S_rules);
    curr_idx += S2S_rules.len() + 2;
    let S2F_rules: Vec<Rule> = generate_rules(&lines, curr_idx);
    println!("S2F{:?}\n", S2F_rules);
    curr_idx += S2F_rules.len() + 2;
    let F2W_rules: Vec<Rule> = generate_rules(&lines, curr_idx);
    println!("F2W{:?}\n", F2W_rules);
    curr_idx += F2W_rules.len() + 2;
    let W2L_rules: Vec<Rule> = generate_rules(&lines, curr_idx);
    println!("W2L{:?}\n", W2L_rules);
    curr_idx += W2L_rules.len() + 2;
    let L2T_rules: Vec<Rule> = generate_rules(&lines, curr_idx);
    println!("L2T{:?}\n", L2T_rules);
    curr_idx += L2T_rules.len() + 2;
    let T2H_rules: Vec<Rule> = generate_rules(&lines, curr_idx);
    println!("T2H{:?}\n", T2H_rules);
    curr_idx += T2H_rules.len() + 2;
    let H2L_rules: Vec<Rule> = generate_rules(&lines, curr_idx);
    println!("H2L{:?}\n", H2L_rules);
    let mut final_dests: Vec<i64> = Vec::new();
    // let mut final_dests_ranges: Vec<i64> = Vec::new();
    for seed in seeds {
        // println!("{}", seed);
        let soil_dest = check_rules(seed, &S2S_rules);
        let fert_dest = check_rules(soil_dest, &S2F_rules);
        let water_dest = check_rules(fert_dest, &F2W_rules);
        let light_dest = check_rules(water_dest, &W2L_rules);
        let temp_dest = check_rules(light_dest, &L2T_rules);
        let humidity_dest = check_rules(temp_dest, &T2H_rules);
        let final_dest = check_rules(humidity_dest, &H2L_rules);
        // println!("{}->{}-{}-{}-{}->{}->{}->{}", seed, soil_dest, fert_dest, water_dest, light_dest, temp_dest, humidity_dest, final_dest);
        final_dests.push(final_dest);
    }
    println!("{:?}", find_min(final_dests));
    // let mut receivers:Vec<Receiver<i64>> = Vec::new();
    let mut handles:Vec<JoinHandle<()>> = Vec::new();
    let mut receivers:Vec<Receiver<i64>> = Vec::new();
    for pair in seed_ranges {
        match pair {
            (lo, range) => {
                let (tx, rx) = mpsc::channel::<i64>();
                let S2S_rules_clone = S2S_rules.clone();
                let S2F_rules_clone = S2F_rules.clone();
                let F2W_rules_clone = F2W_rules.clone();
                let W2L_rules_clone = W2L_rules.clone();
                let L2T_rules_clone = L2T_rules.clone();
                let T2H_rules_clone = T2H_rules.clone();
                let H2L_rules_clone = H2L_rules.clone();
                let hi = lo + range;
                let handle = thread::spawn(move || {
                    let mut vals:Vec<i64> = Vec::new();
                    for seed in lo..hi {
        // println!("{}", seed);
                        let soil_dest = check_rules(seed, &S2S_rules_clone);
                        let fert_dest = check_rules(soil_dest, &S2F_rules_clone);
                        let water_dest = check_rules(fert_dest, &F2W_rules_clone);
                        let light_dest = check_rules(water_dest, &W2L_rules_clone);
                        let temp_dest = check_rules(light_dest, &L2T_rules_clone);
                        let humidity_dest = check_rules(temp_dest, &T2H_rules_clone);
                        let final_dest = check_rules(humidity_dest, &H2L_rules_clone);
                        // println!("{}->{}-{}-{}-{}->{}->{}", seed, soil_dest, water_dest, light_dest, temp_dest, humidity_dest, final_dest);
                        // tx.send(final_dest).unwrap();
                        vals.push(final_dest);
                    }
                    tx.send(find_min(vals)).unwrap();
                    println!("Range done for real {} {} {}",lo, hi, hi -lo);
                });
                handles.push(handle);
                receivers.push(rx);
                println!("Range done {} {} {}",lo, hi, hi -lo);
            }
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_dest_ranges:Vec<i64> = receivers.iter().map(|r| r.recv().unwrap()).collect();
    // println!("{:?}", find_min(final_dests));
    println!("{:?}", find_min(final_dest_ranges))
}
