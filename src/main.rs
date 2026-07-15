mod ffi;

use crate::ffi::bash_adapter;

fn main() {
    let args = std::env::args();
    let vars = std::env::vars();

    let exit_code = bash_adapter::main(args, vars);

    std::process::exit(exit_code);
}
