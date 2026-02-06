#![no_std]
#![no_main]

include!(concat!(env!("OUT_DIR"), "/config.rs"));

use core::mem;
use core::panic::PanicInfo;

use crate::{
    fs::vfs::RootFileTree,
    scheduler::{FifoQueue, PCB},
};

pub mod allocation;
pub mod arch;
pub mod devicetree;
pub mod drivers;
pub mod fs;
pub mod scheduler;
pub mod syscalls;

const PAGE_BYTES: usize = 512;
const BLOCK_SIZE: usize = 512;

// Kernel Entry
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use allocation::*;
    let mut bitmap: PageBitmap<MAX_ADDR> = PageBitmap { bitmap: 0 };
    bitmap.clear();

    let mut process_queue: FifoQueue<PAGE_BYTES> = FifoQueue {
        page_start: bitmap.alloc(),
        bitmap: 0x00,
    };

    // TODO VFS
    let mut vfs_rt: RootFileTree<BLOCK_SIZE> = RootFileTree {
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
