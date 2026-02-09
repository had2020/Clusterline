#![no_std]
#![no_main]

include!(concat!(env!("OUT_DIR"), "/config.rs"));

use core::mem;
use core::panic::PanicInfo;

use crate::{
    fs::vfs::RootFileTree,
    //mmap::Mmap,
    scheduler::{FifoQueue, PCB},
};

pub mod allocation;
pub mod arch;
pub mod devicetree;
pub mod drivers;
pub mod fs;
pub mod scheduler;
pub mod syscalls;
pub mod utils;

extern "C" {
    static _kernel_end: u8; // set by link.ld: highest address of Kernel code
}

pub fn get_free_memory_start() -> usize {
    unsafe { &_kernel_end as *const u8 as usize }
}

// Kernel Entry
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use allocation::*;
    let mut bitmap: PageBitmap = PageBitmap { bitmap: 0 };
    bitmap.clear();

    let sys_page: usize = bitmap.alloc(); // page must at least 512

    let mut process_queue: FifoQueue = FifoQueue {
        page_start: bitmap.alloc(),
        bitmap: 0x00,
    };

    // TODO VFS
    let mut vfs_rt: RootFileTree = RootFileTree {
        page_start: bitmap.alloc(),
    };

    loop {
        process_queue.run_quene();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
