#![no_std]
#![no_main]

include!(concat!(env!("OUT_DIR"), "/config.rs"));

use core::panic::PanicInfo;

pub mod allocation;
pub mod arch;
pub mod devicetree;
pub mod drivers;
pub mod scheduler;
pub mod syscalls;

// Kernel Entry
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(feature = "bitmap_allocater")]
    {
        use allocation::bitmap::*;

        let mut bitmap: PageBitmap<MAX_ADDR> = PageBitmap { bitmap: 0 };

        bitmap.clear();
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
