use biome_rowan::Text;
use rustc_hash::FxHashMap;
use std::iter::FusedIterator;

/// Import occurrences stored in source order with aggregate lookup by specifier.
#[derive(Clone, Debug)]
pub struct ImportPathMap<P> {
    entries: Vec<(Text, P)>,
    summaries: FxHashMap<Text, P>,
}

impl<P> Default for ImportPathMap<P> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            summaries: FxHashMap::default(),
        }
    }
}

impl<P: Clone> ImportPathMap<P> {
    /// Appends an import occurrence and replaces its lookup summary.
    pub(crate) fn insert(&mut self, specifier: Text, path: P) {
        self.entries.push((specifier.clone(), path.clone()));
        self.summaries.insert(specifier, path);
    }

    /// Appends an import occurrence and merges it into the specifier's lookup summary.
    pub(crate) fn insert_with(&mut self, specifier: Text, path: P, merge: impl FnOnce(&P, &mut P)) {
        let mut summary = path.clone();
        if let Some(previous) = self.summaries.get(specifier.text()) {
            merge(previous, &mut summary);
        }
        self.entries.push((specifier.clone(), path));
        self.summaries.insert(specifier, summary);
    }

    /// Appends an import occurrence and replaces its lookup summary.
    pub(crate) fn push(&mut self, specifier: Text, path: P) {
        self.entries.push((specifier.clone(), path.clone()));
        self.summaries.insert(specifier, path);
    }
}

impl<P> ImportPathMap<P> {
    /// Returns the aggregate path stored for `specifier`.
    pub fn get(&self, specifier: &str) -> Option<&P> {
        self.summaries.get(specifier)
    }

    /// Returns import paths in source order, including repeated specifiers.
    pub fn iter(&self) -> ImportPathMapIterator<'_, P> {
        ImportPathMapIterator {
            inner: self.entries.iter(),
        }
    }

    /// Returns specifiers and paths in source order, including repeated specifiers.
    pub fn named_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = (&Text, &P)> + ExactSizeIterator + FusedIterator {
        self.entries
            .iter()
            .map(|(specifier, path)| (specifier, path))
    }

    /// Returns the number of import occurrences.
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
