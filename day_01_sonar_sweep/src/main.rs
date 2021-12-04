use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file = fs::read_to_string(&file_name).unwrap();
    let lines: Vec<&str> = file.split('\n').collect();
    let depths: Vec<i32> = lines.into_iter().map(|e| {e.parse::<i32>().unwrap()}).collect();

    println!("part 1: {}", part_1(&depths));
    println!("part 2: {}", part_2(&depths));
}

fn part_1(depths: &[i32]) -> i32 {
    let mut count = 0;
    let mut last_depth = i32::MAX;
    for &depth in depths {
        if last_depth < depth {
            count += 1;
        }
        last_depth = depth;
    }
    count
}

fn part_2(depths: &[i32]) -> i32 {
    let mut count = 0;
    let mut last_depth = i32::MAX;
    for i in 2..depths.len() {
        let depth = depths[i - 2] + depths[i - 1] + depths[i];
        if last_depth < depth {
            count += 1;
        }
        last_depth = depth;
    }
    count
}
