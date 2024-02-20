use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsxTag, JsSyntaxToken, JsxElement, JsxOpeningElementFields, T};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPiece};

use crate::JsRuleAction;

declare_rule! {
    /// Prevent extra closing tags for components without children
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <div></div>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <Component></Component>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <Foo.bar></Foo.bar>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <div />
    ///```
    ///
    /// ```js
    /// <div>child</div>
    ///```
    ///
    /// ```js
    /// <Component />
    ///```
    ///
    /// ```js
    /// <Component>child</Component>
    ///```
    ///
    /// ```js
    /// <Foo.bar />
    ///```
    ///
    /// ```js
    /// <Foo.bar>child</Foo.bar>
    ///```
    pub UseSelfClosingElements {
        version: "1.0.0",
        name: "useSelfClosingElements",
        source: RuleSource::EslintStylistic("jsx-self-closing-comp"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseSelfClosingElements {
    type Query = Ast<JsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        if ctx.query().children().is_empty() {
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
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a SelfClosingElement instead" }.to_owned(),
            mutation,
        })
    }
}
