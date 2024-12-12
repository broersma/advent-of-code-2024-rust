use itertools::Itertools;
use regex::Regex;

fn main() {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let s = include_str!("../day3.txt");

    let s = &(s
        .split("do()")
        .map(|s| s.split("don't()").collect::<Vec<_>>()[0])
        .join(""));

    println!(
        "{:?}",
        regex
            .captures_iter(s)
            .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
            .sum::<u32>()
    );
}
