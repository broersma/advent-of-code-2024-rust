fn is_correct(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    update.iter().enumerate().all(|(pos, p)| {
        rules
            .iter()
            .filter(|(a, b)| a == p || b == p)
            .all(|(a, b)| {
                if p == a {
                    !update[..pos].contains(b)
                } else {
                    !update[pos..].contains(a)
                }
            })
    })
}

fn main() {
    let parts: Vec<_> = include_str!("../day5.txt").split("\n\n").collect();

    let rules: Vec<(i32, i32)> = parts[0]
        .lines()
        .map(|l| l.split("|").collect())
        .map(|s: Vec<_>| (s[0].parse().unwrap(), s[1].parse().unwrap()))
        .collect();
    let updates: Vec<Vec<i32>> = parts[1]
        .lines()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    println!(
        "{:?}",
        updates
            .iter()
            .filter(|u| is_correct(&u, &rules))
            .map(|u| u[u.len().div_ceil(2) - 1])
            .sum::<i32>()
    );
}
