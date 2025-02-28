use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsxTag, JsSyntaxToken, JsxElement, JsxOpeningElementFields, T};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPiece};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Prevent extra closing tags for components without children
    ///
    /// JSX elements without children should be marked as self-closing. In JSX, it is valid for any element to be self-closing.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div></div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <Component></Component>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <Foo.bar></Foo.bar>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div />
    ///```
    ///
    /// ```jsx
    /// <div>child</div>
    ///```
    ///
    /// ```jsx
    /// <Component />
    ///```
    ///
    /// ```jsx
    /// <Component>child</Component>
    ///```
    ///
    /// ```jsx
    /// <Foo.bar />
    ///```
    ///
    /// ```jsx
    /// <Foo.bar>child</Foo.bar>
    ///```
    ///
    /// ## Options
    ///
    /// ### `ignoreHtmlElements`
    ///
    /// **Since version 2.0.0**.
    ///
    /// Default: `false`
    ///
    /// This option allows you to specify whether to ignore checking native HTML elements.
    ///
    /// In the following example, when the option is set to "true", it will not self close native HTML elements.
    ///
    /// ```json
    /// {
    ///     "//":"...",
    ///     "options": {
    ///         "ignoreHtmlElements": true
    ///     }
    /// }
    /// ```
    ///
    /// ```jsx,ignore
    /// <div></div>
    /// ```
    ///
    ///
    pub UseSelfClosingElements {
        version: "1.0.0",
        name: "useSelfClosingElements",
        language: "js",
        sources: &[RuleSource::EslintStylistic("jsx-self-closing-comp")],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSelfClosingElements {
    type Query = Ast<JsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = Box<UseSelfClosingElementsOptions>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let is_html_element = node
            .opening_element()
            .is_ok_and(|node| node.name().is_ok_and(|name| name.as_jsx_name().is_some()));

        if node.children().is_empty() && !(ctx.options().ignore_html_elements && is_html_element) {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "JSX elements without children should be marked as self-closing. In JSX, it is valid for any element to be self-closing."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let open_element = ctx.query().opening_element().ok()?;
        let JsxOpeningElementFields {
            l_angle_token,
            name,
            type_arguments,
            attributes,
            r_angle_token,
        } = open_element.as_fields();
        let mut r_angle_token = r_angle_token.ok()?;
        let mut leading_trivia = vec![];
        let mut slash_token = String::new();

        for trivia in r_angle_token.leading_trivia().pieces() {
            leading_trivia.push(TriviaPiece::new(trivia.kind(), trivia.text_len()));
            slash_token.push_str(trivia.text());
        }
        // check if previous `open_element` have a whitespace before `>`
        // this step make sure we could convert <div></div> -> <div />
        // <div test="some""></div> -> <div test="some" />
        let prev_token = r_angle_token.prev_token();
        let need_extra_whitespace = prev_token
            .as_ref()
            .map_or(true, |token| !token.trailing_trivia().text().ends_with(' '));

        // drop the leading trivia of `r_angle_token`
        r_angle_token = r_angle_token.with_leading_trivia([]);

        if leading_trivia.is_empty() && need_extra_whitespace {
            slash_token.push(' ');
            leading_trivia.push(TriviaPiece::whitespace(1));
        }

        slash_token += "/";

        let mut self_closing_element_builder = make::jsx_self_closing_element(
            l_angle_token.ok()?,
            name.ok()?,
            attributes,
            JsSyntaxToken::new_detached(T![/], &slash_token, leading_trivia, []),
            r_angle_token,
        );
        if let Some(type_arguments) = type_arguments {
            self_closing_element_builder =
                self_closing_element_builder.with_type_arguments(type_arguments);
        }
        let self_closing_element = self_closing_element_builder.build();
        mutation.replace_node(
            AnyJsxTag::JsxElement(ctx.query().clone()),
            AnyJsxTag::JsxSelfClosingElement(self_closing_element),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use a self-closing element instead." }.to_owned(),
            mutation,
        ))
    }
}

/// Options for the `useSelfClosingElements` rule.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSelfClosingElementsOptions {
    // Whether or not to ignore checking native HTML elements. Default is false.
    pub ignore_html_elements: bool,
}
