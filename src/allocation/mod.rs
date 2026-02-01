/// bitmap where 0 = free and 1 = used. PAGE_BYTES should be base 2 for proformance!
#[repr(C, align(64))]
pub struct PageBitmap<const PAGE_BYTES: usize> {
    pub bitmap: u128,
}

impl<const PAGE_BYTES: usize> PageBitmap<PAGE_BYTES> {
    pub const fn new() -> Self {
        Self { bitmap: 0 }
    }

    pub fn clear(&mut self) {
        unsafe {
            for i in 0..PAGE_BYTES {
                (i as *mut u32).write(0);
            }
        }
        self.bitmap = 0;
    }

    #[inline(always)]
    pub fn alloc(&mut self) -> usize {
        let addr: usize; // TODO handle fail/full
        let mut i: u8 = 1;
        loop {
            // TODO check
            if (self.bitmap << i).trailing_zeros() < (self.bitmap << (i - 1)).trailing_zeros() {
                addr = i as usize * PAGE_BYTES;
                break;
            }

            i += 1;
        }
        addr
    }

    #[inline(always)]
    pub fn dealloc(page: u32) {
        unsafe {
            // UPDATE with extension and microarch inlines
            for i in 0..PAGE_BYTES {
                ((page as usize + i) as *mut usize).write(0);
            }
        }
    }
}
