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

    let mut obstacles = HashSet::new();
    for y in 0..h {
        for x in 0..w {
            if map[y as usize][x as usize] == '#' {
                obstacles.insert((x, y));
            }
        }
    }

    let visited = find_visited(&guard_pos, w, h, &obstacles);

    println!(
        "{:?}",
        visited
            .iter()
            .filter(|&p| *p != guard_pos)
            .filter(|&p| {
                let mut new_obstacles = obstacles.to_owned();
                new_obstacles.insert(*p);
                stuck_in_a_loop(&guard_pos, w, h, &new_obstacles)
            })
            .count()
    );
}

fn find_visited(
    guard_pos: &(i32, i32),
    w: i32,
    h: i32,
    obstacles: &HashSet<(i32, i32)>,
) -> HashSet<(i32, i32)> {
    let mut guard_pos = guard_pos.to_owned();

    let mut d = (0, -1);

    let mut visited = HashSet::new();

    loop {
        visited.insert(guard_pos);

        let new_pos = (guard_pos.0 + d.0, guard_pos.1 + d.1);

        if new_pos.0 < 0 || new_pos.0 >= w || new_pos.1 < 0 || new_pos.1 >= h {
            break;
        }

        if obstacles.contains(&new_pos) {
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

    visited
}

fn stuck_in_a_loop(
    guard_pos: &(i32, i32),
    w: i32,
    h: i32,
    obstacles: &HashSet<(i32, i32)>,
) -> bool {
    let mut guard_pos = guard_pos.to_owned();

    let mut d = (0, -1);

    let mut visited = HashSet::new();

    loop {
        visited.insert((guard_pos, d));

        let new_pos = (guard_pos.0 + d.0, guard_pos.1 + d.1);

        if new_pos.0 < 0 || new_pos.0 >= w || new_pos.1 < 0 || new_pos.1 >= h {
            return false;
        } else if visited.contains(&(new_pos, d)) {
            return true;
        }

        if obstacles.contains(&new_pos) {
            d = match d {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!(),
            }
        } else {
            guard_pos = new_pos;
        }
    }
}
