use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").expect("File not found!");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("Something went wrong!");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String){
    let row_string = contents.split("\n");
    let rows: Vec<&str> = row_string.collect();

    let mut sum: i32 = 0;
    for row in &rows {
        //println!("{} ", i);
       let number_string = row.split("\t");
       let numbers: Vec<&str> = number_string.collect();
       let mut large: i32 = 0;
       let mut small: i32 = 99999;

       for num in numbers{
            let current: i32 = num.parse().unwrap();
            if current > large{
                large = current;
            }

            if current < small{
                small = current;
            }
       } 

       sum = sum + (large - small);
    }

    println!("Part1: {}", sum);
}

fn part2(contents: &String){
    let row_string = contents.split("\n");
    let rows: Vec<&str> = row_string.collect();

    let mut sum: i32 = 0;
    for row in &rows {
       let number_string = row.split("\t");
       let numbers: Vec<&str> = number_string.collect();

       for x in 0..numbers.len(){
            let current: i32 = numbers[x].parse().unwrap();
            let mut found: bool = false;

            for y in 0..numbers.len() {
                if x == y{
                    continue;
                }

                let current2: i32 = numbers[y].parse().unwrap();

                if current > current2 {
                    if current % current2 == 0{
                        sum = sum + (current / current2);
                        found = true;
                        break;
                    }
                }
            }

            if found {
                break;
            }
       } 
    }

    println!("Part2: {}", sum);
}