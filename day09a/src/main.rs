use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
enum Block {
    Free { size: u8 },
    File { id: usize, size: u8 },
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Free { size } => {
                write!(f, "{}", ".".repeat(*size as usize))
            }
            Block::File { id, size } => {
                write!(f, "{}", id.to_string().repeat(*size as usize))
            }
        }
    }
}

fn print_blocks(blocks: &[Block]) {
    for block in blocks {
        print!("{}", block);
    }
    println!();
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

    let mut defragged_blocks: Vec<Block> = Vec::new();
    let mut write_pos = 0usize;
    let mut read_pos = blocks.len() - 1;

    // A bit of a janky defrag loop. Fully defragmented blocks are written to
    // `defragged_blocks`, rather than being written in-place: This is to avoid
    // having to split a block which is annoying to do in Rust. The remaining
    // size of free and file blocks is updated in the original `blocks` vector.
    while write_pos < read_pos {
        match blocks[write_pos] {
            // Skip over files that are already in place
            Block::File { .. } => {
                defragged_blocks.push(blocks[write_pos].clone());
                write_pos += 1;
            }
            // Skip free blocks that have been filled up in a previous iteration
            Block::Free { size: 0 } => {
                // This is actually impossible with the way we increment write_pos
                write_pos += 1;
            }
            // If we find free space, try to fill it from a file at the end
            Block::Free { size: free_size } => {
                // Read backwards from read_block_idx to find the next non-empty file block,
                while read_pos > write_pos {
                    match blocks[read_pos] {
                        Block::File { size, .. } if size > 0 => break,
                        _ => read_pos -= 1,
                    }
                }

                // At this point we know we've found a file block to move
                if let Block::File { id, size } = blocks[read_pos] {
                    if free_size >= size {
                        defragged_blocks.push(Block::File { id, size });
                        // Update remaining size on the write block
                        blocks[write_pos] = Block::Free {
                            size: free_size - size,
                        };
                        blocks[read_pos] = Block::Free { size };
                        read_pos -= 1;
                    } else {
                        defragged_blocks.push(Block::File {
                            id,
                            size: free_size,
                        });
                        blocks[write_pos] = Block::Free { size: 0 };
                        blocks[read_pos] = Block::File {
                            id,
                            size: size - free_size,
                        };
                        write_pos += 1;
                    }
                }
            }
        }
    }

    // Write the remaining block if there is one
    if let Block::File { id, size } = blocks[read_pos] {
        defragged_blocks.push(Block::File { id, size });
    }

    let mut checksum = 0usize;
    let mut block_idx = 0usize;

    for block in defragged_blocks {
        match block {
            Block::File { id, size } => {
                for i in 0..size as usize {
                    checksum += (block_idx + i) * id;
                }
                block_idx += size as usize;
            }
            Block::Free { .. } => {
                break;
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

        assert_eq!(res, 1928);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 6378826667552);
    }
}
