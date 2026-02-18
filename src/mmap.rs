pub struct Mmap {
    pub base: *mut usize,
    pub max: *mut usize,
}

impl Mmap {
    pub fn new() -> Self {
        let check_base: u8 = 0;

        // Ram probe
        let mut offset_4kb: *mut usize = core::ptr::null_mut();

        loop {
            unsafe {
                offset_4kb = offset_4kb.byte_add(4096);
                offset_4kb.write_volatile(67);
                if offset_4kb.read_volatile() == 67 {
                    break;
                }
                offset_4kb.write_volatile(0);
            }
        }

        Mmap {
            base: check_base as *mut usize,
            max: offset_4kb,
        }
    }

    pub fn check(idx: usize) {
        //TODO
    }
}
