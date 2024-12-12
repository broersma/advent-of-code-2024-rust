use std::vec;

use itertools::Itertools;

fn main() {
    let ws: Vec<Vec<_>> = include_str!("../day4.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let h = ws.len() as i32;
    let w = ws[0].len() as i32;

    let mut a_posses: Vec<(i32, i32)> = vec![];

    for y in 0..h {
        for x in 0..w {
            vec![(1, 1), (-1, 1), (-1, -1), (1, -1i32)]
                .iter()
                .map(|(dx, dy)| {
                    (0..3)
                        .map(move |n| (x + dx * n, y + dy * n))
                        .collect::<Vec<_>>()
                })
                .filter(|p: &Vec<(i32, i32)>| {
                    p.into_iter()
                        .all(|(x, y)| *x >= 0 && *x < w && *y >= 0 && *y < h)
                })
                .map(|p| {
                    (
                        p[1],
                        p.into_iter()
                            .map(|(x, y)| ws[y as usize][x as usize])
                            .join(""),
                    )
                })
                .filter(|(_, word)| word == "MAS")
                .for_each(|(a_pos, _)| a_posses.push(a_pos));
        }
    }

    println!(
        "{:?}",
        a_posses
            .into_iter()
            .counts()
            .into_iter()
            .filter(|&(_, c)| c == 2)
            .count()
    );
}
