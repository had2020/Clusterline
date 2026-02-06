// TODO system calls open(), read(), write(), close(), chmod(),

use core::{str::*, usize};

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

pub struct Dentry {
    id: usize,
}

pub struct File {
    id: usize,
}

pub struct RootFileTree<const BLOCK_SIZE: usize> {
    pub page_start: usize,
}

impl RootFileTree<const BLOCK_SIZE: usize> {
    pub fn open(*&mut Dentry, cd: u16) {
        let mut file_ptr: *&mut  
    }  
}
