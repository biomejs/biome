use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    HtmlElement, HtmlSyntaxKind, T, element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_li_container::UseLiContainerOptions;

declare_lint_rule! {
    /// Require `<li>` elements with an HTML element parent to be children of `<ul>`, `<ol>`, or `<menu>`.
    ///
    /// List items need a list container to define their relationship to the other items.
    /// Placing a list item outside a list container produces invalid HTML.
    ///
    /// List items without an HTML element parent are ignored, including root-level items,
    /// Svelte snippets, and Astro fragments. Their container may be supplied where they
    /// are rendered. Svelte control blocks use their surrounding HTML element as the
    /// parent. Parents that are components or Svelte special elements are also ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div><li>Item</li></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <ul><div><li>Item</li></div></ul>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <ul><li>Item</li></ul>
    /// <ol><li>Item</li></ol>
    /// <menu><li>Item</li></menu>
    /// ```
    ///
    /// ```html
    /// <li>Item rendered in a list elsewhere</li>
    /// ```
    ///
    /// ```svelte
    /// {#snippet item()}
    ///     <li>Item</li>
    /// {/snippet}
    /// ```
    ///
    pub UseLiContainer {
        version: "next",
        name: "useLiContainer",
        language: "html",
        recommended: false,
        severity: Severity::Error,
        sources: &[RuleSource::HtmlEslint("require-li-container").inspired()],
    }
}

const LIST_CONTAINERS: TokenSet<HtmlSyntaxKind> = token_set![T![ul], T![ol], T![menu]];
const SVELTE_CONTROL_BLOCK_KINDS: TokenSet<HtmlSyntaxKind> = token_set![
    HtmlSyntaxKind::SVELTE_IF_BLOCK,
    HtmlSyntaxKind::SVELTE_IF_OPENING_BLOCK,
    HtmlSyntaxKind::SVELTE_ELSE_IF_CLAUSE_LIST,
    HtmlSyntaxKind::SVELTE_ELSE_IF_CLAUSE,
    HtmlSyntaxKind::SVELTE_ELSE_CLAUSE,
    HtmlSyntaxKind::SVELTE_EACH_BLOCK,
    HtmlSyntaxKind::SVELTE_AWAIT_BLOCK,
    HtmlSyntaxKind::SVELTE_AWAIT_OPENING_BLOCK,
    HtmlSyntaxKind::SVELTE_AWAIT_THEN_BLOCK,
    HtmlSyntaxKind::SVELTE_AWAIT_CATCH_BLOCK,
    HtmlSyntaxKind::SVELTE_AWAIT_CLAUSES_LIST,
    HtmlSyntaxKind::SVELTE_KEY_BLOCK,
];

impl Rule for UseLiContainer {
    type Query = Ast<AnyHtmlTagElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseLiContainerOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.tag_name_kind() != Some(T![li]) {
            return None;
        }

        let element = match node {
            AnyHtmlTagElement::HtmlOpeningElement(opening) => opening.syntax().parent()?,
            AnyHtmlTagElement::HtmlSelfClosingElement(element) => element.syntax().clone(),
        };
        let is_svelte = ctx.source_type::<HtmlFileSource>().is_svelte();
        let mut ancestors = element.ancestors().skip(1);
        let parent = loop {
            let ancestor = ancestors.next()?;
            if let Some(parent) = HtmlElement::cast(ancestor.clone()) {
                break parent;
            }
            if ancestor.kind() != HtmlSyntaxKind::HTML_ELEMENT_LIST
                && (!is_svelte || !SVELTE_CONTROL_BLOCK_KINDS.contains(ancestor.kind()))
            {
                return None;
            }
        };
        let parent = AnyHtmlTagElement::from(parent.opening_element().ok()?);
        if is_svelte && parent.is_svelte_special_element() {
            return None;
        }

        let parent_kind = parent.tag_name_kind()?;
        if LIST_CONTAINERS.contains(parent_kind) {
            return None;
        }

        Some(node.syntax().text_trimmed_range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "This "<Emphasis>"<li>"</Emphasis>" element is outside a list container."
                },
            )
            .note("List items need a list container to form a valid HTML list.")
            .note(markup! {
                "Make this element a direct child of "<Emphasis>"<ul>"</Emphasis>", "<Emphasis>"<ol>"</Emphasis>", or "<Emphasis>"<menu>"</Emphasis>"."
            }),
        )
    }
}
