use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod day1;
mod day2;

fn main() {
    if let Ok(lines) = read_lines("data/day2.txt") {
        day2::day2(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
