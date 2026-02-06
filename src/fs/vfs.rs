// TODO system calls open(), read(), write(), close(), chmod(),

use crate::allocation::{self, *};

#[repr(u8)]
pub enum FileSystemType {
    SFS,
}

pub struct SuperBlock {
    // TODO with text label_name: [u8; 4]
    total_blocks: usize,
    total_inodes: usize,
    block_size: usize,
    inode_size: usize,
    root_inode_ptr: usize,
    filesystem_type: FileSystemType, // TODO type enum
}

pub struct Inode {
    pub file_size: usize,
    pub data_blocks_locat: usize,
}

pub struct Dentry {}

pub struct File {}

pub struct RootFileTree {
    pub start: usize,
    pub blocks: usize,
}
