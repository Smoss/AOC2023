use std::cmp::Ordering;
use std::default;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use rayon::prelude::*;
use num::integer::lcm;
use num::integer;
use rayon::prelude::*;
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
fn calc_disparity(full_seq: &Vec<i64>) ->(Vec<i64>, i64, i64) {
    match full_seq.as_slice() {
        [_, rest @ ..] => {
            let new_disparity = rest.iter().enumerate().map(|(idx, val)| {(val - full_seq[idx])}).collect::<Vec<i64>>();
            let next_tail = new_disparity.last().unwrap().clone();
            let next_head = new_disparity.first().unwrap().clone();
            // println!("{:?} {:?}", full_seq, new_disparity);
            (new_disparity, next_tail, next_head)
        },
        [..] => (Vec::new(), 0, 0)
    }
    // return Vec::new();
}
fn calc_next_val_for_seq(seq: &String) -> i64{
    let full_seq = seq.split(' ').map(|val| {val.parse::<i64>().unwrap()}).collect::<Vec<i64>>();
    let mut tails: Vec<i64> = Vec::new();
    tails.push(*full_seq.last().unwrap());
    let (mut disparity, mut next_tail, _) = calc_disparity(&full_seq);
    while disparity.iter().find(|val: &&i64| {**val != 0}) != None {
        tails.push(next_tail);
        (disparity, next_tail, _) = calc_disparity(&disparity);
        // break;
    };
    return tails.iter().map(|val| {*val}).sum();
}
fn calc_prev_val_for_seq(seq: &String) -> i64{
    let full_seq = seq.split(' ').map(|val| {val.parse::<i64>().unwrap()}).collect::<Vec<i64>>();
    let mut heads: Vec<i64> = Vec::new();
    heads.push(*full_seq.first().unwrap());
    let (mut disparity, _, mut next_head) = calc_disparity(&full_seq);
    while disparity.iter().find(|val: &&i64| {**val != 0}) != None {
        heads.push(next_head);
        (disparity, _, next_head) = calc_disparity(&disparity);
        // break;
    };
    heads.reverse();
    let mut ret: i64 = 0;
    // println!("{}", ret);
    // println!("{:?}", heads);
    for i in 0..heads.len() {
        ret = heads[i] - ret;
        // println!("{} {} {}", ret + heads[i], heads[i], ret)
    }
    // println!("{}", ret);
    // println!("{:?}", heads);
    ret
}
fn main() {
    let lines = get_input("src\\input.txt");
    let val_1: i64 = lines.clone().par_iter().map(calc_next_val_for_seq).sum();
    println!("First part is {}", val_1);
    let val_2: i64 = lines.clone().par_iter().map(calc_prev_val_for_seq).sum();
    println!("Second part is {}", val_2)
}
