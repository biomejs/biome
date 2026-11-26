use camino::Utf8PathBuf;
use crossbeam::channel::{Receiver, Sender, unbounded};
use papaya::HashSet;
use rustc_hash::FxBuildHasher;

pub type PathInternerSet = HashSet<Utf8PathBuf, FxBuildHasher>;

/// File paths interner cache
///
/// The path interner stores an instance of [PathBuf]
#[derive(Debug)]
pub struct PathInterner {
    storage: PathInternerSet,
    handler: Sender<Utf8PathBuf>,
}

impl PathInterner {
    pub fn new() -> (Self, Receiver<Utf8PathBuf>) {
        let (send, recv) = unbounded();
        let interner = Self {
            storage: HashSet::default(),
            handler: send,
        };

        (interner, recv)
    }

    /// Inserts the path.
    ///
    /// Returns `true` if the path was not previously inserted.
    pub fn intern_path(&self, path: Utf8PathBuf) -> bool {
        let guard = self.storage.pin();
        if guard.contains(&path) {
            return false;
        }
        let owned = path.to_path_buf();
        let result = guard.insert(owned.clone());
        if result {
            self.handler.send(owned).ok();
        }
        result
    }

    pub fn as_intern_set(&self) -> &PathInternerSet {
        &self.storage
    }
}
