use std::ffi::{c_char, c_int};

unsafe extern "C" {
    pub fn bash_main(argc: c_int, argv: *const *const c_char, env: *const *const c_char) -> c_int;
}
