use std::iter::zip;

pub fn main() {
    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();

    include_str!("../day1.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split("   ")
                .map(|d| d.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .for_each(|ns| {
            left.push(ns[0]);
            right.push(ns[1]);
        });

    left.sort();
    right.sort();

    println!(
        "{:?}",
        zip(left, right).map(|(a, b)| a.abs_diff(b)).sum::<u32>()
    );
}
