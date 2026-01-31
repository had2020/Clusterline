// Uses FCFS

use core::{
    ops::Div,
    ptr::{self, write},
};

use crate::allocation::*;

#[repr(u8)]
pub enum ProcessState {
    Running = 0, // can be created or killed here.
    Waiting = 1,
    Ready = 2,
    Terminate = 3,
}

/*
// TODO add reg save pointer
/// ProcessControlBlock
#[repr(C, align(64))]
pub struct PCB {
    pub pc_offset: usize, // TODO shift pc into state to save bytes
    pub stack_offset: usize, // offset starts at 1, grows downwards
    pub heap_offset: usize, // incrementing upwards from always from 1 usize
    pub text_offset: usize, // always page_size + 1 usize
    pub state: ProcessState, // always 1 usize

}

#[repr(align(64))]
pub struct FifoQueue {
    pub len: u16,
    pub next: *mut usize,
    pub t_size: usize,
}

*/

// Total allowed processes 376 512 total btyes just for bitmap
#[repr(align(64))]
pub struct FifoQueue {
    bitmap0: u128,
    bitmap1: u128,
    bitmap2: u128,
    bitmap2_t: u128, // last 8 bytes are reserved for t_usize
}

impl FifoQueue {
    // TODO reallocate if needed ofc. TODO compress with bit shifting
    pub fn add_process(&mut self, page_size: usize, text_size: usize) {
        self.len = self.len + 1;
        let mut next: *mut usize = (self.len as usize * self.t_size as usize) as *mut usize;
        unsafe {
            let dis = core::mem::size_of::<usize>();

            next.write(0_usize); // pc
            next = next.byte_add(dis); // stack_base is always 1

            next.write(1_usize); // stack_base
            next = next.byte_add(dis);

            // TODO handle bigger text_sizes than page
            next.write(page_size - text_size); // heap_base
            next = next.byte_add(dis);

            next.write(text_size); // text_base
            next = next.byte_add(dis);

            next.write(ProcessState::Ready as usize); // ProcessState TODO fix for align
        };
    }

    /*
    pub fn del_process(&mut self, page_size: usize) {
        self.next
    }
    */
}
