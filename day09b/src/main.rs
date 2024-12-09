use std::fmt::{Display, Formatter};
use std::rc::Rc;

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

struct BlockNode {
    block: Block,
    next: Option<Rc<BlockNode>>,
}

impl BlockNode {
    /// Split a given node at the given size
    ///
    /// Returns a new node with size `size`, which points to the new free
    /// node. Returns None if the split is invalid.
    fn split(&mut self, size: u8) -> Option<()> {
        match &mut self.block {
            Block::File { .. } => None,
            Block::Free { size: free_size } if *free_size <= size => None,
            Block::Free { size: free_size } => {
                let remaining_size = *free_size - size;
                *free_size = size;

                let new_block = Block::Free {
                    size: remaining_size,
                };
                let new_next = self.next.take();
                let new_node = BlockNode {
                    block: new_block,
                    next: new_next,
                };
                self.next = Some(Rc::new(new_node));
                Some(())
            }
        }
    }
}

struct BlockList {
    head: Option<Rc<BlockNode>>,
    size: usize,
}

impl BlockList {
    fn from_vec(blocks: Vec<Block>) -> Self {
        let mut blocks_iter = blocks.into_iter();
        let mut size = 0;

        let head = blocks_iter.next().map(|first_block| {
            size += 1;
            let mut current = Rc::new(BlockNode {
                block: first_block,
                next: None,
            });

            let head = Rc::clone(&current);

            for block in blocks_iter {
                size += 1;
                let new_node = Rc::new(BlockNode { block, next: None });
                *current.next = Some(new_node);
                current = current.next.unwrap();
            }

            head
        });

        BlockList { head, size }
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

    // Nuts to efficiency, we're doing this in-place by inserting into
    // the blocks vector with `.slice()`. This is horribly inefficient,
    // but easier than coming up with a linked list-like data structure
    // that actually compiles.

    let mut read_pos = blocks.len() - 1;

    while read_pos > 0 {
        match blocks[read_pos] {
            Block::Free { .. } => {
                read_pos -= 1;
            }
            Block::File { id, size } => {
                // Search from the very beginning to see if there's a free block
                // big enough to hold this
                for write_pos in 0..blocks.len() {
                    match blocks[write_pos] {
                        Block::File { .. } => continue,
                        Block::Free { size: free_size } => {
                            if free_size == size {
                                blocks[write_pos] = Block::File { size, id };
                                break;
                            } else if free_size < size {
                                continue;
                            } else {
                                let moved_block = Block::File { id, size };
                                let free_block = Block::Free {
                                    size: free_size - size,
                                };

                                // Need to split the block. Here's the super inefficient part!
                                blocks.splice(write_pos..write_pos, [moved_block, free_block]);
                            }
                        }
                    }
                }
            }
        }
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
            Block::Free { .. } => {
                break;
            }
        }
    }

    checksum
}

fn main() {
    let input = include_str!("../example.txt");
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

        assert_eq!(res, 6378826667552);
    }
}
