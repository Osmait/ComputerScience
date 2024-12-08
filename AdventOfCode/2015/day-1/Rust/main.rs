use std::{
    fs::{read_to_string, File},
    io::Read,
};

fn main() {
    // let file = read_to_string("../input.txt").unwrap();
    let mut input = File::open("../input.txt").unwrap();
    let mut file = String::new();
    input.read_to_string(&mut file).unwrap();
    println!("{}", file)
}
