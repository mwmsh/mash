use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct BashBuildManager {
    src_dir: PathBuf,
    build_dir: PathBuf,
    libs: Vec<String>,
    parallelism: u32,
}

impl BashBuildManager {
    pub fn new(src_dir: PathBuf, build_dir: PathBuf, libs: Vec<String>, parallelism: u32) -> Self{
        return Self{
            src_dir,
            build_dir,
            libs,
            parallelism
        }
    }
    pub fn is_configured(&self) -> bool {
        return self.build_dir.join("Makefile").exists();
    }

    pub fn configure(&self) {
        Command::new(&self.src_dir.join("configure"))
            .current_dir(&self.build_dir)
            .status()
            .unwrap();
    }

    fn make_internal(&self, path: &PathBuf) {
        Command::new("make")
            .arg(format!("-j{}", self.parallelism))
            .current_dir(&path)
            .status()
            .unwrap();
    }

    pub fn build(&self) {
        fs::create_dir_all(&self.build_dir).unwrap();
        if !self.is_configured() {
            self.configure()
        }
        self.make_bash();
        self.make_libs();
    }
    pub fn make_bash(&self) {
        self.make_internal(&self.build_dir);
    }

    pub fn make_libs(&self) {
        for lib in &self.libs {
            self.make_internal(&self.build_dir.join("lib").join(lib))
        }
    }
}
