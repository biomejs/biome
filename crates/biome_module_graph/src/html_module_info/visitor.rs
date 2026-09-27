use crate::ImportPathMap;
use crate::css_module_info::{CssClassDefinition, CssClassReference, CssModuleVisitor};
use crate::html_module_info::{AstroStyleInfo, AstroStyleVariable, HtmlImport, HtmlModuleInfo};
use crate::module_graph::ModuleGraphFsProxy;
use biome_css_syntax::selector_ext::AnyCssPseudoClassFunctionSelector;
use biome_css_syntax::{AnyCssRoot, CssClassSelector, CssDashedIdentifier, CssFunction};
use biome_db::ParsedSource;
use biome_html_syntax::{
    AnyAstroDirective, AnyHtmlAttribute, AnyHtmlAttributeInitializer, HtmlAttributeList,
    HtmlElement, HtmlRoot, HtmlSelfClosingElement, T, element_ext::AnyHtmlTagElement,
};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsImportLike, AnyJsLiteralExpression, AnyJsObjectMember,
    AnyJsObjectMemberName, AnyJsRoot, AnyJsTemplateElement, JsLogicalOperator,
};
use biome_languages::css::EmbeddingStyleApplicability;
use biome_languages::{CssFileSource, JsFileSource, LanguageDb};
use biome_resolver::{ResolveOptions, ResolvedPath, resolve};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, Text, TextRange, TextSize, TokenText, WalkEvent,
};
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexSet;

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
    db: &'a dyn LanguageDb,
    parsed_source: ParsedSource,
    file_path: Utf8PathBuf,
    directory: &'a Utf8Path,
    fs_proxy: &'a ModuleGraphFsProxy<'a>,
}

impl<'a> HtmlModuleVisitor<'a> {
    pub(crate) fn new(
        db: &'a dyn LanguageDb,
        parsed_source: ParsedSource,
        file_path: Utf8PathBuf,
        directory: &'a Utf8Path,
        fs_proxy: &'a ModuleGraphFsProxy<'a>,
    ) -> Self {
        Self {
            db,
            parsed_source,
            file_path,
            directory,
            fs_proxy,
        }
    }

    pub(crate) fn visit(self) -> HtmlModuleInfo {
        let html_root = self.parsed_source.parsed(self.db).tree::<HtmlRoot>();
        let mut style_classes = IndexSet::default();
        let mut referenced_classes = Vec::new();
        let mut imported_stylesheets = Vec::new();
        let mut import_paths = ImportPathMap::default();
        let mut astro_class_references = IndexSet::new();
        let mut has_unknown_astro_class_reference = false;
        let mut astro_styles = Vec::new();

        // Walk the HTML CST to collect class= references and <link> stylesheets.
        // Void elements like <link> and <meta> parse as HtmlSelfClosingElement;
        // normal elements parse as HtmlElement. Both must be handled.
        for event in html_root.syntax().preorder() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if let Some(element) = HtmlElement::cast(node.clone()) {
                self.visit_html_element(
                    element,
                    &mut referenced_classes,
                    &mut astro_class_references,
                    &mut has_unknown_astro_class_reference,
                    &mut astro_styles,
                );
            } else if let Some(element) = HtmlSelfClosingElement::cast(node) {
                self.visit_self_closing_element(
                    element,
                    &mut referenced_classes,
                    &mut imported_stylesheets,
                    &mut astro_class_references,
                    &mut has_unknown_astro_class_reference,
                );
            }
        }

        for snippet in self.parsed_source.snippets(self.db) {
            let Some(file_source) = self
                .db
                .source_from_index(snippet.document_source_index(self.db))
            else {
                continue;
            };
            let content_offset = snippet.content_offset(self.db);
            if let Some(file_source) = file_source.to_css_file_source() {
                let parsed = snippet.parsed(self.db);
                let has_errors = parsed.has_errors();
                let css_root = parsed.tree::<AnyCssRoot>();
                collect_css_classes(&css_root, &mut style_classes, &file_source, content_offset);
                let css_info =
                    CssModuleVisitor::new(css_root.clone(), self.directory, self.fs_proxy).visit();
                imported_stylesheets.extend(css_info.imports.iter().map(|import| HtmlImport {
                    range: import.range + content_offset,
                    resolved_path: import.resolved_path.clone(),
                    applicability: file_source.embedding_applicability(),
                }));
                collect_astro_style_references(
                    &css_root,
                    &file_source,
                    has_errors,
                    content_offset,
                    &mut astro_styles,
                );
            } else if let Some(file_source) = file_source.to_js_file_source() {
                let parsed = snippet.parsed(self.db);
                let has_errors = parsed.has_errors();
                let js_root = parsed.tree::<AnyJsRoot>();
                self.collect_js_imports(&js_root, content_offset, &mut import_paths);
                if file_source.as_embedding_kind().is_class_attribute() {
                    let Some(root) = js_root.as_js_expression_template_root() else {
                        has_unknown_astro_class_reference = true;
                        continue;
                    };
                    let Some(expression) = root.expression() else {
                        has_unknown_astro_class_reference = true;
                        continue;
                    };
                    if !collect_astro_class_expression(
                        &expression,
                        file_source.as_embedding_kind().is_class_list_attribute(),
                        &mut astro_class_references,
                    ) {
                        has_unknown_astro_class_reference = true;
                    }
                }
                collect_astro_style_definitions(
                    &js_root,
                    &file_source,
                    has_errors,
                    content_offset,
                    &mut astro_styles,
                );
            }
        }
        astro_styles.retain(|style| style.has_supported_css);

        HtmlModuleInfo::new(
            style_classes,
            referenced_classes,
            imported_stylesheets,
            import_paths,
            astro_class_references,
            has_unknown_astro_class_reference,
            astro_styles,
        )
    }

    /// Walks a parsed JS/TS root (from an embedded `<script>` block) and
    /// collects all static and dynamic import specifiers with their resolved paths.
    fn collect_js_imports(
        &self,
        js_root: &AnyJsRoot,
        content_offset: TextSize,
        import_paths: &mut ImportPathMap<HtmlImport>,
    ) {
        for event in js_root.syntax().preorder() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if let Some(any_source) = AnyJsImportLike::cast_ref(&node) {
                match any_source {
                    AnyJsImportLike::JsModuleSource(source) => {
                        if source.imports_only_types() {
                            continue;
                        }
                        let Some(specifier) = source.inner_string_text().ok() else {
                            continue;
                        };
                        let resolved = self.resolved_js_path_from_specifier(specifier.text());
                        import_paths.insert(
                            Text::from(specifier),
                            HtmlImport {
                                range: source.range() + content_offset,
                                resolved_path: resolved,
                                applicability: EmbeddingStyleApplicability::Global,
                            },
                        );
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
                        import_paths.insert(
                            Text::from(argument),
                            HtmlImport {
                                range: source.range() + content_offset,
                                resolved_path: resolved,
                                applicability: EmbeddingStyleApplicability::Global,
                            },
                        );
                    }
                }
            }
        }
    }

    fn visit_html_element(
        &self,
        element: HtmlElement,
        referenced_classes: &mut Vec<CssClassReference>,
        astro_class_references: &mut IndexSet<Text>,
        has_unknown_astro_class_reference: &mut bool,
        astro_styles: &mut Vec<AstroStyleInfo>,
    ) {
        let Ok(opening) = element.opening_element() else {
            return;
        };

        collect_astro_html_classes(
            &opening.attributes(),
            &self.file_path,
            referenced_classes,
            astro_class_references,
            has_unknown_astro_class_reference,
        );

        if AnyHtmlTagElement::from(opening.clone()).tag_name_kind() == Some(T![style]) {
            let style_range = element.range();
            if let Some(style) = opening.attributes().iter().find_map(|attribute| {
                let AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroDefineDirective(
                    directive,
                )) = attribute
                else {
                    return None;
                };
                let value = directive.value().ok()?;
                if value.name().ok()?.value_token().ok()?.text_trimmed() != "vars" {
                    return None;
                }
                Some(AstroStyleInfo {
                    style_range,
                    define_vars_range: value.initializer()?.value().ok()?.range(),
                    definitions: Vec::new(),
                    references: IndexSet::new(),
                    has_supported_css: false,
                })
            }) {
                astro_styles.push(style);
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
        imported_stylesheets: &mut Vec<HtmlImport>,
        astro_class_references: &mut IndexSet<Text>,
        has_unknown_astro_class_reference: &mut bool,
    ) {
        collect_astro_html_classes(
            &element.attributes(),
            &self.file_path,
            referenced_classes,
            astro_class_references,
            has_unknown_astro_class_reference,
        );

        // Collect <link rel="stylesheet"> imports.
        let is_link_tag = element
            .tag_name()
            .is_some_and(|t| t.text().eq_ignore_ascii_case("link"));
        if !is_link_tag {
            return;
        }

        let is_stylesheet = element
            .find_attribute_by_name("rel")
            .and_then(|rel_attr| rel_attr.as_static_value())
            .is_some_and(|rel_val| rel_val.text().eq_ignore_ascii_case("stylesheet"));
        if !is_stylesheet {
            return;
        }

        if let Some(href_value) = element
            .find_attribute_by_name("href")
            .and_then(|href_attr| href_attr.as_static_value())
        {
            let resolved = self.resolved_path_from_specifier(href_value.text());
            imported_stylesheets.push(HtmlImport {
                range: element.range(),
                resolved_path: resolved,
                applicability: EmbeddingStyleApplicability::Global,
            });
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
            resolve_bun_builtins: true,
            resolve_types: true,
            ..Default::default()
        };
        let resolved = resolve(specifier, self.directory, self.fs_proxy, &options);
        ResolvedPath::new(resolved)
    }
}

fn collect_astro_html_classes(
    attributes: &HtmlAttributeList,
    file_path: &Utf8Path,
    referenced_classes: &mut Vec<CssClassReference>,
    astro_class_references: &mut IndexSet<Text>,
    has_unknown_astro_class_reference: &mut bool,
) {
    for attribute in attributes {
        match attribute {
            AnyHtmlAttribute::HtmlAttribute(attribute) => {
                let Some(name) = attribute
                    .name()
                    .ok()
                    .and_then(|name| name.value_token().ok())
                else {
                    continue;
                };
                if !matches!(name.text_trimmed(), "class" | "className") {
                    continue;
                }
                let Some(initializer) = attribute.initializer() else {
                    continue;
                };
                let Ok(value) = initializer.value() else {
                    *has_unknown_astro_class_reference = true;
                    continue;
                };
                collect_class_attribute_reference(&value, file_path, referenced_classes);
                match value {
                    AnyHtmlAttributeInitializer::HtmlString(string) => {
                        let Some(value) = string.inner_string_text().ok() else {
                            *has_unknown_astro_class_reference = true;
                            continue;
                        };
                        if value.text().contains('&') {
                            *has_unknown_astro_class_reference = true;
                        } else {
                            collect_class_token_text(value, astro_class_references);
                        }
                    }
                    AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(_) => {}
                    _ => *has_unknown_astro_class_reference = true,
                }
            }
            AnyHtmlAttribute::AnyAstroDirective(directive) => match directive {
                AnyAstroDirective::AstroClassDirective(directive) => {
                    let Ok(value) = directive.value() else {
                        *has_unknown_astro_class_reference = true;
                        continue;
                    };
                    if value
                        .name()
                        .ok()
                        .and_then(|name| name.value_token().ok())
                        .is_none_or(|name| name.text_trimmed() != "list")
                    {
                        continue;
                    }
                    let Some(initializer) = value.initializer() else {
                        *has_unknown_astro_class_reference = true;
                        continue;
                    };
                    match initializer.value() {
                        Ok(AnyHtmlAttributeInitializer::HtmlString(string)) => {
                            let Some(value) = string.inner_string_text().ok() else {
                                *has_unknown_astro_class_reference = true;
                                continue;
                            };
                            if value.text().contains(['\\', '&']) {
                                *has_unknown_astro_class_reference = true;
                            } else {
                                collect_class_token_text(value, astro_class_references);
                            }
                        }
                        Ok(AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(_)) => {}
                        _ => *has_unknown_astro_class_reference = true,
                    }
                }
                AnyAstroDirective::AstroSetDirective(directive)
                    if directive
                        .value()
                        .ok()
                        .and_then(|value| value.name().ok())
                        .and_then(|name| name.value_token().ok())
                        .is_some_and(|name| name.text_trimmed() == "html") =>
                {
                    *has_unknown_astro_class_reference = true;
                }
                _ => {}
            },
            AnyHtmlAttribute::HtmlSpreadAttribute(_)
            | AnyHtmlAttribute::HtmlAttributeDoubleTextExpression(_)
            | AnyHtmlAttribute::HtmlAttributeSingleTextExpression(_) => {
                *has_unknown_astro_class_reference = true;
            }
            _ => {}
        }
    }
}

fn collect_astro_style_definitions(
    root: &AnyJsRoot,
    file_source: &JsFileSource,
    has_errors: bool,
    content_offset: TextSize,
    styles: &mut [AstroStyleInfo],
) {
    if !file_source.as_embedding_kind().is_astro() {
        return;
    }
    let Some(style) = styles
        .iter_mut()
        .find(|style| style.define_vars_range.contains(content_offset))
    else {
        return;
    };
    if has_errors {
        return;
    }
    let Some(object) = root
        .as_js_expression_template_root()
        .and_then(|root| root.expression())
        .map(AnyJsExpression::omit_parentheses)
        .and_then(|expression| expression.as_js_object_expression().cloned())
    else {
        return;
    };

    for member in object.members().iter().flatten() {
        let (name, range) = match member {
            AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => {
                let Some(token) = member.name().ok().and_then(|name| name.value_token().ok())
                else {
                    continue;
                };
                let name = token.token_text_trimmed();
                if name.text().contains('\\') {
                    continue;
                }
                (Text::from(name), token.text_trimmed_range())
            }
            AnyJsObjectMember::JsPropertyObjectMember(member) => {
                if member.value().is_err() {
                    continue;
                }
                let Some(name) = member
                    .name()
                    .ok()
                    .and_then(|name| name.as_js_literal_member_name().cloned())
                else {
                    continue;
                };
                let Some(token) = name.value().ok() else {
                    continue;
                };
                if token.text_trimmed().contains('\\') {
                    continue;
                }
                let Some(decoded) = AnyJsObjectMemberName::JsLiteralMemberName(name).name() else {
                    continue;
                };
                if !token.text_trimmed().starts_with(['\'', '"']) {
                    let mut bytes = decoded.text().bytes();
                    if !bytes.next().is_some_and(|byte| {
                        byte == b'_' || byte == b'$' || byte.is_ascii_alphabetic()
                    }) || !bytes
                        .all(|byte| byte == b'_' || byte == b'$' || byte.is_ascii_alphanumeric())
                    {
                        continue;
                    }
                }
                (Text::from(decoded), token.text_trimmed_range())
            }
            _ => continue,
        };
        if name.text() != "__proto__" {
            style.definitions.push(AstroStyleVariable {
                name,
                range: range + content_offset,
            });
        }
    }
}

fn collect_astro_style_references(
    root: &AnyCssRoot,
    file_source: &CssFileSource,
    has_errors: bool,
    content_offset: TextSize,
    styles: &mut [AstroStyleInfo],
) {
    let Some(style) = styles
        .iter_mut()
        .find(|style| style.style_range.contains(content_offset))
    else {
        return;
    };
    if !file_source.is_css() || has_errors {
        style.definitions.clear();
        return;
    }
    style.has_supported_css = true;
    for function in root.syntax().descendants().filter_map(CssFunction::cast) {
        let Some(name) = function
            .name()
            .ok()
            .and_then(|name| name.as_css_identifier().cloned())
            .and_then(|name| name.value_token().ok())
        else {
            continue;
        };
        if !name.text_trimmed().eq_ignore_ascii_case("var") {
            continue;
        }
        let Some(first_argument) = function.items().iter().next().and_then(Result::ok) else {
            continue;
        };
        if first_argument.syntax().text_trimmed().contains_char('\\') {
            style.has_supported_css = false;
            return;
        }
        let Some(identifier) = first_argument
            .syntax()
            .descendants()
            .find_map(CssDashedIdentifier::cast)
        else {
            continue;
        };
        let Some(token) = identifier.value_token().ok() else {
            continue;
        };
        let text = token.token_text_trimmed();
        if !text.text().starts_with("--") {
            continue;
        }
        let name = text
            .clone()
            .slice(TextRange::new(TextSize::from(2), text.len()));
        style.references.insert(Text::from(name));
    }
}

fn collect_astro_class_expression(
    expression: &AnyJsExpression,
    is_class_list: bool,
    classes: &mut IndexSet<Text>,
) -> bool {
    let mut pending = vec![(expression.clone(), is_class_list)];
    while let Some((expression, is_class_list)) = pending.pop() {
        match expression.omit_parentheses() {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(string),
            ) => {
                let Some(raw) = string.inner_string_text().ok() else {
                    return false;
                };
                if raw.text().contains('\\') {
                    return false;
                }
                collect_class_token_text(raw, classes);
            }
            AnyJsExpression::JsTemplateExpression(template) if template.is_constant() => {
                if template.syntax().text_trimmed().contains_char('\\') {
                    return false;
                }
                for element in template.elements() {
                    let AnyJsTemplateElement::JsTemplateChunkElement(chunk) = element else {
                        return false;
                    };
                    let Some(token) = chunk.template_chunk_token().ok() else {
                        return false;
                    };
                    collect_class_token_text(token.token_text_trimmed(), classes);
                }
            }
            AnyJsExpression::JsArrayExpression(array) if is_class_list => {
                for element in array.elements().iter() {
                    let Ok(AnyJsArrayElement::AnyJsExpression(expression)) = element else {
                        return false;
                    };
                    pending.push((expression, true));
                }
            }
            AnyJsExpression::JsObjectExpression(object) if is_class_list => {
                for member in object.members().iter() {
                    match member {
                        Ok(AnyJsObjectMember::JsPropertyObjectMember(member)) => {
                            let Some(name) = member
                                .name()
                                .ok()
                                .filter(|name| name.as_js_computed_member_name().is_none())
                                .and_then(|name| name.name())
                                .filter(|name| !name.text().contains('\\'))
                            else {
                                return false;
                            };
                            collect_class_token_text(name, classes);
                        }
                        Ok(AnyJsObjectMember::JsShorthandPropertyObjectMember(member)) => {
                            let Some(name) = member
                                .name()
                                .ok()
                                .and_then(|name| name.value_token().ok())
                                .map(|name| name.token_text_trimmed())
                            else {
                                return false;
                            };
                            if name.text().contains('\\') {
                                return false;
                            }
                            collect_class_token_text(name, classes);
                        }
                        _ => return false,
                    }
                }
            }
            AnyJsExpression::JsConditionalExpression(conditional) if is_class_list => {
                let Some(consequent) = conditional.consequent().ok() else {
                    return false;
                };
                let Some(alternate) = conditional.alternate().ok() else {
                    return false;
                };
                pending.push((alternate, true));
                pending.push((consequent, true));
            }
            AnyJsExpression::JsLogicalExpression(logical)
                if is_class_list && logical.operator() == Ok(JsLogicalOperator::LogicalAnd) =>
            {
                let Some(right) = logical.right().ok() else {
                    return false;
                };
                pending.push((right, true));
            }
            _ => return false,
        }
    }
    true
}

fn collect_class_token_text(value: TokenText, classes: &mut IndexSet<Text>) {
    let mut start = None;
    for (index, byte) in value
        .text()
        .bytes()
        .chain(std::iter::once(b' '))
        .enumerate()
    {
        if byte.is_ascii_whitespace() {
            if let Some(start) = start.take() {
                let range =
                    TextRange::new(TextSize::from(start as u32), TextSize::from(index as u32));
                classes.insert(Text::from(value.clone().slice(range)));
            }
        } else if start.is_none() {
            start = Some(index);
        }
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
    content_offset: TextSize,
) {
    // Applicability for selectors *not* inside :global(...).
    // Selectors inside :global(...) are unconditionally Global.
    let base_applicability = file_source.embedding_applicability();
    let mut global_depth: u32 = 0;

    for event in css_root.syntax().preorder() {
        match event {
            WalkEvent::Enter(node) => {
                if let Some(pseudo_fn) = AnyCssPseudoClassFunctionSelector::cast(node.clone()) {
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
                        range: token.text_trimmed_range(),
                        content_offset: Some(content_offset),
                        applicability,
                    });
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
