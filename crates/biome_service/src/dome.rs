use biome_fs::{BiomePath, PathInternerSet};
use std::collections::BTreeSet;
use std::collections::btree_set::Iter;
use std::iter::{FusedIterator, Peekable};

/// A type that holds the evaluated paths, and provides an iterator to extract
/// specific paths like configuration files, manifests and more.
#[derive(Debug, Default)]
pub struct Dome {
    paths: BTreeSet<BiomePath>,
}

impl Dome {
    pub fn from_intern(intern_paths: &PathInternerSet) -> Self {
        let guard = intern_paths.guard();
        let mut paths = BTreeSet::new();
        for path in intern_paths.iter(&guard) {
            paths.insert(BiomePath::new(path));
        }
        Self { paths }
    }

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
    /// Consumes the iterator and returns the configuration file paths.
    pub fn config_paths(&mut self) -> Vec<BiomePath> {
        let mut config_files = Vec::new();
        while let Some(path) = self.next_config() {
            config_files.push(path.clone());
        }
        config_files
    }

    /// Consumes the iterator and returns a vector containing all manifest file paths.
    pub fn manifest_paths(&mut self) -> Vec<BiomePath> {
        let mut manifests = Vec::new();
        while let Some(path) = self.next_manifest() {
            manifests.push(path.clone());
        }
        manifests
    }

    /// Consumes the iterator and returns a vector containing all ignore file paths.
    pub fn ignore_paths(&mut self) -> Vec<BiomePath> {
        let mut ignore_files = Vec::new();
        while let Some(path) = self.next_ignore() {
            ignore_files.push(path.clone());
        }
        ignore_files
    }

    pub fn next_dir(&mut self) -> Option<&'a BiomePath> {
        if let Some(path) = self.iter.peek() {
            if path.is_dir() {
                self.iter.next()
            } else {
                None
            }
        } else {
            None
        }
    }

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
