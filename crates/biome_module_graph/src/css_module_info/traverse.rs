use crate::CssPropertyDefinition;
use crate::db::ModuleDb;
use crate::module_graph::ModuleInfoKind;
use biome_console::markup;
use biome_css_semantic::db::css_semantic_model;
use biome_db::AnyParsedSource;
use biome_rowan::{TextRange, TokenText};
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use rustc_hash::FxHashSet;
use std::rc::Rc;

/// Minimal step for efficient CSS class checking during traversal.
/// Used in the happy path where we're just checking if classes exist.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CssClassStep {
    /// The path of the CSS file discovered in this step
    pub css_path: Utf8PathBuf,
    /// The CSS class names and their selector ranges found in this CSS file
    pub css_classes: IndexMap<TextRange, TokenText>,
}

/// Rich diagnostic information including component chain.
/// Only built when generating error diagnostics (class not found).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CssTraversalStep {
    /// The path of the CSS file discovered in this step
    pub css_path: Utf8PathBuf,
    /// The JS/JSX/HTML file that imports this CSS file
    pub importer_path: Utf8PathBuf,
    /// The chain of JS/JSX files from the starting file to the importer
    /// For example, [Button.jsx, Block.jsx, Page.jsx, App.jsx]
    /// where Button.jsx is the starting file and App.jsx imports the CSS
    pub component_chain: Vec<Utf8PathBuf>,
    /// CSS files imported directly by the component vs by a parent
    pub is_direct: bool,
}

/// Tree structure representing import relationships for diagnostic display.
/// This captures the full hierarchical structure of how CSS files are discovered
/// through the component import tree.
#[derive(Debug, Clone, PartialEq)]
pub struct ImportTreeNode {
    /// The path of this file (JS/JSX/HTML component)
    pub file_path: Utf8PathBuf,
    /// CSS files directly imported by this file
    pub css_imports: Vec<Utf8PathBuf>,
    /// Parent components that import this file (recursive tree structure)
    pub parent_components: Vec<Self>,
}

/// Defines how a reverse module-graph traversal recognizes and processes importers.
///
/// The traversal enumerates candidate modules without imposing an order. A
/// policy filters those candidates, interprets their import relationships, and
/// decides whether to yield items, continue upward, or both.
///
/// Policies own cycle detection because some searches deduplicate modules
/// globally while others must preserve independent branches. Implementations
/// must reject paths already visited in the relevant scope;
/// [UpwardTraversal] does not provide independent cycle detection.
pub(crate) trait UpwardTraversalPolicy {
    /// State carried by a single traversal branch.
    ///
    /// Every continuation returned by [Self::visit_importer] supplies the state
    /// to use when that importer becomes the current module.
    type BranchState;

    /// A value yielded by the traversal.
    type Item;

    /// Returns whether `importer` has an eligible edge to `current_path`.
    ///
    /// This predicate runs before the candidate's module information is cloned.
    /// It must also reject cycles according to the policy's cycle strategy.
    fn should_visit_importer(
        &self,
        current_path: &Utf8Path,
        importer_path: &Utf8Path,
        importer: &ModuleInfoKind,
        state: &Self::BranchState,
    ) -> bool;

    /// Processes the eligible relationship from `importer` to `current_path`.
    ///
    /// Policies that distinguish authored occurrences return one action per
    /// occurrence. Each action may yield zero or more items and may continue
    /// with branch-specific state.
    fn visit_importer(
        &mut self,
        db: &dyn ModuleDb,
        current_path: &Utf8Path,
        importer_path: &Utf8Path,
        importer: &ModuleInfoKind,
        state: &Self::BranchState,
    ) -> Vec<UpwardTraversalAction<Self::Item, Self::BranchState>>;
}

/// One policy decision for an eligible importer relationship.
///
/// `items` are yielded before any pending upward traversal resumes. A
/// `continue_with` value schedules the importer as the next module on that
/// branch; `None` stops the branch.
pub(crate) struct UpwardTraversalAction<Item, State> {
    /// Values produced while processing the importer relationship.
    pub(crate) items: Vec<Item>,

    /// State for continuing upward from the importer, or `None` to stop.
    pub(crate) continue_with: Option<State>,
}

/// A pending operation in an [UpwardTraversal].
///
/// Separating visits from yielded items allows the iterator to emit all items
/// discovered at the current level before it resumes traversal through pending
/// importers.
enum UpwardTraversalWork<Item, State> {
    /// Expands modules that import `path` using the policy's branch `state`.
    Visit { path: Utf8PathBuf, state: State },

    /// Emits a value previously produced by the policy.
    Yield(Item),
}

/// Lazily traverses modules that import a starting module.
///
/// Traversal follows reverse import edges. Candidate importer order and yielded
/// item order are unspecified. Edge precedence, branch termination, duplicate
/// handling, and cycle detection are delegated to [UpwardTraversalPolicy].
pub(crate) struct UpwardTraversal<'db, Policy>
where
    Policy: UpwardTraversalPolicy,
{
    db: &'db dyn ModuleDb,
    policy: Policy,
    stack: Vec<UpwardTraversalWork<Policy::Item, Policy::BranchState>>,
}

impl<'db, Policy> UpwardTraversal<'db, Policy>
where
    Policy: UpwardTraversalPolicy,
{
    /// Starts an upward traversal at `start` with policy-defined branch state.
    pub(crate) fn new(
        db: &'db dyn ModuleDb,
        start: Utf8PathBuf,
        policy: Policy,
        state: Policy::BranchState,
    ) -> Self {
        Self {
            db,
            policy,
            stack: vec![UpwardTraversalWork::Visit { path: start, state }],
        }
    }
}

impl<Policy> Iterator for UpwardTraversal<'_, Policy>
where
    Policy: UpwardTraversalPolicy,
{
    type Item = Policy::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop()? {
                UpwardTraversalWork::Yield(item) => return Some(item),
                UpwardTraversalWork::Visit { path, state } => {
                    let mut importers = Vec::new();
                    self.db.for_each_module(&mut |importer_path, importer| {
                        if self
                            .policy
                            .should_visit_importer(&path, importer_path, importer, &state)
                        {
                            importers.push((importer_path.to_path_buf(), importer.clone()));
                        }
                    });

                    let mut items = Vec::new();
                    let mut visits = Vec::new();
                    for (importer_path, importer) in importers {
                        let actions = self.policy.visit_importer(
                            self.db,
                            &path,
                            &importer_path,
                            &importer,
                            &state,
                        );
                        for action in actions {
                            for item in action.items {
                                items.push(UpwardTraversalWork::Yield(item));
                            }
                            if let Some(next_state) = action.continue_with {
                                visits.push(UpwardTraversalWork::Visit {
                                    path: importer_path.clone(),
                                    state: next_state,
                                });
                            }
                        }
                    }
                    self.stack.extend(visits);
                    self.stack.extend(items.into_iter().rev());
                }
            }
        }
    }
}

/// Traverses component importers to discover CSS classes visible to them.
///
/// Modules are visited at most once across the entire traversal. Each eligible
/// JavaScript importer contributes its static CSS imports. HTML importers
/// contribute linked, static, and dynamic CSS imports together with classes
/// from global inline styles.
pub(crate) struct CssClassTraversalPolicy {
    visited: FxHashSet<Utf8PathBuf>,
}

impl CssClassTraversalPolicy {
    /// Creates a class traversal with `start` already visited.
    ///
    /// Seeding the starting module prevents an import cycle from yielding its
    /// direct CSS imports a second time.
    pub(crate) fn new(start: &Utf8Path) -> Self {
        Self {
            visited: [start.to_path_buf()].into_iter().collect(),
        }
    }
}

impl UpwardTraversalPolicy for CssClassTraversalPolicy {
    type BranchState = ();
    type Item = CssClassStep;

    /// Accepts an unvisited JavaScript or HTML module that imports `current_path`.
    fn should_visit_importer(
        &self,
        current_path: &Utf8Path,
        importer_path: &Utf8Path,
        importer: &ModuleInfoKind,
        _state: &Self::BranchState,
    ) -> bool {
        !matches!(importer, ModuleInfoKind::Css(_))
            && !self.visited.contains(importer_path)
            && imports_path(importer, current_path)
    }

    /// Collects the importer's CSS classes and continues through its importers.
    ///
    /// The importer is marked globally visited before traversal continues.
    fn visit_importer(
        &mut self,
        db: &dyn ModuleDb,
        _current_path: &Utf8Path,
        importer_path: &Utf8Path,
        importer: &ModuleInfoKind,
        _state: &Self::BranchState,
    ) -> Vec<UpwardTraversalAction<Self::Item, Self::BranchState>> {
        self.visited.insert(importer_path.to_path_buf());

        let items = match importer {
            ModuleInfoKind::Js(js_info) => js_info
                .static_import_paths
                .values()
                .filter_map(|import_path| css_class_step(db, import_path.as_path()?))
                .collect(),
            ModuleInfoKind::Html(html_info) => {
                let mut items = html_info
                    .imported_stylesheets
                    .iter()
                    .chain(html_info.static_import_paths.values())
                    .chain(html_info.dynamic_import_paths.values())
                    .filter_map(|import_path| css_class_step(db, import_path.as_path()?))
                    .collect::<Vec<_>>();
                items.push(CssClassStep {
                    css_path: importer_path.to_path_buf(),
                    css_classes: html_info.get_global_styles(),
                });
                items
            }
            ModuleInfoKind::Css(_) => Vec::new(),
        };

        vec![UpwardTraversalAction {
            items,
            continue_with: Some(()),
        }]
    }
}

/// Traverses importer branches to resolve visible CSS `@property` definitions.
///
/// Every authored edge is evaluated independently because preceding sibling
/// imports differ by edge position. A branch stops at its nearest definition.
/// Definitions reached through multiple paths are deduplicated by source path
/// and range.
pub(crate) struct CssPropertyTraversalPolicy<'name> {
    name: &'name str,
    yielded: FxHashSet<CssPropertyDefinition>,
}

/// Persistent ancestry for one CSS property traversal branch.
///
/// Forking a branch shares its existing ancestry, while membership checks keep
/// cycle detection local to that branch.
#[derive(Clone)]
pub(crate) struct CssPropertyBranch(Rc<CssPropertyBranchNode>);

struct CssPropertyBranchNode {
    path: Utf8PathBuf,
    parent: Option<Rc<Self>>,
}

impl CssPropertyBranch {
    /// Starts a branch with `path` as its first visited module.
    pub(crate) fn new(path: Utf8PathBuf) -> Self {
        Self(Rc::new(CssPropertyBranchNode { path, parent: None }))
    }

    /// Returns whether `path` occurs in this branch's ancestry.
    fn contains(&self, path: &Utf8Path) -> bool {
        let mut node = Some(&*self.0);
        while let Some(current) = node {
            if current.path == path {
                return true;
            }
            node = current.parent.as_deref();
        }
        false
    }

    /// Forks this branch with `path` appended to its ancestry.
    fn with_path(&self, path: Utf8PathBuf) -> Self {
        Self(Rc::new(CssPropertyBranchNode {
            path,
            parent: Some(self.0.clone()),
        }))
    }
}

impl<'name> CssPropertyTraversalPolicy<'name> {
    /// Creates a policy that resolves definitions for `name`.
    pub(crate) fn new(name: &'name str) -> Self {
        Self {
            name,
            yielded: FxHashSet::default(),
        }
    }

    /// Stops a branch at a definition or continues it when no definition exists.
    ///
    /// A definition already yielded through another path still stops the branch.
    fn action(
        &mut self,
        definition: Option<CssPropertyDefinition>,
        next_branch: CssPropertyBranch,
    ) -> UpwardTraversalAction<CssPropertyDefinition, CssPropertyBranch> {
        match definition {
            Some(definition) => UpwardTraversalAction {
                items: self
                    .yielded
                    .insert(definition.clone())
                    .then_some(definition)
                    .into_iter()
                    .collect(),
                continue_with: None,
            },
            None => UpwardTraversalAction {
                items: Vec::new(),
                continue_with: Some(next_branch),
            },
        }
    }
}

impl UpwardTraversalPolicy for CssPropertyTraversalPolicy<'_> {
    type BranchState = CssPropertyBranch;
    type Item = CssPropertyDefinition;

    /// Accepts an importer that has not occurred in this branch's ancestry.
    fn should_visit_importer(
        &self,
        current_path: &Utf8Path,
        importer_path: &Utf8Path,
        importer: &ModuleInfoKind,
        state: &Self::BranchState,
    ) -> bool {
        !state.contains(importer_path) && imports_path(importer, current_path)
    }

    /// Resolves the definition visible from each authored edge to `current_path`.
    ///
    /// CSS importers prefer their local definition, then preceding sibling
    /// imports. JavaScript and HTML importers search preceding CSS contexts.
    fn visit_importer(
        &mut self,
        db: &dyn ModuleDb,
        current_path: &Utf8Path,
        importer_path: &Utf8Path,
        importer: &ModuleInfoKind,
        state: &Self::BranchState,
    ) -> Vec<UpwardTraversalAction<Self::Item, Self::BranchState>> {
        let next_branch = state.with_path(importer_path.to_path_buf());
        match importer {
            ModuleInfoKind::Css(info) => info
                .imports
                .iter()
                .filter(|(_, import)| import.resolved_path.as_path() == Some(current_path))
                .map(|(child_range, _)| {
                    let definition = local_property_definition(db, importer_path, self.name)
                        .or_else(|| {
                            last_property_in_imports_before(
                                db,
                                info,
                                *child_range,
                                self.name,
                                &mut [importer_path.to_path_buf()].into_iter().collect(),
                            )
                        });
                    self.action(definition, next_branch.clone())
                })
                .collect(),
            ModuleInfoKind::Js(info) => {
                let imports = info
                    .static_import_paths
                    .values()
                    .chain(info.dynamic_import_paths.values())
                    .filter_map(|import| import.as_path())
                    .collect::<Vec<_>>();
                self.actions_for_ordered_imports(
                    db,
                    importer_path,
                    current_path,
                    &imports,
                    next_branch,
                )
            }
            ModuleInfoKind::Html(info) => {
                let imports = info
                    .imported_stylesheets
                    .iter()
                    .chain(info.static_import_paths.values())
                    .chain(info.dynamic_import_paths.values())
                    .filter_map(|import| import.as_path())
                    .collect::<Vec<_>>();
                self.actions_for_ordered_imports(
                    db,
                    importer_path,
                    current_path,
                    &imports,
                    next_branch,
                )
            }
        }
    }
}

impl CssPropertyTraversalPolicy<'_> {
    /// Evaluates every occurrence of `current_path` in an ordered import list.
    ///
    /// For each occurrence, preceding imports are searched from nearest to
    /// farthest. Imports authored after the occurrence are not visible.
    fn actions_for_ordered_imports(
        &mut self,
        db: &dyn ModuleDb,
        importer_path: &Utf8Path,
        current_path: &Utf8Path,
        imports: &[&Utf8Path],
        next_branch: CssPropertyBranch,
    ) -> Vec<UpwardTraversalAction<CssPropertyDefinition, CssPropertyBranch>> {
        imports
            .iter()
            .enumerate()
            .filter(|(_, path)| **path == current_path)
            .map(|(child_index, _)| {
                let mut ancestry = [importer_path.to_path_buf()].into_iter().collect();
                let definition = imports.iter().take(child_index).rev().find_map(|path| {
                    last_property_in_css_context(db, path, self.name, &mut ancestry)
                });
                self.action(definition, next_branch.clone())
            })
            .collect()
    }
}

/// Returns the last visible definition in a CSS import context.
///
/// The stylesheet's local definition takes precedence. Otherwise, imports are
/// searched in reverse source order, and each imported stylesheet applies the
/// same rule. Paths already present in `ancestry` are skipped. The traversal is
/// iterative so authored import depth cannot overflow the call stack.
pub(crate) fn last_property_in_css_context(
    db: &dyn ModuleDb,
    path: &Utf8Path,
    name: &str,
    ancestry: &mut FxHashSet<Utf8PathBuf>,
) -> Option<CssPropertyDefinition> {
    enum Work {
        Visit(Utf8PathBuf),
        Leave(Utf8PathBuf),
    }

    let mut stack = vec![Work::Visit(path.to_path_buf())];
    while let Some(work) = stack.pop() {
        match work {
            Work::Visit(path) => {
                if !ancestry.insert(path.clone()) {
                    continue;
                }
                if let Some(definition) = local_property_definition(db, &path, name) {
                    return Some(definition);
                }

                stack.push(Work::Leave(path.clone()));
                if let Some(info) = db.css_module_info_for_path(&path) {
                    stack.extend(info.imports.values().filter_map(|import| {
                        import
                            .resolved_path
                            .as_path()
                            .map(|path| Work::Visit(path.to_path_buf()))
                    }));
                }
            }
            Work::Leave(path) => {
                ancestry.remove(&path);
            }
        }
    }
    None
}

/// Returns the last definition visible through imports preceding `child_range`.
///
/// Imports are searched in reverse source order. `child_range` identifies the
/// authored edge to the child module, so later sibling imports are excluded.
fn last_property_in_imports_before(
    db: &dyn ModuleDb,
    info: &crate::CssModuleInfo,
    child_range: TextRange,
    name: &str,
    ancestry: &mut FxHashSet<Utf8PathBuf>,
) -> Option<CssPropertyDefinition> {
    info.imports
        .iter()
        .rev()
        .filter(|(range, _)| range.start() < child_range.start())
        .find_map(|(_, import)| {
            let path = import.resolved_path.as_path()?;
            last_property_in_css_context(db, path, name, ancestry)
        })
}

/// Returns the local `@property` definition for `name` in a CSS module.
///
/// The parsed source is read directly so range-only edits invalidate tracked
/// callers even when the semantic contents of the definition are unchanged.
fn local_property_definition(
    db: &dyn ModuleDb,
    path: &Utf8Path,
    name: &str,
) -> Option<CssPropertyDefinition> {
    db.css_module_info_for_path(path)?;
    let parsed = db.parsed_source_for_path(path)?;
    let _ = parsed.parsed(db);
    let source = AnyParsedSource::ParsedSource(parsed);
    let model = css_semantic_model(db, &source);
    let at_property = model.global_custom_variables().get(name)?.at_property()?;
    Some(CssPropertyDefinition {
        module_path: path.to_path_buf(),
        range: at_property.range(),
    })
}

/// Returns whether `module` imports `path` through a supported import edge.
fn imports_path(module: &ModuleInfoKind, path: &Utf8Path) -> bool {
    match module {
        ModuleInfoKind::Js(info) => info
            .static_import_paths
            .values()
            .chain(info.dynamic_import_paths.values())
            .any(|import| import.as_path() == Some(path)),
        ModuleInfoKind::Css(info) => info
            .imports
            .values()
            .any(|import| import.resolved_path.as_path() == Some(path)),
        ModuleInfoKind::Html(info) => info
            .imported_stylesheets
            .iter()
            .chain(info.static_import_paths.values())
            .chain(info.dynamic_import_paths.values())
            .any(|import| import.as_path() == Some(path)),
    }
}

/// Returns the CSS classes defined by the module at `path`, or `None` when the
/// path does not identify a CSS module.
fn css_class_step(db: &dyn ModuleDb, path: &Utf8Path) -> Option<CssClassStep> {
    let info = db.css_module_info_for_path(path)?;
    Some(CssClassStep {
        css_path: path.to_path_buf(),
        css_classes: info.classes.clone(),
    })
}

/// Newtype for displaying ImportTreeNode with working directory context
pub struct ImportTreeDisplay<'a> {
    node: &'a ImportTreeNode,
    working_directory: Option<&'a Utf8Path>,
}

impl<'a> ImportTreeDisplay<'a> {
    pub fn new(node: &'a ImportTreeNode, working_directory: Option<&'a Utf8Path>) -> Self {
        Self {
            node,
            working_directory,
        }
    }
}

impl<'a> biome_console::fmt::Display for ImportTreeDisplay<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        let root_node = RootTreeNode {
            node: self.node,
            working_directory: self.working_directory,
        };

        fmt.write_markup(markup! {{root_node}})
    }
}

struct RelativePath<'a> {
    path: &'a Utf8Path,
    working_directory: Option<&'a Utf8Path>,
}

impl<'a> biome_console::fmt::Display for RelativePath<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        if let Some(wd) = self.working_directory {
            let relative = self.path.strip_prefix(wd).unwrap_or(self.path);
            fmt.write_str(relative.as_str())
        } else {
            fmt.write_str(self.path.as_str())
        }
    }
}

struct RootTreeNode<'a> {
    node: &'a ImportTreeNode,
    working_directory: Option<&'a Utf8Path>,
}

impl<'a> biome_console::fmt::Display for RootTreeNode<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        let display_path = RelativePath {
            path: &self.node.file_path,
            working_directory: self.working_directory,
        };
        fmt.write_markup(markup! {{display_path}" (this file)"})?;

        if !self.node.css_imports.is_empty() {
            fmt.write_str(" -> which imports ")?;
            for (i, css_path) in self.node.css_imports.iter().enumerate() {
                if i > 0 {
                    fmt.write_str(", ")?;
                }
                let css_display = RelativePath {
                    path: css_path,
                    working_directory: self.working_directory,
                };
                fmt.write_markup(markup! {{css_display}})?;
            }
        }

        fmt.write_str("\n")?;

        if !self.node.parent_components.is_empty() {
            let group = ImportedByGroup {
                parents: &self.node.parent_components,
                working_directory: self.working_directory,
                is_last: true,
            };
            fmt.write_markup(markup! {{group}})?;
        }

        Ok(())
    }
}

struct ImportedByGroup<'a> {
    parents: &'a [ImportTreeNode],
    working_directory: Option<&'a Utf8Path>,
    is_last: bool,
}

impl<'a> biome_console::fmt::Display for ImportedByGroup<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        let branch = if self.is_last { "└─ " } else { "├─ " };
        fmt.write_markup(markup! {"  "{branch}"imported by:\n"})?;

        let prefix = if self.is_last {
            "    ".to_string()
        } else {
            "  │ ".to_string()
        };

        let mut sorted_parents: Vec<_> = self.parents.iter().collect();
        sorted_parents.sort_by_key(|p| &p.file_path);

        for (i, parent) in sorted_parents.iter().enumerate() {
            let is_last_parent = i == sorted_parents.len() - 1;
            let item = TreeItem {
                node: parent,
                working_directory: self.working_directory,
                prefix: prefix.clone(),
                is_last: is_last_parent,
            };
            fmt.write_markup(markup! {{item}})?;
        }

        Ok(())
    }
}

struct TreeItem<'a> {
    node: &'a ImportTreeNode,
    working_directory: Option<&'a Utf8Path>,
    prefix: String,
    is_last: bool,
}

impl<'a> biome_console::fmt::Display for TreeItem<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        let branch = if self.is_last { "└─ " } else { "├─ " };
        let display_path = RelativePath {
            path: &self.node.file_path,
            working_directory: self.working_directory,
        };

        fmt.write_markup(markup! {{self.prefix}{branch}"• "{display_path}})?;

        if !self.node.css_imports.is_empty() {
            fmt.write_str(" -> which imports ")?;
            for (i, css_path) in self.node.css_imports.iter().enumerate() {
                if i > 0 {
                    fmt.write_str(", ")?;
                }
                let css_display = RelativePath {
                    path: css_path,
                    working_directory: self.working_directory,
                };
                fmt.write_markup(markup! {{css_display}})?;
            }
        }

        fmt.write_str("\n")?;

        if !self.node.parent_components.is_empty() {
            let child_prefix = format!("{}{}", self.prefix, if self.is_last { "  " } else { "│ " });
            fmt.write_markup(markup! {{child_prefix}"└─ imported by:\n"})?;

            let item_prefix = format!("{}  ", child_prefix);

            let mut sorted_parents: Vec<_> = self.node.parent_components.iter().collect();
            sorted_parents.sort_by_key(|p| &p.file_path);

            for (i, parent) in sorted_parents.iter().enumerate() {
                let is_last_parent = i == sorted_parents.len() - 1;
                let parent_item = TreeItem {
                    node: parent,
                    working_directory: self.working_directory,
                    prefix: item_prefix.clone(),
                    is_last: is_last_parent,
                };
                fmt.write_markup(markup! {{parent_item}})?;
            }
        }

        Ok(())
    }
}
