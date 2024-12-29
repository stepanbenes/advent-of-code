use std::ops::Range;

enum FileBlock {
    FreeSpace,
    File(u32),
}

impl std::fmt::Debug for FileBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FileBlock::FreeSpace => write!(f, "."),
            FileBlock::File(file_id) => write!(f, "{}", file_id),
        }
    }
}

trait Checksum {
    fn checksum(&self) -> u64;
}

impl Checksum for [FileBlock] {
    fn checksum(&self) -> u64 {
        let mut checksum = 0;
        for (i, value) in self.iter().enumerate() {
            match value {
                FileBlock::File(id) => {
                    checksum += (i as u64) * (*id as u64);
                }
                FileBlock::FreeSpace => {} // skip
            }
        }
        checksum
    }
}

pub fn get_filesystem_checksum() -> u64 {
    let input = get_input();
    let mut filesystem_blocks = get_filesystem_blocks(&input);
    //print_filesystem(&filesystem_blocks);
    while move_last_file_block_to_front(&mut filesystem_blocks) {
        //print!("*");
        //print_filesystem(&filesystem_blocks);
    }
    filesystem_blocks.checksum()
}

pub fn get_filesystem_checksum_without_fragmentation() -> u64 {
    let input = get_input();
    let mut filesystem_blocks = get_filesystem_blocks(&input);
    //print_filesystem(&filesystem_blocks);
    move_entire_files_to_front_empty_spaces(&mut filesystem_blocks);
    filesystem_blocks.checksum()
}

fn get_input() -> Vec<u8> {
    let input = include_str!("../input/day9.txt");
    //let input = "2333133121414131402";
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[allow(dead_code)]
fn print_filesystem(filesystem: &[FileBlock]) {
    for value in filesystem {
        print!("{:?}", value);
    }
    println!()
}

fn get_filesystem_blocks(input: &Vec<u8>) -> Vec<FileBlock> {
    let mut is_free_space = false;
    let mut file_id = 0;
    let mut filesystem_blocks = Vec::new();
    for &value in input {
        if is_free_space {
            for _ in 0..value {
                filesystem_blocks.push(FileBlock::FreeSpace);
            }
        } else {
            for _ in 0..value {
                filesystem_blocks.push(FileBlock::File(file_id));
            }
            file_id += 1;
        }
        is_free_space = !is_free_space;
    }
    filesystem_blocks
}

fn move_last_file_block_to_front(filesystem: &mut [FileBlock]) -> bool {
    // swap last file block with first free space
    let mut last_file_block_index = filesystem.len() - 1;
    while last_file_block_index > 0 {
        match filesystem[last_file_block_index] {
            FileBlock::File(_) => break,
            _ => last_file_block_index -= 1,
        }
    }
    let mut first_free_space_index = 0;
    while first_free_space_index < filesystem.len() {
        match filesystem[first_free_space_index] {
            FileBlock::FreeSpace => break,
            _ => first_free_space_index += 1,
        }
    }
    if last_file_block_index < first_free_space_index {
        return false;
    }
    filesystem.swap(last_file_block_index, first_free_space_index);
    true
}

fn move_entire_files_to_front_empty_spaces(filesystem: &mut [FileBlock]) {
    let max_file_id = filesystem
        .iter()
        .filter_map(|block| match block {
            FileBlock::File(id) => Some(*id),
            _ => None,
        })
        .max()
        .unwrap();
    for file_id in (0..=max_file_id).rev() {
        move_entire_file_to_first_fitting_empty_space(filesystem, file_id);
        //print_filesystem(filesystem);
    }
}

fn move_entire_file_to_first_fitting_empty_space(filesystem: &mut [FileBlock], file_id: u32) {
    let file_slice = find_file_slice(filesystem, file_id);
    let empty_space_slice = find_first_matching_empty_space_slice(filesystem, file_slice.len());
    //println!("empty_space_position_and_length: {:?}", empty_space_slice);
    // swap file blocks with empty space blocks
    if let Some(empty_space_slice) = empty_space_slice {
        if empty_space_slice.start < file_slice.start {
            for i in 0..file_slice.len() {
                filesystem[empty_space_slice.start + i] = FileBlock::File(file_id);
                filesystem[file_slice.start + i] = FileBlock::FreeSpace;
            }
        }
    }
}

fn find_file_slice(filesystem: &[FileBlock], file_id: u32) -> Range<usize> {
    let mut file_start = 0;
    let mut file_length = 0;
    for (i, block) in filesystem.iter().enumerate() {
        match block {
            FileBlock::File(id) => {
                if *id == file_id {
                    if file_length == 0 {
                        file_start = i;
                    }
                    file_length += 1;
                }
            }
            _ => {
                if file_length > 0 {
                    return file_start..file_start + file_length;
                }
                file_length = 0;
            }
        }
    }

    file_start..file_start + file_length
}

fn find_first_matching_empty_space_slice(
    filesystem: &[FileBlock],
    file_size: usize,
) -> Option<Range<usize>> {
    let mut empty_space_start = 0;
    let mut empty_space_length = 0;
    for (i, block) in filesystem.iter().enumerate() {
        match block {
            FileBlock::FreeSpace => {
                if empty_space_length == 0 {
                    empty_space_start = i;
                }
                empty_space_length += 1;
            }
            _ => {
                if empty_space_length >= file_size {
                    return Some(empty_space_start..empty_space_start + empty_space_length);
                }
                empty_space_length = 0;
            }
        }
    }
    if empty_space_length >= file_size {
        return Some(empty_space_start..empty_space_start + empty_space_length);
    }
    None
}
