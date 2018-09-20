use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").expect("File not found!");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("Something went wrong!");

    part1(&contents);
}

fn part1(contents: &String){
    let row_string = contents.split("\n");
    let rows: Vec<&str> = row_string.collect();

    for i in &rows {
        println!("{} ", i);
    }
}