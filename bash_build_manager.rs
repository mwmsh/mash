use std::path::{Path, PathBuf};
use std::process::Command;

mod filesystem {
    use std::fs;
    use std::path::Path;

    pub fn read_utf8_file(name: &Path) -> String {
        let error = format!("Failed to read file {}", name.display());
        fs::read_to_string(name).expect(&error)
    }

    pub fn create_dir(path: &Path, recursive: bool) {
        let error = format!("Failed to create dir {}", path.display());
        if recursive {
            fs::create_dir_all(path).expect(&error);
            return;
        }

        fs::create_dir(path).expect(&error);
    }
}

mod env {
    pub fn read(name: &str) -> String {
        let error = format!("Failed to read env variable {name}");
        std::env::var(name).expect(&error)
    }
}

pub struct BuildConfig {
    src_dir: PathBuf,
    build_dir: PathBuf,
    libs: Vec<String>,
    parallelism: usize,
}

impl BuildConfig {
    pub fn from_env() -> BuildConfig {
        let src_dir = PathBuf::from(env::read("CARGO_MANIFEST_DIR")).join("bash");
        let out_dir = PathBuf::from(env::read("OUT_DIR"));
        let build_dir = out_dir.join("build/bash");
        let parallelism = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        let libs: Vec<String> = [
            "intl", "malloc", "termcap", "glob", "readline", "sh", "tilde",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        Self {
            src_dir,
            build_dir,
            libs,
            parallelism,
        }
    }
}

pub struct CargoLinker;

impl CargoLinker {
    pub fn register_search_dir(&self, dir: &Path) {
        // Printing this in builld.rs leads cargo to use the passed dir as a link search dir
        println!("cargo:rustc-link-search=native={}", dir.display())
    }

    pub fn register_lib(&self, lib: &str) {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    fn register_framework(&self, f: &str) {
        println!("cargo:rustc-link-lib=framework={}", f);
    }
}

impl LinkManifest {
    pub fn from_flags(flags: &str) -> LinkManifest {
        let params: Vec<&str> = flags.split_whitespace().collect();

        let mut libs: Vec<String> = vec![];
        let mut paths: Vec<String> = vec![];
        let mut frameworks: Vec<String> = vec![];
        let mut idx = 0;
        while idx < params.len() {
            let param = &params[idx];
            if param.starts_with("-l") {
                libs.push(param[2..].to_string());
            } else if param.starts_with("lib/") {
                libs.push(param.to_string());
            } else if param.starts_with("-L") {
                paths.push(param[2..].to_string());
            } else if param.starts_with("-Wl,-framework") {
                idx += 1;
                if idx >= params.len() {
                    panic!("End of params encountered before framework name");
                }
                let param = &params[idx];
                frameworks.push(param[4..].to_string())
            }

            idx += 1;
        }

        LinkManifest {
            libs,
            paths,
            frameworks,
        }
    }
}

pub struct LinkManifest {
    libs: Vec<String>,
    paths: Vec<String>,
    frameworks: Vec<String>,
}

pub struct BashBuildManager {
    config: BuildConfig,
    linker: CargoLinker,
}

impl BashBuildManager {
    pub fn create() -> Self {
        BashBuildManager::new(BuildConfig::from_env(), CargoLinker {})
    }

    pub fn new(config: BuildConfig, linker: CargoLinker) -> Self {
        Self { config, linker }
    }
    pub fn is_configured(&self) -> bool {
        self.config.build_dir.join("Makefile").exists()
    }

    pub fn configure(&self) {
        filesystem::create_dir(&self.config.build_dir, true);

        let status = Command::new(self.config.src_dir.join("configure"))
            .current_dir(&self.config.build_dir)
            .status()
            .expect("failed to execute configure");

        assert!(status.success(), "configure returned a non-zero exit code");
    }

    fn make_internal(&self, path: &Path) {
        let status = Command::new("make")
            .arg(format!("-j{}", self.config.parallelism))
            .current_dir(path)
            .status()
            .expect("failed to execute make");

        assert!(
            status.success(),
            "make returned a non-zero exit code {}",
            status.code().unwrap()
        )
    }

    pub fn run(&self) {
        self.build();
        self.link();
    }

    fn build(&self) {
        if !self.is_configured() {
            self.configure()
        }
        self.make_bash();
        self.make_libs();
    }

    fn link(&self) {
        println!("Linking mash...");
        let flags = filesystem::read_utf8_file(&self.config.build_dir.join("flags.txt"));
        println!("Flags.txt:");
        println!("{flags}");
        let manifest = LinkManifest::from_flags(&flags);

        self.linker.register_search_dir(&self.config.build_dir);
        self.linker.register_lib("bash");

        for lib in manifest.libs {
            self.linker.register_lib(&lib);
        }

        for dir in manifest.paths {
            let p = PathBuf::from(&dir);
            if p.is_relative() {
                self.linker
                    .register_search_dir(&self.config.build_dir.join(&dir).canonicalize().unwrap());
                continue;
            }

            self.linker.register_search_dir(&p);
        }

        for framework in manifest.frameworks {
            self.linker.register_framework(&framework)
        }
    }

    fn make_bash(&self) {
        self.make_internal(&self.config.build_dir);
    }

    fn make_libs(&self) {
        for lib in &self.config.libs {
            self.make_internal(&self.config.build_dir.join("lib").join(lib))
        }
    }
}
