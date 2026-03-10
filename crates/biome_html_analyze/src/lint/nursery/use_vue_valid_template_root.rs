use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{HtmlElement, HtmlRoot};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::use_vue_valid_template_root::UseVueValidTemplateRootOptions;

declare_lint_rule! {
    /// Enforce valid Vue `<template>` root usage.
    ///
    /// This rule reports only root-level `<template>` elements. If the
    /// `<template>` has a `src` attribute, the element must be empty. Otherwise,
    /// the element must contain content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template src="./foo.html">content</template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template></template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>content</template>
    /// ```
    ///
    /// ```vue
    /// <template src="./foo.html"></template>
    /// ```
    ///
    pub UseVueValidTemplateRoot {
        version: "2.3.11",
        name: "useVueValidTemplateRoot",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-template-root").same()],
        fix_kind: FixKind::Unsafe,
    }
}

pub enum ViolationKind {
    MustBeEmpty(HtmlElement),
    MustHaveContent(HtmlElement),
}

impl Rule for UseVueValidTemplateRoot {
    type Query = Ast<HtmlRoot>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidTemplateRootOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let root = ctx.query();
        // Find top-level `<template>` elements only
        let element = root
            .html()
            .into_iter()
            .filter_map(|el| HtmlElement::cast(el.into_syntax()))
            .find(|el| {
                el.opening_element()
                    .ok()
                    .and_then(|op| op.name().ok())
                    .and_then(|name| name.token_text_trimmed())
                    .is_some_and(|text| text == "template")
            })?;

        let has_src = element.find_attribute_by_name("src").is_some();
        let has_non_whitespace_content = !element.children().is_empty();

        if has_src {
            if has_non_whitespace_content {
                return Some(ViolationKind::MustBeEmpty(element));
            }
        } else if !has_non_whitespace_content {
            return Some(ViolationKind::MustHaveContent(element));
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(match state {
            ViolationKind::MustBeEmpty(el) => RuleDiagnostic::new(
                rule_category!(),
                el.range(),
                markup! {
                    "The root `<template>` with a " <Emphasis>"src"</Emphasis> " attribute must be empty."
                },
            )
            .note(markup! {
                "The src attribute indicates that the content is loaded from an external file."
            })
            .note(markup! {
                "Remove content when using the " <Emphasis>"src"</Emphasis> " attribute."
            }),
            ViolationKind::MustHaveContent(el) => RuleDiagnostic::new(
                rule_category!(),
                el.range(),
                markup! {
                    "The root `<template>` is empty."
                },
            )
            .note(markup! {
                "The root `<template>` must contain content when no " <Emphasis>"src"</Emphasis> " attribute is present."
            })
            .note(markup! {
                "Add content inside the `<template>` or use the " <Emphasis>"src"</Emphasis> " attribute."
            }),
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<crate::HtmlRuleAction> {
        match state {
            // Unsafe fix: remove the content when `src` is present
            ViolationKind::MustBeEmpty(el) => {
                let mut mutation = BatchMutationExt::begin(ctx.root());
                mutation.remove_node(el.children());
                Some(biome_analyze::RuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove inline content from `<template>`." }.to_owned(),
                    mutation,
                ))
            }
            ViolationKind::MustHaveContent(_el) => None,
        }
    }
}
