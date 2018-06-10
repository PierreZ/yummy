use std::path::PathBuf;

pub fn open(path: PathBuf) {
    info!(
        "Looking for config file on {:?}",
        path
    );
}
