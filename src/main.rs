#![no_std]
#![no_main]

include!(concat!(env!("OUT_DIR"), "/config.rs"));

use core::panic::PanicInfo;

pub mod allocation;
pub mod arch;
pub mod devicetree;
pub mod drivers;

// Kernel Entry
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(feature = "bitmap_allocater")]
    {
        use allocation::bitmap::PageBitmap;

        let mut bitmap: PageBitmap<MAX_ADDR> = PageBitmap { bitmap: 0 };
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
