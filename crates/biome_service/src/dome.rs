use biome_fs::BiomePath;
use std::collections::btree_set::Iter;
use std::collections::BTreeSet;
use std::iter::{FusedIterator, Peekable};

/// A type that holds the evaluated paths, and provides an iterator to extract
/// specific paths like configuration files, manifests and more.
#[derive(Debug, Default)]
pub struct Dome {
    paths: BTreeSet<BiomePath>,
}

impl Dome {
    pub fn with_path(mut self, path: impl Into<BiomePath>) -> Self {
        self.paths.insert(path.into());
        self
    }

    pub fn new(paths: BTreeSet<BiomePath>) -> Self {
        Self { paths }
    }

    pub fn iter(&self) -> DomeIterator {
        DomeIterator {
            iter: self.paths.iter().peekable(),
        }
    }

    pub fn to_paths(self) -> BTreeSet<BiomePath> {
        self.paths
    }
}

pub struct DomeIterator<'a> {
    iter: Peekable<Iter<'a, BiomePath>>,
}

impl<'a> DomeIterator<'a> {
    pub fn next_config(&mut self) -> Option<&'a BiomePath> {
        if let Some(path) = self.iter.peek() {
            if path.is_config() {
                self.iter.next()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn next_ignore(&mut self) -> Option<&'a BiomePath> {
        if let Some(path) = self.iter.peek() {
            if path.is_ignore() {
                self.iter.next()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn next_manifest(&mut self) -> Option<&'a BiomePath> {
        if let Some(path) = self.iter.peek() {
            if path.is_manifest() {
                self.iter.next()
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> Iterator for DomeIterator<'a> {
    type Item = &'a BiomePath;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl FusedIterator for DomeIterator<'_> {}
