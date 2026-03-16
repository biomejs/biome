//! Module graph tracking inferred information such as imports and exports and
//! their types across modules.
//!
//! This can be used by lint rules for things such as cycle detection, and
//! detecting broken imports.
//!
//! The module graph is instantiated and updated inside the Workspace Server.

mod fs_proxy;

use crate::css_module_info::traverse::ImportTreeTraversal;
use crate::css_module_info::{
    CssClassStep, CssModuleInfo, CssModuleVisitor, CssTraversalStep, ImportTreeNode,
    SerializedCssModuleInfo,
};
use crate::html_module_info::{
    HtmlEmbeddedContent, HtmlModuleInfo, HtmlModuleVisitor, SerializedHtmlModuleInfo,
};
use crate::{
    JsExport, JsModuleInfo, JsOwnExport, ModuleDiagnostic, SerializedJsModuleInfo,
    js_module_info::JsModuleVisitor,
};
use biome_css_syntax::AnyCssRoot;
use biome_fs::BiomePath;
use biome_html_syntax::HtmlRoot;
use biome_js_syntax::AnyJsRoot;
use biome_js_type_info::ImportSymbol;
use biome_jsdoc_comment::JsdocComment;
use biome_project_layout::ProjectLayout;
use biome_resolver::{FsWithResolverProxy, PathInfo};
use camino::{Utf8Path, Utf8PathBuf};
pub(crate) use fs_proxy::ModuleGraphFsProxy;
use indexmap::IndexSet;
use papaya::{HashMap, HashMapRef, LocalGuard};
use rustc_hash::{FxBuildHasher, FxHashSet};
use std::collections::VecDeque;
use std::{collections::BTreeSet, ops::Deref};

pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs", "json", "node",
];

/// Data structure for tracking imports and exports across files.
///
/// The module graph is also augmented with type information, allowing types
/// to be looked up from imports as well.
///
/// The module graph is simply a flat mapping from paths to module info
/// structures. This approach makes both lookups easy and makes it very easy for
/// us to invalidate part of the graph when there are file system changes.
#[derive(Debug, Default)]
pub struct ModuleGraph {
    /// Cached module info per file.
    data: HashMap<Utf8PathBuf, ModuleInfo, FxBuildHasher>,

    /// Cache that tracks the presence of files, directories, and symlinks
    /// across the project.
    path_info: HashMap<Utf8PathBuf, Option<PathInfo>>,
}

impl ModuleGraph {
    /// Returns whether the given `path` is indexed in the module graph.
    pub fn contains(&self, path: &Utf8Path) -> bool {
        self.data.pin().contains_key(path)
    }

    /// Returns the module info, such as imports and exports and their types,
    /// for the given `path`.
    pub fn js_module_info_for_path(&self, path: &Utf8Path) -> Option<JsModuleInfo> {
        self.data.pin().get(path).and_then(|info| match info {
            ModuleInfo::Js(module_info) => Some(module_info.clone()),
            _ => None,
        })
    }

    /// Returns the CSS module info for the given `path`, if it is a CSS file
    /// tracked in the module graph.
    pub fn css_module_info_for_path(&self, path: &Utf8Path) -> Option<CssModuleInfo> {
        self.data.pin().get(path).and_then(|info| match info {
            ModuleInfo::Css(module_info) => Some(module_info.clone()),
            _ => None,
        })
    }

    /// Returns the HTML module info for the given `path`, if it is an HTML file
    /// tracked in the module graph.
    pub fn html_module_info_for_path(&self, path: &Utf8Path) -> Option<HtmlModuleInfo> {
        self.data.pin().get(path).and_then(|info| match info {
            ModuleInfo::Html(module_info) => Some(module_info.clone()),
            _ => None,
        })
    }

    /// Returns all files that transitively import `path` (through CSS `@import`
    /// chains and HTML `<link>` references).
    ///
    /// The returned set includes only JS/HTML files (potential class consumers),
    /// not intermediate CSS files.
    ///
    /// This performs a breadth-first search over the import graph starting from
    /// `path`, visiting each importer level by level before moving deeper. It is
    /// intended to be called at lint time when the graph is fully built.
    pub fn transitive_importers_of(&self, path: &Utf8Path) -> Vec<Utf8PathBuf> {
        let data = self.data.pin();
        let mut result = Vec::new();
        let mut visited: FxHashSet<Utf8PathBuf> = FxHashSet::default();
        let mut queue = VecDeque::new();
        queue.push_back(path.to_path_buf());

        while let Some(current) = queue.pop_front() {
            if !visited.insert(current.clone()) {
                continue;
            }

            // Scan all entries in the module graph for files that import `current`.
            for (file_path, module_info) in data.iter() {
                if file_path == &current {
                    continue;
                }
                let imports_current = match module_info {
                    ModuleInfo::Js(js_info) => js_info
                        .static_import_paths
                        .values()
                        .chain(js_info.dynamic_import_paths.values())
                        .any(|p| p.as_path() == Some(current.as_path())),
                    ModuleInfo::Css(css_info) => css_info
                        .imports
                        .values()
                        .any(|p| p.resolved_path.as_path() == Some(current.as_path())),
                    ModuleInfo::Html(html_info) => {
                        html_info
                            .imported_stylesheets
                            .iter()
                            .any(|p| p.as_path() == Some(current.as_path()))
                            || html_info
                                .static_import_paths
                                .values()
                                .any(|p| p.as_path() == Some(current.as_path()))
                            || html_info
                                .dynamic_import_paths
                                .values()
                                .any(|p| p.as_path() == Some(current.as_path()))
                    }
                };

                if imports_current && !visited.contains(file_path.as_path()) {
                    // Collect JS/HTML as consumers; re-enqueue CSS for transitive traversal.
                    match module_info {
                        ModuleInfo::Js(_) | ModuleInfo::Html(_) => {
                            result.push(file_path.clone());
                        }
                        ModuleInfo::Css(_) => {
                            queue.push_back(file_path.clone());
                        }
                    }
                }
            }
        }

        result
    }

    /// Returns `true` if the given CSS `class_name` is referenced in any
    /// JS or HTML file that transitively imports `css_path`.
    pub fn is_class_referenced_by_importers(&self, css_path: &Utf8Path, class_name: &str) -> bool {
        let importers = self.transitive_importers_of(css_path);
        let data = self.data.pin();

        // For each file that imports the CSS (directly or transitively),
        // check if the class is used in that file OR in any component it imports
        for importer_path in &importers {
            if self.is_class_used_in_component_tree(&data, importer_path, class_name) {
                return true;
            }
        }
        false
    }

    /// Checks if a class is used in a file or any of its imported components (transitively).
    /// This handles component hierarchies like: App.jsx imports Button.jsx → Button uses class
    fn is_class_used_in_component_tree(
        &self,
        data: &HashMapRef<'_, Utf8PathBuf, ModuleInfo, FxBuildHasher, LocalGuard<'_>>,
        file_path: &Utf8Path,
        class_name: &str,
    ) -> bool {
        let mut visited = FxHashSet::default();
        let mut queue = VecDeque::new();
        queue.push_back(file_path.to_path_buf());

        while let Some(current_path) = queue.pop_front() {
            // Skip if already visited (prevent infinite loops in circular imports)
            if !visited.insert(current_path.clone()) {
                continue;
            }

            if let Some(module_info) = data.get(current_path.as_path()) {
                match module_info {
                    ModuleInfo::Js(js_info) => {
                        // Check if this file uses the class
                        if js_info
                            .referenced_classes
                            .iter()
                            .any(|r| r.matches(class_name))
                        {
                            return true;
                        }
                        // Add all JS/TS imports to check transitively
                        // (components imported by this file might use the class)
                        for import_path in js_info
                            .static_import_paths
                            .values()
                            .chain(js_info.dynamic_import_paths.values())
                        {
                            if let Some(path) = import_path.as_path() {
                                queue.push_back(path.to_path_buf());
                            }
                        }
                    }
                    ModuleInfo::Html(html_info) => {
                        // Check if this HTML file uses the class
                        if html_info
                            .referenced_classes
                            .iter()
                            .any(|r| r.matches(class_name))
                        {
                            return true;
                        }
                    }
                    ModuleInfo::Css(_) => {
                        // CSS files don't use classes, skip
                    }
                }
            }
        }

        false
    }

    /// Returns an iterator that lazily traverses the import tree for the given JS file.
    ///
    /// Each step yields a `CssClassStep` containing minimal data:
    /// - The CSS file path
    /// - The CSS classes found in that file
    ///
    /// This allows consumers to stop iteration early if a class is found, avoiding
    /// unnecessary traversal of the entire tree. For diagnostic information including
    /// component chains, use `build_diagnostic_traversal_chain()` instead.
    pub fn traverse_import_tree_for_classes(
        &self,
        js_path: &Utf8Path,
    ) -> impl Iterator<Item = CssClassStep> + '_ {
        // Collect direct imports from the JS file itself
        let direct_steps: Vec<_> = if let Some(js_info) = self.js_module_info_for_path(js_path) {
            js_info
                .static_import_paths
                .values()
                .filter_map(|import_path| {
                    let path = import_path.as_path()?;
                    let css_info = self.css_module_info_for_path(path)?;

                    Some(CssClassStep {
                        css_path: path.to_path_buf(),
                        css_classes: css_info.classes.clone(),
                    })
                })
                .collect()
        } else {
            Vec::new()
        };

        // Set up DFS stack for upward traversal
        let stack = vec![js_path.to_path_buf()];

        let mut visited = FxHashSet::default();
        visited.insert(js_path.to_path_buf());

        // Chain direct imports with upward traversal
        direct_steps.into_iter().chain(ImportTreeTraversal {
            module_graph: self,
            stack,
            visited,
            current_css_iter: None,
        })
    }

    /// Builds diagnostic information with full component chains for error reporting.
    ///
    /// This re-traverses the import tree to build the component chain for each CSS file.
    /// Only call this when generating diagnostics (e.g., when a class is not found),
    /// not during the happy path.
    ///
    /// Returns a vector of `TraversalStep` with complete diagnostic information including
    /// component chains showing the path from the starting file to each CSS import.
    pub fn build_diagnostic_traversal_chain(&self, js_path: &Utf8Path) -> Vec<CssTraversalStep> {
        // Re-use the existing eager collection function which has all the chain-building logic
        let (_classes, traversal_path) = self.collect_available_classes_for_js_file(js_path);
        traversal_path
    }

    /// Builds a tree structure representing the import relationships for diagnostic display.
    ///
    /// This creates a hierarchical view of:
    /// - The starting file (root)
    /// - CSS files it directly imports
    /// - Parent components that import the file
    /// - CSS files imported by those parents (recursively)
    ///
    /// Returns `None` if the file is not found in the module graph.
    pub fn build_import_tree(&self, js_path: &Utf8Path) -> Option<ImportTreeNode> {
        // Build the root node for the starting file
        let mut root = ImportTreeNode {
            file_path: js_path.to_path_buf(),
            css_imports: Vec::new(),
            parent_components: Vec::new(),
        };

        // Collect CSS files directly imported by this JS file
        if let Some(js_info) = self.js_module_info_for_path(js_path) {
            root.css_imports = js_info
                .static_import_paths
                .values()
                .filter_map(|import_path| {
                    let path = import_path.as_path()?;
                    // Only include CSS files
                    self.css_module_info_for_path(path)?;
                    Some(path.to_path_buf())
                })
                .collect();
        } else {
            // File not found in module graph
            return None;
        }

        // Build parent tree recursively
        let mut visited = FxHashSet::default();
        visited.insert(js_path.to_path_buf());
        root.parent_components = self.build_parent_nodes(js_path, &mut visited);

        Some(root)
    }

    /// Helper function to recursively build parent component nodes.
    fn build_parent_nodes(
        &self,
        current_path: &Utf8Path,
        visited: &mut FxHashSet<Utf8PathBuf>,
    ) -> Vec<ImportTreeNode> {
        let data = self.data();
        let mut parents = Vec::new();

        // Find all files that import current_path
        for (file_path, module_info) in data.iter() {
            if visited.contains(file_path) {
                continue;
            }

            let imports_current = match module_info {
                ModuleInfo::Js(js_info) => js_info
                    .static_import_paths
                    .values()
                    .chain(js_info.dynamic_import_paths.values())
                    .any(|p| p.as_path() == Some(current_path)),
                ModuleInfo::Html(html_info) => {
                    // An HTML-like file (Vue/Astro/Svelte) can import another
                    // HTML-like file via its embedded <script> block imports, or
                    // link a stylesheet via <link rel="stylesheet">.
                    html_info
                        .imported_stylesheets
                        .iter()
                        .chain(html_info.static_import_paths.values())
                        .chain(html_info.dynamic_import_paths.values())
                        .any(|p| p.as_path() == Some(current_path))
                }
                ModuleInfo::Css(_) => false,
            };

            if imports_current {
                // Collect CSS imports from this parent
                let css_imports: Vec<Utf8PathBuf> = match module_info {
                    ModuleInfo::Js(js_info) => js_info
                        .static_import_paths
                        .values()
                        .filter_map(|import_path| {
                            let path = import_path.as_path()?;
                            // Only include CSS files
                            self.css_module_info_for_path(path)?;
                            Some(path.to_path_buf())
                        })
                        .collect(),
                    ModuleInfo::Html(html_info) => html_info
                        .imported_stylesheets
                        .iter()
                        .chain(html_info.static_import_paths.values())
                        .chain(html_info.dynamic_import_paths.values())
                        .filter_map(|stylesheet_path| {
                            let path = stylesheet_path.as_path()?;
                            self.css_module_info_for_path(path)?;
                            Some(path.to_path_buf())
                        })
                        .collect(),
                    ModuleInfo::Css(_) => Vec::new(),
                };

                // Clone visited set for this branch to allow the same node in different branches
                // while still preventing cycles within a branch
                let mut branch_visited = visited.clone();
                branch_visited.insert(file_path.clone());

                // Recursively build this parent's tree
                let parent_components = self.build_parent_nodes(file_path, &mut branch_visited);

                parents.push(ImportTreeNode {
                    file_path: file_path.clone(),
                    css_imports,
                    parent_components,
                });
            }
        }

        parents
    }

    /// Returns an iterator that lazily traverses the import tree for the given HTML file,
    /// yielding CSS class steps from:
    ///
    /// 1. All inline `<style>` blocks in the HTML file itself — both globally and
    ///    locally scoped, since scoped styles are valid for the component's own
    ///    elements.
    /// 2. CSS files directly linked via `<link rel="stylesheet">`.
    /// 3. CSS files imported by parent JS/HTML files that transitively import
    ///    this HTML file (upward traversal) — only globally scoped classes from
    ///    parent files are included.
    pub fn traverse_import_tree_for_html_classes(
        &self,
        html_path: &Utf8Path,
    ) -> impl Iterator<Item = CssClassStep> + '_ {
        let mut inline_steps = Vec::new();
        let mut linked_steps = Vec::new();

        if let Some(html_info) = self.html_module_info_for_path(html_path) {
            // 1. All inline style classes (global and local).
            // For same-file checks, scoped styles still apply to the component's own
            // elements, so both Global and Local classes are valid. Only when classes
            // are traversed from imported/parent files do we restrict to Global.
            let all_inline_classes: IndexSet<_> = html_info
                .style_classes
                .iter()
                .map(|c| c.name.clone())
                .collect();
            if !all_inline_classes.is_empty() {
                inline_steps.push(CssClassStep {
                    css_path: html_path.to_path_buf(),
                    css_classes: all_inline_classes,
                });
            }

            // 2. Directly linked external stylesheets.
            for stylesheet_path in &html_info.imported_stylesheets {
                if let Some(path) = stylesheet_path.as_path()
                    && let Some(css_info) = self.css_module_info_for_path(path)
                {
                    linked_steps.push(CssClassStep {
                        css_path: path.to_path_buf(),
                        css_classes: css_info.classes.clone(),
                    });
                }
            }

            // 3. CSS files imported via static imports from embedded scripts
            //    (e.g., Astro frontmatter `import "./styles.css"`).
            for import_path in html_info
                .static_import_paths
                .values()
                .chain(html_info.dynamic_import_paths.values())
            {
                if let Some(path) = import_path.as_path()
                    && let Some(css_info) = self.css_module_info_for_path(path)
                {
                    linked_steps.push(CssClassStep {
                        css_path: path.to_path_buf(),
                        css_classes: css_info.classes.clone(),
                    });
                }
            }
        }

        // 4. Upward traversal: CSS imported by parent files that import this HTML file.
        let stack = vec![html_path.to_path_buf()];
        let mut visited = FxHashSet::default();
        visited.insert(html_path.to_path_buf());

        inline_steps
            .into_iter()
            .chain(linked_steps)
            .chain(ImportTreeTraversal {
                module_graph: self,
                stack,
                visited,
                current_css_iter: None,
            })
    }

    /// Builds a tree structure representing the import relationships for an HTML file,
    /// for use in diagnostic display.
    ///
    /// Mirrors [`Self::build_import_tree`] but for HTML files: it includes
    /// directly linked CSS (`<link rel="stylesheet">`) and traverses upward
    /// through parent importers.
    ///
    /// Returns `None` if the file is not found in the module graph.
    pub fn build_import_tree_for_html(&self, html_path: &Utf8Path) -> Option<ImportTreeNode> {
        let html_info = self.html_module_info_for_path(html_path)?;

        let css_imports: Vec<_> = html_info
            .imported_stylesheets
            .iter()
            .chain(html_info.static_import_paths.values())
            .filter_map(|stylesheet_path| {
                let path = stylesheet_path.as_path()?;
                self.css_module_info_for_path(path)?;
                Some(path.to_path_buf())
            })
            .collect();

        let mut root = ImportTreeNode {
            file_path: html_path.to_path_buf(),
            css_imports,
            parent_components: Vec::new(),
        };

        let mut visited = FxHashSet::default();
        visited.insert(html_path.to_path_buf());
        root.parent_components = self.build_parent_nodes(html_path, &mut visited);

        Some(root)
    }

    /// Collects all CSS classes available through the import tree of the given JS file.
    ///
    /// Traverses the import tree starting from `js_path`, collecting classes from:
    /// 1. CSS files directly imported by the JS file itself
    /// 2. CSS files imported by parent JS files that import this JS file
    ///
    /// Returns a tuple of (available_classes, traversal_path) where:
    /// - `available_classes` is a set of all CSS class names found
    /// - `traversal_path` is a vector describing the files visited (for diagnostics)
    pub fn collect_available_classes_for_js_file(
        &self,
        js_path: &Utf8Path,
    ) -> (FxHashSet<String>, Vec<CssTraversalStep>) {
        let data = self.data.pin();
        let mut available_classes = FxHashSet::default();
        let mut traversal_path = Vec::new();
        let mut visited = FxHashSet::default();

        // Start with the JS file itself - collect directly imported CSS
        if let Some(js_info) = self.js_module_info_for_path(js_path) {
            for import_path in js_info
                .static_import_paths
                .values()
                .chain(js_info.dynamic_import_paths.values())
            {
                if let Some(path) = import_path.as_path()
                    && let Some(css_info) = self.css_module_info_for_path(path)
                {
                    for class in css_info.classes.iter() {
                        let class_name = class.text().to_string();
                        available_classes.insert(class_name.clone());
                    }
                    traversal_path.push(CssTraversalStep {
                        css_path: path.to_path_buf(),
                        importer_path: js_path.to_path_buf(),
                        component_chain: vec![js_path.to_path_buf()],
                        is_direct: true,
                    });
                }
            }
        }

        // Now traverse upward: find files that import this JS file
        // and collect CSS they import
        let mut queue: VecDeque<_> = VecDeque::new();
        queue.push_back((js_path.to_path_buf(), vec![js_path.to_path_buf()]));
        visited.insert(js_path.to_path_buf());

        while let Some((current_path, current_chain)) = queue.pop_front() {
            // Find all files that import current_path
            for (file_path, module_info) in data.iter() {
                if visited.contains(file_path.as_path()) {
                    continue;
                }

                let imports_current = match module_info {
                    ModuleInfo::Js(js_info) => js_info
                        .static_import_paths
                        .values()
                        .chain(js_info.dynamic_import_paths.values())
                        .any(|p| p.as_path() == Some(current_path.as_path())),
                    ModuleInfo::Html(html_info) => html_info
                        .imported_stylesheets
                        .iter()
                        .chain(html_info.static_import_paths.values())
                        .chain(html_info.dynamic_import_paths.values())
                        .any(|p| p.as_path() == Some(current_path.as_path())),
                    ModuleInfo::Css(_) => false,
                };

                if imports_current {
                    visited.insert(file_path.clone());

                    match module_info {
                        ModuleInfo::Js(js_info) => {
                            let mut new_chain = current_chain.clone();
                            new_chain.push(file_path.clone());

                            // Collect CSS files imported by this parent JS file
                            for import_path in js_info
                                .static_import_paths
                                .values()
                                .chain(js_info.dynamic_import_paths.values())
                            {
                                if let Some(path) = import_path.as_path()
                                    && let Some(css_info) = self.css_module_info_for_path(path)
                                {
                                    for class in css_info.classes.iter() {
                                        let class_name = class.text().to_string();
                                        available_classes.insert(class_name.clone());
                                    }
                                    traversal_path.push(CssTraversalStep {
                                        css_path: path.to_path_buf(),
                                        importer_path: file_path.clone(),
                                        component_chain: new_chain.clone(),
                                        is_direct: false,
                                    });
                                }
                            }
                            // Continue traversing upward
                            queue.push_back((file_path.clone(), new_chain));
                        }
                        ModuleInfo::Html(html_info) => {
                            let mut new_chain = current_chain.clone();
                            new_chain.push(file_path.clone());

                            // Collect CSS files linked by this parent HTML-like file.
                            for stylesheet_path in html_info
                                .imported_stylesheets
                                .iter()
                                .chain(html_info.static_import_paths.values())
                                .chain(html_info.dynamic_import_paths.values())
                            {
                                if let Some(path) = stylesheet_path.as_path()
                                    && let Some(css_info) = self.css_module_info_for_path(path)
                                {
                                    for class in css_info.classes.iter() {
                                        let class_name = class.text().to_string();
                                        available_classes.insert(class_name.clone());
                                    }
                                    traversal_path.push(CssTraversalStep {
                                        css_path: path.to_path_buf(),
                                        importer_path: file_path.clone(),
                                        component_chain: new_chain.clone(),
                                        is_direct: false,
                                    });
                                }
                            }
                            // Continue traversing upward
                            queue.push_back((file_path.clone(), new_chain));
                        }
                        ModuleInfo::Css(_) => {}
                    }
                }
            }
        }

        (available_classes, traversal_path)
    }

    /// Returns the data of the module graph in test
    pub fn data(&self) -> HashMapRef<'_, Utf8PathBuf, ModuleInfo, FxBuildHasher, LocalGuard<'_>> {
        self.data.pin()
    }

    /// Updates the module graph to add, update, or remove files.
    ///
    /// Only JavaScript/TypeScript files need to be provided as part of
    /// `added_or_updated_paths` and `removed_paths`. Manifests are expected to
    /// be resolved through the `project_layout`. As such, the `project_layout`
    /// must have been updated before calling this method.
    ///
    /// Returns the dependencies of all the paths that were added or updated.
    pub fn update_graph_for_js_paths(
        &self,
        fs: &dyn FsWithResolverProxy,
        project_layout: &ProjectLayout,
        added_or_updated_paths: &[(
            &BiomePath,
            AnyJsRoot,
            std::sync::Arc<biome_js_semantic::SemanticModel>,
        )],
        enable_type_inference: bool,
    ) -> (ModuleDependencies, Vec<ModuleDiagnostic>) {
        // Make sure all directories are registered for the added/updated paths.
        let path_info = self.path_info.pin();
        for (path, _, _) in added_or_updated_paths {
            let mut parent = path.parent();
            while let Some(path) = parent {
                let mut inserted = false;
                path_info.get_or_insert_with(path.to_path_buf(), || {
                    inserted = true;
                    fs.path_info(path).ok()
                });
                if !inserted {
                    break;
                }
                parent = path.parent();
            }
        }

        let fs_proxy = ModuleGraphFsProxy::new(fs, self, project_layout);
        let mut dependencies = ModuleDependencies::default();
        let mut diagnostics = Vec::new();

        // Traverse all the added and updated paths and insert their module
        // info.
        let modules = self.data.pin();
        for (path, root, semantic_model) in added_or_updated_paths {
            let directory = path.parent().unwrap_or(path);
            let visitor = JsModuleVisitor::new(
                root.clone(),
                path.to_path_buf(),
                directory,
                &fs_proxy,
                semantic_model.clone(),
                enable_type_inference,
            );

            let module_info = visitor.collect_info();
            for import_path in module_info.all_import_paths() {
                if let Some(path) = import_path.as_path() {
                    dependencies.insert(path.to_path_buf());
                }
            }

            for diagnostic in module_info.diagnostics() {
                diagnostics.push(diagnostic.clone());
            }

            modules.insert(path.to_path_buf(), module_info.into());
        }

        (dependencies, diagnostics)
    }

    pub fn update_graph_for_css_paths(
        &self,
        fs: &dyn FsWithResolverProxy,
        project_layout: &ProjectLayout,
        added_or_updated_paths: &[(&BiomePath, AnyCssRoot)],
        _semantic_model: Option<&biome_css_semantic::model::SemanticModel>,
    ) -> (ModuleDependencies, Vec<ModuleDiagnostic>) {
        // Make sure all directories are registered for the added/updated paths.
        let path_info = self.path_info.pin();
        for (path, _) in added_or_updated_paths {
            let mut parent = path.parent();
            while let Some(path) = parent {
                let mut inserted = false;
                path_info.get_or_insert_with(path.to_path_buf(), || {
                    inserted = true;
                    fs.path_info(path).ok()
                });
                if !inserted {
                    break;
                }
                parent = path.parent();
            }
        }

        let fs_proxy = ModuleGraphFsProxy::new(fs, self, project_layout);
        let mut dependencies = ModuleDependencies::default();
        let diagnostics = Vec::new();

        // Traverse all the added and updated paths and insert their module
        // info.
        let modules = self.data.pin();

        for (path, root) in added_or_updated_paths {
            let directory = path.parent().unwrap_or(path);
            let visitor = CssModuleVisitor::new(root.clone(), directory, &fs_proxy);

            let module = visitor.visit();

            for (_, path) in module.0.imports.deref() {
                if let Some(path) = path.resolved_path.as_path() {
                    dependencies.insert(path.to_path_buf());
                }
            }

            modules.insert(path.to_path_buf(), module.into());
        }

        (dependencies, diagnostics)
    }

    /// Updates the module graph for a single HTML file.
    ///
    /// Accepts a slice of `(path, html_root, embedded_content)` triples where
    /// `embedded_content` is a flat list of [`HtmlEmbeddedContent`] blocks —
    /// one variant per `<style>` or `<script>` block found in the file. The
    /// caller is responsible for parsing the blocks and resolving any metadata
    /// (e.g. `CssFileSource` carrying [`EmbeddingApplicability`]); this method
    /// handles all downstream logic.
    ///
    /// Collected data:
    /// - CSS class names from `<style>` blocks (scoped per `EmbeddingApplicability`).
    /// - CSS class references from `class="…"` HTML attributes.
    /// - External stylesheet paths from `<link rel="stylesheet" href="…">`.
    /// - Static JS import paths from `<script>` blocks (for upward traversal in
    ///   Vue/Astro/Svelte component hierarchies).
    pub fn update_graph_for_html_paths(
        &self,
        fs: &dyn FsWithResolverProxy,
        project_layout: &ProjectLayout,
        added_or_updated_paths: &[(&BiomePath, HtmlRoot, Vec<HtmlEmbeddedContent>)],
    ) -> (ModuleDependencies, Vec<ModuleDiagnostic>) {
        // Register directory path info (same pattern as CSS/JS).
        let path_info = self.path_info.pin();
        for (path, _, _) in added_or_updated_paths {
            let mut parent = path.parent();
            while let Some(path) = parent {
                let mut inserted = false;
                path_info.get_or_insert_with(path.to_path_buf(), || {
                    inserted = true;
                    fs.path_info(path).ok()
                });
                if !inserted {
                    break;
                }
                parent = path.parent();
            }
        }

        let fs_proxy = ModuleGraphFsProxy::new(fs, self, project_layout);
        let mut dependencies = ModuleDependencies::default();
        let diagnostics = Vec::new();

        // Traverse all the added and updated paths and insert their module info.
        let modules = self.data.pin();

        for (path, html_root, embedded_content) in added_or_updated_paths {
            let directory = path.parent().unwrap_or(path);
            let visitor = HtmlModuleVisitor::new(
                html_root.clone(),
                embedded_content,
                path.to_path_buf(),
                directory,
                &fs_proxy,
            );

            let module = visitor.visit();

            for resolved_path in &module.imported_stylesheets {
                if let Some(p) = resolved_path.as_path() {
                    dependencies.insert(p.to_path_buf());
                }
            }

            for resolved_path in module
                .static_import_paths
                .values()
                .chain(module.dynamic_import_paths.values())
            {
                if let Some(p) = resolved_path.as_path() {
                    dependencies.insert(p.to_path_buf());
                }
            }

            modules.insert(path.to_path_buf(), module.into());
        }

        (dependencies, diagnostics)
    }

    pub fn update_graph_for_removed_paths(&self, removed_paths: &[&BiomePath]) {
        let modules = self.data.pin();
        // Clean up removed paths from the module graph and path info cache.
        let path_info = self.path_info.pin();
        // Clean up removed paths.
        for removed_path in removed_paths {
            modules.remove(removed_path.as_path());
            path_info.remove(removed_path.as_path());
        }
    }

    pub fn get_or_insert_path_info(
        &self,
        path: &Utf8Path,
        fs: &dyn FsWithResolverProxy,
    ) -> Option<PathInfo> {
        self.path_info
            .pin()
            .get_or_insert_with(path.to_path_buf(), || fs.path_info(path).ok())
            .clone()
    }

    /// Unloads all paths from the graph within the given `path`.
    ///
    /// This method works both for unloading folders as well as individual
    /// files.
    pub fn unload_path(&self, path: &Utf8Path) {
        let data = self.data.pin();
        for indexed_path in data.keys() {
            if indexed_path.starts_with(path) {
                data.remove(indexed_path);
            }
        }
    }

    /// Finds an exported symbol by `symbol_name` as exported by `module`.
    ///
    /// Follows re-exports if necessary.
    pub(crate) fn find_exported_symbol(
        &self,
        module: &JsModuleInfo,
        symbol_name: &str,
    ) -> Option<JsOwnExport> {
        let data = self.data.pin();
        let mut seen_paths = BTreeSet::new();

        find_exported_symbol_with_seen_paths(&data, module, symbol_name, &mut seen_paths)
            .map(|(_, export)| export.clone())
    }

    /// Finds an exported symbol by `symbol_name` as exported by `module`.
    ///
    /// Follows re-exports if necessary.
    pub(crate) fn find_jsdoc_for_exported_symbol(
        &self,
        module: &JsModuleInfo,
        symbol_name: &str,
    ) -> Option<JsdocComment> {
        let data = self.data.pin();
        let mut seen_paths = BTreeSet::new();

        find_exported_symbol_with_seen_paths(&data, module, symbol_name, &mut seen_paths).and_then(
            |(module, export)| match export {
                JsOwnExport::Binding(binding_range) => module
                    .binding_type_data(*binding_range)
                    .and_then(|data| data.jsdoc.clone()),
                JsOwnExport::Type(_) => None,
                JsOwnExport::Namespace(reexport) => reexport.jsdoc_comment.clone(),
            },
        )
    }
}

#[derive(Debug)]
pub enum ModuleInfo {
    Js(JsModuleInfo),
    Css(CssModuleInfo),
    Html(HtmlModuleInfo),
}

impl From<JsModuleInfo> for ModuleInfo {
    fn from(value: JsModuleInfo) -> Self {
        Self::Js(value)
    }
}

impl From<CssModuleInfo> for ModuleInfo {
    fn from(value: CssModuleInfo) -> Self {
        Self::Css(value)
    }
}

impl From<HtmlModuleInfo> for ModuleInfo {
    fn from(value: HtmlModuleInfo) -> Self {
        Self::Html(value)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SerializedModuleInfo {
    Js(SerializedJsModuleInfo),
    Css(SerializedCssModuleInfo),
    Html(SerializedHtmlModuleInfo),
}

impl SerializedModuleInfo {
    pub fn as_js_module_info(&self) -> Option<&SerializedJsModuleInfo> {
        match self {
            Self::Js(module) => Some(module),
            _ => None,
        }
    }

    pub fn as_css_module_info(&self) -> Option<&SerializedCssModuleInfo> {
        match self {
            Self::Css(module) => Some(module),
            _ => None,
        }
    }

    pub fn as_html_module_info(&self) -> Option<&SerializedHtmlModuleInfo> {
        match self {
            Self::Html(module) => Some(module),
            _ => None,
        }
    }
}

impl ModuleInfo {
    pub fn dump(&self) -> SerializedModuleInfo {
        match self {
            Self::Js(module) => SerializedModuleInfo::Js(module.dump()),
            Self::Css(module) => SerializedModuleInfo::Css(module.dump()),
            Self::Html(module) => SerializedModuleInfo::Html(module.dump()),
        }
    }

    pub fn as_js_module_info(&self) -> Option<&JsModuleInfo> {
        match self {
            Self::Js(module) => Some(module),
            _ => None,
        }
    }

    pub fn as_css_module_info(&self) -> Option<&CssModuleInfo> {
        match self {
            Self::Css(module) => Some(module),
            _ => None,
        }
    }

    pub fn as_html_module_info(&self) -> Option<&HtmlModuleInfo> {
        match self {
            Self::Html(module) => Some(module),
            _ => None,
        }
    }
}

fn find_exported_symbol_with_seen_paths<'a>(
    data: &'a HashMapRef<Utf8PathBuf, ModuleInfo, FxBuildHasher, LocalGuard>,
    module: &'a JsModuleInfo,
    symbol_name: &str,
    seen_paths: &mut BTreeSet<&'a Utf8Path>,
) -> Option<(&'a JsModuleInfo, &'a JsOwnExport)> {
    match module.exports.get(symbol_name) {
        Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
            Some((module, own_export))
        }
        Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
            match &reexport.import.symbol {
                ImportSymbol::All => {
                    // TODO: Follow namespace exports.
                    None
                }
                // The source-side name may differ from the export name when the
                // reexport uses an alias, e.g. `export { l as beforeEach } from
                // './tasks'`. In that case we must look up the source-side name
                // (`l`) in the target module, not the alias (`beforeEach`).
                ImportSymbol::Named(source_name) => {
                    let lookup = source_name.text();
                    match reexport.import.resolved_path.as_deref() {
                        Ok(path) if seen_paths.insert(path) => data.get(path).and_then(|module| {
                            if let ModuleInfo::Js(module) = module {
                                find_exported_symbol_with_seen_paths(
                                    data, module, lookup, seen_paths,
                                )
                            } else {
                                None
                            }
                        }),
                        _ => None,
                    }
                }
                ImportSymbol::Default => match reexport.import.resolved_path.as_deref() {
                    Ok(path) if seen_paths.insert(path) => data.get(path).and_then(|module| {
                        if let ModuleInfo::Js(module) = module {
                            find_exported_symbol_with_seen_paths(
                                data,
                                module,
                                symbol_name,
                                seen_paths,
                            )
                        } else {
                            None
                        }
                    }),
                    _ => None,
                },
            }
        }
        None => module.blanket_reexports.iter().find_map(|reexport| {
            match reexport.import.resolved_path.as_deref() {
                Ok(path) if seen_paths.insert(path) => data.get(path).and_then(|module| {
                    if let ModuleInfo::Js(module) = module {
                        find_exported_symbol_with_seen_paths(data, module, symbol_name, seen_paths)
                    } else {
                        None
                    }
                }),
                _ => None,
            }
        }),
    }
}

/// Represents all the files that are imported/depended on by a module.
#[derive(Debug, Default)]
pub struct ModuleDependencies(FxHashSet<Utf8PathBuf>);

impl ModuleDependencies {
    /// Adds a dependency to the module dependencies, if it wasn't added yet.
    pub fn insert(&mut self, dependency_path: Utf8PathBuf) {
        self.0.insert(dependency_path);
    }
}

impl AsRef<FxHashSet<Utf8PathBuf>> for ModuleDependencies {
    fn as_ref(&self) -> &FxHashSet<Utf8PathBuf> {
        &self.0
    }
}

impl Deref for ModuleDependencies {
    type Target = FxHashSet<Utf8PathBuf>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<FxHashSet<Utf8PathBuf>> for ModuleDependencies {
    fn from(dependencies: FxHashSet<Utf8PathBuf>) -> Self {
        Self(dependencies)
    }
}

impl FromIterator<Utf8PathBuf> for ModuleDependencies {
    fn from_iter<T: IntoIterator<Item = Utf8PathBuf>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for ModuleDependencies {
    type Item = Utf8PathBuf;

    type IntoIter = <FxHashSet<Utf8PathBuf> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
