use crate::CssRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_css_factory::make::css_declaration;
use biome_css_syntax::{CssDeclaration, CssDeclarationImportant};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_important_styles::NoImportantStylesOptions;

declare_lint_rule! {
    /// Disallow `!important` declarations.
    ///
    /// The CSS cascade normally chooses between conflicting declarations by considering factors
    /// such as cascade layers, selector specificity, and source order. `!important` raises a
    /// declaration's priority and can make those conflicts harder to predict, override, and debug.
    /// Prefer adjusting the cascade, selectors, or declaration order.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// .style {
    ///     color: red !important;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// .style {
    ///     color: red;
    /// }
    /// ```
    ///
    /// ## Useful links
    ///
    /// - [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/important)
    pub NoImportantStyles {
        version: "2.0.0",
        name: "noImportantStyles",
        language: "css",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::Stylelint("declaration-no-important").same(), RuleSource::EslintCss("no-important").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoImportantStyles {
    type Query = Ast<CssDeclarationImportant>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoImportantStylesOptions;

    fn run(_: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Avoid the use of the "<Emphasis>"!important"</Emphasis>" style."
                },
            )
            .note("This style reverses the cascade logic, and precedence is reversed. This could lead to having styles with higher specificity being overridden by styles with lower specificity.")

        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<CssRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let declaration = node.syntax().parent().and_then(CssDeclaration::cast)?;
        let mut declaration_property = declaration.property().ok()?;

        if let Some(leading_trivia) = node.syntax().first_leading_trivia() {
            declaration_property = declaration_property
                .clone()
                .with_trailing_trivia_pieces(leading_trivia.pieces())?
                .with_leading_trivia_pieces(
                    declaration_property
                        .syntax()
                        .first_leading_trivia()?
                        .pieces(),
                )?;
        }

        if let Some(trailing_trivia) = node.syntax().last_trailing_trivia() {
            declaration_property = declaration_property
                .clone()
                .with_leading_trivia_pieces(
                    declaration_property
                        .syntax()
                        .first_leading_trivia()?
                        .pieces(),
                )?
                .with_trailing_trivia_pieces(trailing_trivia.pieces())?;
        }

        let new_declaration = css_declaration(declaration_property).build();

        mutation.replace_node_discard_trivia(declaration, new_declaration);

        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the style." }.to_owned(),
            mutation,
        ))
    }
}
