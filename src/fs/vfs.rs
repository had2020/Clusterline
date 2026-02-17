// TODO system calls open(), read(), write(), close(), chmod(),

use core::{str::*, usize};

#[repr(u8)]
pub enum FileSystemType {
    SFS,
}

#[repr(C, align(64))]
pub struct SuperBlock {
    pub label: u64,
    pub total_blocks: usize,
    pub total_inodes: usize,
    pub block_size: usize,
    pub inode_size: usize,
    pub root_inode_ptr: usize,
    pub filesystem_type: FileSystemType, // TODO type enum
}

#[repr(C, align(64))]
pub struct Inode {
    pub filename_packed: u64,
    pub file_size: usize,
    pub data_blocks_locat: usize,
    pub parent_idx: u32,       // index in the global array
    pub first_child_idx: u32,  // for directory traversal
    pub next_sibling_idx: u32, // for directory traversal
    pub data_region: u64,      // start block on disk
    pub size: u32,
}

pub struct RootFileTree {
    pub page_start: usize,
    // TODO after first POST test
}

impl RootFileTree {
    pub fn open() {}
}
