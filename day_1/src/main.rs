use std::fs;
use regex::Regex;

fn match_digit(value: &str) -> char {
    match value {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => value.chars().nth(0).unwrap()
    }
}

fn main() {
    let file_path = "src\\input.txt";
    let digits = Regex::new(r"\d|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap();
    // let digits = Regex::new(r"\d").unwrap();
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let mut return_value = 0;
    println!("fuck");
    println!("{}", lines.len());
    for line in lines.iter() {
        println!("What {}", line);
        let all_digits: Vec<&str> = digits.find_iter(line).map(|m| m.as_str()).collect();
        println!("{:?}", all_digits);
        assert!(
            all_digits.len() > 0
        );
        let digits_len = all_digits.len();
        let first_digit = match_digit(all_digits[0]);
        let second_digit = match_digit(all_digits[digits_len -1]);
        let added_value = format!("{}{}", first_digit, second_digit);
        println!("{} {}", added_value, return_value);
        return_value += added_value.parse::<i32>().unwrap();
        println!("{}", return_value)
    }

    println!("The value is:\n{}", return_value);
}