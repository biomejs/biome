use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    HtmlElement, HtmlSyntaxKind, HtmlSyntaxNode, T, element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_misplaced_list_elements::NoMisplacedListElementsOptions;

declare_lint_rule! {
    /// Require `<li>` elements with an HTML element parent to be children of `<ul>`, `<ol>`, or `<menu>`.
    ///
    /// List items need a list container to define their relationship to the other items.
    /// Placing a list item outside a list container produces invalid HTML.
    ///
    /// The parent is the element the item is rendered into. List items that aren't inside
    /// any element in the file are ignored, because the file may be rendered inside a list
    /// somewhere else, for example as a component. Items directly inside a `<template>` element are also
    /// ignored, because the template's content is inserted by a script, not rendered in place.
    /// Items whose parent is a component are also ignored,
    /// because the element a component renders is unknown.
    ///
    /// In Svelte, control blocks such as `{#if}` and `{#each}`, as well as `<svelte:boundary>`,
    /// `<svelte:fragment>`, and `<slot>`, are skipped when finding the parent, while items
    /// inside snippets, `<svelte:element>`, and other Svelte special elements are ignored.
    ///
    /// In Vue, `<template>` elements with a `v-if`, `v-else-if`, `v-else`, or `v-for`
    /// directive, `<transition>`, `<keep-alive>`, `<suspense>`, and `<slot>` are looked
    /// through to find the parent. Items directly inside the component's top-level
    /// `<template>`, and items inside `<component>`, `<transition-group>`, or `<teleport>`,
    /// are ignored.
    ///
    /// In Astro, fragments and `<slot>` are looked through to find the parent. Markup inside
    /// a `{...}` expression is checked separately from the markup around it, so an item in
    /// an expression is only compared with elements in the same expression. For example,
    /// `<div>{items.map((item) => <li>{item}</li>)}</div>` isn't reported, because the
    /// `<div>` is outside the expression.
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
    /// ```svelte,expect_diagnostic
    /// <div>
    ///     {#each items as item}
    ///         <li>{item}</li>
    ///     {/each}
    /// </div>
    /// ```
    ///
    /// ```astro,expect_diagnostic
    /// <div>
    ///     <>
    ///         <li>Item</li>
    ///     </>
    /// </div>
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
    /// ```vue
    /// <template>
    ///     <li>Item rendered in a list elsewhere</li>
    /// </template>
    /// ```
    ///
    /// ```svelte
    /// {#snippet item()}
    ///     <li>Item</li>
    /// {/snippet}
    /// ```
    ///
    /// ```astro
    /// <ul>
    ///     <>
    ///         <li>Item</li>
    ///     </>
    /// </ul>
    /// ```
    ///
    pub NoMisplacedListElements {
        version: "2.5.15",
        name: "noMisplacedListElements",
        language: "html",
        recommended: true,
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

impl Rule for NoMisplacedListElements {
    type Query = Ast<AnyHtmlTagElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoMisplacedListElementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.tag_name_kind() != Some(T![li]) {
            return None;
        }

        let element = match node {
            AnyHtmlTagElement::HtmlOpeningElement(opening) => opening.syntax().parent()?,
            AnyHtmlTagElement::HtmlSelfClosingElement(element) => element.syntax().clone(),
        };
        let parent = rendered_parent(&element, ctx.source_type::<HtmlFileSource>())?;
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

/// Returns the opening tag of the element that `element` is rendered into.
///
/// Returns `None` when the element has no parent element, or when the parent does not
/// determine where its children are rendered. See [`ParentRole`].
fn rendered_parent(
    element: &HtmlSyntaxNode,
    source_type: &HtmlFileSource,
) -> Option<AnyHtmlTagElement> {
    for ancestor in element.ancestors().skip(1) {
        let kind = ancestor.kind();
        if kind == HtmlSyntaxKind::HTML_ELEMENT_LIST
            || kind == HtmlSyntaxKind::ASTRO_FRAGMENT
            || (source_type.is_svelte() && SVELTE_CONTROL_BLOCK_KINDS.contains(kind))
        {
            continue;
        }

        let parent = HtmlElement::cast(ancestor)?;
        let opening = AnyHtmlTagElement::from(parent.opening_element().ok()?);
        match ParentRole::of(&opening, source_type) {
            ParentRole::Element => return Some(opening),
            ParentRole::Transparent => {}
            ParentRole::Unknown => return None,
        }
    }
    None
}

/// How a parent element relates to the element its children are rendered into.
enum ParentRole {
    /// The parent is the element its children are rendered into.
    Element,
    /// The parent renders its children in its own place, so they are rendered into the
    /// nearest enclosing element.
    Transparent,
    /// The element the children are rendered into can't be determined.
    Unknown,
}

impl ParentRole {
    fn of(opening: &AnyHtmlTagElement, source_type: &HtmlFileSource) -> Self {
        if source_type.is_svelte() && opening.is_svelte_special_element() {
            return match opening.tag_name().as_ref().map(|name| name.text()) {
                Some("svelte:boundary" | "svelte:fragment") => Self::Transparent,
                _ => Self::Unknown,
            };
        }

        if source_type.is_vue() {
            match opening.tag_name().as_ref().map(|name| name.text()) {
                Some("transition" | "keep-alive" | "suspense") => return Self::Transparent,
                Some("component" | "transition-group" | "teleport") => return Self::Unknown,
                _ => {}
            }
        }

        match opening.tag_name_kind() {
            // Vue renders the content of a `<template>` with a control flow directive in
            // place of the template.
            Some(T![template])
                if source_type.is_vue() && has_vue_control_flow_directive(opening) =>
            {
                Self::Transparent
            }
            // The content of a native `<template>` is inserted by a script, which decides
            // where it is rendered.
            Some(T![template]) => Self::Unknown,
            Some(T![slot])
                if source_type.is_vue() || source_type.is_svelte() || source_type.is_astro() =>
            {
                Self::Transparent
            }
            Some(_) => Self::Element,
            None => Self::Unknown,
        }
    }
}

fn has_vue_control_flow_directive(opening: &AnyHtmlTagElement) -> bool {
    opening.attributes().into_iter().any(|attribute| {
        attribute
            .as_any_vue_directive()
            .and_then(|directive| directive.as_vue_directive())
            .and_then(|directive| directive.name_token().ok())
            .is_some_and(|name| {
                matches!(
                    name.text_trimmed(),
                    "v-if" | "v-else-if" | "v-else" | "v-for"
                )
            })
    })
}
