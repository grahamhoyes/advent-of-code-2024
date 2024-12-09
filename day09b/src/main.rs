#[derive(Debug, Clone)]
enum Block {
    Free { size: u8 },
    File { id: usize, size: u8 },
}

fn solution(input: &str) -> usize {
    let mut blocks: Vec<Block> = input
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .flat_map(|(id, sizes)| {
            [
                // First block in the pair is a file
                Some(Block::File { id, size: sizes[0] }),
                // Second block is free. Possibly None if there are an odd number
                // of blocks, gets filtered out by .flatten()
                sizes.get(1).map(|&size| Block::Free { size }),
            ]
        })
        .flatten()
        .collect();

    // Nuts to efficiency, we're doing this in-place by inserting into
    // the blocks vector with `.slice()`. This is horribly inefficient,
    // but easier than coming up with a linked list-like data structure
    // that actually compiles.

    let mut read_pos = blocks.len() - 1;
    let mut max_id = blocks.len() - 1;

    while read_pos > 0 {
        match blocks[read_pos] {
            Block::Free { .. } => {}
            Block::File { id, .. } if id > max_id => {}
            Block::File { id, size } => {
                // Search from the very beginning to see if there's a free block
                // big enough to hold this
                for write_pos in 0..read_pos {
                    match blocks[write_pos] {
                        Block::File { .. } => continue,
                        Block::Free { size: free_size } => {
                            #[allow(clippy::comparison_chain)]
                            if free_size == size {
                                blocks[write_pos] = Block::File { size, id };
                                blocks[read_pos] = Block::Free { size };
                                break;
                            } else if free_size > size {
                                blocks[read_pos] = Block::Free { size };

                                let moved_block = Block::File { id, size };
                                let free_block = Block::Free {
                                    size: free_size - size,
                                };

                                // Need to split the block. Here's the super inefficient part!
                                blocks.splice(write_pos..=write_pos, [moved_block, free_block]);

                                // And since we extended the size by one, scoot the read pointer over
                                // to where it should be
                                read_pos += 1;
                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                }
                max_id = id;
            }
        }

        read_pos -= 1;
    }

    let mut checksum = 0usize;
    let mut block_idx = 0usize;

    for block in blocks {
        match block {
            Block::File { id, size } => {
                for i in 0..size as usize {
                    checksum += (block_idx + i) * id;
                }
                block_idx += size as usize;
            }
            Block::Free { size } => {
                block_idx += size as usize;
            }
        }
    }

    checksum
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 2858);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 6413328569890);
    }
}
