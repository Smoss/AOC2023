use std::fs;
fn get_input(file_name: &str) -> Vec<String> {
    // let digits = Regex::new(r"\d").unwrap();
    println!("In file {}", file_name);

    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    return contents.clone().split('\n').map(|line|line.trim().to_string()).collect();
}
fn get_nums(line: &String) -> Vec<i64> {
    match line.split(':').collect::<Vec<&str>>().as_slice() {
        [_, line_raw] => line_raw.split(' ').filter(|val| val.len() > 0).map(|val| val.parse::<i64>().unwrap()).collect(),
        [..] => Vec::new()
    }
}
fn get_num(line: &String) -> i64 {
    match line.split(':').collect::<Vec<&str>>().as_slice() {
        [_, line_raw] => line_raw.replace(" ", "").parse::<i64>().unwrap(),
        [..] => 0
    }
}
fn calc_ways(times_line: &String, distances_line: &String) -> i64 {
    let times: Vec<i64> = get_nums(times_line);
    let dists: Vec<i64> = get_nums(distances_line);
    assert!(times.len() == dists.len());
    let mut  tot = 1;
    for i in 0..times.len() {
        let time = times[i];
        let dist = dists[i];
        let mut success_count = 0;
        for i in 1..time {
            success_count += if (time - i) * i > dist {1} else {0} 
        }
        tot *= success_count;
    }
    return tot;
}
fn calc_way(times_line: &String, distances_line: &String) -> i64 {
    let time: i64 = get_num(times_line);
    let dist: i64 = get_num(distances_line);
    // assert!(times.len() == dists.len());
    let mut  tot = 1;
    // for i in 0..times.len() {
    //     let time = times[i];
    //     let dist = dists[i];
    let mut success_count = 0;
    for i in 1..time {
        success_count += if (time - i) * i > dist {1} else {0} 
    }
    return success_count;
        // tot *= success_count;
    // }
    // return tot;
}
fn main() {
    let lines = get_input("src\\input.txt");
    let val = match lines.as_slice() {
        [times, distances, ..]  => calc_ways(&times, &distances),
        [..] => 0,
    };
    let val_2 = match lines.as_slice() {
        [times, distances, ..]  => calc_way(&times, &distances),
        [..] => 0,
    };
    println!("Ways {}", val);
    println!("Ways 2 {}", val_2)
}
