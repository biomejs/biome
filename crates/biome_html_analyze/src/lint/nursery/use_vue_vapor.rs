use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyHtmlAttribute, HtmlAttributeList, HtmlOpeningElement, HtmlSyntaxKind, HtmlSyntaxToken,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPiece};
use biome_rule_options::use_vue_vapor::UseVueVaporOptions;

declare_lint_rule! {
    /// Enforce opting in to Vue Vapor mode in `<script setup>` blocks.
    ///
    /// Vue 3.6 introduces an opt-in “Vapor mode” for SFC `<script setup>` blocks:
    /// `<script setup vapor>`.
    ///
    /// Vapor mode only works for Vue Single File Components (SFCs) using `<script setup>`.
    ///
    /// This rule reports `<script setup>` opening tags that are missing the `vapor` attribute.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup vapor>
    /// </script>
    /// ```
    ///
    /// ## Related Rules
    ///
    /// - [noVueOptionsApi](https://biomejs.dev/linter/rules/no-vue-options-api): Disallows the Options API format, which is incompatible with Vapor Mode
    ///
    pub UseVueVapor {
        version: "2.3.11",
        name: "useVueVapor",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseVueVapor {
    type Query = Ast<HtmlOpeningElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseVueVaporOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let opening = ctx.query();

        let name = opening.name().ok()?;
        let name_text = name.token_text_trimmed()?;
        if !name_text.eq_ignore_ascii_case("script") {
            return None;
        }

        let attributes = opening.attributes();
        attributes.find_by_name("setup")?;

        if attributes.find_by_name("vapor").is_some() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"<script setup>"</Emphasis>" is missing the "<Emphasis>"vapor"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "Add "<Emphasis>"vapor"</Emphasis>" to opt in to Vue Vapor mode: "<Emphasis>"<script setup vapor>"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<crate::HtmlRuleAction> {
        let opening = ctx.query();
        let old_attributes = opening.attributes();

        // Only apply the fix for <script setup> that doesn't already have vapor.
        if old_attributes.find_by_name("setup").is_none()
            || old_attributes.find_by_name("vapor").is_some()
        {
            return None;
        }

        let new_attributes = insert_after_setup(old_attributes)?;

        let mut mutation = BatchMutationExt::begin(ctx.root());
        mutation.replace_node(opening.attributes(), new_attributes);

        Some(biome_analyze::RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add the "<Emphasis>"vapor"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Clone, Copy, Debug)]
enum VaporAttributeSpacing {
    /// Add a leading space before `vapor` (used when `setup` is the last attribute).
    Leading,
    /// Add a trailing space after `vapor` (used when there are more attributes after `setup`).
    Trailing,
}

fn make_vapor_attribute(spacing: VaporAttributeSpacing) -> AnyHtmlAttribute {
    let vapor_token = match spacing {
        VaporAttributeSpacing::Leading => HtmlSyntaxToken::new_detached(
            HtmlSyntaxKind::IDENT,
            " vapor",
            [TriviaPiece::whitespace(1)],
            [],
        ),
        VaporAttributeSpacing::Trailing => HtmlSyntaxToken::new_detached(
            HtmlSyntaxKind::IDENT,
            "vapor ",
            [],
            [TriviaPiece::whitespace(1)],
        ),
    };

    AnyHtmlAttribute::HtmlAttribute(
        make::html_attribute(make::html_attribute_name(vapor_token)).build(),
    )
}

fn insert_after_setup(old_attributes: HtmlAttributeList) -> Option<HtmlAttributeList> {
    let mut items: Vec<AnyHtmlAttribute> = old_attributes.iter().collect();

    let setup_index = items.iter().position(is_setup_attribute)?;

    let spacing = if setup_index + 1 == items.len() {
        VaporAttributeSpacing::Leading
    } else {
        VaporAttributeSpacing::Trailing
    };

    items.insert(setup_index + 1, make_vapor_attribute(spacing));

    Some(make::html_attribute_list(items))
}

fn is_setup_attribute(attribute: &AnyHtmlAttribute) -> bool {
    match attribute {
        AnyHtmlAttribute::HtmlAttribute(attr) => attr
            .name()
            .ok()
            .and_then(|name| name.value_token().ok())
            .is_some_and(|tok| tok.text_trimmed() == "setup"),
        _ => false,
    }
}
