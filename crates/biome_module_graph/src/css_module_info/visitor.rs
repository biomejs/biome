use crate::css_module_info::{CssImport, CssImports, CssModuleInfo};
use crate::module_graph::ModuleGraphFsProxy;
use biome_css_syntax::{
    AnyCssImportUrl, AnyCssRoot, CssClassSelector, CssPseudoClassFunctionSelector,
};
use biome_resolver::{ResolveOptions, ResolvedPath, resolve};
use biome_rowan::{AstNode, Text, TokenText, WalkEvent};
use camino::Utf8Path;
use indexmap::IndexSet;

pub const SUPPORTED_EXTENSIONS: &[&str] = &["css"];

pub(crate) struct CssModuleVisitor<'a> {
    root: AnyCssRoot,
    directory: &'a Utf8Path,
    fs_proxy: &'a ModuleGraphFsProxy<'a>,
}

impl<'a> CssModuleVisitor<'a> {
    pub(crate) fn new(
        root: AnyCssRoot,
        directory: &'a Utf8Path,
        fs_proxy: &'a ModuleGraphFsProxy,
    ) -> Self {
        Self {
            root,
            directory,
            fs_proxy,
        }
    }

    pub(crate) fn visit(self) -> CssModuleInfo {
        let mut imports = CssImports::default();
        let mut classes: IndexSet<TokenText> = IndexSet::default();
        // Tracks nesting depth inside `:global(...)` pseudo-class selectors.
        // Class selectors inside `:global()` are globally scoped and cannot be
        // statically traced to specific `class="..."` references, so we skip them.
        let mut global_depth: u32 = 0;

        let iter = self.root.syntax().preorder();
        for event in iter {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(node) = AnyCssImportUrl::cast(node.clone()) {
                        self.visit_any_css_import_url(node, &mut imports);
                    } else if let Some(pseudo_fn) =
                        CssPseudoClassFunctionSelector::cast(node.clone())
                    {
                        if pseudo_fn.is_global_pseudo() {
                            global_depth += 1;
                        }
                    } else if global_depth == 0
                        && let Some(class_selector) = CssClassSelector::cast(node)
                    {
                        Self::visit_class_selector(class_selector, &mut classes);
                    }
                }
                WalkEvent::Leave(node) => {
                    if let Some(pseudo_fn) = CssPseudoClassFunctionSelector::cast(node)
                        && pseudo_fn.is_global_pseudo()
                    {
                        global_depth = global_depth.saturating_sub(1);
                    }
                }
            }
        }

        CssModuleInfo::new(imports, classes)
    }

    /// Extracts the class name from a `CssClassSelector` and inserts the
    /// `TokenText` into the set.
    ///
    /// Each token represents a single class name (e.g., "header" from `.header`).
    fn visit_class_selector(node: CssClassSelector, classes: &mut IndexSet<TokenText>) {
        if let Ok(name) = node.name()
            && let Some(name) = name.as_css_custom_identifier()
            && let Ok(token) = name.value_token()
        {
            classes.insert(token.token_text_trimmed());
        }
    }

    fn visit_any_css_import_url(&self, node: AnyCssImportUrl, imports: &mut CssImports) {
        let Some(specifier) = node.inner_string_text() else {
            return;
        };

        let resolved_path = self.resolved_path_from_specifier(&specifier);

        let text: Text = specifier.into();
        imports.insert(
            text.clone(),
            CssImport {
                specifier: text,
                resolved_path,
            },
        );
    }

    fn resolved_path_from_specifier(&self, specifier: &str) -> ResolvedPath {
        let options = ResolveOptions {
            assume_relative: true,
            condition_names: &[],
            default_files: &[],
            extensions: SUPPORTED_EXTENSIONS,
            extension_aliases: &[],
            ..Default::default()
        };
        let resolved_path = resolve(specifier, self.directory, self.fs_proxy, &options);
        ResolvedPath::new(resolved_path)
    }
}
