use crate::css_module_info::{CssImport, CssImports, CssModuleInfo};
use biome_css_syntax::selector_ext::AnyCssPseudoClassFunctionSelector;
use biome_css_syntax::{AnyCssImportUrl, AnyCssRoot, CssClassSelector};
use biome_rowan::{AstNode, Text, TextRange, TokenText, WalkEvent};
use indexmap::IndexMap;

pub(crate) struct CssModuleVisitor {
    root: AnyCssRoot,
}

impl CssModuleVisitor {
    pub(crate) fn new(root: AnyCssRoot) -> Self {
        Self { root }
    }

    pub(crate) fn visit(mut self) -> CssModuleInfo {
        let mut imports = CssImports::default();
        let mut classes: IndexMap<TextRange, TokenText> = IndexMap::default();
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
                        AnyCssPseudoClassFunctionSelector::cast(node.clone())
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
                    if let Some(pseudo_fn) = AnyCssPseudoClassFunctionSelector::cast(node)
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
    fn visit_class_selector(node: CssClassSelector, classes: &mut IndexMap<TextRange, TokenText>) {
        if let Ok(name) = node.name()
            && let Some(name) = name.as_css_custom_identifier()
            && let Ok(token) = name.value_token()
        {
            // Store the range of the class name token (without the dot)
            classes
                .entry(token.text_trimmed_range())
                .or_insert_with(|| token.token_text_trimmed());
        }
    }

    fn visit_any_css_import_url(&mut self, node: AnyCssImportUrl, imports: &mut CssImports) {
        let Some(specifier) = node.inner_string_text() else {
            return;
        };

        let text: Text = specifier.into();
        imports.push(
            text.clone(),
            CssImport {
                range: node.range(),
                specifier: text,
            },
        );
    }
}
