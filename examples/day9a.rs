use itertools::{repeat_n, Itertools};

fn main() {
    let mut original_map: Vec<_> = include_str!("../day9.txt")
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect();

    let mut map = vec![0];

    map.append(&mut original_map);

    let mut blocks: Vec<u64> = map
        .into_iter()
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(i, mut c)| {
            let mut v1 = repeat_n(99999999u64, c.next().unwrap() as usize).collect_vec();
            let mut v2 = repeat_n(i as u64, c.next().unwrap() as usize).collect_vec();

            v1.append(&mut v2);

            v1
        })
        .collect();

    let mut start = 0;
    let mut end = blocks.len() - 1;

    loop {
        if blocks[start] != 99999999 {
            start += 1;
        } else {
            blocks[start] = blocks[end];
            blocks[end] = 99999999;
            end -= 1;
        }

        if start > end {
            break;
        }
    }

    println!(
        "{:?}",
        blocks
            .iter()
            .take_while(|&n| *n != 99999999)
            .enumerate()
            .map(|(i, id)| i as u64 * id)
            .sum::<u64>()
    );
}
