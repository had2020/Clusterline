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

pub struct FifoQueue {
    pub len: u32,
    pub element_size: // TODO perhaps OS vec type?
}
