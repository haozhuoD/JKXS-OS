use hashbrown::HashMap;
use alloc::sync::Arc;
use alloc::string::{ToString, String};
use spin::{Lazy, RwLock};
use fat32_fs::VFile;

static FSIDX: Lazy<RwLock<HashMap<String, Arc<VFile>>>> = 
    Lazy::new(|| RwLock::new(HashMap::new()));

pub fn find_vfile_idx(path: &str) -> Option<Arc<VFile>> {
    FSIDX.read().get(path).map(|vfile| Arc::clone(vfile))
}

pub fn insert_vfile_idx(path: &str, vfile: Arc<VFile>) {
    FSIDX.write().insert(path.to_string(), vfile);
}

pub fn remove_vfile_idx(path: &str) {
    FSIDX.write().remove(path);
}

pub fn print_inner() {
    println!("{:#?}", FSIDX.read().keys());
}
