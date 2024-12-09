fn count_num(list: &Vec<u32>, num: u32) -> u32 {
    list.into_iter().filter(|&&a| a == num).count() as u32
}

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

    println!(
        "{:?}",
        left.into_iter()
            .map(|a| a * count_num(&right, a))
            .sum::<u32>()
    );
}
