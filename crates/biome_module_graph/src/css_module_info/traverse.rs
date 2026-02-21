use crate::ModuleInfo;
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

            // Find the first unvisited file that imports current_path
            for (file_path, module_info) in data.iter() {
                if self.visited.contains(file_path) {
                    continue;
                }

                let imports_current = match module_info {
                    crate::ModuleInfo::Js(js_info) => js_info
                        .static_import_paths
                        .values()
                        .chain(js_info.dynamic_import_paths.values())
                        .any(|p| p.as_path() == Some(current_path.as_path())),
                    crate::ModuleInfo::Html(html_info) => html_info
                        .imported_stylesheets
                        .iter()
                        .any(|p| p.as_path() == Some(current_path.as_path())),
                    crate::ModuleInfo::Css(_) => false,
                };

                if imports_current {
                    self.visited.insert(file_path.clone());

                    // Push this parent onto the stack for further upward traversal
                    self.stack.push(file_path.clone());

                    // Collect CSS files imported by this parent
                    let css_steps: Vec<_> = match module_info {
                        crate::ModuleInfo::Js(js_info) => js_info
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

                    // If no CSS files, continue with the next iteration
                    // The parent was pushed to stack, so we'll find its parents next
                    break;
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
        use biome_console::markup;

        let display_path = RelativePath {
            path: &self.node.file_path,
            working_directory: self.working_directory,
        };
        fmt.write_markup(markup! {{display_path}" (this file)\n"})?;

        let has_parents = !self.node.parent_components.is_empty();
        let has_imports = !self.node.css_imports.is_empty();

        if has_parents {
            let group = ImportedByGroup {
                parents: &self.node.parent_components,
                working_directory: self.working_directory,
                is_last: !has_imports,
            };
            fmt.write_markup(markup! {{group}})?;
        }

        if has_imports {
            let group = ImportsGroup {
                imports: &self.node.css_imports,
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

        let prefix = if self.is_last { "    " } else { "  │ " };

        for (i, parent) in self.parents.iter().enumerate() {
            let is_last_parent = i == self.parents.len() - 1;
            let item = TreeItem {
                node: parent,
                working_directory: self.working_directory,
                prefix,
                is_last: is_last_parent,
            };
            fmt.write_markup(markup! {{item}})?;
        }

        Ok(())
    }
}

/// "imports:" group
struct ImportsGroup<'a> {
    imports: &'a [Utf8PathBuf],
    working_directory: Option<&'a Utf8Path>,
    is_last: bool,
}

impl<'a> biome_console::fmt::Display for ImportsGroup<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        use biome_console::markup;

        let branch = if self.is_last { "└─ " } else { "├─ " };
        fmt.write_markup(markup! {"  "{branch}"imports:\n"})?;

        let prefix = if self.is_last { "    " } else { "  │ " };

        for (i, css_path) in self.imports.iter().enumerate() {
            let is_last_css = i == self.imports.len() - 1;
            let branch = if is_last_css { "└─ " } else { "├─ " };
            let css_display = RelativePath {
                path: css_path,
                working_directory: self.working_directory,
            };
            fmt.write_markup(markup! {{prefix}{branch}"• "{css_display}"\n"})?;
        }

        Ok(())
    }
}

/// A tree item (component file)
struct TreeItem<'a> {
    node: &'a ImportTreeNode,
    working_directory: Option<&'a Utf8Path>,
    prefix: &'a str,
    is_last: bool,
}

impl<'a> biome_console::fmt::Display for TreeItem<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        use biome_console::markup;

        let branch = if self.is_last { "└─ " } else { "├─ " };
        let display_path = RelativePath {
            path: &self.node.file_path,
            working_directory: self.working_directory,
        };
        fmt.write_markup(markup! {{self.prefix}{branch}"• "{display_path}"\n"})?;

        // Calculate child prefix for nested content
        let child_prefix = Prefix {
            parent: self.prefix,
            is_last: self.is_last,
        };

        // Show nested parent components recursively (imported by chain)
        if !self.node.parent_components.is_empty() {
            let nested_parents = NestedImportedByGroup {
                parents: &self.node.parent_components,
                working_directory: self.working_directory,
                parent_prefix: self.prefix,
                parent_is_last: self.is_last,
                is_last: self.node.css_imports.is_empty(),
            };
            fmt.write_markup(markup! {{nested_parents}})?;
        }

        // Show nested imports for this component
        if !self.node.css_imports.is_empty() {
            let nested_imports = NestedImportsGroup {
                imports: &self.node.css_imports,
                working_directory: self.working_directory,
                parent_prefix: self.prefix,
                parent_is_last: self.is_last,
            };
            fmt.write_markup(markup! {{nested_imports}})?;
        }

        Ok(())
    }
}

/// Nested "imported by:" group (shown under a tree item)
struct NestedImportedByGroup<'a> {
    parents: &'a [ImportTreeNode],
    working_directory: Option<&'a Utf8Path>,
    parent_prefix: &'a str,
    parent_is_last: bool,
    is_last: bool,
}

impl<'a> biome_console::fmt::Display for NestedImportedByGroup<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        use biome_console::markup;

        let child_prefix = Prefix {
            parent: self.parent_prefix,
            is_last: self.parent_is_last,
        };

        let branch = if self.is_last { "└─ " } else { "├─ " };
        fmt.write_markup(markup! {{child_prefix}{branch}"imported by:\n"})?;

        let item_prefix = NestedItemPrefix {
            parent: self.parent_prefix,
            parent_is_last: self.parent_is_last,
            group_is_last: self.is_last,
        };

        for (i, parent) in self.parents.iter().enumerate() {
            let is_last_parent = i == self.parents.len() - 1;
            let item = NestedTreeItem {
                node: parent,
                working_directory: self.working_directory,
                prefix: &item_prefix,
                is_last: is_last_parent,
            };
            fmt.write_markup(markup! {{item}})?;
        }

        Ok(())
    }
}

/// Nested imports group (shown under a tree item)
struct NestedImportsGroup<'a> {
    imports: &'a [Utf8PathBuf],
    working_directory: Option<&'a Utf8Path>,
    parent_prefix: &'a str,
    parent_is_last: bool,
}

impl<'a> biome_console::fmt::Display for NestedImportsGroup<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        use biome_console::markup;

        let child_prefix = Prefix {
            parent: self.parent_prefix,
            is_last: self.parent_is_last,
        };

        fmt.write_markup(markup! {{child_prefix}"└─ imports:\n"})?;

        let item_prefix = ItemPrefix {
            parent: self.parent_prefix,
            parent_is_last: self.parent_is_last,
        };

        for (i, css_path) in self.imports.iter().enumerate() {
            let is_last_css = i == self.imports.len() - 1;
            let item_branch = if is_last_css { "└─ " } else { "├─ " };
            let css_display = RelativePath {
                path: css_path,
                working_directory: self.working_directory,
            };
            fmt.write_markup(markup! {{item_prefix}{item_branch}"• "{css_display}"\n"})?;
        }

        Ok(())
    }
}

/// Display wrapper for tree prefixes
struct Prefix<'a> {
    parent: &'a str,
    is_last: bool,
}

impl<'a> biome_console::fmt::Display for Prefix<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        fmt.write_str(self.parent)?;
        fmt.write_str(if self.is_last { "  " } else { "│ " })
    }
}

/// Display wrapper for item prefixes
struct ItemPrefix<'a> {
    parent: &'a str,
    parent_is_last: bool,
}

impl<'a> biome_console::fmt::Display for ItemPrefix<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        fmt.write_str(self.parent)?;
        fmt.write_str(if self.parent_is_last {
            "    "
        } else {
            "│   "
        })
    }
}

/// Nested item prefix (for items under nested groups)
struct NestedItemPrefix<'a> {
    parent: &'a str,
    parent_is_last: bool,
    group_is_last: bool,
}

impl<'a> biome_console::fmt::Display for NestedItemPrefix<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        fmt.write_str(self.parent)?;
        fmt.write_str(if self.parent_is_last { "  " } else { "│ " })?;
        fmt.write_str(if self.group_is_last { "  " } else { "│ " })
    }
}

/// Nested tree item (shown under nested groups)
struct NestedTreeItem<'a> {
    node: &'a ImportTreeNode,
    working_directory: Option<&'a Utf8Path>,
    prefix: &'a NestedItemPrefix<'a>,
    is_last: bool,
}

impl<'a> biome_console::fmt::Display for NestedTreeItem<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        use biome_console::markup;

        let branch = if self.is_last { "└─ " } else { "├─ " };
        let display_path = RelativePath {
            path: &self.node.file_path,
            working_directory: self.working_directory,
        };
        fmt.write_markup(markup! {{self.prefix}{branch}"• "{display_path}"\n"})?;

        // Recursively show this item's parents and imports
        if !self.node.parent_components.is_empty() || !self.node.css_imports.is_empty() {
            let deeper_nested = DeeperNestedContent {
                node: self.node,
                working_directory: self.working_directory,
                base_prefix: self.prefix,
                item_is_last: self.is_last,
            };
            fmt.write_markup(markup! {{deeper_nested}})?;
        }

        Ok(())
    }
}

/// Content nested even deeper (for recursive parent/import chains)
struct DeeperNestedContent<'a> {
    node: &'a ImportTreeNode,
    working_directory: Option<&'a Utf8Path>,
    base_prefix: &'a NestedItemPrefix<'a>,
    item_is_last: bool,
}

impl<'a> biome_console::fmt::Display for DeeperNestedContent<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        use biome_console::markup;

        // Build prefix for the deeper level
        let deeper_prefix = DeeperPrefix {
            base: self.base_prefix,
            item_is_last: self.item_is_last,
        };

        let has_parents = !self.node.parent_components.is_empty();
        let has_imports = !self.node.css_imports.is_empty();

        // Show imported by chain
        if has_parents {
            let branch = if has_imports { "├─ " } else { "└─ " };
            fmt.write_markup(markup! {{deeper_prefix}{branch}"imported by:\n"})?;

            let item_list_prefix = DeeperItemListPrefix {
                base: self.base_prefix,
                item_is_last: self.item_is_last,
                group_is_last: !has_imports,
            };

            for (i, parent) in self.node.parent_components.iter().enumerate() {
                let is_last_parent = i == self.node.parent_components.len() - 1;
                let item = NestedTreeItem {
                    node: parent,
                    working_directory: self.working_directory,
                    prefix: &item_list_prefix,
                    is_last: is_last_parent,
                };
                fmt.write_markup(markup! {{item}})?;
            }
        }

        // Show imports
        if has_imports {
            fmt.write_markup(markup! {{deeper_prefix}"└─ imports:\n"})?;

            let item_list_prefix = DeeperItemListPrefix {
                base: self.base_prefix,
                item_is_last: self.item_is_last,
                group_is_last: true,
            };

            for (i, css_path) in self.node.css_imports.iter().enumerate() {
                let is_last_css = i == self.node.css_imports.len() - 1;
                let css_branch = if is_last_css { "└─ " } else { "├─ " };
                let css_display = RelativePath {
                    path: css_path,
                    working_directory: self.working_directory,
                };
                fmt.write_markup(markup! {{item_list_prefix}{css_branch}"• "{css_display}"\n"})?;
            }
        }

        Ok(())
    }
}

/// Prefix for deeper nested levels
struct DeeperPrefix<'a> {
    base: &'a NestedItemPrefix<'a>,
    item_is_last: bool,
}

impl<'a> biome_console::fmt::Display for DeeperPrefix<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        use biome_console::markup;
        fmt.write_markup(markup! {{self.base}})?;
        fmt.write_str(if self.item_is_last { "  " } else { "│ " })
    }
}

/// Prefix for item lists in deeper levels
struct DeeperItemListPrefix<'a> {
    base: &'a NestedItemPrefix<'a>,
    item_is_last: bool,
    group_is_last: bool,
}

impl<'a> biome_console::fmt::Display for DeeperItemListPrefix<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        use biome_console::markup;
        fmt.write_markup(markup! {{self.base}})?;
        fmt.write_str(if self.item_is_last { "  " } else { "│ " })?;
        fmt.write_str(if self.group_is_last { "  " } else { "│ " })
    }
}

impl<'a> biome_console::fmt::Display for ItemPrefix<'a> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        fmt.write_str(self.parent)?;
        fmt.write_str(if self.parent_is_last {
            "    "
        } else {
            "│   "
        })
    }
}
