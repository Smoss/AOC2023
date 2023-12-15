use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;
fn get_input(file_name: &str) -> Vec<String> {
    // let digits = Regex::new(r"\d").unwrap();
    println!("In file {}", file_name);

    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    return contents.clone().split('\n').map(|line|line.trim().to_string()).collect();
}
fn calc_value(hand_counts: Vec<&i64>) -> i64{
    let mut h_c_clone: Vec<&i64> = hand_counts.clone();
    h_c_clone.sort();
    h_c_clone.reverse();
    match h_c_clone.as_slice() {
        [5, ..] => 6,
        [4, ..] => 5,
        [3, 2, ..] => 4,
        [3, ..] => 3,
        [2, 2, ..] => 2,
        [2, ..] => 1,
        [..] => 0,
    }
}
fn calc_value_with_joker(hand_counts: Vec<&i64>, joker_count: &i64) -> i64{
    let mut h_c_clone: Vec<&i64> = hand_counts.clone();
    h_c_clone.sort();
    h_c_clone.reverse();
    let lead_count =  joker_count + *h_c_clone.get(0).unwrap_or(&&0);
    match lead_count {
        5 => return 6,
        4 => return 5,
        _ => ()
    }
    match joker_count {
        2 => 3,
        1 => match h_c_clone.as_slice() {
            [2, 2, ..] => 4,
            [2, ..] => 3,
            [..] => 1
        }
        _ => calc_value(h_c_clone)
    }
}
fn calc_hand(hand: &String) -> i64 {
    // let hand: &str = line.split(' ').collect::<Vec<&str>>()[0];
    let mut card_counts: HashMap<char, i64> = HashMap::new();
    for char in hand.chars() {
        card_counts.insert(char, card_counts.get(&char).or_else(||{Some(&0)}).unwrap() + 1);
    }
    return calc_value(card_counts.values().collect());
}
fn calc_hand_with_jokers(hand: &String) -> i64 {
    // let hand: &str = line.split(' ').collect::<Vec<&str>>()[0];
    let mut card_counts: HashMap<char, i64> = HashMap::new();
    let mut joker_count = 0 as i64;
    for char in hand.chars() {
        if char == 'J' {
            joker_count += 1
        } else {
            card_counts.insert(char, card_counts.get(&char).or_else(||{Some(&0)}).unwrap() + 1);
        }
    }
    return calc_value_with_joker(card_counts.values().collect(), &joker_count);
}
fn get_hand(line: &String) -> (String, i64)  {
    match line.split(' ').collect::<Vec<&str>>().as_slice() {
        [hand, bid, ..] => (hand.to_string(), bid.parse::<i64>().unwrap()),
        [..] => ("".to_string(), 0)
    }
}
fn get_card_val_1(card: &char) -> i64 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        // 'J' => 1,
        _ => card.to_string().parse::<i64>().unwrap()
    }
}fn get_card_val_2(card: &char) -> i64 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        // 'J' => 1,
        _ => card.to_string().parse::<i64>().unwrap()
    }
}
fn comp_hands_with_vals(hand_vals: &HashMap<String, i64>, get_card_val: &dyn Fn(&char) -> i64, left: &String, right: &String) -> Ordering {
    let left_val = hand_vals.get(left).unwrap();
    let right_val = hand_vals.get(right).unwrap();
    if left_val > right_val {
        return  Ordering::Greater;
    } else if right_val > left_val {
        return Ordering::Less;
    } else {
        for i in 0..left.len() {
            let left_card_val = get_card_val(&left.chars().collect::<Vec<char>>()[i]);
            let right_card_val = get_card_val(&right.chars().collect::<Vec<char>>()[i]);
            if left_card_val != right_card_val {
                return left_card_val.cmp(&right_card_val);
            }
        }
        return Ordering::Less;
    }
}
// Determine the value of each hand, 1 through 7, then store that and write a sort function that uses 
fn main() {
    let lines: Vec<String> = get_input("src\\input.txt");
    let mut hands_with_bid: HashMap<String, i64> = HashMap::new();
    for pair in lines.iter().map(get_hand).collect::<Vec<(String, i64)>>().as_slice() {
        match pair {
            (hand, val) => hands_with_bid.insert(hand.clone(), *val),
            // (..) => Some(0)
        };
    }
    let mut hand_vals_1: HashMap<String, i64> = HashMap::new();
    for hand in hands_with_bid.keys() {
        let val = calc_hand(hand);
        hand_vals_1.insert(hand.clone(), val);
    }
    let mut hand_vals_2: HashMap<String, i64> = HashMap::new();
    
    for hand in hands_with_bid.keys() {
        let val = calc_hand_with_jokers(hand);
        // let val = calc_value_with_joker(hand);
        hand_vals_2.insert(hand.clone(), val);
    }
    let comp_hands_with_val_partial_1: &dyn Fn(&&String, &&String) -> Ordering = &(|left: &&String, right: &&String| -> Ordering {comp_hands_with_vals(&hand_vals_1, &get_card_val_1, left, right)});
    let comp_hands_with_val_partial_2: &dyn Fn(&&String, &&String) -> Ordering = &(|left: &&String, right: &&String| -> Ordering {comp_hands_with_vals(&hand_vals_2, &get_card_val_2, left, right)});
    let mut hands_1: Vec<&String> = hands_with_bid.keys().clone().collect();
    hands_1.sort_by(comp_hands_with_val_partial_1);
    let vals_1: Vec<i64> = hands_1.iter().enumerate().map(|pair| {
        match pair {
            (idx, hand) => (idx + 1) as i64 * hands_with_bid.get(*hand).unwrap()
        }
    }).collect();
    let mut hands_2: Vec<&String> = hands_with_bid.keys().clone().collect();
    hands_2.sort_by(comp_hands_with_val_partial_2);
    let vals_2: Vec<i64> = hands_2.iter().enumerate().map(|pair| {
        match pair {
            (idx, hand) => (idx + 1) as i64 * hands_with_bid.get(*hand).unwrap()
        }
    }).collect();
    // println!("{:?}", hands);
    // println!("{:?}", hand_vals_1);
    println!("Total value is {}", vals_1.iter().sum::<i64>());
    println!("Total value is {}", vals_2.iter().sum::<i64>())
}
