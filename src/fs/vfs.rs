// TODO system calls open(), read(), write(), close(), chmod(),

#[repr(u8)]
pub enum FileSystemType {
    SFS,
}

pub struct SuperBlock {
    // TODO with text label_name: [u8; 4]
    pub total_blocks: usize,
    pub total_inodes: usize,
    pub block_size: usize,
    pub inode_size: usize,
    pub root_inode_ptr: usize,
    pub filesystem_type: FileSystemType, // TODO type enum
}

pub struct Inode {
    pub file_size: usize,
    pub data_blocks_locat: usize,
}

// Super compress latin chars
// All 26 chars have a number 
// going up from 2, A being the first at 2
// 1 in any case in a breakaway for the next letter
// TODO use the RLatinCompress crate .

pub struct Dentry {
    name: u128,
}

pub struct File {
    name: u128,
}

pub struct RootFileTree<const BLOCK_SIZE: usize> {
    pub page_start: usize,
}

impl RootFileTree<const BLOCK_SIZE: usize> {
    pub fn open(name: Sting) {
        let mut file_ptr: *&mut 
    }  
}
