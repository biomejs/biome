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
pub(crate) enum UpwardTraversalWork<Item, Branch> {
    /// Finds modules that import `imported_path`.
    ExploreImporters {
        imported_path: Utf8PathBuf,
        branch: Branch,
    },

    /// Returns an item produced by the visitor.
    Emit(Item),
}

impl<Item, Branch> UpwardTraversalWork<Item, Branch> {
    pub(crate) fn explore(imported_path: Utf8PathBuf, branch: Branch) -> Self {
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
pub(crate) trait UpwardTraversal: UpwardTraversalVisitor {
    fn db(&self) -> &dyn ModuleDb;

    fn stack(&mut self) -> &mut Vec<UpwardTraversalWork<Self::Item, Self::Branch>>;

    /// Wraps this traversal in the standard-library iterator interface.
    fn into_upward_iter(self) -> UpwardTraversalIterator<Self>
    where
        Self: Sized,
    {
        TraversalIterator(UpwardTraversalAdapter(self))
    }

    /// Returns the next item produced while exploring importers.
    fn next_upward(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack().pop()? {
                UpwardTraversalWork::Emit(item) => return Some(item),
                UpwardTraversalWork::ExploreImporters {
                    imported_path,
                    branch,
                } => {
                    let mut importers = Vec::new();
                    self.db().for_each_module(&mut |importer| {
                        if self.should_visit_importer(&imported_path, importer, &branch) {
                            importers.push(importer);
                        }
                    });

                    let mut items = Vec::new();
                    let mut explorations = Vec::new();
                    for importer in importers {
                        let importer_path = importer.path(self.db()).to_path_buf();
                        let actions = self.visit_importer(&imported_path, importer, &branch);
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
                    self.stack().extend(explorations);
                    self.stack().extend(items.into_iter().rev());
                }
            }
        }
    }
}

/// Adapts an [UpwardTraversal] to the direction-neutral [Traversal] interface.
pub(crate) struct UpwardTraversalAdapter<T>(T);

impl<T> Traversal for UpwardTraversalAdapter<T>
where
    T: UpwardTraversal,
{
    type Item = T::Item;

    fn next_item(&mut self) -> Option<Self::Item> {
        self.0.next_upward()
    }
}

/// Iterator adapter for an [UpwardTraversal].
pub(crate) type UpwardTraversalIterator<T> = TraversalIterator<UpwardTraversalAdapter<T>>;
