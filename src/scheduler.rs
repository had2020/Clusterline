// Uses FCFS

use core::ptr::write;

use crate::allocation::*;

#[repr(u8)]
pub enum ProcessState {
    Running, // can be created or killed here.
    Waiting,
    Ready,
    Terminated,
}

// TODO add reg save pointer
/// ProcessControlBlock
#[repr(C, align(64))]
pub struct PCB {
    pub pc_offset: usize,
    pub stack_offset: usize,
    pub heap_offset: usize,
    pub text_offset: usize,
    pub task_id: u16,
    pub state: ProcessState,
}

// elements are sized from ProcessControlBlock
#[repr(align(64))]
pub struct FifoQueue {
    pub len: u16,
    pub next: *mut usize,
    pub t_size: usize,
}

impl FifoQueue {
    // TODO reallocate if needed.
    pub fn add_process(&mut self) {
        self.len = self.len.saturating_add(1);
        unsafe {
            //self.next.write_bytes(0_u8, 1);
            self.next.write(PCB { stack_offset });

            self.next = self.next.byte_add(self.t_size);
        }

        // TODO write process control block
    }
}
