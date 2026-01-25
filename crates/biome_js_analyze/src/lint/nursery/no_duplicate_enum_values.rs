use std::collections::HashSet;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, TsEnumDeclaration,
};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::no_duplicate_enum_values::NoDuplicateEnumValuesOptions;

declare_lint_rule! {
    /// Disallow duplicate enum member values.
    ///
    /// Although TypeScript supports duplicate enum member values, people usually expect members to have unique values within the same enum.
    /// Duplicate values can lead to bugs that are hard to track down.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// enum E {
    ///   A = 0,
    ///   B = 0,
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// enum E {
    ///   A = "A",
    ///   B = 'A',
    ///   C = `A`,
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// enum E {
    ///   A = 0,
    ///   B = 1,
    /// }
    /// ```
    ///
    /// ```ts
    /// enum E {
    ///   A = "A",
    ///   B = 'B',
    ///   C = `C`,
    /// }
    /// ```
    ///
    pub NoDuplicateEnumValues {
        version: "2.3.12",
        name: "noDuplicateEnumValues",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("no-duplicate-enum-values").same()],
    }
}

impl Rule for NoDuplicateEnumValues {
    type Query = Ast<TsEnumDeclaration>;
    type State = Vec<TextRange>;
    type Signals = Option<Self::State>;
    type Options = NoDuplicateEnumValuesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut found: HashSet<EnumValue> = HashSet::new();
        let mut duplicates = vec![];

        let mut enum_idx: Option<f64> = Some(0.0);
        for member in node.members() {
            let Some(member) = member.ok() else {
                continue;
            };

            let Some(initializer) = member.initializer() else {
                // enum members without an initializer after a member which initialized a string are considered invalid and linted by TypeScript
                let Some(enum_idx_value) = enum_idx else {
                    continue;
                };

                if !found.insert(EnumValue::Number(enum_idx_value.to_string())) {
                    duplicates.push(member.range());
                }

                enum_idx = Some(enum_idx_value + 1.0);

                continue;
            };
            let Some(expr) = initializer.expression().ok() else {
                continue;
            };

            match expr {
                AnyJsExpression::AnyJsLiteralExpression(literal_expr) => match literal_expr {
                    AnyJsLiteralExpression::JsNumberLiteralExpression(number_expr) => {
                        let Some(number) = number_expr.as_number() else {
                            continue;
                        };

                        if !found.insert(EnumValue::Number(number.to_string())) {
                            duplicates.push(member.range());
                        }

                        enum_idx = Some(number + 1.0);
                    }
                    AnyJsLiteralExpression::JsStringLiteralExpression(string_expr) => {
                        enum_idx = None;

                        let Some(token_text) = string_expr.inner_string_text().ok() else {
                            continue;
                        };

                        if !found.insert(EnumValue::String(token_text.to_string())) {
                            duplicates.push(member.range());
                        }
                    }
                    _ => {
                        enum_idx = None;
                    }
                },
                AnyJsExpression::JsTemplateExpression(template_expr) => {
                    enum_idx = None;
                    let elements = template_expr.elements();

                    if elements.len() == 1 {
                        let Some(first) = elements.first() else {
                            continue;
                        };

                        if let AnyJsTemplateElement::JsTemplateChunkElement(chunk_element) = first
                            && !found.insert(EnumValue::String(chunk_element.to_trimmed_string()))
                        {
                            duplicates.push(member.range());
                        }
                    }
                }
                _ => {
                    enum_idx = None;
                }
            }
        }

        if duplicates.is_empty() {
            None
        } else {
            Some(duplicates)
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.first()?,
            markup! {
                "Duplicate enum member value."
            },
        );

        for range in &state[1..] {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "Another duplicate enum member value."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "Expected members to have unique values. Duplicate values can lead to bugs that are hard to track down."
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum EnumValue {
    Number(String),
    String(String),
}
