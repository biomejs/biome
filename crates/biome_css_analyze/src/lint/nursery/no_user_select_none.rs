use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssGenericComponentValue, AnyCssGenericPropertyValueOrExpression, AnyScssExpressionItem,
    CssDeclaration, CssGenericProperty, CssSyntaxKind, decode_css_identifier, single_expression_item,
};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::no_user_select_none::NoUserSelectNoneOptions;

declare_lint_rule! {
    /// Disallow `user-select: none` and its vendor-prefixed forms.
    ///
    /// Disabling text selection can prevent users from selecting and copying text, making content harder to use. Allow selection unless an element needs to prevent it.
    ///
    /// In SCSS, the rule checks the literal `none` keyword without evaluating variables or functions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// .button {
    ///   user-select: none;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// .button {
    ///   -webkit-user-select: none;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// .text {
    ///   user-select: text;
    ///   -webkit-user-select: auto;
    /// }
    /// ```
    ///
    /// The rule checks declarations, not feature queries.
    ///
    /// ```css
    /// @supports (user-select: none) {
    ///   .button {
    ///     user-select: text;
    ///   }
    /// }
    /// ```
    ///
    pub NoUserSelectNone {
        version: "next",
        name: "noUserSelectNone",
        language: "css",
        recommended: false,
        issue_number: Some("9034"),
        sources: &[RuleSource::Stylelint("defensive-css/no-user-select-none").same()],
    }
}

impl Rule for NoUserSelectNone {
    type Query = Ast<CssGenericProperty>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoUserSelectNoneOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !node
            .syntax()
            .parent()
            .is_some_and(|parent| CssDeclaration::can_cast(parent.kind()))
            || node
                .syntax()
                .ancestors()
                .skip(1)
                .any(|ancestor| ancestor.kind() == CssSyntaxKind::CSS_SUPPORTS_FEATURE_DECLARATION)
        {
            return None;
        }

        let name = node.name().ok()?.identifier_text()?;
        let name = decode_css_identifier(name.text());
        if !name.eq_ignore_ascii_case("user-select")
            && !name.eq_ignore_ascii_case("-webkit-user-select")
            && !name.eq_ignore_ascii_case("-moz-user-select")
        {
            return None;
        }

        let value = match node.value().ok()? {
            AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(values)
                if values.len() == 1 =>
            {
                let AnyCssGenericComponentValue::AnyCssValue(value) = values.first()? else {
                    return None;
                };
                value
            }
            AnyCssGenericPropertyValueOrExpression::ScssExpression(expression) => {
                let AnyScssExpressionItem::AnyCssValue(value) = single_expression_item(&expression)?
                else {
                    return None;
                };
                value
            }
            _ => return None,
        };
        let identifier = value.identifier_text()?;
        decode_css_identifier(identifier.text())
            .eq_ignore_ascii_case("none")
            .then(|| value.range())
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "Avoid disabling text selection with "<Emphasis>"user-select: none"</Emphasis>"."
                },
            )
            .note(markup! {
                "Disabling text selection can prevent users from selecting and copying text."
            })
            .note(markup! {
                "Allow selection unless the element needs to prevent it."
            }),
        )
    }
}
