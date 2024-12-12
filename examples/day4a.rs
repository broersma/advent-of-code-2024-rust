use itertools::Itertools;

fn main() {
    let ws: Vec<Vec<_>> = include_str!("../day4.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let h = ws.len() as i32;
    let w = ws[0].len() as i32;

    let mut count = 0;

    for y in 0..h {
        for x in 0..w {
            count += vec![
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1i32),
            ]
            .iter()
            .map(|(dx, dy)| {
                (0..4)
                    .map(move |n| (x + dx * n, y + dy * n))
                    .collect::<Vec<_>>()
            })
            .filter(|p| {
                p.into_iter()
                    .all(|(x, y)| *x >= 0 && *x < w && *y >= 0 && *y < h)
            })
            .map(|p| {
                p.into_iter()
                    .map(|(x, y)| ws[y as usize][x as usize])
                    .join("")
            })
            .filter(|word| word == "XMAS")
            .count();
        }
    }

    println!("{}", count);
}
