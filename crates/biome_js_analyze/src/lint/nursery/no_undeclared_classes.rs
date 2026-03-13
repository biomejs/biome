use crate::services::module_graph::ResolvedImports;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression,
    AnyJsObjectMember, AnyJsxAttributeValue, JsxAttribute, binding_ext::AnyJsBindingDeclaration,
};
use biome_module_graph::{ImportTreeDisplay, ImportTreeNode};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TextSize};
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

impl Rule for NoUndeclaredClasses {
    type Query = ResolvedImports<JsxAttribute>;
    type State = UndeclaredClass;
    type Signals = Vec<Self::State>;
    type Options = NoUndeclaredClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attr = ctx.query();
        let semantic_model = ctx
            .get_service::<SemanticModel>()
            .expect("Semantic service not available");

        let class_entries = extract_class_entries(attr, &semantic_model);
        if class_entries.is_empty() {
            return Vec::new();
        }

        let module_graph = ctx.module_graph();
        let file_path = ctx.file_path();

        let mut signals = Vec::new();

        for entry in &class_entries {
            // Lazily check if this class exists in any CSS file
            let mut found_class = false;
            for step in module_graph.traverse_import_tree_for_classes(file_path) {
                if step
                    .css_classes
                    .iter()
                    .any(|token| token.text() == entry.name.as_ref())
                {
                    found_class = true;
                    break;
                }
            }

            if !found_class {
                let import_tree = module_graph.build_import_tree(file_path);
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

/// Extracts all statically-resolvable class names from a JSX `className` or `class` attribute.
fn extract_class_entries(attr: &JsxAttribute, model: &SemanticModel) -> Vec<ClassNameEntry> {
    let mut entries = Vec::new();
    extract_class_entries_impl(attr, model, &mut entries);
    entries
}

fn extract_class_entries_impl(
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
                    AnyJsCallArgument::JsSpread(_) => continue,
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
                    | AnyJsObjectMember::JsMetavariable(_) => continue,
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
                    AnyJsArrayElement::JsArrayHole(_) | AnyJsArrayElement::JsSpread(_) => {
                        continue;
                    }
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
