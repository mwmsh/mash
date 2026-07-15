use crate::bash_build_manager::BashBuildManager;
mod bash_build_manager;

fn main() {
    BashBuildManager::create().run();
}
