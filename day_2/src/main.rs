use std::collections::HashMap;
use std::fs;

fn check_marble_count<'a>(marble_raw: &'a str, game_state: &HashMap<&'a str, i32>) -> HashMap<&'a str, i32> {
    let mut new_game_state = game_state.clone();
    let marble: Vec<&str> = marble_raw.split(' ').collect();
    match marble.as_slice() {
        [count, color, ..] => if game_state.get(color).unwrap() < &(count.parse::<i32>().unwrap()) {
            new_game_state.insert(color, count.parse::<i32>().unwrap());
        },
        [] => (),
        [..] => ()
    }
    new_game_state
}

fn get_min_hash_map<'a>(revealed_marbles: &Vec<&'a str>) -> HashMap<&'a str, i32> {
    let mut game_state: HashMap<&str, i32> = HashMap::from(
        [
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]
    );
    for marble_set in revealed_marbles {
        let marble_values: Vec<&str> = marble_set.split(", ").collect();
        for marble_raw in marble_values {
            game_state = check_marble_count(marble_raw, &game_state);
        }
    }
    game_state
}

fn check_game_base<F>(revealed_state: &str, function_to_call: F) -> i32 where F: (Fn(&i32, &Vec<&str>) -> i32) {
    match revealed_state.split(':').collect::<Vec<&str>>().as_slice() {
        [game_name, raw_marbles] => function_to_call(
            &game_name.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
            &raw_marbles.trim().split("; ").collect::<Vec<&str>>()
        ),
        [] => 0,
        [..] => 0
    }

}

fn check_game_part_1(game_id: &i32, revealed_marbles: &Vec<&str>) -> i32 {
    let check_state: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let game_state: HashMap<&str, i32> = get_min_hash_map(revealed_marbles);
    let mut works = true;
    for key in check_state.keys() {
        if game_state.get(key) > check_state.get(key) {
            works = false
        }
    }
    if works {
        *game_id
    } else {
        0
    }
}

fn check_game_part_2(_: &i32, revealed_marbles: &Vec<&str>) -> i32 {
    let game_state: HashMap<&str, i32> = get_min_hash_map(revealed_marbles);
    let mut val = 1;
    for min_val in game_state.values().collect::<Vec<&i32>>().iter() {
        val *= *min_val;
    }
    val
}
fn main() {
    let file_path = "src\\input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();
    
    let value_1:i32 = lines.iter().map(
        | line | {check_game_base(line, &check_game_part_1)}        
    ).sum();
    let value_2:i32 = lines.iter().map(
        | line | {check_game_base(line, &check_game_part_2)}        
    ).sum();
    println!("{}", value_1);
    println!("{}", value_2);
}
