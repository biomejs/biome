use crate::db::ModuleDb;
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::traverse::{UpwardTraversalAction, UpwardTraversalVisitor};
use crate::{CssPropertyDefinition, JsImportPath, JsImportPhase, JsModuleInfo};
use biome_console::markup;
use biome_css_syntax::decode_css_identifier;
use biome_fs::normalize_path;
use biome_rowan::{TextRange, TextSize, TokenText};
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

/// Traverses component importers to discover CSS classes visible to them.
///
/// Modules are visited at most once across the entire traversal. Each eligible
/// JavaScript importer contributes its static CSS imports. HTML importers
/// contribute linked, static, and dynamic CSS imports together with classes
/// from global inline styles.
pub(crate) struct CssClassTraversal<'db> {
    db: &'db dyn ModuleDb,
    visited: FxHashSet<Utf8PathBuf>,
}

impl<'db> CssClassTraversal<'db> {
    /// Creates a class traversal with `start` already visited.
    ///
    /// Seeding the starting module prevents an import cycle from yielding its
    /// direct CSS imports a second time.
    pub(crate) fn new(db: &'db dyn ModuleDb, start: &Utf8Path) -> Self {
        Self {
            db,
            visited: [start.to_path_buf()].into_iter().collect(),
        }
    }
}

impl UpwardTraversalVisitor for CssClassTraversal<'_> {
    type Branch = ();
    type Item = CssClassStep;

    fn db(&self) -> &dyn ModuleDb {
        self.db
    }

    fn should_visit_importer(
        &self,
        imported_path: &Utf8Path,
        importer: ModuleInfo,
        _branch: &Self::Branch,
    ) -> bool {
        let importer_path = importer.path(self.db);
        let importer = importer.kind(self.db);
        !matches!(importer, ModuleInfoKind::Css(_))
            && !self.visited.contains(importer_path)
            && imports_path(&importer, imported_path)
    }

    /// Collects the importer's CSS classes and continues through its importers.
    ///
    /// The importer is marked globally visited before traversal continues.
    fn visit_importer(
        &mut self,
        _imported_path: &Utf8Path,
        importer: ModuleInfo,
        _branch: &Self::Branch,
    ) -> Vec<UpwardTraversalAction<Self::Item, Self::Branch>> {
        let db = self.db;
        let importer_path = importer.path(db);
        let importer = importer.kind(db);
        self.visited.insert(importer_path.to_path_buf());

        let items = match importer {
            ModuleInfoKind::Js(js_info) => js_info
                .import_paths
                .iter()
                .filter(|import| import.kind.is_static())
                .filter_map(|import_path| css_class_step(db, import_path.as_path()?))
                .collect(),
            ModuleInfoKind::Html(html_info) => {
                let mut items = html_info
                    .imported_stylesheets
                    .iter()
                    .chain(html_info.import_paths.iter())
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

/// Finds the nearest CSS custom property definitions available to a module.
///
/// It searches modules that import the starting module. Each import path is
/// searched separately and stops at the first definition it finds.
pub(crate) struct CssPropertyTraversal<'db, 'name> {
    /// Module information used during the search.
    db: &'db dyn ModuleDb,
    /// Custom property name being resolved.
    name: &'name str,
    /// Definitions already returned through another import path.
    yielded: FxHashSet<CssPropertyDefinition>,
}

/// Persistent ancestry for one CSS property traversal branch.
///
/// Forking a branch shares its existing ancestry, while membership checks keep
/// cycle detection local to that branch.
#[derive(Clone, Default)]
pub(crate) struct CssPropertyBranch(Option<Rc<CssPropertyBranchNode>>);

struct CssPropertyBranchNode {
    path: Utf8PathBuf,
    parent: Option<Rc<Self>>,
}

/// One definition or import positioned in an HTML-like document.
struct HtmlPropertyContext {
    position: TextSize,
    kind: HtmlPropertyContextKind,
}

enum HtmlPropertyContextKind {
    Definition(CssPropertyDefinition),
    Import(Utf8PathBuf),
}

enum ModuleContextFrame {
    Module(Utf8PathBuf, CssPropertyBranch),
    Definition(CssPropertyDefinition),
}

impl CssPropertyBranch {
    /// Starts a branch with `path` as its first visited module.
    pub(crate) fn new(path: Utf8PathBuf) -> Self {
        Self::default().with_path(path)
    }

    /// Returns whether `path` occurs in this branch's ancestry.
    fn contains(&self, path: &Utf8Path) -> bool {
        let mut node = self.0.as_deref();
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
        Self(Some(Rc::new(CssPropertyBranchNode {
            path,
            parent: self.0.clone(),
        })))
    }
}

impl<'db, 'name> CssPropertyTraversal<'db, 'name> {
    /// Creates a visitor that resolves definitions for `name`.
    pub(crate) fn new(db: &'db dyn ModuleDb, name: &'name str) -> Self {
        Self {
            db,
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

    /// Evaluates every occurrence of `imported_path` in an import list.
    fn actions_for_imports(
        &mut self,
        imported_path: &Utf8Path,
        imports: &[&Utf8Path],
        next_branch: CssPropertyBranch,
    ) -> Vec<UpwardTraversalAction<CssPropertyDefinition, CssPropertyBranch>> {
        let definition = imports
            .iter()
            .rev()
            .find_map(|path| self.last_property_in_module_context_from(path, next_branch.clone()));
        imports
            .iter()
            .filter(|path| **path == imported_path)
            .map(|_| self.action(definition.clone(), next_branch.clone()))
            .collect()
    }

    /// Returns the last visible definition in an HTML-like document.
    pub(crate) fn last_property_in_html_context(
        &self,
        path: &Utf8Path,
    ) -> Option<CssPropertyDefinition> {
        let branch = CssPropertyBranch::new(path.to_path_buf());
        self.html_property_contexts(path, false)
            .into_iter()
            .rev()
            .find_map(|context| match context.kind {
                HtmlPropertyContextKind::Definition(definition) => Some(definition),
                HtmlPropertyContextKind::Import(path) => {
                    self.last_property_in_module_context_from(&path, branch.clone())
                }
            })
    }

    /// Returns the last visible definition imported by a JavaScript module.
    pub(crate) fn last_property_in_js_context(
        &self,
        path: &Utf8Path,
    ) -> Option<CssPropertyDefinition> {
        let info = self.db.js_module_info_for_path(path)?;
        let branch = CssPropertyBranch::new(path.to_path_buf());
        js_import_paths_in_source_order(&info)
            .rev()
            .filter_map(|import| import.as_path())
            .find_map(|path| self.last_property_in_module_context_from(path, branch.clone()))
    }

    /// Returns the last visible definition in a CSS import context.
    ///
    /// The stylesheet's local definition takes precedence. Otherwise, imports
    /// are searched in reverse source order without revisiting a module in the
    /// same branch.
    pub(crate) fn last_property_in_css_context(
        &self,
        path: &Utf8Path,
    ) -> Option<CssPropertyDefinition> {
        self.last_property_in_module_context_from(path, CssPropertyBranch::default())
    }

    fn last_property_in_module_context_from(
        &self,
        path: &Utf8Path,
        branch: CssPropertyBranch,
    ) -> Option<CssPropertyDefinition> {
        let mut stack = vec![ModuleContextFrame::Module(path.to_path_buf(), branch)];
        while let Some(frame) = stack.pop() {
            let (path, branch) = match frame {
                ModuleContextFrame::Module(path, branch) => (path, branch),
                ModuleContextFrame::Definition(definition) => return Some(definition),
            };
            if branch.contains(&path) {
                continue;
            }
            let branch = branch.with_path(path.clone());
            let Some(module) = self.db.module_for_path(&path) else {
                continue;
            };
            match module.kind(self.db) {
                ModuleInfoKind::Css(info) => {
                    if let Some(definition) = self.local_property_definition(&path) {
                        return Some(definition);
                    }
                    stack.extend(info.imports.iter().filter_map(|import| {
                        let path = import.resolved_path.as_path()?;
                        if branch.contains(path) {
                            return None;
                        }
                        Some(ModuleContextFrame::Module(
                            path.to_path_buf(),
                            branch.clone(),
                        ))
                    }));
                }
                ModuleInfoKind::Html(_) => {
                    stack.extend(
                        self.html_property_contexts(&path, true)
                            .into_iter()
                            .filter_map(|context| match context.kind {
                                HtmlPropertyContextKind::Definition(definition) => {
                                    Some(ModuleContextFrame::Definition(definition))
                                }
                                HtmlPropertyContextKind::Import(path) => (!branch.contains(&path))
                                    .then(|| ModuleContextFrame::Module(path, branch.clone())),
                            }),
                    );
                }
                ModuleInfoKind::Js(info) => {
                    stack.extend(js_import_paths_in_source_order(&info).filter_map(|import| {
                        let path = import.as_path()?;
                        if branch.contains(path) {
                            return None;
                        }
                        Some(ModuleContextFrame::Module(
                            path.to_path_buf(),
                            branch.clone(),
                        ))
                    }));
                }
            }
        }
        None
    }

    /// Returns the last definition visible through the stylesheet's imports.
    fn last_property_in_imports(
        &self,
        info: &crate::CssModuleInfo,
        branch: CssPropertyBranch,
    ) -> Option<CssPropertyDefinition> {
        info.imports.iter().rev().find_map(|import| {
            let path = import.resolved_path.as_path()?;
            self.last_property_in_module_context_from(path, branch.clone())
        })
    }

    fn local_property_definition(&self, path: &Utf8Path) -> Option<CssPropertyDefinition> {
        let info = self.db.css_module_info_for_path(path)?;
        let definition = info
            .property_registrations
            .iter()
            .rev()
            .find(|definition| {
                decode_css_identifier(definition.name.text()) == decode_css_identifier(self.name)
            })?;
        Some(CssPropertyDefinition {
            module_path: normalize_path(path),
            range: definition.range,
        })
    }

    fn html_property_contexts(
        &self,
        path: &Utf8Path,
        only_global: bool,
    ) -> Vec<HtmlPropertyContext> {
        let Some(info) = self.db.html_module_info_for_path(path) else {
            return Vec::new();
        };
        let mut contexts = info
            .imported_stylesheets
            .iter()
            .chain(info.import_paths.iter())
            .filter(|import| !only_global || import.applicability.is_global())
            .filter_map(|import| {
                Some(HtmlPropertyContext {
                    position: import.range.start(),
                    kind: HtmlPropertyContextKind::Import(import.as_path()?.to_path_buf()),
                })
            })
            .collect::<Vec<_>>();

        contexts.extend(
            info.property_registrations
                .iter()
                .filter(|definition| {
                    !only_global
                        || definition.applicability.is_global()
                        || definition.globally_scoped
                })
                .filter(|definition| {
                    decode_css_identifier(definition.name.text())
                        == decode_css_identifier(self.name)
                })
                .map(|definition| HtmlPropertyContext {
                    position: definition.range.start(),
                    kind: HtmlPropertyContextKind::Definition(CssPropertyDefinition {
                        module_path: normalize_path(path),
                        range: definition.range,
                    }),
                }),
        );

        contexts.sort_by_key(|context| context.position);
        contexts
    }

    fn actions_for_html_imports(
        &mut self,
        imported_path: &Utf8Path,
        importer_path: &Utf8Path,
        next_branch: CssPropertyBranch,
    ) -> Vec<UpwardTraversalAction<CssPropertyDefinition, CssPropertyBranch>> {
        let contexts = self.html_property_contexts(importer_path, true);
        let global_definition = contexts
            .iter()
            .rev()
            .find_map(|context| match &context.kind {
                HtmlPropertyContextKind::Definition(definition) => Some(definition.clone()),
                HtmlPropertyContextKind::Import(path) => {
                    self.last_property_in_module_context_from(path, next_branch.clone())
                }
            });
        contexts
            .iter()
            .enumerate()
            .filter(|(_, context)| {
                matches!(
                    &context.kind,
                    HtmlPropertyContextKind::Import(path) if path == imported_path
                )
            })
            .map(|(child_index, _)| {
                let definition = global_definition.clone().or_else(|| {
                    contexts.iter().take(child_index).rev().find_map(|context| {
                        match &context.kind {
                            HtmlPropertyContextKind::Definition(_) => None,
                            HtmlPropertyContextKind::Import(path) => {
                                self.last_property_in_module_context_from(path, next_branch.clone())
                            }
                        }
                    })
                });
                self.action(definition, next_branch.clone())
            })
            .collect()
    }
}

impl UpwardTraversalVisitor for CssPropertyTraversal<'_, '_> {
    type Branch = CssPropertyBranch;
    type Item = CssPropertyDefinition;

    fn db(&self) -> &dyn ModuleDb {
        self.db
    }

    fn should_visit_importer(
        &self,
        imported_path: &Utf8Path,
        importer: ModuleInfo,
        branch: &Self::Branch,
    ) -> bool {
        let importer_path = importer.path(self.db);
        !branch.contains(importer_path) && imports_path(&importer.kind(self.db), imported_path)
    }

    /// Resolves the definition visible from each import of `imported_path`.
    ///
    /// CSS importers prefer their local definition, then sibling imports. HTML
    /// and JavaScript importers search their CSS contexts; only globally
    /// applicable embedded styles escape an HTML importer.
    fn visit_importer(
        &mut self,
        imported_path: &Utf8Path,
        importer: ModuleInfo,
        branch: &Self::Branch,
    ) -> Vec<UpwardTraversalAction<Self::Item, Self::Branch>> {
        let db = self.db;
        let importer_path = importer.path(db);
        let next_branch = branch.with_path(importer_path.to_path_buf());
        let importer = importer.kind(db);
        match importer {
            ModuleInfoKind::Css(info) => info
                .imports
                .iter()
                .filter(|import| import.resolved_path.as_path() == Some(imported_path))
                .map(|_| {
                    let local_definition = self.local_property_definition(importer_path);
                    let definition = local_definition
                        .or_else(|| self.last_property_in_imports(&info, next_branch.clone()));
                    self.action(definition, next_branch.clone())
                })
                .collect(),
            ModuleInfoKind::Js(info) => {
                let imports = js_import_paths_in_source_order(&info)
                    .filter_map(|import| import.as_path())
                    .collect::<Vec<_>>();
                self.actions_for_imports(imported_path, &imports, next_branch)
            }
            ModuleInfoKind::Html(_) => {
                self.actions_for_html_imports(imported_path, importer_path, next_branch)
            }
        }
    }
}

/// Returns whether `module` imports `path` through a supported import.
fn imports_path(module: &ModuleInfoKind, path: &Utf8Path) -> bool {
    match module {
        ModuleInfoKind::Js(info) => info
            .import_paths
            .iter()
            .filter(|import| import.kind.is_dynamic() || import.phase != JsImportPhase::Type)
            .any(|import| import.as_path() == Some(path)),
        ModuleInfoKind::Css(info) => info
            .imports
            .iter()
            .any(|import| import.resolved_path.as_path() == Some(path)),
        ModuleInfoKind::Html(info) => info
            .imported_stylesheets
            .iter()
            .chain(info.import_paths.iter())
            .any(|import| import.as_path() == Some(path)),
    }
}

fn js_import_paths_in_source_order(
    info: &JsModuleInfo,
) -> impl DoubleEndedIterator<Item = &JsImportPath> {
    info.import_paths
        .iter()
        .filter(|import| import.kind.is_dynamic() || import.phase != JsImportPhase::Type)
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
