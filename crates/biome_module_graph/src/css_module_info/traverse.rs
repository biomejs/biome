use crate::db::ModuleDb;
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::traverse::{UpwardTraversalAction, UpwardTraversalVisitor};
use crate::{CssPropertyDefinition, HtmlImport, JsImportPath, JsImportPhase, JsModuleInfo};
use biome_astro_semantic::{BlockKind, astro_model_from_source};
use biome_console::markup;
use biome_css_semantic::db::{
    css_property_definitions_from_model, css_property_definitions_from_snippet,
    css_property_definitions_from_source,
};
use biome_css_semantic::{
    SemanticModelOptions, SyntheticCustomProperty, semantic_model_with_options,
};
use biome_css_syntax::{AnyCssRoot, decode_css_identifier};
use biome_fs::normalize_path;
use biome_parser::db::{SourceExt, TrackedSnippet};
use biome_rowan::{TextRange, TextSize, TokenText};
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use rustc_hash::FxHashSet;
use std::rc::Rc;

#[salsa::tracked]
fn css_property_definitions_from_embedded_snippet<'db>(
    db: &'db dyn ModuleDb,
    snippet: TrackedSnippet<'db>,
) -> Vec<biome_css_semantic::db::CssPropertyDefinition> {
    let Some(file_source) = db
        .source_from_index(snippet.document_source_index(db))
        .and_then(|source| source.to_css_file_source())
    else {
        return Vec::new();
    };
    let mut options = SemanticModelOptions::from(&file_source);
    if options.flavor != biome_css_semantic::SemanticFlavor::Astro {
        return css_property_definitions_from_snippet(db, snippet).clone();
    }

    let astro_model = astro_model_from_source(db, snippet.source(db));
    if let Some(block) = astro_model.blocks().find(|block| {
        block.kind() == BlockKind::Style
            && block
                .content_range()
                .is_some_and(|range| range.contains_range(snippet.content_range(db)))
    }) {
        options.synthetic_custom_properties = block
            .define_vars()
            .map(|variable| {
                SyntheticCustomProperty::new(format!("--{}", variable.semantic_name()))
                    .with_host_range(variable.host_range())
            })
            .collect();
        options.custom_properties_open = !block.define_vars_open_reasons().is_empty();
    }
    let root: AnyCssRoot = snippet.parsed(db).tree();
    let model = semantic_model_with_options(&root, options);
    css_property_definitions_from_model(&model)
}

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

/// Returns the CSS classes that `importer` provides to the modules it imports.
///
/// A JavaScript module provides the CSS files it imports statically. An HTML
/// module provides its linked stylesheets, the CSS files its scripts import,
/// and the classes of its global inline styles. The global styles step is
/// returned even when it contains no classes. A CSS module provides nothing.
pub(crate) fn provided_css_class_steps(
    db: &dyn ModuleDb,
    importer: ModuleInfo,
) -> Vec<CssClassStep> {
    match importer.kind(db) {
        ModuleInfoKind::Js(js_info) => js_info
            .import_paths
            .iter()
            .filter(|import| import.kind.is_static())
            .filter_map(|import_path| {
                let resolved = import_path.resolve_js(db, importer);
                css_class_step(db, resolved.path().as_path()?)
            })
            .collect(),
        ModuleInfoKind::Html(html_info) => {
            let mut items = html_info
                .imported_stylesheets
                .iter()
                .filter_map(|import_path| {
                    let resolved = import_path.resolve_css(db, importer);
                    css_class_step(db, resolved.path().as_path()?)
                })
                .collect::<Vec<_>>();
            items.extend(html_info.import_paths.iter().filter_map(|import_path| {
                let resolved = import_path.resolve_html(db, importer);
                css_class_step(db, resolved.path().as_path()?)
            }));
            items.push(CssClassStep {
                css_path: importer.path(db).to_path_buf(),
                css_classes: html_info.get_global_styles(),
            });
            items
        }
        ModuleInfoKind::Css(_) => Vec::new(),
    }
}

/// Returns whether [provided_css_class_steps] returns at least one step for
/// `module`.
pub(crate) fn provides_css_classes(db: &dyn ModuleDb, module: ModuleInfo) -> bool {
    match module.kind(db) {
        ModuleInfoKind::Js(js_info) => js_info
            .import_paths
            .iter()
            .filter(|import| import.kind.is_static())
            .any(|import_path| {
                import_path
                    .resolve_js(db, module)
                    .path()
                    .as_path()
                    .is_some_and(|path| db.css_module_info_for_path(path).is_some())
            }),
        ModuleInfoKind::Html(_) => true,
        ModuleInfoKind::Css(_) => false,
    }
}

/// Returns the resolved paths that `module` imports and to which it provides
/// CSS classes.
///
/// JavaScript type-only imports are skipped because they don't load the
/// imported module. CSS modules return no paths: a CSS `@import` doesn't
/// expose the importing stylesheet to components.
pub(crate) fn css_class_import_paths(db: &dyn ModuleDb, module: ModuleInfo) -> Vec<Utf8PathBuf> {
    match module.kind(db) {
        ModuleInfoKind::Js(info) => js_import_paths_in_source_order(info)
            .filter_map(|import| import.resolve_js(db, module).path().as_path())
            .map(Utf8Path::to_path_buf)
            .collect(),
        ModuleInfoKind::Html(info) => info
            .imported_stylesheets
            .iter()
            .filter_map(|import| import.resolve_css(db, module).path().as_path())
            .chain(
                info.import_paths
                    .iter()
                    .filter_map(|import| import.resolve_html(db, module).path().as_path()),
            )
            .map(Utf8Path::to_path_buf)
            .collect(),
        ModuleInfoKind::Css(_) => Vec::new(),
    }
}

/// Finds the nearest CSS custom property definitions available to a module.
///
/// It searches modules that import the starting module. Each import path is
/// searched separately and stops at the first definition it finds.
pub(crate) struct CssPropertyTraversal<'db, 'name> {
    /// Module information and parsed sources used during the search.
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
        imports: &[Utf8PathBuf],
        next_branch: CssPropertyBranch,
    ) -> Vec<UpwardTraversalAction<CssPropertyDefinition, CssPropertyBranch>> {
        let definition = imports
            .iter()
            .rev()
            .find_map(|path| self.last_property_in_module_context_from(path, next_branch.clone()));
        imports
            .iter()
            .filter(|path| path.as_path() == imported_path)
            .map(|_| self.action(definition.clone(), next_branch.clone()))
            .collect()
    }

    /// Returns the last visible definition in an HTML-like document.
    pub(crate) fn last_property_in_html_context(
        &self,
        path: &Utf8Path,
    ) -> Option<CssPropertyDefinition> {
        let module = self.db.module_for_path(path)?;
        let branch = CssPropertyBranch::new(path.to_path_buf());
        self.html_property_contexts(module, false)
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
        let module = self.db.module_for_path(path)?;
        let module_kind = module.kind(self.db);
        let info = module_kind.as_js_module_info()?;
        let branch = CssPropertyBranch::new(path.to_path_buf());
        js_import_paths_in_source_order(info)
            .rev()
            .find_map(|import| {
                let resolved = import.resolve_js(self.db, module);
                self.last_property_in_module_context_from(
                    resolved.path().as_path()?,
                    branch.clone(),
                )
            })
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
        // A search ends at its first definition, so an exhausted dependency need
        // not be searched through another path. Keep this set local: separate
        // importer branches exclude different ancestors and can resolve differently.
        let mut visited = FxHashSet::default();
        while let Some(frame) = stack.pop() {
            let (path, branch) = match frame {
                ModuleContextFrame::Module(path, branch) => (path, branch),
                ModuleContextFrame::Definition(definition) => return Some(definition),
            };
            if branch.contains(&path) || !visited.insert(path.clone()) {
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
                        let resolved = import.resolve_css(self.db, module);
                        let path = resolved.path().as_path()?;
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
                        self.html_property_contexts(module, true)
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
                    stack.extend(js_import_paths_in_source_order(info).filter_map(|import| {
                        let resolved = import.resolve_js(self.db, module);
                        let path = resolved.path().as_path()?;
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
        module: ModuleInfo,
        info: &crate::CssModuleInfo,
        branch: CssPropertyBranch,
    ) -> Option<CssPropertyDefinition> {
        info.imports.iter().rev().find_map(|import| {
            let resolved = import.resolve_css(self.db, module);
            self.last_property_in_module_context_from(resolved.path().as_path()?, branch.clone())
        })
    }

    fn local_property_definition(&self, path: &Utf8Path) -> Option<CssPropertyDefinition> {
        // The semantic extractor assumes the parsed source contains a CSS root.
        self.db.css_module_info_for_path(path)?;
        let parsed = self.db.parsed_source_for_path(path)?;
        let definition = css_property_definitions_from_source(self.db, parsed)
            .iter()
            .rev()
            .find(|definition| definition.matches(self.name))?;
        Some(CssPropertyDefinition {
            module_path: normalize_path(path),
            range: definition.range(),
        })
    }

    fn html_property_contexts(
        &self,
        module: ModuleInfo,
        only_global: bool,
    ) -> Vec<HtmlPropertyContext> {
        let path = module.path(self.db);
        let module_kind = module.kind(self.db);
        let Some(info) = module_kind.as_html_module_info() else {
            return Vec::new();
        };
        let is_visible = |import: &&HtmlImport| !only_global || import.applicability.is_global();
        let mut contexts = info
            .imported_stylesheets
            .iter()
            .filter(is_visible)
            .map(|import| (import, import.resolve_css(self.db, module)))
            .chain(
                info.import_paths
                    .iter()
                    .filter(is_visible)
                    .map(|import| (import, import.resolve_html(self.db, module))),
            )
            .filter_map(|(import, resolved)| {
                Some(HtmlPropertyContext {
                    position: import.range.start(),
                    kind: HtmlPropertyContextKind::Import(resolved.path().as_path()?.to_path_buf()),
                })
            })
            .collect::<Vec<_>>();

        if let Some(source) = self.db.parsed_source_for_path(path) {
            let astro_model = self
                .db
                .source_from_index(source.document_source_index(self.db))
                .and_then(|source_type| {
                    source_type
                        .to_html_file_source()
                        .filter(|html| html.is_astro())
                })
                .map(|_| astro_model_from_source(self.db, source));
            if let Some(astro_model) = astro_model {
                let name = decode_css_identifier(self.name);
                let semantic_name = name.strip_prefix("--");
                contexts.extend(astro_model.blocks().filter_map(|block| {
                    if block.kind() != BlockKind::Style
                        || (only_global && !block.is_globally_scoped())
                    {
                        return None;
                    }
                    let semantic_name = semantic_name?;
                    let variable = block.define_vars().find(|variable| {
                        decode_css_identifier(variable.semantic_name()) == semantic_name
                    })?;
                    let range = variable.host_range();
                    Some(HtmlPropertyContext {
                        position: range.start(),
                        kind: HtmlPropertyContextKind::Definition(CssPropertyDefinition {
                            module_path: normalize_path(path),
                            range,
                        }),
                    })
                }));
            }
            for snippet in source.snippets(self.db) {
                let Some(file_source) = self
                    .db
                    .source_from_index(snippet.document_source_index(self.db))
                    .and_then(|source| source.to_css_file_source())
                else {
                    continue;
                };
                if file_source.as_embedding_kind().is_html_style_attribute() {
                    continue;
                }
                let snippet_is_global = file_source.embedding_applicability().is_global();
                let definitions = css_property_definitions_from_embedded_snippet(self.db, *snippet);
                contexts.extend(
                    definitions
                        .iter()
                        .filter(|definition| {
                            definition.matches(self.name)
                                && (!only_global
                                    || snippet_is_global
                                    || definition.is_globally_scoped())
                        })
                        .map(|definition| {
                            let range = definition.range_in_host(snippet.content_offset(self.db));
                            HtmlPropertyContext {
                                position: range.start(),
                                kind: HtmlPropertyContextKind::Definition(CssPropertyDefinition {
                                    module_path: normalize_path(path),
                                    range,
                                }),
                            }
                        }),
                );
            }
        }

        contexts.sort_by_key(|context| context.position);
        contexts
    }

    fn actions_for_html_imports(
        &mut self,
        imported_path: &Utf8Path,
        importer: ModuleInfo,
        next_branch: CssPropertyBranch,
    ) -> Vec<UpwardTraversalAction<CssPropertyDefinition, CssPropertyBranch>> {
        let contexts = self.html_property_contexts(importer, true);
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
        !branch.contains(importer_path) && imports_path(self.db, importer, imported_path)
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
        let importer_kind = importer.kind(db);
        match importer_kind {
            ModuleInfoKind::Css(info) => info
                .imports
                .iter()
                .filter(|import| {
                    import.resolve_css(db, importer).path().as_path() == Some(imported_path)
                })
                .map(|_| {
                    let local_definition = self.local_property_definition(importer_path);
                    let definition = local_definition.or_else(|| {
                        self.last_property_in_imports(importer, info, next_branch.clone())
                    });
                    self.action(definition, next_branch.clone())
                })
                .collect(),
            ModuleInfoKind::Js(info) => {
                let imports = js_import_paths_in_source_order(info)
                    .filter_map(|import| {
                        import
                            .resolve_js(db, importer)
                            .path()
                            .as_path()
                            .map(Utf8Path::to_path_buf)
                    })
                    .collect::<Vec<_>>();
                self.actions_for_imports(imported_path, &imports, next_branch)
            }
            ModuleInfoKind::Html(_) => {
                self.actions_for_html_imports(imported_path, importer, next_branch)
            }
        }
    }
}

/// Returns whether `module` imports `path` through a supported import.
fn imports_path(db: &dyn ModuleDb, module: ModuleInfo, path: &Utf8Path) -> bool {
    match module.kind(db) {
        ModuleInfoKind::Js(info) => info
            .import_paths
            .iter()
            .filter(|import| import.kind.is_dynamic() || import.phase != JsImportPhase::Type)
            .any(|import| import.resolve_js(db, module).path().as_path() == Some(path)),
        ModuleInfoKind::Css(info) => info
            .imports
            .iter()
            .any(|import| import.resolve_css(db, module).path().as_path() == Some(path)),
        ModuleInfoKind::Html(info) => {
            info.imported_stylesheets
                .iter()
                .any(|import| import.resolve_css(db, module).path().as_path() == Some(path))
                || info
                    .import_paths
                    .iter()
                    .any(|import| import.resolve_html(db, module).path().as_path() == Some(path))
        }
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
