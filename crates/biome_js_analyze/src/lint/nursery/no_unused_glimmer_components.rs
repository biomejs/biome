use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::{HtmlElement, HtmlFileSource};
use biome_js_syntax::{
    AnyJsImportClause, AnyJsModuleItem, AnyJsNamedImportSpecifier, JsFileSource, JsModule,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_unused_glimmer_components::NoUnusedGlimmerComponentsOptions;
use regex::Regex;
use rustc_hash::FxHashSet;
use std::sync::LazyLock;

/// Regex to match Glimmer <template> tags
static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>").expect("Invalid Glimmer template regex")
});

declare_lint_rule! {
    /// Disallow importing components that are never used in Glimmer templates.
    ///
    /// This rule detects component imports in `.gjs` and `.gts` files that are never
    /// referenced in the file's `<template>` blocks. This is specific to Glimmer/Ember
    /// template tag format and helps identify dead code.
    ///
    /// The rule analyzes both the JavaScript imports and the Glimmer templates to
    /// ensure all imported components are actually used.
    ///
    /// ## Examples
    ///
    /// This rule only applies to `.gjs` and `.gts` files.
    ///
    /// ### Invalid
    ///
    /// In a `.gjs` file, if you import `Card` but never use it in the template:
    ///
    /// ```js,ignore
    /// import Button from './Button';
    /// import Card from './Card';  // Card is imported but never used
    ///
    /// // In template: only <Button> is used, <Card> is never referenced
    /// ```
    ///
    /// ### Valid
    ///
    /// In a `.gjs` file where all imported components are used:
    ///
    /// ```js,ignore
    /// import Button from './Button';
    /// import Card from './Card';
    ///
    /// // In template: both <Button> and <Card> are used
    /// ```
    ///
    pub NoUnusedGlimmerComponents {
        version: "next",
        name: "noUnusedGlimmerComponents",
        language: "js",
        recommended: false,
    }
}

pub struct UnusedComponentState {
    component_name: String,
    import_range: TextRange,
}

impl Rule for NoUnusedGlimmerComponents {
    type Query = Ast<JsModule>;
    type State = UnusedComponentState;
    type Signals = Box<[Self::State]>;
    type Options = NoUnusedGlimmerComponentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        // Only run on .gjs and .gts files (Glimmer embedding)
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.as_embedding_kind().is_glimmer() {
            return Vec::new().into_boxed_slice();
        }

        let root = ctx.query();

        // Collect imported component names (PascalCase identifiers)
        let mut imported_components: FxHashSet<(String, TextRange)> = FxHashSet::default();

        for item in root.items() {
            if let AnyJsModuleItem::JsImport(import) = item {
                if let Ok(import_clause) = import.import_clause() {
                    match import_clause {
                        AnyJsImportClause::JsImportNamedClause(named_clause) => {
                            if let Ok(specifiers) = named_clause.named_specifiers() {
                                for specifier in specifiers.specifiers() {
                                    if let Ok(specifier) = specifier {
                                        match specifier {
                                            AnyJsNamedImportSpecifier::JsNamedImportSpecifier(spec) => {
                                                // Get the local name (what it's imported as)
                                                if let Ok(local_name) = spec.local_name() {
                                                    let name = local_name.to_trimmed_text().to_string();
                                                    // Check if it's PascalCase (likely a component)
                                                    if is_pascal_case(&name) {
                                                        imported_components.insert((
                                                            name,
                                                            spec.range(),
                                                        ));
                                                    }
                                                }
                                            }
                                            AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(spec) => {
                                                if let Ok(local_name) = spec.local_name() {
                                                    let name = local_name.to_trimmed_text().to_string();
                                                    if is_pascal_case(&name) {
                                                        imported_components.insert((
                                                            name,
                                                            spec.range(),
                                                        ));
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        AnyJsImportClause::JsImportDefaultClause(default_clause) => {
                            // Default imports like `import Button from './Button'`
                            if let Ok(specifier) = default_clause.default_specifier() {
                                if let Ok(local_name) = specifier.local_name() {
                                    let name = local_name.to_trimmed_text().to_string();
                                    if is_pascal_case(&name) {
                                        imported_components.insert((name, specifier.range()));
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // If no components were imported, nothing to check
        if imported_components.is_empty() {
            return Vec::new().into_boxed_slice();
        }

        // Get the full source text to parse templates
        let source_text = root.syntax().text_with_trivia().to_string();

        // Find all components used in templates
        let used_components = find_components_in_templates(&source_text);

        // Find unused components
        let mut unused_components = Vec::new();
        for (component_name, import_range) in imported_components {
            if !used_components.contains(&component_name) {
                unused_components.push(UnusedComponentState {
                    component_name,
                    import_range,
                });
            }
        }

        unused_components.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.import_range,
                markup! {
                    "Component "<Emphasis>{state.component_name}</Emphasis>" is imported but never used in any template."
                },
            )
            .note(markup! {
                "Remove the unused import or add the component to a "<Emphasis>"<template>"</Emphasis>" block."
            }),
        )
    }
}

/// Check if a string is in PascalCase (starts with uppercase letter)
fn is_pascal_case(s: &str) -> bool {
    !s.is_empty() && s.chars().next().map_or(false, |c| c.is_uppercase())
}

/// Find all component names used in Glimmer templates
fn find_components_in_templates(source: &str) -> FxHashSet<String> {
    let mut used_components = FxHashSet::default();

    // Find all <template> blocks
    for template_match in GLIMMER_TEMPLATE.find_iter(source) {
        let template_content = template_match.as_str();

        // Parse the template with the HTML parser in Glimmer mode
        let file_source = HtmlFileSource::glimmer();
        let options = HtmlParseOptions::from(&file_source);
        let parse = parse_html(template_content, options);

        // Walk the HTML AST looking for element tags
        let root = parse.tree();
        let root_node = root.syntax();

        // Find all HTML elements (tags) in the template
        for node in root_node.descendants() {
            // Check if this node can be cast to HtmlElement
            if HtmlElement::can_cast(node.kind()) {
                let element = HtmlElement::unwrap_cast(node.clone());
                // Get the opening element and extract the tag name
                if let Ok(opening) = element.opening_element() {
                    if let Ok(name) = opening.name() {
                        if let Ok(value_token) = name.value_token() {
                            let tag_name = value_token.text_trimmed().to_string();
                            // If it's PascalCase, it's likely a component reference
                            if is_pascal_case(&tag_name) {
                                used_components.insert(tag_name);
                            }
                        }
                    }
                }
            }
        }
    }

    used_components
}
