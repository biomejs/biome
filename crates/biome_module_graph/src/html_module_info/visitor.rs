use crate::css_module_info::{CssClassDefinition, CssClassReference};
use crate::html_module_info::{HtmlEmbeddedContent, HtmlModuleInfo};
use crate::module_graph::ModuleGraphFsProxy;
use biome_css_syntax::{
    AnyCssRoot, CssClassSelector, CssFileSource, CssPseudoClassFunctionSelector,
    EmbeddingStyleApplicability,
};
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, HtmlElement, HtmlRoot, HtmlSelfClosingElement,
};
use biome_js_syntax::{AnyJsImportLike, AnyJsRoot};
use biome_resolver::{ResolveOptions, ResolvedPath, resolve};
use biome_rowan::{AstNode, AstSeparatedList, Text, TokenText, WalkEvent};
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::{IndexMap, IndexSet};

pub const SUPPORTED_CSS_EXTENSIONS: &[&str] = &["css"];

/// Extension aliases to try when resolving HTML-like component imports.
/// Mirrors the JS visitor's EXTENSION_ALIASES but adds framework extensions.
const HTML_EXTENSION_ALIASES: &[(&str, &[&str])] = &[
    ("js", &["ts", "tsx", "d.ts", "js", "jsx"]),
    ("mjs", &["mts", "d.mts", "mjs"]),
    ("cjs", &["cts", "d.cts", "cjs"]),
];

const HTML_SUPPORTED_EXTENSION_ALIASES: &[&str] = &[
    "ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs", "json", "node",
    // HTML-like framework component extensions
    "vue", "astro", "svelte",
];

pub(crate) struct HtmlModuleVisitor<'a> {
    html_root: HtmlRoot,
    /// All embedded content blocks (CSS and JS) extracted from this HTML-like file.
    embedded_content: &'a [HtmlEmbeddedContent],
    file_path: Utf8PathBuf,
    directory: &'a Utf8Path,
    fs_proxy: &'a ModuleGraphFsProxy<'a>,
}

impl<'a> HtmlModuleVisitor<'a> {
    pub(crate) fn new(
        html_root: HtmlRoot,
        embedded_content: &'a [HtmlEmbeddedContent],
        file_path: Utf8PathBuf,
        directory: &'a Utf8Path,
        fs_proxy: &'a ModuleGraphFsProxy<'a>,
    ) -> Self {
        Self {
            html_root,
            embedded_content,
            file_path,
            directory,
            fs_proxy,
        }
    }

    pub(crate) fn visit(self) -> HtmlModuleInfo {
        let mut style_classes = IndexSet::default();
        let mut referenced_classes = Vec::new();
        let mut imported_stylesheets = Vec::new();
        let mut static_import_paths = IndexMap::default();
        let mut dynamic_import_paths = IndexMap::default();

        // Walk the HTML CST to collect class= references and <link> stylesheets.
        // Void elements like <link> and <meta> parse as HtmlSelfClosingElement;
        // normal elements parse as HtmlElement. Both must be handled.
        for event in self.html_root.syntax().preorder() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if let Some(element) = HtmlElement::cast(node.clone()) {
                self.visit_html_element(
                    element,
                    &mut referenced_classes,
                    &mut imported_stylesheets,
                );
            } else if let Some(element) = HtmlSelfClosingElement::cast(node) {
                self.visit_self_closing_element(
                    element,
                    &mut referenced_classes,
                    &mut imported_stylesheets,
                );
            }
        }

        // Dispatch each embedded content block to the appropriate collector.
        for content in self.embedded_content {
            match content {
                // CSS block: collect class definitions (with applicability scoping).
                HtmlEmbeddedContent::Css(css_root, file_source) => {
                    collect_css_classes(css_root, &mut style_classes, file_source);
                }
                // JS block: collect static import paths for upward traversal.
                HtmlEmbeddedContent::Js(js_root) => {
                    self.collect_js_imports(
                        js_root,
                        &mut static_import_paths,
                        &mut dynamic_import_paths,
                    );
                }
            }
        }

        HtmlModuleInfo::new(
            style_classes,
            referenced_classes,
            imported_stylesheets,
            static_import_paths,
            dynamic_import_paths,
        )
    }

    /// Walks a parsed JS/TS root (from an embedded `<script>` block) and
    /// collects all static import specifiers with their resolved paths.
    fn collect_js_imports(
        &self,
        js_root: &AnyJsRoot,
        static_import_paths: &mut IndexMap<Text, ResolvedPath>,
        dynamic_import_paths: &mut IndexMap<Text, ResolvedPath>,
    ) {
        for event in js_root.syntax().preorder() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            // Only handle static module sources (import … from "…").
            // Skip dynamic imports (import("…") / require("…")).
            if let Some(any_source) = AnyJsImportLike::cast_ref(&node) {
                match any_source {
                    AnyJsImportLike::JsModuleSource(source) => {
                        let Some(specifier) = source.inner_string_text().ok() else {
                            continue;
                        };
                        let resolved = self.resolved_js_path_from_specifier(specifier.text());
                        static_import_paths
                            .entry(Text::from(specifier))
                            .or_insert(resolved);
                    }
                    // require("") isn't actually supported in the environments we're interested in. For example require() shouldn't be
                    // supported in HTML-ish languages.
                    // So, it's ignored by design.
                    AnyJsImportLike::JsCallExpression(_) => {}
                    AnyJsImportLike::JsImportCallExpression(source) => {
                        let Some(arguments) = source.arguments().ok() else {
                            continue;
                        };
                        let Some(argument) = arguments
                            .args()
                            .iter()
                            .flatten()
                            .next()
                            .and_then(|argument| argument.as_any_js_expression().cloned())
                            .and_then(|expr| expr.as_any_js_literal_expression().cloned())
                            .and_then(|expr| expr.as_js_string_literal_expression().cloned())
                            .and_then(|str| str.inner_string_text().ok())
                        else {
                            continue;
                        };

                        let resolved = self.resolved_js_path_from_specifier(argument.text());
                        dynamic_import_paths
                            .entry(Text::from(argument))
                            .or_insert(resolved);
                    }
                }
            }
        }
    }

    fn visit_html_element(
        &self,
        element: HtmlElement,
        referenced_classes: &mut Vec<CssClassReference>,
        _imported_stylesheets: &mut Vec<ResolvedPath>,
    ) {
        let Ok(opening) = element.opening_element() else {
            return;
        };

        for attr in opening.attributes() {
            let Some(attr) = attr.as_html_attribute() else {
                continue;
            };

            let Some(name_token) = attr.name().ok().and_then(|name| name.value_token().ok()) else {
                continue;
            };

            let name_text = name_token.text_trimmed();

            if name_text.eq_ignore_ascii_case("class") {
                // Collect the class attribute reference
                if let Some(initializer) = attr.initializer()
                    && let Ok(value_node) = initializer.value()
                {
                    collect_class_attribute_reference(
                        &value_node,
                        &self.file_path,
                        referenced_classes,
                    );
                }
            }
        }
    }

    /// Handles void/self-closing elements.
    ///
    /// Collects `class="..."` references from any self-closing element (e.g.
    /// `<img class="hero" />`, `<input class="field" />`), and additionally
    /// handles `<link rel="stylesheet" href="...">` for stylesheet imports.
    fn visit_self_closing_element(
        &self,
        element: HtmlSelfClosingElement,
        referenced_classes: &mut Vec<CssClassReference>,
        imported_stylesheets: &mut Vec<ResolvedPath>,
    ) {
        // Collect class= references from all self-closing elements.
        for attr in element.attributes() {
            let Some(attr) = attr.as_html_attribute() else {
                continue;
            };
            let Some(name_token) = attr.name().ok().and_then(|n| n.value_token().ok()) else {
                continue;
            };
            if name_token.text_trimmed().eq_ignore_ascii_case("class")
                && let Some(initializer) = attr.initializer()
                && let Ok(value_node) = initializer.value()
            {
                collect_class_attribute_reference(&value_node, &self.file_path, referenced_classes);
            }
        }

        // Collect <link rel="stylesheet"> imports.
        let is_link_tag = element
            .tag_name()
            .is_some_and(|t| t.text().eq_ignore_ascii_case("link"));
        if !is_link_tag {
            return;
        }

        let is_stylesheet = element
            .find_attribute_by_name("rel")
            .and_then(|rel_attr| rel_attr.value())
            .is_some_and(|rel_val| rel_val.text().eq_ignore_ascii_case("stylesheet"));
        if !is_stylesheet {
            return;
        }

        if let Some(href_value) = element
            .find_attribute_by_name("href")
            .and_then(|href_attr| href_attr.value())
        {
            let resolved = self.resolved_path_from_specifier(href_value.text());
            imported_stylesheets.push(resolved);
        }
    }

    fn resolved_path_from_specifier(&self, specifier: &str) -> ResolvedPath {
        let options = ResolveOptions {
            assume_relative: true,
            condition_names: &[],
            default_files: &[],
            extensions: SUPPORTED_CSS_EXTENSIONS,
            extension_aliases: &[],
            ..Default::default()
        };
        let resolved = resolve(specifier, self.directory, self.fs_proxy, &options);
        ResolvedPath::new(resolved)
    }

    /// Resolves a JS/TS/framework module specifier from an embedded `<script>`.
    ///
    /// Uses the same resolution options as `JsModuleVisitor::resolved_path_from_specifier`,
    /// plus framework-specific extensions (`.vue`, `.astro`, `.svelte`).
    fn resolved_js_path_from_specifier(&self, specifier: &str) -> ResolvedPath {
        let options = ResolveOptions {
            condition_names: &["types", "import", "default"],
            default_files: &["index"],
            extensions: HTML_SUPPORTED_EXTENSION_ALIASES,
            extension_aliases: HTML_EXTENSION_ALIASES,
            resolve_node_builtins: true,
            resolve_types: true,
            ..Default::default()
        };
        let resolved = resolve(specifier, self.directory, self.fs_proxy, &options);
        ResolvedPath::new(resolved)
    }
}

/// Collects CSS class names from a CSS AST, annotating each with its
/// [`EmbeddingStyleApplicability`] based on the embedding context.
///
/// # Applicability rules
///
/// - Selectors inside `:global(...)` pseudo-class blocks are always
///   [`EmbeddingStyleApplicability::Global`], regardless of the file source.
/// - All other selectors take their applicability from
///   [`CssFileSource::embedding_applicability`]:
///   - Plain HTML `<style>` → `Global`
///   - Vue `<style>` (no `scoped`) → `Global`
///   - Vue `<style scoped>` → `Local`
///   - Astro `<style>` (default) → `Local`
///   - Astro `<style is:global>` → `Global`
///   - Svelte `<style>` (default) → `Local`
///
/// Each [`CssClassDefinition`] in the output represents a single class name
/// (e.g., `"header"` from `.header`) together with whether it is local or global.
pub(crate) fn collect_css_classes(
    css_root: &AnyCssRoot,
    classes: &mut IndexSet<CssClassDefinition>,
    file_source: &CssFileSource,
) {
    // Applicability for selectors *not* inside :global(...).
    // Selectors inside :global(...) are unconditionally Global.
    let base_applicability = file_source.embedding_applicability();
    let mut global_depth: u32 = 0;

    for event in css_root.syntax().preorder() {
        match event {
            WalkEvent::Enter(node) => {
                if let Some(pseudo_fn) = CssPseudoClassFunctionSelector::cast(node.clone()) {
                    if pseudo_fn.is_global_pseudo() {
                        global_depth += 1;
                    }
                } else if let Some(class_selector) = CssClassSelector::cast(node)
                    && let Ok(name) = class_selector.name()
                    && let Some(name) = name.as_css_custom_identifier()
                    && let Ok(token) = name.value_token()
                {
                    // Selectors inside :global(...) are always globally scoped,
                    // even within a locally scoped <style> block.
                    let applicability = if global_depth > 0 {
                        EmbeddingStyleApplicability::Global
                    } else {
                        base_applicability
                    };
                    classes.insert(CssClassDefinition {
                        name: token.token_text_trimmed(),
                        applicability,
                    });
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
}

/// Extracts the inner (quote-stripped) text from an HTML `class="..."`
/// attribute value, if it is a static string literal.
///
/// Returns `None` if the value is not an `HtmlString` or has malformed structure.
fn extract_html_class_attribute_inner(
    value_node: &AnyHtmlAttributeInitializer,
) -> Option<TokenText> {
    let AnyHtmlAttributeInitializer::HtmlString(html_string) = value_node else {
        return None;
    };
    html_string.inner_string_text().ok()
}

/// Creates a `CssClassReference` from an HTML `class="..."` attribute value.
///
/// The reference stores the full attribute value token (e.g., "foo bar baz"),
/// which may contain multiple space-separated class names.
fn collect_class_attribute_reference(
    value_node: &AnyHtmlAttributeInitializer,
    file_path: &Utf8Path,
    classes: &mut Vec<CssClassReference>,
) {
    if let Some(inner) = extract_html_class_attribute_inner(value_node) {
        classes.push(CssClassReference::new(inner, file_path.to_path_buf()));
    }
}
