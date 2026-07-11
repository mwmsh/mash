use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn make(path: &PathBuf, parallelism: i32) {
    Command::new("make")
        .arg(format!("-j{parallelism}"))
        .current_dir(&path)
        .status()
        .unwrap();
}

pub fn make_all(paths: Vec<&PathBuf>, parallelism: i32) {
    for p in paths {
        make(p, parallelism)
    }
}

pub fn is_configured(build_dir: &PathBuf) -> bool {
    return build_dir.join("Makefile").exists();
}

pub fn configure(build_dir: &PathBuf) {
    Command::new("configure")
        .current_dir(&build_dir)
        .status()
        .unwrap();
}

fn main() {
    let bash_build_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("build/bash");

    fs::create_dir_all(&bash_build_dir).unwrap();

    if !is_configured(&bash_build_dir) {
        configure(&bash_build_dir)
    }

    make_all(
        vec![
            &bash_build_dir,
            &bash_build_dir.join("lib/intl"),
            &bash_build_dir.join("lib/malloc"),
            &bash_build_dir.join("lib/termcap"),
        ],
        12,
    );
}
