use std::time::Instant;

#[derive(Debug, PartialEq, Clone)]
enum Block {
    File(i32, usize),
    Empty(usize),
}

fn generate_disk(input: &str) -> Vec<Block> {
    let mut file_id = 0;

    let mut disk: Vec<Block> = Vec::with_capacity(input.len());

    input.chars().for_each(|ch| {
        let len = ch.to_digit(10).unwrap() as usize;
        match disk.last() {
            Some(Block::File(_, _)) => disk.push(Block::Empty(len)),
            _ => {
                disk.push(Block::File(file_id, len));
                file_id += 1;
            }
        }
    });

    disk
}

fn move_all(mut disk: Vec<Block>) -> Vec<Block> {
    let mut new_disk: Vec<Block> = Vec::with_capacity(disk.len() * 2);

    let mut disk = disk.as_mut_slice();

    while let Some((block, next)) = disk.split_first_mut() {
        disk = next;
        match block {
            Block::File(file_id, size) => new_disk.push(Block::File(*file_id, *size)),
            Block::Empty(mut empty_size) => loop {
                match disk.last() {
                    Some(Block::File(file_id, file_size)) => match file_size.cmp(&empty_size) {
                        std::cmp::Ordering::Less => {
                            empty_size -= file_size;
                            new_disk.push(Block::File(*file_id, *file_size));
                            let len = disk.len();
                            disk = &mut disk[0..len - 1];
                        }
                        std::cmp::Ordering::Equal => {
                            new_disk.push(Block::File(*file_id, empty_size));
                            let len = disk.len();
                            disk = &mut disk[0..len - 1];
                            break;
                        }
                        std::cmp::Ordering::Greater => {
                            new_disk.push(Block::File(*file_id, empty_size));
                            *disk.last_mut().unwrap() =
                                Block::File(*file_id, file_size - empty_size);
                            break;
                        }
                    },
                    Some(_) => {
                        let len = disk.len();
                        disk = &mut disk[0..len - 1];
                    }
                    _ => break,
                }
            },
        }
    }

    new_disk
}

fn calculate_checksum(input: &[Block]) -> u64 {
    let mut idx = 0;
    let mut sum = 0;

    for block in input {
        match block {
            Block::File(id, size) => {
                for _ in 0..*size {
                    sum += *id as u64 * idx;
                    idx += 1;
                }
            }
            Block::Empty(size) => idx += *size as u64,
        }
    }

    sum
}

fn solve1(input: &str) -> u64 {
    let disk = generate_disk(input);

    let new_disk = move_all(disk);

    calculate_checksum(&new_disk)
}

fn defragment(mut disk: Vec<Block>) -> Vec<Block> {
    let mut new_disk: Vec<Block> = Vec::with_capacity(disk.len());

    while let Some(block) = disk.pop() {
        match block {
            Block::File(file_id, size) => new_disk.push(Block::File(file_id, size)),
            Block::Empty(mut empty_size) => {
                while let Some(block) = disk.iter_mut().find(|block| match block {
                    Block::File(_, file_size) => *file_size <= empty_size,
                    _ => false,
                }) {
                    new_disk.push(block.clone());
                    if let Block::File(_, size) = *block {
                        empty_size -= size;
                        *block = Block::Empty(size);
                    }
                    if empty_size == 0 {
                        break;
                    }
                }

                if empty_size > 0 {
                    new_disk.push(Block::Empty(empty_size));
                }
            }
        }
    }

    new_disk
}

fn solve2(input: &str) -> u64 {
    let disk = generate_disk(input);

    let disk = disk.into_iter().rev().collect();

    let new_disk = defragment(disk);

    calculate_checksum(&new_disk)
}

const PUZZLE: &str = include_str!("./puzzle.txt");

fn main() {
    let start = Instant::now();
    println!("Result of 1: {}", solve1(PUZZLE));
    println!("Solved 1 in {:?}\n\n", start.elapsed());

    let start = Instant::now();
    println!("Result of 2: {}", solve2(PUZZLE));
    println!("Solved 2 in {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test1() {
        assert_eq!(solve1(EXAMPLE), 1928);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(EXAMPLE), 2858);
    }
}
