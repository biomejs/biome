use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, AnySvelteBindingProperty, AnySvelteDirectiveInitializerClause,
    AnySvelteTemplateElement, HtmlAttributeInitializerClause, SvelteBindDirective,
    SvelteClassDirective, SvelteDirectiveValue, SvelteStyleDirective,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, declare_node_union};
use biome_rule_options::use_svelte_shorthand_directive::UseSvelteShorthandDirectiveOptions;
use biome_unicode_table::is_js_ident;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce the shorthand syntax for Svelte `bind:`, `class:`, and `style:` directives.
    ///
    /// When the value of one of these directives is a variable with the same name as the directive,
    /// the value can be omitted: `bind:value={value}` is the same as `bind:value`.
    /// The shorthand avoids repeating the name and keeps templates shorter.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <input bind:value={value} />
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <div class:active={active}></div>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <div style:color={color}></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <input bind:value />
    /// <input bind:value={name} />
    /// <div class:active></div>
    /// <div class:active={isActive}></div>
    /// <div style:color></div>
    /// <div style:color="red"></div>
    /// ```
    ///
    pub UseSvelteShorthandDirective {
        version: "next",
        name: "useSvelteShorthandDirective",
        language: "html",
        domains: &[RuleDomain::Svelte],
        recommended: true,
        sources: &[RuleSource::EslintSvelte("shorthand-directive").same()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSvelteShorthandDirective {
    type Query = Ast<AnySvelteShorthandableDirective>;
    type State = HtmlAttributeInitializerClause;
    type Signals = Option<Self::State>;
    type Options = UseSvelteShorthandDirectiveOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let value = ctx.query().value().ok()?;
        let name = match value.property().ok()? {
            AnySvelteBindingProperty::SvelteName(name) => name.ident_token().ok()?,
            AnySvelteBindingProperty::SvelteLiteral(name) => name.value_token().ok()?,
            AnySvelteBindingProperty::SvelteMemberProperty(_) => return None,
        };
        let name = name.text_trimmed();
        if !is_js_ident(name) {
            return None;
        }

        let AnySvelteDirectiveInitializerClause::HtmlAttributeInitializerClause(initializer) =
            value.initializer()?
        else {
            return None;
        };
        let expression = match initializer.value().ok()? {
            AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(expression) => {
                expression
            }
            // `style:color="{color}"`
            AnyHtmlAttributeInitializer::SvelteTemplateAttributeValue(template) => {
                let mut elements = template.elements().iter();
                match (elements.next(), elements.next()) {
                    (
                        Some(AnySvelteTemplateElement::HtmlAttributeSingleTextExpression(
                            expression,
                        )),
                        None,
                    ) => expression,
                    _ => return None,
                }
            }
            _ => return None,
        };
        let expression = expression.expression().ok()?.html_literal_token().ok()?;

        // `name` is a valid identifier, so a matching expression is exactly a
        // reference to the variable with the same name.
        (expression.text_trimmed().trim() == name).then_some(initializer)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This directive can use the shorthand syntax."
                },
            )
            .note(markup! {
                "The value is a variable with the same name as the directive, so repeating it is redundant."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node_keep_trivia(state.clone());
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the shorthand syntax." }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    pub AnySvelteShorthandableDirective =
        SvelteBindDirective
        | SvelteClassDirective
        | SvelteStyleDirective
}

impl AnySvelteShorthandableDirective {
    fn value(&self) -> biome_rowan::SyntaxResult<SvelteDirectiveValue> {
        match self {
            Self::SvelteBindDirective(directive) => directive.value(),
            Self::SvelteClassDirective(directive) => directive.value(),
            Self::SvelteStyleDirective(directive) => directive.value(),
        }
    }
}
