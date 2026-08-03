use biome_rowan::Text;
use rustc_hash::FxHashMap;
use std::iter::FusedIterator;

/// Import paths stored in source order with lookup by specifier.
///
/// [`Self::insert`] keeps only the last occurrence of a specifier. [`Self::push`]
/// keeps every occurrence for languages where an import's position is
/// significant. In both cases, [`Self::get`] returns the last stored occurrence.
#[derive(Clone, Debug)]
pub struct ImportPathMap<P> {
    entries: Vec<(Text, P)>,
    indices: FxHashMap<Text, usize>,
}

impl<P> Default for ImportPathMap<P> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            indices: FxHashMap::default(),
        }
    }
}

impl<P> ImportPathMap<P> {
    /// Replaces an earlier import with the same specifier and moves the new
    /// value to its source-order position.
    pub(crate) fn insert(&mut self, specifier: Text, path: P) {
        self.insert_with(specifier, path, |_, _| {});
    }

    /// Inserts an import after merging an earlier occurrence with the new path.
    pub(crate) fn insert_with(
        &mut self,
        specifier: Text,
        mut path: P,
        merge: impl FnOnce(&P, &mut P),
    ) {
        if let Some(index) = self.indices.get(specifier.text()).copied() {
            debug_assert!(index < self.entries.len());
            merge(&self.entries[index].1, &mut path);
            self.entries.remove(index);
            self.indices.remove(specifier.text());
            for entry_index in self.indices.values_mut() {
                if *entry_index > index {
                    *entry_index -= 1;
                }
            }
        }

        let index = self.entries.len();
        self.entries.push((specifier.clone(), path));
        self.indices.insert(specifier, index);
    }

    /// Appends an import without removing earlier occurrences of its specifier.
    pub(crate) fn push(&mut self, specifier: Text, path: P) {
        let index = self.entries.len();
        self.entries.push((specifier.clone(), path));
        self.indices.insert(specifier, index);
    }

    /// Returns the last path stored for `specifier`.
    pub fn get(&self, specifier: &str) -> Option<&P> {
        let index = *self.indices.get(specifier)?;
        self.entries.get(index).map(|(_, path)| path)
    }

    /// Returns import paths in source order.
    pub fn iter(&self) -> ImportPathMapIterator<'_, P> {
        ImportPathMapIterator {
            inner: self.entries.iter(),
        }
    }

    /// Returns specifiers and paths in source order.
    pub fn named_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = (&Text, &P)> + ExactSizeIterator + FusedIterator {
        self.entries
            .iter()
            .map(|(specifier, path)| (specifier, path))
    }

    /// Returns the number of stored imports.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` when no imports are stored.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn get_index(&self, index: usize) -> Option<&P> {
        self.entries.get(index).map(|(_, path)| path)
    }
}

/// Iterator over import paths in an [`ImportPathMap`].
pub struct ImportPathMapIterator<'a, P> {
    inner: std::slice::Iter<'a, (Text, P)>,
}

impl<'a, P> Iterator for ImportPathMapIterator<'a, P> {
    type Item = &'a P;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, path)| path)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<P> DoubleEndedIterator for ImportPathMapIterator<'_, P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(_, path)| path)
    }
}

impl<P> ExactSizeIterator for ImportPathMapIterator<'_, P> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<P> FusedIterator for ImportPathMapIterator<'_, P> {}
