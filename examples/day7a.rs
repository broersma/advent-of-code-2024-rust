use itertools::{repeat_n, Itertools};

fn can_be_true(eq: &(i64, Vec<i64>)) -> bool {
    let ops = vec!['+', '*'];

    let (t, ns) = eq;

    let options = repeat_n(ops.iter(), ns.len() - 1).multi_cartesian_product();

    for option in options {
        let mut nso = ns.to_owned();

        nso.reverse();

        let value = nso.pop().unwrap();

        let mut total = value;

        for op in option {
            let value = nso.pop().unwrap();
            total = match op {
                '*' => total * value,
                '+' => total + value,
                _ => panic!(),
            }
        }

        if total == *t {
            return true;
        }
    }

    false
}

fn main() {
    let eqs: Vec<(i64, Vec<i64>)> = include_str!("../day7.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (t, ns) = l.split(": ").collect_tuple().unwrap();
            (
                t.parse().unwrap(),
                ns.split(" ").map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect();

    println!(
        "{:?}",
        eqs.iter()
            .filter(|eq| can_be_true(eq))
            .map(|(a, _)| a)
            .sum::<i64>()
    );
}
