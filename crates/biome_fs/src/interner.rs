use crossbeam::channel::{unbounded, Receiver, Sender};
use papaya::HashSet;
use rustc_hash::FxBuildHasher;
use std::path::PathBuf;

/// File paths interner cache
///
/// The path interner stores an instance of [PathBuf]
pub struct PathInterner {
    storage: HashSet<PathBuf, FxBuildHasher>,
    handler: Sender<PathBuf>,
}

impl PathInterner {
    pub fn new() -> (Self, Receiver<PathBuf>) {
        let (send, recv) = unbounded();
        let interner = Self {
            storage: HashSet::default(),
            handler: send,
        };

        (interner, recv)
    }

    /// Insert the path.
    /// Returns `true` if the path was not previously inserted.
    pub fn intern_path(&self, path: PathBuf) -> bool {
        let result = self.storage.pin().insert(path.clone());
        if result {
            self.handler.send(path).ok();
        }
        result
    }
}
