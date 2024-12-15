use itertools::Itertools;

const FREE: u64 = 9999999999999u64;

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

    let mut blocks: Vec<(u64, u64)> = map
        .into_iter()
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(i, mut c)| vec![(c.next().unwrap(), FREE), (c.next().unwrap(), i as u64)])
        .filter(|(c, _)| *c != 0)
        .collect();

    let mut end: usize = 1;

    while end <= blocks.len() {
        let last_file_position = blocks.len() - end;
        let last_file = blocks[last_file_position];

        if last_file.1 != FREE {
            let free_position = blocks
                .iter()
                .find_position(|(size, id)| *id == FREE && *size >= last_file.0)
                .map(|(position, _)| position)
                .filter(|position| *position < last_file_position);

            if let Some(free_position) = free_position {
                let old_block_size = blocks[free_position].0;
                blocks[last_file_position].1 = FREE;
                blocks.insert(free_position, last_file);

                if old_block_size == last_file.0 {
                    blocks.remove(free_position + 1);
                } else {
                    blocks[free_position + 1].0 -= last_file.0;
                }
            }
        }

        end += 1;
    }

    let mut checksum = 0;
    let mut current_i = 0;
    for (size, id) in blocks {
        for i in current_i..current_i + size {
            if id != FREE {
                checksum += i * id;
            }
        }
        current_i += size;
    }

    println!("{:?}", checksum);
}
