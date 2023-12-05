use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn day1(lines: Lines<BufReader<File>>) {
    let mut count: i32 = 0;
    let mut count_day2: i32 = 0;

    for line in lines {
        if let Ok(ip) = line {
            let result = day1_part1(&ip);
            count += result;

            let day2_result = day1_part2(&ip);
            count_day2 += day2_result;
        }
    }
    println!("Part 1: {}", count);
    println!("Part 2: {}", count_day2);
}

pub fn day1_part1(line: &String) -> i32 {
    let nums: String = line.chars().filter(|x| !x.is_alphabetic()).collect();
    let first = nums.chars().next().unwrap();
    let last = nums.chars().last().unwrap();
    return join(first, last);
}

pub fn day1_part2(line: &String) -> i32 {
    const NUM_AS_WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut num_word_map = HashMap::new();
    let mut num_indices: Vec<(usize, char)> = line
        .char_indices()
        .filter_map(|(i, c)| if c.is_numeric() { Some((i, c)) } else { None })
        .collect();

    for (i, &word) in NUM_AS_WORDS.iter().enumerate() {
        num_word_map.insert(word, (i + 1).to_string().chars().next().unwrap());
        let mut start = 0;
        while let Some(pos) = line[start..].find(word) {
            let abs_pos = pos + start;
            // Clone the value here
            if let Some(&number) = num_word_map.get(word) {
                num_indices.push((abs_pos, number));
            }
            start = abs_pos + word.len();
        }
    }

    let first = num_indices.iter().min_by_key(|x| x.0).map(|x| x.1).unwrap();
    let last = num_indices.iter().max_by_key(|x| x.0).map(|x| x.1).unwrap();

    return join(first, last);
}

fn join(first: char, last: char) -> i32 {
    let join = format!("{}{}", first, last);
    let result: Result<i32, _> = join.parse();
    match result {
        Ok(number) => number,
        Err(e) => panic!("Failed to parse integer: {}", e),
    }
}
