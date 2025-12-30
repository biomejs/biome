use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsxAttribute, JsObjectExpression, JsVariableDeclarator, JsxElement,
    jsx_ext::AnyJsxElement,
};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_inline_script_id::UseInlineScriptIdOptions;
use rustc_hash::FxHashSet;

use crate::{
    nextjs::{NextUtility, is_next_import},
    services::semantic::Semantic,
};

declare_lint_rule! {
    /// Enforce `id` attribute on `next/script` components with inline content or `dangerouslySetInnerHTML`.
    ///
    /// Using inline scripts or `dangerouslySetInnerHTML` in `next/script` components requires an `id` attribute to ensure that Next.js can track and optimize them correctly.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// import Script from 'next/script'
    ///
    /// export default function Page() {
    ///   return (
    ///      <Script>{`console.log('Hello world!');`}</Script>
    ///   )
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// import Script from 'next/script'
    ///
    /// export default function Page() {
    ///   return (
    ///      <Script dangerouslySetInnerHTML={{ __html: `console.log('Hello world!');` }} />
    ///   )
    /// }
    /// ```
    ///
    /// ### Valid
    /// ```jsx
    /// import Script from 'next/script'
    ///
    /// export default function Page() {
    ///   return (
    ///      <Script id="my-script">{`console.log('Hello world!');`}</Script>
    ///   )
    /// }
    /// ```
    ///
    /// ```jsx
    /// import Script from 'next/script'
    ///
    /// export default function Page() {
    ///   return (
    ///      <Script id="my-script" dangerouslySetInnerHTML={{ __html: `console.log('Hello world!');` }} />
    ///   )
    /// }
    /// ```
    ///
    pub UseInlineScriptId {
        version: "next",
        name: "useInlineScriptId",
        language: "jsx",
        sources: &[RuleSource::EslintNext("inline-script-id").same()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Next],
    }
}

impl Rule for UseInlineScriptId {
    type Query = Semantic<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseInlineScriptIdOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let jsx_element = ctx.query();

        let semantic_model = ctx.model();
        let reference = jsx_element.name().ok()?;
        let reference = reference.as_jsx_reference_identifier()?;
        let binding = semantic_model.binding(reference)?;
        if !is_next_import(&binding, NextUtility::Script) {
            return None;
        }

        let mut attribute_names = FxHashSet::default();
        for attribute in jsx_element.attributes() {
            match attribute {
                AnyJsxAttribute::JsxAttribute(a) => {
                    if let Ok(name_value) = a.name_value_token() {
                        let name = name_value.token_text();
                        attribute_names.insert(name.to_string());
                    }
                }
                AnyJsxAttribute::JsxSpreadAttribute(spread) => {
                    let argument = spread.argument().ok()?;
                    match argument {
                        AnyJsExpression::JsObjectExpression(obj_expr) => {
                            collect_property_names(&obj_expr, &mut attribute_names)?;
                        }
                        AnyJsExpression::JsIdentifierExpression(ident_expr) => {
                            if let Some(reference) = ident_expr.name().ok()
                                && let Some(binding) = semantic_model.binding(&reference)
                                && let Some(declarator) = binding
                                    .syntax()
                                    .ancestors()
                                    .find_map(JsVariableDeclarator::cast)
                                && let Some(initializer) = declarator.initializer()
                                && let Some(expression) = initializer.expression().ok()
                                && let AnyJsExpression::JsObjectExpression(obj_expr) = expression
                            {
                                collect_property_names(&obj_expr, &mut attribute_names)?;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        let has_children = jsx_element
            .parent::<JsxElement>()
            .is_some_and(|parent| !parent.children().is_empty());
        if (has_children || attribute_names.contains("dangerouslySetInnerHTML"))
            && !attribute_names.contains("id")
        {
            return Some(jsx_element.syntax().text_range_with_trivia());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    ""<Emphasis>"next/script"</Emphasis>" components with inline content or `dangerouslySetInnerHTML` must specify "<Emphasis>"id"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "See the "<Hyperlink href="https://nextjs.org/docs/messages/inline-script-id">"Next.js docs"</Hyperlink>" for more details."
            })
        )
    }
}

fn collect_property_names(
    obj_expr: &JsObjectExpression,
    set: &mut FxHashSet<String>,
) -> Option<()> {
    for member in obj_expr.members() {
        let member = member.ok()?;
        if let Some(property_member) = member.as_js_property_object_member()
            && let Some(name) = property_member.name().ok().and_then(|n| n.name())
        {
            set.insert(name.to_string());
        } else if let Some(shorthand) = member.as_js_shorthand_property_object_member()
            && let Some(name) = shorthand.name().ok().and_then(|n| n.name().ok())
        {
            set.insert(name.to_string());
        }
    }
    Some(())
}
