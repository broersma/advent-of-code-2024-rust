use std::{
    collections::{HashMap, HashSet},
    vec,
};

use itertools::Itertools;

fn get_anti_nodes_pair(a: &(i32, i32), b: &(i32, i32), w: &i32, h: &i32) -> Vec<(i32, i32)> {
    let d = (a.0 - b.0, a.1 - b.1);

    let mut vec: Vec<(i32, i32)> = vec![];

    let mut n = 0;
    loop {
        let (x, y) = (a.0 + d.0 * n, a.1 + d.1 * n);
        if x < 0 || x >= *w || y < 0 || y >= *h {
            break;
        }

        vec.push((x, y));
        n += 1;
    }

    let d = (-d.0, -d.1);

    n = 0;
    loop {
        let (x, y) = (b.0 + d.0 * n, b.1 + d.1 * n);
        if x < 0 || x >= *w || y < 0 || y >= *h {
            break;
        }

        vec.push((x, y));
        n += 1;
    }

    vec
}

fn get_anti_nodes(positions: &Vec<(i32, i32)>, w: &i32, h: &i32) -> HashSet<(i32, i32)> {
    let pairs: Vec<_> = positions
        .iter()
        .combinations(2)
        .map(|p| (p[0], p[1]))
        .collect();
    return pairs
        .iter()
        .flat_map(|(a, b)| get_anti_nodes_pair(a, b, w, h))
        .collect();
}

fn main() {
    let map: Vec<Vec<_>> = include_str!("../day8.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let h = map.len() as i32;
    let w = map[0].len() as i32;

    let mut frequencies_to_positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            if map[y as usize][x as usize] != '.' {
                let positions = frequencies_to_positions
                    .entry(map[y as usize][x as usize])
                    .or_insert(vec![]);
                (*positions).push((x, y));
            }
        }
    }

    let mut antinode_positions: HashSet<(i32, i32)> = HashSet::new();

    for positions in frequencies_to_positions.values() {
        get_anti_nodes(&positions, &w, &h)
            .iter()
            // TODO remove .filter(|(x, y)| *x >= 0 && *x < w && *y >= 0 && *y < h)
            .for_each(|p| {
                antinode_positions.insert(*p);
            });
    }

    println!("{:?}", antinode_positions.len());
}
