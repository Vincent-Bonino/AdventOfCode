/// solve_part_one is [`Memory::optimize_space`]
use crate::day09::model::{Memory, MemoryObject};
use itertools::Itertools;
use std::ops::Index;

pub fn solve_part_two(memory: &Memory) -> i128 {
    let mut working_objects: Vec<MemoryObject> = memory.objects.clone();

    let file_ids: Vec<usize> = memory
        .objects
        .iter()
        .filter_map(|obj| match obj {
            MemoryObject::File(id, _length) => Some(*id),
            MemoryObject::Free(_) => None,
        })
        .sorted()
        .rev()
        .collect();

    for file_id in file_ids {
        let file_index: usize = get_file_index(&working_objects, file_id);
        let free_index: Option<usize> = has_space_for_target(&working_objects, file_index);

        if free_index.is_none() {
            continue; // Nothing to do, the file is too big to be moved
        }

        // Move the file to the free space
        let free_index: usize = free_index.unwrap();
        let free_object: &MemoryObject = working_objects.get(free_index).unwrap();
        let free_length: usize = free_object.length();

        let file_object: &MemoryObject = working_objects.get(file_index).unwrap();
        let file_length: usize = file_object.length();

        // Swap the two objects
        working_objects.swap(file_index, free_index);

        if free_length > file_length {
            // Insert a new free object, filling the space not taken by the just-moved file
            // free_index because the swap occurred
            working_objects.insert(
                free_index + 1,
                MemoryObject::Free(free_length - file_length),
            );

            // To avoid some more insert-remove and move all the elements of the vec:

            // 1. Add a new free object, to replace the file object, at the end of the vec
            working_objects.push(MemoryObject::Free(file_length));

            // 2. Remove the old free object, replacing it by the new
            // file_index because the swap already occurred
            // +1 because the insertion already occurred
            working_objects.swap_remove(file_index + 1);
        }
    }

    Memory::from_objects(&working_objects).checksum() as i128
}

/// Get the index of a [`MemoryObject::File`] in a collection of [`MemoryObject`].
fn get_file_index(objects: &[MemoryObject], file_id: usize) -> usize {
    for (index, object) in objects.iter().enumerate() {
        if let MemoryObject::File(id, _length) = object {
            if *id == file_id {
                return index;
            }
        }
    }
    panic!("Not found")
}

/// Determine if `objects` has space for the `i`th element at its left.
/// If the `i`th element is a [`MemoryObject::Free`], it can't be moved.
///
/// Returns None if the object can't be moved.
/// Return Some(idx) otherwise, idx being the index of the free object.
fn has_space_for_target(objects: &[MemoryObject], target_index: usize) -> Option<usize> {
    // Do not move free objects
    if let Some(MemoryObject::Free(_)) = objects.get(target_index) {
        return None;
    }

    // Look to the target's left and look for enough space
    let target_length: usize = objects.get(target_index)?.length();

    for (index, object) in objects.iter().enumerate() {
        if index >= target_index {
            break; // Look only to the left of the target
        }

        if let MemoryObject::File(_, _) = object {
            continue; // Can't move a file on a file
        }

        if object.length() >= target_length {
            return Some(index);
        }
    }

    None
}
