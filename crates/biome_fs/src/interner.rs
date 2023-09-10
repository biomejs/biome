use crossbeam::channel::{unbounded, Receiver, Sender};
use indexmap::IndexSet;
use std::path::PathBuf;
use std::sync::RwLock;

/// File paths interner cache
///
/// The path interner stores an instance of [PathBuf]
pub struct PathInterner {
    storage: RwLock<IndexSet<PathBuf>>,
    handler: Sender<PathBuf>,
}

impl PathInterner {
    pub fn new() -> (Self, Receiver<PathBuf>) {
        let (send, recv) = unbounded();
        let interner = Self {
            storage: RwLock::new(IndexSet::new()),
            handler: send,
        };

        (interner, recv)
    }

    /// Insert the path.
    pub fn intern_path(&self, path: PathBuf) -> bool {
        let (_, result) = self.storage.write().unwrap().insert_full(path.clone());
        if result {
            self.handler.send(path).ok();
        }
        result
    }
}
