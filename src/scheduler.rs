// Uses FCFS

use core::{
    ops::Div,
    ptr::{self, write},
};

use crate::allocation::{self, *};

#[repr(u8)]
#[derive(Clone, Copy)]
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

#[repr(C, align(64))]
pub struct PCB {
    virtual_head: usize,
    pc: usize,
    stack_head: usize, // stack_base is always 0
    heap_head: usize,
    text_head: usize,    // heap base is one less
    state: ProcessState, // text base is one less
}

/// Bitmap process allocation with 128 max processes.
#[repr(align(64))]
pub struct FifoQueue<const PAGE_BYTES: usize> {
    page_start: usize,
    bitmap: u128,
}

impl<const PAGE_BYTES: usize> FifoQueue<PAGE_BYTES> {
    pub fn add_process(&mut self, text_bytes: usize, pb: PageBitmap<PAGE_BYTES>) {
        let mut i: usize = 1;

        loop {
            if (self.bitmap << i).trailing_zeros() < (self.bitmap << (i - 1)).trailing_zeros() {
                i += self.page_start;
                break;
            }

            i += 1;
        }

        let mut next: *mut PCB = i as *mut PCB;

        unsafe {
            next = next.byte_add(core::mem::size_of::<PCB>());
            next.write(PCB {
                virtual_head: allocation::PageBitmap::alloc(&mut self)
                pc: 0,
                stack_head: 0,
                heap_head: PAGE_BYTES - text_bytes,
                text_head: (PAGE_BYTES - text_bytes) - 1,
                state: ProcessState::Ready,
            });
        }

        /*
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
        */
    }

    pub fn del_process(&mut self, page_size: usize) {}
}
