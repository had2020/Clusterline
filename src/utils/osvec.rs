use crate::allocation::{self, *};

pub struct OsVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize, // Processes only
}

impl<T> OsVec<T> {
    pub fn new() -> Self {
        OsVec {
            ptr: core::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
}
