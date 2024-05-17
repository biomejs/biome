use crossbeam::channel::{unbounded, Receiver, Sender};
use rustc_hash::FxHashSet;
use std::path::PathBuf;
use std::sync::RwLock;

/// File paths interner cache
///
/// The path interner stores an instance of [PathBuf]
pub struct PathInterner {
    storage: RwLock<FxHashSet<PathBuf>>,
    handler: Sender<PathBuf>,
}

impl PathInterner {
    pub fn new() -> (Self, Receiver<PathBuf>) {
        let (send, recv) = unbounded();
        let interner = Self {
            storage: RwLock::new(FxHashSet::default()),
            handler: send,
        };

        (interner, recv)
    }

    /// Insert the path.
    /// Returns `true` if the path was not previously inserted.
    pub fn intern_path(&self, path: PathBuf) -> bool {
        let result = self.storage.write().unwrap().insert(path.clone());
        if result {
            self.handler.send(path).ok();
        }
        result
    }
}
