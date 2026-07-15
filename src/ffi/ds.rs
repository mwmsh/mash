use std::ffi::{CString, c_char, c_int};

pub struct CArray {
    data: Vec<CString>,
    ptrs: Vec<*const c_char>,
}

impl CArray {
    pub fn from_arr(v: Vec<String>) -> CArray {
        let data: Vec<CString> = v
            .iter()
            .map(|s| CString::new(s.to_owned()).unwrap())
            .collect();
        let mut ptrs: Vec<*const c_char> = data.iter().map(|s| s.as_ptr()).collect();
        ptrs.push(std::ptr::null());
        Self { data, ptrs }
    }

    pub fn as_ptr(&self) -> *const *const c_char {
        self.ptrs.as_ptr()
    }

    pub fn len(&self) -> c_int {
        self.data.len() as c_int
    }
}
