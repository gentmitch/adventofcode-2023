use std::fs::File;
use std::io::{BufReader, Lines};

const BLUE: i16 = 14;
const GREEN: i16 = 13;
const RED: i16 = 12;

pub fn day2(lines: Lines<BufReader<File>>) {
    for line in lines {
        if let Ok(ip) = line {
            day2_part1(&ip);
        }
    }
}

fn day2_part1(line: &String) {
    let parts_no_spaces: String = line.replace(" ", "");
    let parts: Vec<&str> = parts_no_spaces.split(":").collect();
    let game: usize = parts[0].replace("Game", "").parse().unwrap();
    let sets: Vec<&str> = parts[1].split(";").collect();

    for set in sets {
        let cubes: Vec<&str> = set.split(',').collect();
        println!("{:?}", cubes)
    }
}
