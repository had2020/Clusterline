// Uses FCFS

use core::ptr::write;

use crate::allocation::*;

#[repr(u8)]
pub enum ProcessState {
    Running = 0, // can be created or killed here.
    Waiting = 1,
    Ready = 2,
    Terminated = 3,
}

// TODO add reg save pointer
/// ProcessControlBlock
#[repr(C, align(64))]
pub struct PCB {
    pub pc_offset: usize,
    pub stack_offset: usize,
    pub heap_offset: usize,
    pub text_offset: usize,
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
    pub fn add_process(&mut self, page_size: usize) {
        self.len = self.len.saturating_add(1);
        unsafe {
            /*
            //self.next.write_bytes(0_u8, 1);
            self.next.write(PCB {
                pc_offset: 0,
                stack_offset: 1,
                heap_offset: page_size,
                text_offset: page_size + 1,
                state: ProcessState::Ready;
            }
            */

            let h = core::mem::size_of::<usize>()

            self.next.write(0);
            self.next = self.next.byte_add(1); // stack_offset is always 1
            self.next.write(page_size);
            self.next = self.next.byte_add(core::mem::size_of::<usize>());
            self.next.write(page_size + 1);
            self.next = self.next.byte_add(?)
            self.next.write(ProcessState::Ready as usize);
        };

        //self.next = self.next.byte_add(self.t_size);

        // TODO write process control block
    }
}
