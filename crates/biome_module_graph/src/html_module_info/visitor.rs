use crate::css_module_info::{CssClass, collect_class_tokens, is_global_pseudo};
use crate::html_module_info::HtmlModuleInfo;
use crate::module_graph::ModuleGraphFsProxy;
use biome_css_syntax::{AnyCssRoot, CssClassSelector, CssPseudoClassFunctionSelector};
use biome_html_syntax::{AnyHtmlAttribute, AnyHtmlAttributeInitializer, HtmlElement, HtmlRoot};
use biome_resolver::{ResolveOptions, ResolvedPath, resolve};
use biome_rowan::{AstNode, TextRange, TextSize, TokenText, WalkEvent};
use camino::Utf8Path;
use indexmap::IndexSet;

pub const SUPPORTED_CSS_EXTENSIONS: &[&str] = &["css"];

pub(crate) struct HtmlModuleVisitor<'a> {
    html_root: HtmlRoot,
    embedded_css_roots: &'a [AnyCssRoot],
    directory: &'a Utf8Path,
    fs_proxy: &'a ModuleGraphFsProxy<'a>,
}

impl<'a> HtmlModuleVisitor<'a> {
    pub(crate) fn new(
        html_root: HtmlRoot,
        embedded_css_roots: &'a [AnyCssRoot],
        directory: &'a Utf8Path,
        fs_proxy: &'a ModuleGraphFsProxy<'a>,
    ) -> Self {
        Self {
            html_root,
            embedded_css_roots,
            directory,
            fs_proxy,
        }
    }

    pub(crate) fn visit(self) -> HtmlModuleInfo {
        let mut style_classes: IndexSet<CssClass> = IndexSet::default();
        let mut referenced_classes: IndexSet<CssClass> = IndexSet::default();
        let mut imported_stylesheets: Vec<ResolvedPath> = Vec::new();

        // Collect CSS class names from already-parsed <style> block ASTs.
        // These are passed in from the workspace server — no re-parsing needed.
        for css_root in self.embedded_css_roots {
            collect_css_classes(css_root, &mut style_classes);
        }

        // Walk the HTML CST to collect class= references and <link> stylesheets.
        for event in self.html_root.syntax().preorder() {
            if let WalkEvent::Enter(node) = event
                && let Some(element) = HtmlElement::cast(node)
            {
                self.visit_html_element(
                    element,
                    &mut referenced_classes,
                    &mut imported_stylesheets,
                );
            }
        }

        HtmlModuleInfo::new(style_classes, referenced_classes, imported_stylesheets)
    }

    fn visit_html_element(
        &self,
        element: HtmlElement,
        referenced_classes: &mut IndexSet<CssClass>,
        imported_stylesheets: &mut Vec<ResolvedPath>,
    ) {
        let Ok(opening) = element.opening_element() else {
            return;
        };

        for attr in opening.attributes() {
            let AnyHtmlAttribute::HtmlAttribute(attr) = attr else {
                continue;
            };

            let Ok(name) = attr.name() else {
                continue;
            };

            let Ok(name_token) = name.value_token() else {
                continue;
            };

            let name_text = name_token.text_trimmed();

            if name_text.eq_ignore_ascii_case("class") {
                // Extract individual class tokens from class="foo bar baz", each
                // with its precise byte range within the source file.
                if let Some(initializer) = attr.initializer()
                    && let Ok(value_node) = initializer.value()
                {
                    collect_class_attribute_tokens(&value_node, referenced_classes);
                }
            } else if name_text.eq_ignore_ascii_case("href") {
                // Only care about href on <link rel="stylesheet"> elements
                let is_link_tag = element
                    .tag_name()
                    .is_some_and(|t| t.text().eq_ignore_ascii_case("link"));
                if is_link_tag
                    && element
                        .find_attribute_by_name("rel")
                        .and_then(|rel_attr| rel_attr.value())
                        .is_some_and(|rel_val| rel_val.text().eq_ignore_ascii_case("stylesheet"))
                    && let Some(href_value) = attr.value()
                {
                    let resolved = self.resolved_path_from_specifier(href_value.text());
                    imported_stylesheets.push(resolved);
                }
            }
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
}

/// Collects all CSS class names from a CSS AST, including those inside nested
/// rules and at-rules. Skips class selectors inside `:global(...)` pseudo-class
/// selectors, which are globally scoped and cannot be traced to specific
/// `class="..."` attribute references.
///
/// Each class name occupies the whole token, so the token-relative range runs
/// from `0` to the token's text length.
pub(crate) fn collect_css_classes(css_root: &AnyCssRoot, classes: &mut IndexSet<CssClass>) {
    let mut global_depth: u32 = 0;

    for event in css_root.syntax().preorder() {
        match event {
            WalkEvent::Enter(node) => {
                if let Some(pseudo_fn) = CssPseudoClassFunctionSelector::cast(node.clone()) {
                    if is_global_pseudo(&pseudo_fn) {
                        global_depth += 1;
                    }
                } else if global_depth == 0
                    && let Some(class_selector) = CssClassSelector::cast(node)
                    && let Ok(name) = class_selector.name()
                    && let Ok(token) = name.value_token()
                {
                    let token_text = token.token_text_trimmed();
                    let len = u32::from(token_text.len());
                    classes.insert(CssClass {
                        token: token_text,
                        range: TextRange::new(TextSize::from(0), TextSize::from(len)),
                    });
                }
            }
            WalkEvent::Leave(node) => {
                if let Some(pseudo_fn) = CssPseudoClassFunctionSelector::cast(node)
                    && is_global_pseudo(&pseudo_fn)
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

/// Splits a `class="foo bar baz"` attribute value into individual [`CssClass`]
/// entries.
///
/// Each entry holds the quote-stripped inner token text and a byte range
/// relative to the start of that text. Applying the range to `token.text()`
/// gives the class name as a `&str` with no allocation.
fn collect_class_attribute_tokens(
    value_node: &AnyHtmlAttributeInitializer,
    classes: &mut IndexSet<CssClass>,
) {
    if let Some(inner) = extract_html_class_attribute_inner(value_node) {
        collect_class_tokens(&inner, classes);
    }
}
