use crate::db::ModuleDb;
use crate::module_graph::ModuleInfo;
use camino::{Utf8Path, Utf8PathBuf};

/// Produces items by advancing a graph traversal.
pub(crate) trait Traversal {
    /// A value yielded by the traversal.
    type Item;

    /// Advances the traversal and returns its next item.
    fn next_item(&mut self) -> Option<Self::Item>;
}

/// Adapts a [Traversal] to the standard-library [Iterator] interface.
pub(crate) struct TraversalIterator<T>(T);

impl<T> Iterator for TraversalIterator<T>
where
    T: Traversal,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_item()
    }
}

/// Controls what an upward traversal does when it finds an importer.
///
/// The visitor is responsible for rejecting import cycles.
pub(crate) trait UpwardTraversalVisitor {
    /// Data carried while following one path through importers.
    type Branch;

    /// A value yielded by the traversal.
    type Item;

    /// Returns the module database searched for importers.
    fn db(&self) -> &dyn ModuleDb;

    /// Wraps this visitor in an upward traversal iterator.
    fn into_upward_iter(
        self,
        start: Utf8PathBuf,
        branch: Self::Branch,
    ) -> UpwardTraversalIterator<Self>
    where
        Self: Sized,
    {
        TraversalIterator(UpwardTraversal::new(self, start, branch))
    }

    /// Returns whether `importer` should be visited from `imported_path`.
    fn should_visit_importer(
        &self,
        imported_path: &Utf8Path,
        importer: ModuleInfo,
        branch: &Self::Branch,
    ) -> bool;

    /// Visits `importer`, which imports `imported_path`.
    ///
    /// Each returned action emits items, continues from the importer, or both.
    fn visit_importer(
        &mut self,
        imported_path: &Utf8Path,
        importer: ModuleInfo,
        branch: &Self::Branch,
    ) -> Vec<UpwardTraversalAction<Self::Item, Self::Branch>>;
}

/// The result of visiting an importer.
///
/// `items` are emitted before any pending upward traversal resumes. A
/// `continue_with` value schedules the importer as the next module on that
/// branch; `None` stops the branch.
pub(crate) struct UpwardTraversalAction<Item, Branch> {
    /// Values produced by the visitor.
    pub(crate) items: Vec<Item>,

    /// Branch for continuing from the importer, or `None` to stop.
    pub(crate) continue_with: Option<Branch>,
}

/// Work waiting to be processed by an [UpwardTraversal].
enum UpwardTraversalWork<Item, Branch> {
    /// Finds modules that import `imported_path`.
    ExploreImporters {
        imported_path: Utf8PathBuf,
        branch: Branch,
    },

    /// Returns an item produced by the visitor.
    Emit(Item),
}

impl<Item, Branch> UpwardTraversalWork<Item, Branch> {
    fn explore(imported_path: Utf8PathBuf, branch: Branch) -> Self {
        Self::ExploreImporters {
            imported_path,
            branch,
        }
    }
}

/// Traverses from an imported module to the modules that import it.
///
/// The visitor chooses which importers to visit and whether to continue from
/// them. Importer and item order are unspecified.
pub(crate) struct UpwardTraversal<V>
where
    V: UpwardTraversalVisitor,
{
    visitor: V,
    stack: Vec<UpwardTraversalWork<V::Item, V::Branch>>,
}

impl<V> UpwardTraversal<V>
where
    V: UpwardTraversalVisitor,
{
    fn new(visitor: V, start: Utf8PathBuf, branch: V::Branch) -> Self {
        Self {
            visitor,
            stack: vec![UpwardTraversalWork::explore(start, branch)],
        }
    }
}

impl<V> Traversal for UpwardTraversal<V>
where
    V: UpwardTraversalVisitor,
{
    type Item = V::Item;

    fn next_item(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop()? {
                UpwardTraversalWork::Emit(item) => return Some(item),
                UpwardTraversalWork::ExploreImporters {
                    imported_path,
                    branch,
                } => {
                    let mut importers = Vec::new();
                    self.visitor.db().for_each_module(&mut |importer| {
                        if self
                            .visitor
                            .should_visit_importer(&imported_path, importer, &branch)
                        {
                            importers.push(importer);
                        }
                    });

                    let mut items = Vec::new();
                    let mut explorations = Vec::new();
                    for importer in importers {
                        let importer_path = importer.path(self.visitor.db()).to_path_buf();
                        let actions =
                            self.visitor
                                .visit_importer(&imported_path, importer, &branch);
                        for action in actions {
                            for item in action.items {
                                items.push(UpwardTraversalWork::Emit(item));
                            }
                            if let Some(next_branch) = action.continue_with {
                                explorations.push(UpwardTraversalWork::ExploreImporters {
                                    imported_path: importer_path.clone(),
                                    branch: next_branch,
                                });
                            }
                        }
                    }
                    self.stack.extend(explorations);
                    self.stack.extend(items.into_iter().rev());
                }
            }
        }
    }
}

/// Iterator adapter for an [UpwardTraversal].
pub(crate) type UpwardTraversalIterator<V> = TraversalIterator<UpwardTraversal<V>>;
