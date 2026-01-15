// Uses FCFS

use crate::allocation::*;

#[repr(u8)]
pub enum ProcessState {
    Running, // can be created or killed here.
    Runable,
    Locked,
}

#[repr(align(64))]
pub struct ProcessControlBlock {
    pub stack_pointer: usize,
    pub task_id: u64,
    pub state: ProcessState,
}

#[repr(align(64))]
pub union FifoQueue {
    pub len: u16,
    pub element_size: u8,
    next_ptr: usize,
}

pub impl FifoQueue {
    pub fn grow() {} 
}
