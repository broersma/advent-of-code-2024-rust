use regex::Regex;

fn main() {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let s = include_str!("../day3.txt");

    println!(
        "{:?}",
        regex
            .captures_iter(s)
            .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
            .sum::<u32>()
    );
}
