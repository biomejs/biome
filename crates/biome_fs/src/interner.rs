use camino::Utf8PathBuf;
use crossbeam::channel::{unbounded, Receiver, Sender};
use papaya::HashSet;
use rustc_hash::FxBuildHasher;

pub type PathInternerSet = HashSet<Utf8PathBuf, FxBuildHasher>;

/// File paths interner cache
///
/// The path interner stores an instance of [PathBuf]
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
        let result = self.storage.pin().insert(path.clone());
        if result {
            self.handler.send(path).ok();
        }
        result
    }

    pub fn into_paths(self) -> PathInternerSet {
        self.storage
    }
}
