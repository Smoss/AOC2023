use std::fs;
use std::collections::HashSet;

fn winning_count(line: &&str) -> i32 {
    match line.split(':').collect::<Vec<&str>>().as_slice() {
        [_, cards] => {
            match cards.split('|').collect::<Vec<&str>>().as_slice() {
                [winning_numbers_raw, played_numbers_raw] => {
                    let winning_numbers:HashSet<i32> = winning_numbers_raw.trim().split(' ').filter(|chk| chk.len() > 0).map(|num| num.parse::<i32>().unwrap()).collect::<HashSet<i32>>();
                    let matches:i32 = played_numbers_raw.trim().split(' ').filter(|chk| chk.len() > 0).map(|num| num.parse::<i32>().unwrap()).filter(|num| winning_numbers.contains(num)).collect::<Vec<_>>().len() as i32;
                    matches
                },
                [..] => 0
            }
        },
        [..] => 0
    }
}
fn main() {
    let file_path = "src\\input.txt";
    // let digits = Regex::new(r"\d").unwrap();
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();
    let points: Vec<i32> = lines.iter().map(winning_count).collect();
    let value_1:i32 = points.iter().map(|wins| -> i32 {if *wins > 0 {
        (2 as i32).pow(*wins as u32 - 1)
    } else {0}}).sum();
    let mut cards_count:Vec<i32> = lines.iter().map(|_| 1).collect();
    for i in 0..cards_count.len() {
        let next_idx = i+1;
        let next_end = next_idx + points[i] as usize;
        let card_count = cards_count[i];
        for j in next_idx..next_end as usize {
            if j >= points.len() {break};
            cards_count[j] += card_count as i32;
        }
    }
    println!("Part 1: {}", value_1);
    println!("Part 2: {}", cards_count.iter().sum::<i32>())
}
