use std::{collections::HashMap, fs::read_to_string};

fn part_1() {
    let file = read_to_string("../input2.txt").expect("Failed to read file");
    let clean_data = file.trim();
    let hash = HashMap::from([('(', 1), (')', -1)]);
    let mut total = 0;
    clean_data.chars().for_each(|p| {
        let value = hash.get(&p).expect("erroroorr  ");
        total += value
    });
    println!("{}", total)
}
fn part_2() {
    let file = read_to_string("../input2.txt").expect("Failed to read file");
    let clean_data = file.trim();
    let hash = HashMap::from([('(', 1), (')', -1)]);
    let mut total = 0;
    let mut basement = 0;
    clean_data.chars().for_each(|p| {
        let value = hash.get(&p).expect("error");
        basement += 1;
        total += value;
        if total == -1 {
            println!("test {}", basement);
        }
    });
    println!("{}", total)
}
fn main() {
    part_2();
}
