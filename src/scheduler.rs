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
    pub stack_pointer: *mut usize,
    pub text_pointer: *mut usize,
    pub task_id: u16,
    pub state: ProcessState,
}

// elements are sized from ProcessControlBlock
#[repr(align(64))]
pub struct FifoQueue {
    pub len: u16,
    pub next: *mut usize,
}

impl FifoQueue {
    // TODO reallocate if needed.
    pub fn add_process(&mut self) {
        self.len.saturating_add(1);
        unsafe {
            self.next.byte_add(1);
        }

        // TODO write process control block
    }
}
