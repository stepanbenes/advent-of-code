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
                FileBlock::FreeSpace => break,
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

fn get_input() -> Vec<u8> {
    let input = include_str!("../input/day9.txt");
    //let input = "2333133121414131402";
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[allow(dead_code)]
fn print_filesystem(filesystem: &Vec<FileBlock>) {
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
