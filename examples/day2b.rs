use itertools::Itertools;

fn is_safe(levels: &Vec<&u32>) -> bool {
    let diffs = levels
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| *a as i32 - *b as i32)
        .collect::<Vec<_>>();

    diffs.iter().all(|d: &i32| 1 <= d.abs() && d.abs() <= 3)
        && (diffs.iter().all(|d| *d > 0) || diffs.iter().all(|d| *d < 0))
}

fn is_safe_with_dampener(levels: &Vec<u32>) -> bool {
    levels
        .into_iter()
        .combinations(levels.len() - 1)
        .any(|c| is_safe(&c))
}

pub fn main() {
    let reports = include_str!("../day2.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(" ")
                .map(|d| d.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        });

    println!(
        "{:?}",
        reports.filter(|r| is_safe_with_dampener(&r)).count()
    );
}
