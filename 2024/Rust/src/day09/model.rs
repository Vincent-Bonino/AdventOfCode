use nom::character::complete::i128;
use std::fmt::{write, Debug, Formatter};

const CAPACITY: usize = 20_000; // From input's length

#[derive(Default)]
pub struct Memory {
    pub blocks: Vec<MemoryBlock>,
    pub objects: Vec<MemoryObject>,
}

impl Debug for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result: String = String::with_capacity(self.blocks.len());
        for block in self.blocks.iter() {
            result.push_str(&format!("{:?}", block));
        }
        write!(f, "{}", result)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MemoryObject {
    File(usize, usize), // ID, length
    Free(usize),        // Length
}

impl MemoryObject {
    pub fn length(&self) -> usize {
        match self {
            MemoryObject::File(_id, length) => *length,
            MemoryObject::Free(length) => *length,
        }
    }
}

#[derive(Copy, Clone)]
pub enum MemoryBlock {
    File(usize), // ID
    Free,
}

impl Debug for MemoryBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryBlock::Free => write!(f, "."),
            MemoryBlock::File(id) => write!(f, "{}", id),
        }
    }
}

impl Memory {
    pub fn from_objects(objects: &[MemoryObject]) -> Self {
        let mut blocks: Vec<MemoryBlock> = Vec::with_capacity(CAPACITY);

        for object in objects.iter() {
            for _ in 0..object.length() {
                blocks.push(match object {
                    MemoryObject::File(id, _length) => MemoryBlock::File(*id),
                    MemoryObject::Free(_length) => MemoryBlock::Free,
                });
            }
        }

        Self {
            blocks,
            objects: Vec::from(objects),
        }
    }

    pub fn from_string(value: &str) -> Self {
        let mut blocks: Vec<MemoryBlock> = Vec::with_capacity(CAPACITY);
        let mut objects: Vec<MemoryObject> = Vec::with_capacity(CAPACITY);

        let mut next_file_id: usize = 0;

        for (index, len) in value.trim().chars().enumerate() {
            let length: usize = len.to_string().parse::<usize>().unwrap();
            let block: MemoryBlock = {
                match index % 2 {
                    0 => {
                        let tmp_block = MemoryBlock::File(next_file_id);
                        next_file_id += 1;
                        tmp_block
                    }
                    1 => MemoryBlock::Free,
                    _ => unreachable!(),
                }
            };

            // Add the object
            objects.push(match block {
                MemoryBlock::File(id) => MemoryObject::File(id, length),
                MemoryBlock::Free => MemoryObject::Free(length),
            });

            // Add the memory block enough times
            for _ in 0..length {
                blocks.push(block);
            }
        }

        Self { blocks, objects }
    }

    pub fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(index, block)| match block {
                MemoryBlock::File(id) => index * id,
                MemoryBlock::Free => 0,
            })
            .sum()
    }

    /// Create a new Memory with optimized space usage.
    /// Uses a double-pointer approach.
    pub fn optimize_space(&self) -> Self {
        let length: usize = self.blocks.len();
        let mut optimized_blocks: Vec<MemoryBlock> = Vec::with_capacity(length);

        let mut left_pointer: usize = 0; // Aimed to point at the first free block to use
        let mut right_pointer: usize = length - 1; // Aimed to point at the last file block to move

        while left_pointer <= right_pointer {
            // Move right_pointer if it points to an empty block
            if let MemoryBlock::Free = self.blocks.get(right_pointer).unwrap() {
                right_pointer -= 1;
            }

            // Copy if left_pointer points to a file block
            if let MemoryBlock::File(id) = self.blocks.get(left_pointer).unwrap() {
                optimized_blocks.push(MemoryBlock::File(*id));
                left_pointer += 1;
            }

            // Now:
            //  - left_pointer points to a free block
            //  - right_pointer points to a file block
            // So copy
            if let (MemoryBlock::Free, MemoryBlock::File(id)) = (
                self.blocks.get(left_pointer).unwrap(),
                self.blocks.get(right_pointer).unwrap(),
            ) {
                optimized_blocks.push(MemoryBlock::File(*id));
                left_pointer += 1;
                right_pointer -= 1;
            }
        }

        Memory {
            blocks: optimized_blocks,
            objects: vec![], // Objects do not mean anything is this context
        }
    }
}
