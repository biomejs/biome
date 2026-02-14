use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::TsEnumDeclaration;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_consistent_enum_value_type::UseConsistentEnumValueTypeOptions;

use crate::services::typed::Typed;

declare_lint_rule! {
    /// Disallow enums from having both number and string members.
    ///
    /// TypeScript enums are allowed to assign numeric or string values to their members.
    /// Most enums contain either all numbers or all strings, but in theory you can mix-and-match within the same enum.
    /// Mixing enum member types is generally considered confusing and a bad practice.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// enum Status {
    ///   Unknown,
    ///   Closed = 1,
    ///   Open = 'open',
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// enum Status {
    ///   Unknown = 0,
    ///   Closed = 1,
    ///   Open = 2,
    /// }
    /// ```
    ///
    /// ```ts
    /// enum Status {
    ///   Unknown,
    ///   Closed,
    ///   Open,
    /// }
    /// ```
    ///
    /// ```ts
    /// enum Status {
    ///   Unknown = 'unknown',
    ///   Closed = 'closed',
    ///   Open = 'open',
    /// }
    /// ```
    ///
    pub UseConsistentEnumValueType {
        version: "2.3.13",
        name: "useConsistentEnumValueType",
        language: "ts",
        recommended: false,
        domains: &[RuleDomain::Types],
        sources: &[RuleSource::EslintTypeScript("no-mixed-enums").same()],
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum EnumValueType {
    Number,
    String,
    Unknown,
}

impl Rule for UseConsistentEnumValueType {
    type Query = Typed<TsEnumDeclaration>;
    type State = Vec<TextRange>;
    type Signals = Option<Self::State>;
    type Options = UseConsistentEnumValueTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut found = vec![];
        let mut enum_type: Option<EnumValueType> = None;

        for member in node.members() {
            let Some(member) = member.ok() else {
                continue;
            };

            let Some(initializer) = member.initializer() else {
                if let Some(enum_type) = enum_type.clone() {
                    if enum_type != EnumValueType::Number {
                        found.push(member.range());
                    }
                } else {
                    enum_type = Some(EnumValueType::Number);
                }
                continue;
            };
            let Some(expr) = initializer.expression().ok() else {
                continue;
            };

            let expr_type = ctx.type_of_expression(&expr);

            if expr_type.is_string_or_string_literal() {
                if let Some(enum_type) = enum_type.clone() {
                    if enum_type != EnumValueType::String {
                        found.push(member.range());
                    }
                } else {
                    enum_type = Some(EnumValueType::String);
                }
                continue;
            }

            if expr_type.is_number_or_number_literal() {
                if let Some(enum_type) = enum_type.clone() {
                    if enum_type != EnumValueType::Number {
                        found.push(member.range());
                    }
                } else {
                    enum_type = Some(EnumValueType::Number);
                }
                continue;
            }

            if let Some(enum_type) = enum_type.clone() {
                if enum_type != EnumValueType::Unknown {
                    found.push(member.range());
                }
            } else {
                enum_type = Some(EnumValueType::Unknown);
            }
        }

        if found.is_empty() { None } else { Some(found) }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.first()?,
            markup! {
                "Inconsistent enum value type."
            },
        );

        for range in &state[1..] {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "Another inconsistent enum value type."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "Mixing number and string enums can be confusing. Make sure to use a consistent value type within your enum."
        }))
    }
}
