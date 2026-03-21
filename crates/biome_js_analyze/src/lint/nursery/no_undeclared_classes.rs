use crate::services::module_graph::ResolvedImports;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression,
    AnyJsObjectMember, AnyJsxAttributeValue, JsExpressionTemplateRoot, JsFileSource, JsxAttribute,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_module_graph::{ImportTreeDisplay, ImportTreeNode};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TextSize, declare_node_union};
use biome_rule_options::no_undeclared_classes::NoUndeclaredClassesOptions;

declare_lint_rule! {
    /// Reports CSS class names in JSX `className` or `class` attributes that are not defined
    /// in any imported CSS file.
    ///
    /// When a JSX file imports CSS files, every class name used in `className=` or `class=`
    /// attributes is checked against the available class definitions. Classes that are not
    /// defined are reported.
    ///
    /// This rule checks string literals, variable references (resolved through the semantic
    /// model), call expressions like `clsx(...)` / `classnames(...)`, object expression keys,
    /// and array expressions. Dynamic class names that cannot be statically resolved are
    /// silently skipped.
    ///
    /// In Astro files, `class:list={...}` directives and `class={...}` attribute expressions
    /// are also checked. CSS files imported in the frontmatter (`import "./styles.css"`) are
    /// included in the class resolution.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,ignore
    /// import "./styles.css";
    /// export default () => <div className="missing" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,ignore
    /// import "./styles.css";
    /// export default () => <div className="header" />;
    /// ```
    ///
    pub NoUndeclaredClasses {
        version: "next",
        name: "noUndeclaredClasses",
        language: "js",
        recommended: false,
        issue_number: Some("9156"),
        domains: &[RuleDomain::Project],
    }
}

declare_node_union! {
    /// A node that may contain CSS class references:
    /// - `JsxAttribute`: JSX `className="..."` or `class="..."` attributes
    /// - `JsExpressionTemplateRoot`: Embedded expression snippets from framework
    ///   directives like Astro's `class:list={...}` or `class={...}`
    pub AnyClassLikeAttribute = JsxAttribute | JsExpressionTemplateRoot
}

impl Rule for NoUndeclaredClasses {
    type Query = ResolvedImports<AnyClassLikeAttribute>;
    type State = UndeclaredClass;
    type Signals = Vec<Self::State>;
    type Options = NoUndeclaredClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let Some(semantic_model) = ctx.get_service::<SemanticModel>() else {
            // No semantic model available (e.g., embedded snippets without full analysis).
            // For JsExpressionTemplateRoot snippets, we can still extract string literals
            // without the semantic model. For JsxAttribute, the model should always exist.
            return match node {
                AnyClassLikeAttribute::JsExpressionTemplateRoot(root) => {
                    run_without_semantic(root, ctx)
                }
                AnyClassLikeAttribute::JsxAttribute(_) => Vec::new(),
            };
        };

        let class_entries = match node {
            AnyClassLikeAttribute::JsxAttribute(attr) => {
                extract_class_entries_from_jsx(attr, semantic_model)
            }
            AnyClassLikeAttribute::JsExpressionTemplateRoot(root) => {
                extract_class_entries_from_embedded(root, ctx, semantic_model)
            }
        };

        if class_entries.is_empty() {
            return Vec::new();
        }

        let module_graph = ctx.module_graph();
        let file_path = ctx.file_path();

        // Determine whether to use JS or HTML traversal based on file type.
        // Astro files store their module info as HtmlModuleInfo, so CSS imports
        // from frontmatter are accessed via the HTML traversal.
        let file_source = ctx.source_type::<JsFileSource>();
        let is_html_like = file_source.as_embedding_kind().is_astro();

        // Collect all reachable CSS steps. If no CSS is reachable at all,
        // skip to avoid false positives on files without any stylesheets.
        let css_steps: Vec<_> = if is_html_like {
            module_graph
                .traverse_import_tree_for_html_classes(file_path)
                .collect()
        } else {
            module_graph
                .traverse_import_tree_for_classes(file_path)
                .collect()
        };

        if css_steps.is_empty() {
            return Vec::new();
        }

        let mut signals = Vec::new();

        for entry in &class_entries {
            let found_class = css_steps.iter().any(|step| {
                step.css_classes
                    .iter()
                    .any(|c| c.text() == entry.name.as_ref())
            });

            if !found_class {
                let import_tree = if is_html_like {
                    module_graph.build_import_tree_for_html(file_path)
                } else {
                    module_graph.build_import_tree(file_path)
                };
                signals.push(UndeclaredClass {
                    range: entry.range,
                    name: entry.name.clone(),
                    import_tree,
                });
            }
        }

        signals
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            state.range,
            markup! {
                "The CSS class "<Emphasis>{&state.name}</Emphasis>" is not defined in any imported stylesheet."
            },
        )
        .note(markup! {
            "Referencing undefined classes often indicates a typo or a missing stylesheet, and will result in elements not being styled as intended."
        });

        // Show the import tree with checked CSS files
        if let Some(import_tree) = &state.import_tree {
            let working_directory = ctx.working_directory();
            let tree_display = ImportTreeDisplay::new(import_tree, working_directory);
            diag = diag.note(markup! {
                "Checked import tree:\n\n"{tree_display}
            });
        }

        Some(diag.note(markup! {
            "Either import a CSS file that defines this class or remove this class name."
        }))
    }
}

/// Stores the text range and name of an undeclared class.
pub struct UndeclaredClass {
    /// Range of this class name token within the source file.
    pub range: TextRange,
    /// The class name that was not found.
    pub name: Box<str>,
    /// The import tree structure for displaying which files/CSS were checked.
    pub import_tree: Option<ImportTreeNode>,
}

/// A single class name occurrence found in the source code, with its range.
struct ClassNameEntry {
    /// Range pointing to this class name in the source file.
    range: TextRange,
    /// The class name text.
    name: Box<str>,
}

/// Extracts class names from a JSX `className` or `class` attribute.
fn extract_class_entries_from_jsx(
    attr: &JsxAttribute,
    model: &SemanticModel,
) -> Vec<ClassNameEntry> {
    let mut entries = Vec::new();
    extract_class_entries_from_jsx_impl(attr, model, &mut entries);
    entries
}

fn extract_class_entries_from_jsx_impl(
    attr: &JsxAttribute,
    model: &SemanticModel,
    out: &mut Vec<ClassNameEntry>,
) -> Option<()> {
    let name_token = attr.name_value_token().ok()?;
    let name_text = name_token.text_trimmed();

    // Check for className (React) or class (SolidJS, etc.)
    if name_text != "className" && name_text != "class" {
        return None;
    }

    let value = attr.initializer()?.value().ok()?;

    match value {
        AnyJsxAttributeValue::JsxString(s) => {
            let token = s.value_token().ok()?;
            let inner_text = s.inner_string_text().ok()?;
            // +1 to skip the opening quote
            let inner_start = u32::from(token.text_trimmed_range().start()) + 1;
            collect_class_names_from_string(&inner_text, inner_start, out);
        }
        AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_attr) => {
            let expression = expr_attr.expression().ok()?.omit_parentheses();
            collect_class_names_from_expression(&expression, model, out);
        }
        // JSX tags as attribute values cannot contain class names
        AnyJsxAttributeValue::AnyJsxTag(_) => return None,
    }

    Some(())
}

/// Extracts class names from an embedded expression template root
/// (e.g., from Astro `class:list={...}` or `class={...}`).
///
/// Only processes the expression if it came from a class-related attribute,
/// as determined by the `is_class_attribute` flag in `EmbeddingKind`.
fn extract_class_entries_from_embedded(
    root: &JsExpressionTemplateRoot,
    ctx: &RuleContext<NoUndeclaredClasses>,
    model: &SemanticModel,
) -> Vec<ClassNameEntry> {
    let file_source = ctx.source_type::<JsFileSource>();
    if !file_source.as_embedding_kind().is_class_attribute() {
        return Vec::new();
    }

    let mut entries = Vec::new();
    if let Ok(expression) = root.expression() {
        collect_class_names_from_expression(&expression.omit_parentheses(), model, &mut entries);
    }
    entries
}

/// Runs class checking on an embedded expression template root when no semantic
/// model is available. Can still extract string literals, object keys, and array
/// elements — just cannot resolve variable references.
fn run_without_semantic(
    root: &JsExpressionTemplateRoot,
    ctx: &RuleContext<NoUndeclaredClasses>,
) -> Vec<UndeclaredClass> {
    let file_source = ctx.source_type::<JsFileSource>();
    if !file_source.as_embedding_kind().is_class_attribute() {
        return Vec::new();
    }

    let mut entries = Vec::new();
    if let Ok(expression) = root.expression() {
        collect_class_names_from_expression_no_semantic(
            &expression.omit_parentheses(),
            &mut entries,
        );
    }

    if entries.is_empty() {
        return Vec::new();
    }

    let module_graph = ctx.module_graph();
    let file_path = ctx.file_path();

    // Collect all reachable CSS steps. If no CSS is reachable at all,
    // skip to avoid false positives on files without any stylesheets.
    let css_steps: Vec<_> = module_graph
        .traverse_import_tree_for_html_classes(file_path)
        .collect();

    if css_steps.is_empty() {
        return Vec::new();
    }

    let mut signals = Vec::new();
    for entry in &entries {
        let found_class = css_steps.iter().any(|step| {
            step.css_classes
                .iter()
                .any(|c| c.text() == entry.name.as_ref())
        });

        if !found_class {
            let import_tree = module_graph.build_import_tree_for_html(file_path);
            signals.push(UndeclaredClass {
                range: entry.range,
                name: entry.name.clone(),
                import_tree,
            });
        }
    }

    signals
}

/// Like `collect_class_names_from_expression` but without a semantic model.
/// Cannot resolve variable references, but handles string literals, arrays, objects,
/// and call expressions.
fn collect_class_names_from_expression_no_semantic(
    expr: &AnyJsExpression,
    out: &mut Vec<ClassNameEntry>,
) -> Option<()> {
    match expr {
        AnyJsExpression::AnyJsLiteralExpression(lit) => match lit {
            AnyJsLiteralExpression::JsStringLiteralExpression(string_lit) => {
                let token = string_lit.value_token().ok()?;
                let inner_text = string_lit.inner_string_text().ok()?;
                let inner_start = u32::from(token.text_trimmed_range().start()) + 1;
                collect_class_names_from_string(&inner_text, inner_start, out);
            }
            AnyJsLiteralExpression::JsBigintLiteralExpression(_)
            | AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
            | AnyJsLiteralExpression::JsNullLiteralExpression(_)
            | AnyJsLiteralExpression::JsNumberLiteralExpression(_)
            | AnyJsLiteralExpression::JsRegexLiteralExpression(_) => return None,
        },
        AnyJsExpression::JsCallExpression(call_expr) => {
            for arg in call_expr.arguments().ok()?.args() {
                match arg.ok()? {
                    AnyJsCallArgument::AnyJsExpression(arg_expr) => {
                        collect_class_names_from_expression_no_semantic(
                            &arg_expr.omit_parentheses(),
                            out,
                        );
                    }
                    AnyJsCallArgument::JsSpread(_) => {}
                }
            }
        }
        AnyJsExpression::JsObjectExpression(obj_expr) => {
            for member in obj_expr.members() {
                match member.ok()? {
                    AnyJsObjectMember::JsPropertyObjectMember(prop) => {
                        let member_name = prop.name().ok()?;
                        let key_text = member_name.name()?;
                        let range = member_name.range();
                        let key_len = u32::from(key_text.len());
                        let adjusted_range = if range.len() > TextSize::from(key_len) {
                            let start = range.start() + TextSize::from(1);
                            let end = range.end() - TextSize::from(1);
                            TextRange::new(start, end)
                        } else {
                            range
                        };
                        out.push(ClassNameEntry {
                            range: adjusted_range,
                            name: key_text.text().into(),
                        });
                    }
                    AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => {
                        let ident = shorthand.name().ok()?;
                        let token = ident.value_token().ok()?;
                        out.push(ClassNameEntry {
                            range: token.text_trimmed_range(),
                            name: token.text_trimmed().into(),
                        });
                    }
                    AnyJsObjectMember::JsSpread(_)
                    | AnyJsObjectMember::JsGetterObjectMember(_)
                    | AnyJsObjectMember::JsSetterObjectMember(_)
                    | AnyJsObjectMember::JsMethodObjectMember(_)
                    | AnyJsObjectMember::JsBogusMember(_)
                    | AnyJsObjectMember::JsMetavariable(_) => {}
                }
            }
        }
        AnyJsExpression::JsArrayExpression(array_expr) => {
            for element in array_expr.elements().iter() {
                match element.ok()? {
                    AnyJsArrayElement::AnyJsExpression(elem_expr) => {
                        collect_class_names_from_expression_no_semantic(
                            &elem_expr.omit_parentheses(),
                            out,
                        );
                    }
                    AnyJsArrayElement::JsArrayHole(_) | AnyJsArrayElement::JsSpread(_) => {}
                }
            }
        }
        // All other expressions: cannot resolve without semantic model
        _ => return None,
    }
    Some(())
}

/// Splits a string by whitespace and creates a `ClassNameEntry` for each class name,
/// computing the correct source range for each.
fn collect_class_names_from_string(text: &str, base_offset: u32, out: &mut Vec<ClassNameEntry>) {
    let mut search_from: u32 = 0;
    for class_name in text.split_ascii_whitespace() {
        let pos_in_text = text[search_from as usize..]
            .find(class_name)
            .map_or(search_from, |o| search_from + o as u32);

        let start = TextSize::from(base_offset + pos_in_text);
        let end = start + TextSize::from(class_name.len() as u32);
        out.push(ClassNameEntry {
            range: TextRange::new(start, end),
            name: class_name.into(),
        });

        search_from = pos_in_text + class_name.len() as u32;
    }
}

/// Recursively collects class names from a JS expression.
///
/// Handles:
/// - String literals: `"foo bar"` → split by whitespace
/// - Identifier references: `cls` → follow binding to declaration initializer
/// - Call expressions: `clsx("foo", { bar: true })` → recurse into each argument
/// - Object expressions: `{ foo: true, bar: false }` → extract keys as class names
/// - Array expressions: `["foo", "bar"]` → recurse into each element
///
/// Returns `None` for unresolvable expressions (template literals, conditionals, etc.),
/// but always appends any successfully extracted entries to `out` before returning.
fn collect_class_names_from_expression(
    expr: &AnyJsExpression,
    model: &SemanticModel,
    out: &mut Vec<ClassNameEntry>,
) -> Option<()> {
    match expr {
        // String literal: "foo bar" → split by whitespace
        AnyJsExpression::AnyJsLiteralExpression(lit) => match lit {
            AnyJsLiteralExpression::JsStringLiteralExpression(string_lit) => {
                let token = string_lit.value_token().ok()?;
                let inner_text = string_lit.inner_string_text().ok()?;
                // +1 to skip the opening quote
                let inner_start = u32::from(token.text_trimmed_range().start()) + 1;
                collect_class_names_from_string(&inner_text, inner_start, out);
            }
            // Non-string literals (numbers, booleans, null, etc.) are not class names
            AnyJsLiteralExpression::JsBigintLiteralExpression(_)
            | AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
            | AnyJsLiteralExpression::JsNullLiteralExpression(_)
            | AnyJsLiteralExpression::JsNumberLiteralExpression(_)
            | AnyJsLiteralExpression::JsRegexLiteralExpression(_) => return None,
        },

        // Identifier: follow binding → declaration → initializer
        AnyJsExpression::JsIdentifierExpression(ident_expr) => {
            let name = ident_expr.name().ok()?;
            let binding = model.binding(&name)?;
            let decl = binding.tree().declaration()?;
            let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = decl else {
                return None;
            };
            let init_expr = declarator
                .initializer()?
                .expression()
                .ok()?
                .omit_parentheses();
            collect_class_names_from_expression(&init_expr, model, out);
        }

        // Call expression: clsx("foo", { bar: true }, ["baz"]) → recurse into each argument
        AnyJsExpression::JsCallExpression(call_expr) => {
            for arg in call_expr.arguments().ok()?.args() {
                match arg.ok()? {
                    AnyJsCallArgument::AnyJsExpression(arg_expr) => {
                        collect_class_names_from_expression(
                            &arg_expr.omit_parentheses(),
                            model,
                            out,
                        );
                    }
                    // Spread arguments cannot be statically resolved
                    AnyJsCallArgument::JsSpread(_) => {}
                }
            }
        }

        // Object expression: { "foo": true, bar: cond } → keys are class names
        AnyJsExpression::JsObjectExpression(obj_expr) => {
            for member in obj_expr.members() {
                match member.ok()? {
                    AnyJsObjectMember::JsPropertyObjectMember(prop) => {
                        let member_name = prop.name().ok()?;
                        let key_text = member_name.name()?;
                        let range = member_name.range();
                        let key_len = u32::from(key_text.len());
                        // For quoted string keys like "btn-invalid", trim the quotes from range
                        let adjusted_range = if range.len() > TextSize::from(key_len) {
                            let start = range.start() + TextSize::from(1);
                            let end = range.end() - TextSize::from(1);
                            TextRange::new(start, end)
                        } else {
                            range
                        };
                        out.push(ClassNameEntry {
                            range: adjusted_range,
                            name: key_text.text().into(),
                        });
                    }
                    // Shorthand properties: { foo } — the identifier is the class name
                    AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => {
                        let ident = shorthand.name().ok()?;
                        let token = ident.value_token().ok()?;
                        out.push(ClassNameEntry {
                            range: token.text_trimmed_range(),
                            name: token.text_trimmed().into(),
                        });
                    }
                    // Spread, getters, setters, methods, bogus, metavariable — skip
                    AnyJsObjectMember::JsSpread(_)
                    | AnyJsObjectMember::JsGetterObjectMember(_)
                    | AnyJsObjectMember::JsSetterObjectMember(_)
                    | AnyJsObjectMember::JsMethodObjectMember(_)
                    | AnyJsObjectMember::JsBogusMember(_)
                    | AnyJsObjectMember::JsMetavariable(_) => {}
                }
            }
        }

        // Array expression: ["foo", "bar", { baz: true }] → recurse into each element
        AnyJsExpression::JsArrayExpression(array_expr) => {
            for element in array_expr.elements().iter() {
                match element.ok()? {
                    AnyJsArrayElement::AnyJsExpression(elem_expr) => {
                        collect_class_names_from_expression(
                            &elem_expr.omit_parentheses(),
                            model,
                            out,
                        );
                    }
                    // Holes and spreads cannot be statically resolved
                    AnyJsArrayElement::JsArrayHole(_) | AnyJsArrayElement::JsSpread(_) => {}
                }
            }
        }

        // Expressions that cannot produce static class name strings
        AnyJsExpression::JsTemplateExpression(_)
        | AnyJsExpression::JsConditionalExpression(_)
        | AnyJsExpression::JsBinaryExpression(_)
        | AnyJsExpression::JsLogicalExpression(_)
        | AnyJsExpression::JsArrowFunctionExpression(_)
        | AnyJsExpression::JsFunctionExpression(_)
        | AnyJsExpression::JsClassExpression(_)
        | AnyJsExpression::JsAssignmentExpression(_)
        | AnyJsExpression::JsAwaitExpression(_)
        | AnyJsExpression::JsComputedMemberExpression(_)
        | AnyJsExpression::JsImportCallExpression(_)
        | AnyJsExpression::JsImportMetaExpression(_)
        | AnyJsExpression::JsInExpression(_)
        | AnyJsExpression::JsInstanceofExpression(_)
        | AnyJsExpression::JsNewExpression(_)
        | AnyJsExpression::JsNewTargetExpression(_)
        | AnyJsExpression::JsParenthesizedExpression(_)
        | AnyJsExpression::JsPostUpdateExpression(_)
        | AnyJsExpression::JsPreUpdateExpression(_)
        | AnyJsExpression::JsSequenceExpression(_)
        | AnyJsExpression::JsStaticMemberExpression(_)
        | AnyJsExpression::JsSuperExpression(_)
        | AnyJsExpression::JsThisExpression(_)
        | AnyJsExpression::JsUnaryExpression(_)
        | AnyJsExpression::JsYieldExpression(_)
        | AnyJsExpression::JsxTagExpression(_)
        | AnyJsExpression::JsBogusExpression(_)
        | AnyJsExpression::JsMetavariable(_)
        | AnyJsExpression::TsAsExpression(_)
        | AnyJsExpression::TsInstantiationExpression(_)
        | AnyJsExpression::TsNonNullAssertionExpression(_)
        | AnyJsExpression::TsSatisfiesExpression(_)
        | AnyJsExpression::TsTypeAssertionExpression(_) => return None,
    }

    Some(())
}
