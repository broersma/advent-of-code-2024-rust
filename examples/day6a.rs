use std::collections::HashSet;

fn main() {
    let map: Vec<Vec<_>> = include_str!("../day6.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let h = map.len() as i32;
    let w = map[0].len() as i32;

    let mut guard_pos = (0, 0);
    for y in 0..h {
        for x in 0..w {
            if map[y as usize][x as usize] == '^' {
                guard_pos = (x, y);
            }
        }
    }

    let mut d = (0, -1);

    let mut visited = HashSet::new();

    loop {
        visited.insert(guard_pos);

        let new_pos = (guard_pos.0 + d.0, guard_pos.1 + d.1);

        if new_pos.0 < 0 || new_pos.0 >= w || new_pos.1 < 0 || new_pos.1 >= h {
            break;
        }

        if map[new_pos.1 as usize][new_pos.0 as usize] == '#' {
            d = match d {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!(),
            }
        } else {
            guard_pos = new_pos
        }
    }

    println!("{:?}", visited.len());
}
