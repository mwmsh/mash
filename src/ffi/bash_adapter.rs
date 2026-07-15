use crate::ffi;
use crate::ffi::ds::CArray;
use std::env::{Args, Vars};
use std::ffi::c_int;

pub fn main(argv: Args, env: Vars) -> i32 {
    let args: CArray = CArray::from_arr(argv.collect());
    let env: CArray = CArray::from_arr(env.map(|(k, v)| format!("{k}={v}")).collect());

    let exit_code: c_int = unsafe { ffi::bash::bash_main(args.len(), args.as_ptr(), env.as_ptr()) };

    exit_code as i32
}
