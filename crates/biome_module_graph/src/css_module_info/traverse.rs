use crate::ModuleInfo;
use biome_console::markup;
use biome_rowan::TokenText;
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexSet;
use rustc_hash::FxHashSet;
use std::vec::IntoIter;

/// Minimal step for efficient CSS class checking during traversal.
/// Used in the happy path where we're just checking if classes exist.
#[derive(Debug, Clone)]
pub struct CssClassStep {
    /// The path of the CSS file discovered in this step
    pub css_path: Utf8PathBuf,
    /// The CSS class names found in this CSS file
    pub css_classes: IndexSet<TokenText>,
}

/// Rich diagnostic information including component chain.
/// Only built when generating error diagnostics (class not found).
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct ImportTreeNode {
    /// The path of this file (JS/JSX/HTML component)
    pub file_path: Utf8PathBuf,
    /// CSS files directly imported by this file
    pub css_imports: Vec<Utf8PathBuf>,
    /// Parent components that import this file (recursive tree structure)
    pub parent_components: Vec<Self>,
}

/// Iterator for lazily traversing the import tree upward from a JS file,
/// yielding CSS files imported by parent components (minimal data for performance).
///
/// Uses depth-first search (DFS) with an iterative stack to explore the import tree.
/// This approach is simpler and more memory-efficient than BFS.
pub struct ImportTreeTraversal<'a> {
    pub(crate) module_graph: &'a crate::ModuleGraph,
    /// DFS stack of file paths to process
    pub(crate) stack: Vec<Utf8PathBuf>,
    /// Set of already-visited files to prevent cycles
    pub(crate) visited: FxHashSet<Utf8PathBuf>,
    /// Iterator over CSS files from the current parent file
    pub(crate) current_css_iter: Option<IntoIter<CssClassStep>>,
}

impl<'a> Iterator for ImportTreeTraversal<'a> {
    type Item = CssClassStep;

    fn next(&mut self) -> Option<Self::Item> {
        // First, yield any remaining CSS files from the current parent
        if let Some(ref mut iter) = self.current_css_iter {
            if let Some(step) = iter.next() {
                return Some(step);
            } else {
                // Exhausted current CSS iterator, clear it
                self.current_css_iter = None;
            }
        }

        // DFS: Process next file from the stack
        while let Some(current_path) = self.stack.pop() {
            let data = self.module_graph.data();

            // Find ALL unvisited files that import current_path
            // We need to process all of them, not just the first one
            for (file_path, module_info) in data.iter() {
                if self.visited.contains(file_path) {
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
                    self.visited.insert(file_path.clone());

                    // Push this parent onto the stack for further upward traversal
                    self.stack.push(file_path.clone());

                    // Collect CSS files imported by this parent
                    let css_steps: Vec<_> = match module_info {
                        ModuleInfo::Js(js_info) => js_info
                            .static_import_paths
                            .values()
                            .filter_map(|import_path| {
                                let path = import_path.as_path()?;
                                let css_info = self.module_graph.css_module_info_for_path(path)?;

                                Some(CssClassStep {
                                    css_path: path.to_path_buf(),
                                    css_classes: css_info.classes.clone(),
                                })
                            })
                            .collect(),
                        ModuleInfo::Html(html_info) => html_info
                            .imported_stylesheets
                            .iter()
                            .chain(html_info.static_import_paths.values())
                            .chain(html_info.dynamic_import_paths.values())
                            .filter_map(|stylesheet_path| {
                                let path = stylesheet_path.as_path()?;
                                let css_info = self.module_graph.css_module_info_for_path(path)?;

                                Some(CssClassStep {
                                    css_path: path.to_path_buf(),
                                    css_classes: css_info.classes.clone(),
                                })
                            })
                            .collect(),
                        ModuleInfo::Css(_) => Vec::new(),
                    };

                    if !css_steps.is_empty() {
                        // Create a real iterator from the vector
                        self.current_css_iter = Some(css_steps.into_iter());
                        // Recursively call next() to yield the first CSS file
                        return self.next();
                    }

                    // Continue checking other files that might also import current_path
                    // Don't break - we need to find ALL importers
                }
            }
        }

        None
    }
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
        use biome_console::markup;

        let root_node = RootTreeNode {
            node: self.node,
            working_directory: self.working_directory,
        };

        fmt.write_markup(markup! {{root_node}})
    }
}

/// Display wrapper for relative paths
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

/// Root level tree node (displayed with "(this file)" suffix)
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

        // Show imports inline if present
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

/// "imported by:" group
struct ImportedByGroup<'a> {
    parents: &'a [ImportTreeNode],
    working_directory: Option<&'a Utf8Path>,
    is_last: bool,
}

impl<'a> biome_console::fmt::Display for ImportedByGroup<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        use biome_console::markup;

        let branch = if self.is_last { "└─ " } else { "├─ " };
        fmt.write_markup(markup! {"  "{branch}"imported by:\n"})?;

        // Prefix for items: use pipe if this group is not the last sibling, spaces if it is
        let prefix = if self.is_last {
            "    ".to_string()
        } else {
            "  │ ".to_string()
        };

        // Sort parents by file path for deterministic output
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

/// A tree item (component file)
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

        // Write the file path
        fmt.write_markup(markup! {{self.prefix}{branch}"• "{display_path}})?;

        // Show imports inline if present
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

        // Show nested parent components recursively (imported by chain)
        if !self.node.parent_components.is_empty() {
            let child_prefix = format!("{}{}", self.prefix, if self.is_last { "  " } else { "│ " });
            fmt.write_markup(markup! {{child_prefix}"└─ imported by:\n"})?;

            let item_prefix = format!("{}  ", child_prefix);

            // Sort parents by file path for deterministic output
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
