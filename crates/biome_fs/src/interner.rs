use camino::Utf8PathBuf;
use crossbeam::channel::{Receiver, Sender, unbounded};
use papaya::HashSet;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::sync::Mutex;

use crate::path::BiomePath;

pub type PathInternerSet = HashSet<Utf8PathBuf, FxBuildHasher>;

/// Selects one stable logical identity for each physical file.
#[derive(Debug, Default)]
pub struct PhysicalFileOwners {
    owners: Mutex<FxHashMap<Utf8PathBuf, PhysicalFileOwner>>,
}

#[derive(Debug)]
struct PhysicalFileOwner {
    logical_path: BiomePath,
    is_symlink: bool,
}

impl PhysicalFileOwners {
    /// Selects `logical_path` when it has precedence over the current owner.
    ///
    /// Direct paths take precedence over symbolic-link paths. Paths of the same kind use lexical
    /// order. `on_selected` runs while ownership is locked so callers can replace stored paths
    /// without racing another selection.
    pub fn select(
        &self,
        physical_path: Utf8PathBuf,
        logical_path: BiomePath,
        is_symlink: bool,
        on_selected: impl FnOnce(Option<&BiomePath>, &BiomePath),
    ) {
        let mut owners = self
            .owners
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match owners.entry(physical_path) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                on_selected(None, &logical_path);
                entry.insert(PhysicalFileOwner {
                    logical_path,
                    is_symlink,
                });
            }
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                let current = entry.get();
                if (is_symlink, &logical_path) < (current.is_symlink, &current.logical_path) {
                    on_selected(Some(&current.logical_path), &logical_path);
                    entry.insert(PhysicalFileOwner {
                        logical_path,
                        is_symlink,
                    });
                }
            }
        }
    }
}

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
