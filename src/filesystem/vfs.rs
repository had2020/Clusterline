// TODO system calls open(), read(), write(), close(),

// TODO handle large amounts of filesystems in a tree
pub struct MountPoint {
    next_ptr: *mut MountPoint,
}

impl MountPoint {
    pub fn mount() {}

    pub fn unmount() {}
}

pub struct SuperBlock {
    block_size: usize,
    inode_size: usize,
    root_inode_idx: usize,
}

pub struct Inode {
    pub file_size: usize,
    pub data_blocks_locat: usize,
}

pub struct Dentry {}

pub struct File {}
