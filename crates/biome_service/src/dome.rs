use biome_fs::BiomePath;
use rustc_hash::FxHashSet;
use std::collections::hash_set::Iter;
use std::iter::Peekable;

#[derive(Debug, Default)]
/// A type that holds the evaluated paths, and provides an iterator to extract
/// specific paths like configuration files, manifests and more.
pub struct Dome {
    paths: FxHashSet<BiomePath>,
}

impl Dome {
    pub fn with_path(mut self, path: impl Into<BiomePath>) -> Self {
        self.paths.insert(path.into());
        self
    }

    pub fn new(paths: FxHashSet<BiomePath>) -> Self {
        Self { paths }
    }

    pub fn iter(&self) -> DomeIterator {
        DomeIterator {
            iter: self.paths.iter().peekable(),
        }
    }

    pub fn to_paths(self) -> FxHashSet<BiomePath> {
        self.paths
    }
}

pub struct DomeIterator<'a> {
    iter: Peekable<Iter<'a, BiomePath>>,
}

impl<'a> DomeIterator<'a> {
    pub fn next_config(&mut self) -> Option<&'a BiomePath> {
        while let Some(path) = self.iter.peek() {
            return if path.is_config() {
                self.iter.next()
            } else {
                None
            };
        }
        None
    }

    pub fn next_manifest(&mut self) -> Option<&'a BiomePath> {
        while let Some(path) = self.iter.peek() {
            return if path.is_manifest() {
                self.iter.next()
            } else {
                None
            };
        }
        None
    }
}

impl<'a> Iterator for DomeIterator<'a> {
    type Item = &'a BiomePath;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
