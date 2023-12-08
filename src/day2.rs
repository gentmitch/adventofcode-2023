use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};

const BLUE: i32 = 14;
const GREEN: i32 = 13;
const RED: i32 = 12;

pub fn day2(lines: Lines<BufReader<File>>) {
    let timer = std::time::Instant::now();
    let mut possible: Vec<i32> = Vec::new();
    let mut power_sets: Vec<i32> = Vec::new();

    for (i, line) in lines.enumerate() {
        let id: i32 = (i + 1).try_into().unwrap();
        if let Ok(ip) = line {
            let (isPossible, power_set) = day2_part1(&ip);
            power_sets.push(power_set);

            if isPossible {
                possible.push(id);
            }
        }
    }
    println!("Day 2: {:?}", possible.iter().sum::<i32>());
    println!("Day 2 Power set sum: {:?}", power_sets.iter().sum::<i32>());
    println!("Time: {:?}", timer.elapsed());
}
fn day2_part1(line: &String) -> (bool, i32) {
    let game = line.split(": ").nth(1).unwrap();
    let sets = game.split(";");

    let mut isPossible = true;
    let mut min_set: HashMap<_, i32> = [("red", 0), ("green", 0), ("blue", 0)].into();
    println!("Min: {:?}", &sets.clone().collect::<Vec<&str>>());

    for s in sets {
        let mut colors: HashMap<_, i32> = [("red", RED), ("green", GREEN), ("blue", BLUE)]
            .into_iter()
            .collect();

        let s: Vec<&str> = s.split(", ").collect();

        for p in s {
            let mut sub_set = p.split_ascii_whitespace();
            let mut num: i32 = sub_set.next().unwrap().parse().unwrap();
            let color = sub_set.next().unwrap();
            let entry = colors.entry(color).or_default();

            if let Some(x) = min_set.get_mut(color) {
                if x < &mut num {
                    *x = num;
                }
            }

            if num > *entry {
                isPossible = false;
            }
        }
    }
    let min_set_mult = min_set.iter().map(|(_, v)| v).product::<i32>();
    (isPossible, min_set_mult)
}
