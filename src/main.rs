#![no_std]
#![no_main]

include!(concat!(env!("OUT_DIR"), "/config.rs"));

use core::mem;
use core::panic::PanicInfo;

use crate::scheduler::{FifoQueue, PCB};

pub mod allocation;
pub mod arch;
pub mod devicetree;
pub mod drivers;
pub mod scheduler;
pub mod syscalls;

// Kernel Entry
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use allocation::*;
    let mut bitmap: PageBitmap<MAX_ADDR> = PageBitmap { bitmap: 0 };
    bitmap.clear();

    let mut kenrel_ptr: usize = bitmap.alloc().unwrap();

    let mut process_queue: FifoQueue = FifoQueue {
        len: 0,
        next: core::ptr::null_mut(),
        t_size: mem::size_of::<PCB>(),
    };

    loop {} // scheduler should handle unless no tasks then TODO
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
