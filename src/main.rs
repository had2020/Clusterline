#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod allocation;
pub mod arch;
pub mod drivers;

// Kernel Entry
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(feature = "bitmap_allocater")]
    {
        use allocation::bitmap::*;

        #[cfg(feature = "")]
        let mut bitmap: PageBitmap = new_pagebitmap!();
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
