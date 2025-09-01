use std::collections::HashMap;

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsModuleItem, AnyJsStatement, JsIdentifierBinding, JsModuleItemList,
    JsSyntaxKind,
};
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::use_vue_define_macros_order::UseVueDefineMacrosOrderOptions;

declare_lint_rule! {
    /// Enforce specific order of Vue define macros.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub UseVueDefineMacrosOrder {
        version: "next",
        name: "useVueDefineMacrosOrder",
        language: "js",
        sources: &[RuleSource::EslintVueJs("define-macros-order").same()],
        recommended: false,
        domains: &[RuleDomain::Vue],
        // TODO. Implement autofix
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseVueDefineMacrosOrder {
    type Query = Ast<JsModuleItemList>;
    type State = (Box<str>, TextRange);
    type Signals = Option<Self::State>;
    type Options = UseVueDefineMacrosOrderOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let path = ctx.file_path();
        if path.extension() != Some("vue") {
            return None;
        }

        let order = &ctx.options().order;
        let order_map: HashMap<&str, usize> = order
            .iter()
            .enumerate()
            .map(|(idx, s)| (s.as_ref(), idx))
            .collect();

        struct FoundMacro {
            order_idx: usize,
            range: TextRange,
            has_out_of_order_content_prior: bool,
        }

        let mut non_macro_found = false;
        let mut found_macro: Option<FoundMacro> = None;

        for item in ctx.query().into_iter() {
            if is_skippable_before_macro(&item) {
                continue;
            }

            if let Some(name) = get_possible_macro(&item)
                && let Some(&order_idx) = order_map.get(name.as_ref())
            {
                match &found_macro {
                    None => {
                        found_macro = Some(FoundMacro {
                            order_idx,
                            range: item.range(),
                            has_out_of_order_content_prior: non_macro_found,
                        });
                    }
                    Some(c) if order_idx < c.order_idx => {
                        found_macro = Some(FoundMacro {
                            order_idx,
                            range: item.range(),
                            has_out_of_order_content_prior: true,
                        });
                    }
                    _ => {
                        // Same/higher order, do nothing
                    }
                }
            } else {
                non_macro_found = true;
            }
        }

        if let Some(vue_macro) = found_macro
            && vue_macro.has_out_of_order_content_prior
        {
            return Some((order[vue_macro.order_idx].clone(), vue_macro.range));
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (name, range) = state;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Vue macros order are not sorted."
                },
            )
            .note(markup! {
                <Emphasis>{name}</Emphasis> " should be the first statement in `<script setup>` (after any potential import statements or type definitions)."
            }),
        )
    }
}

fn get_possible_macro(item: &AnyJsModuleItem) -> Option<TokenText> {
    let statement = item.as_any_js_statement()?;
    let expr_statement = statement.as_js_expression_statement()?;
    let expr = expr_statement.expression().ok()?;

    get_call_expression_name(&expr)
}

fn get_variable_call_expr_name(statement: &AnyJsStatement) -> Option<TokenText> {
    let statement = statement.as_js_variable_statement()?;
    let declaration = statement.declaration().ok()?;
    let a = declaration.declarators().into_iter().find_map(|p| {
        let p = p.ok()?;

        None;
    });

    Some(name)
}

fn get_call_expression_name(expr: &AnyJsExpression) -> Option<TokenText> {
    let callee = expr.as_js_call_expression()?.callee().ok()?;
    let name = callee.get_callee_object_name()?.token_text_trimmed();

    // TODO. Special case withDefaults

    Some(name)
}

/// What can appear before the macro without causing a violation.
fn is_skippable_before_macro(item: &AnyJsModuleItem) -> bool {
    matches!(
        item.syntax().kind(),
        JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION
            | JsSyntaxKind::TS_INTERFACE_DECLARATION
            | JsSyntaxKind::TS_MODULE_DECLARATION
            | JsSyntaxKind::JS_DEBUGGER_STATEMENT
            | JsSyntaxKind::JS_EMPTY_STATEMENT
            | JsSyntaxKind::JS_IMPORT
            // If this is correct for your parser:
            | JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE
    )
}
