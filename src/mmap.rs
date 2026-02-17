pub struct Mmap {
    pub base: *mut usize,
    pub max: usize,
}

impl Mmap {
    pub fn new() -> Self {
        let check_base: u8 = 0;

        // Ram probe

        let mut offset_4kb: usize = 0;

        loop {}

        Mmap {
            base: check_base as *mut usize,
            max: (), //TODO after Device tree
        }
    }

    pub fn check(idx: usize) {
        //TODO
    }
}
