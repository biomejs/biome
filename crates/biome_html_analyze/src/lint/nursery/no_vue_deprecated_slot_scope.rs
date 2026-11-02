use crate::HtmlRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlContent, AnyHtmlElement, AnyHtmlTagName, AnyVueDirective,
    AnyVueDirectiveArgument, HtmlAttribute, HtmlAttributeInitializerClause, HtmlAttributeList,
    HtmlElement, HtmlElementList, HtmlSyntaxKind, HtmlSyntaxToken, T, element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::no_vue_deprecated_slot_scope::NoVueDeprecatedSlotScopeOptions;

declare_lint_rule! {
    /// Disallow the deprecated `slot-scope` attribute in Vue templates.
    ///
    /// Vue 2.6 replaced `slot-scope` with the `v-slot` directive, and Vue 3 removed
    /// `slot-scope`. In Vue 3, the variables that `slot-scope` declares are never
    /// set, so the slot content can't read the values the component passes to it.
    ///
    /// Use `v-slot` on a `<template>` element inside the component instead. You can
    /// also write `v-slot` as `#`, for example `#default="props"`.
    ///
    /// The fix replaces `slot-scope` with `v-slot` when the result is valid. If
    /// `slot-scope` is on an element other than `<template>`, the fix wraps that
    /// element in a `<template v-slot>`. There is no fix when the element also has a
    /// `slot` attribute or a `v-for` directive, or when the component has other
    /// content for its default slot.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <ListComponent>
    ///     <template slot-scope="props">
    ///       {{ props.title }}
    ///     </template>
    ///   </ListComponent>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <ListComponent>
    ///     <span slot-scope="props">{{ props.title }}</span>
    ///   </ListComponent>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <ListComponent>
    ///     <template v-slot="props">
    ///       {{ props.title }}
    ///     </template>
    ///   </ListComponent>
    ///   <ListComponent>
    ///     <template #item="{ title }">
    ///       {{ title }}
    ///     </template>
    ///   </ListComponent>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue scoped slots](https://vuejs.org/guide/components/slots.html#scoped-slots)
    /// - [Vue `v-slot` directive](https://vuejs.org/api/built-in-directives.html#v-slot)
    pub NoVueDeprecatedSlotScope {
        version: "2.6.5",
        name: "noVueDeprecatedSlotScope",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-slot-scope-attribute").same()],
    }
}

impl Rule for NoVueDeprecatedSlotScope {
    type Query = Ast<HtmlAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedSlotScopeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }
        let name = ctx.query().name().ok()?.value_token().ok()?;
        (name.text_trimmed() == "slot-scope").then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The "<Emphasis>"slot-scope"</Emphasis>" attribute is deprecated."
                },
            )
            .note(markup! {
                "Vue 3 doesn't support "<Emphasis>"slot-scope"</Emphasis>", so the variables it declares are never set."
            })
            .note(markup! {
                "Use the "<Emphasis>"v-slot"</Emphasis>" directive on a "<Emphasis>"<template>"</Emphasis>" element instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let attribute = ctx.query();
        let tag = attribute
            .parent::<HtmlAttributeList>()?
            .parent::<AnyHtmlTagElement>()?;
        let element = match &tag {
            AnyHtmlTagElement::HtmlOpeningElement(opening) => {
                AnyHtmlElement::HtmlElement(opening.parent::<HtmlElement>()?)
            }
            AnyHtmlTagElement::HtmlSelfClosingElement(element) => {
                AnyHtmlElement::HtmlSelfClosingElement(element.clone())
            }
        };
        let is_template = tag.tag_name_kind() == Some(T![template]);
        if !can_convert_to_v_slot(&tag, &element, is_template) {
            return None;
        }

        let mut mutation = ctx.root().begin();
        if is_template {
            mutation.replace_node(
                AnyHtmlAttribute::HtmlAttribute(attribute.clone()),
                v_slot_directive(attribute.initializer()),
            );
            return Some(HtmlRuleAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                ctx.metadata().applicability(),
                markup! {
                    "Replace "<Emphasis>"slot-scope"</Emphasis>" with "<Emphasis>"v-slot"</Emphasis>"."
                }
                .to_owned(),
                mutation,
            ));
        }

        let inner = without_attribute(&tag, attribute)?;
        let inner = match (element.clone(), inner) {
            (AnyHtmlElement::HtmlElement(element), AnyHtmlTagElement::HtmlOpeningElement(opening)) => {
                AnyHtmlElement::HtmlElement(make::html_element(
                    opening,
                    element.children(),
                    element.closing_element().ok()?,
                ))
            }
            (_, AnyHtmlTagElement::HtmlSelfClosingElement(element)) => {
                AnyHtmlElement::HtmlSelfClosingElement(element)
            }
            _ => return None,
        }
        .trim_leading_trivia()?;

        let initializer = attribute
            .initializer()
            .and_then(HtmlAttributeInitializerClause::trim_trailing_trivia);
        let opening = make::html_opening_element(
            make::token(T![<]),
            template_tag_name(
                make::token(T![template]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            ),
            make::html_attribute_list([v_slot_directive(initializer)]),
            make::token(T![>]),
        );
        let closing = make::html_closing_element(
            make::token(T![<]),
            make::token(T![/]),
            template_tag_name(make::token(T![template])),
            make::token(T![>]),
        );
        let wrapper = make::html_element(opening, make::html_element_list([inner]), closing);
        mutation.replace_node(element, AnyHtmlElement::HtmlElement(wrapper));

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Wrap the element in a "<Emphasis>"<template>"</Emphasis>" with "<Emphasis>"v-slot"</Emphasis>"."
            }
            .to_owned(),
            mutation,
        ))
    }
}

/// Returns `true` if replacing `slot-scope` with `v-slot` on `tag`, or wrapping
/// `element` in a `<template v-slot>`, gives a valid template.
///
/// `v-slot` only works on a `<template>` that is a direct child of a component,
/// and a component accepts only one `v-slot` for each slot. `v-slot` without a
/// name targets the default slot, so the conversion is refused when any other
/// child of the component already provides content for the default slot.
fn can_convert_to_v_slot(tag: &AnyHtmlTagElement, element: &AnyHtmlElement, is_template: bool) -> bool {
    let attributes = tag.attributes();
    if attributes.find_attribute_or_vue_binding("slot").is_some()
        || attributes
            .iter()
            .any(|attribute| is_v_slot(&attribute) || has_directive(&attribute, "v-for"))
    {
        return false;
    }
    // Wrapping an element in a `<template>` would separate it from the other
    // branches of its `v-if` chain.
    if !is_template
        && attributes.iter().any(|attribute| {
            CONDITIONAL_DIRECTIVES
                .iter()
                .any(|name| has_directive(&attribute, name))
        })
    {
        return false;
    }

    let Some(siblings) = element.parent::<HtmlElementList>() else {
        return false;
    };
    let Some(owner) = siblings
        .parent::<HtmlElement>()
        .and_then(|owner| owner.opening_element().ok())
    else {
        return false;
    };
    // Component names such as `<MyList>` have no tag kind, and names such as
    // `<my-list>` aren't known HTML tags.
    let owner_is_component = owner.name().is_ok_and(|name| {
        name.tag_name_kind()
            .is_none_or(|kind| kind == HtmlSyntaxKind::HTML_UNKNOWN_TAG)
    });
    if !owner_is_component || owner.attributes().iter().any(|attribute| is_v_slot(&attribute)) {
        return false;
    }

    let Some(target_chain) = conditional_chains(&siblings)
        .find(|(_, child)| child == element)
        .map(|(chain, _)| chain)
    else {
        return false;
    };
    !conditional_chains(&siblings)
        .any(|(chain, child)| chain != target_chain && fills_default_slot(&child))
}

/// Pairs each child of `children`, except whitespace, with the index of the
/// `v-if` / `v-else-if` / `v-else` chain it belongs to. Children outside a
/// chain get an index of their own.
fn conditional_chains(
    children: &HtmlElementList,
) -> impl Iterator<Item = (usize, AnyHtmlElement)> + '_ {
    let mut chain = 0;
    let mut is_chain_open = false;
    children
        .iter()
        .filter(|child| !is_whitespace(child))
        .map(move |child| {
            let attributes = child.attributes();
            let has = |name| {
                attributes
                    .as_ref()
                    .is_some_and(|list| list.iter().any(|attribute| has_directive(&attribute, name)))
            };
            let continues_chain = has("v-else-if") || has("v-else");
            if !(continues_chain && is_chain_open) {
                chain += 1;
            }
            is_chain_open = has("v-if") || has("v-else-if");
            (chain, child)
        })
}

/// Returns `true` if `child` renders into the default slot of its parent
/// component: text, an element without a slot name, or an element whose slot
/// name is `default`.
fn fills_default_slot(child: &AnyHtmlElement) -> bool {
    let Some(tag) = child.clone().as_any_html_tag_element() else {
        return true;
    };
    let attributes = tag.attributes();
    if let Some(slot) = attributes.find_attribute_by_name("slot") {
        return slot
            .as_html_attribute()
            .and_then(HtmlAttribute::as_static_value)
            .is_none_or(|value| value.text() == "default");
    }
    if attributes.find_vue_binding("slot").is_some() {
        return false;
    }
    let slot_name = attributes.iter().find_map(|attribute| {
        match attribute.as_any_vue_directive()? {
            AnyVueDirective::VueVSlotShorthandDirective(directive) => Some(directive.arg().ok()),
            AnyVueDirective::VueDirective(directive) if is_v_slot(&attribute) => {
                Some(directive.arg().and_then(|argument| argument.arg()))
            }
            _ => None,
        }
    });
    match slot_name {
        // An element without `v-slot`, or with `v-slot` and no slot name.
        None | Some(None) => true,
        Some(Some(AnyVueDirectiveArgument::VueStaticArgument(name))) => name
            .name_token()
            .is_ok_and(|name| name.text_trimmed() == "default"),
        Some(Some(_)) => false,
    }
}

/// Returns `true` if `child` is text made only of whitespace.
fn is_whitespace(child: &AnyHtmlElement) -> bool {
    matches!(
        child,
        AnyHtmlElement::AnyHtmlContent(AnyHtmlContent::HtmlContent(content))
            if content.value_token().is_ok_and(|token| token.text_trimmed().trim().is_empty())
    )
}

/// Returns `true` if `attribute` is a `v-slot` directive, written in full or as `#`.
fn is_v_slot(attribute: &AnyHtmlAttribute) -> bool {
    matches!(
        attribute.as_any_vue_directive(),
        Some(AnyVueDirective::VueVSlotShorthandDirective(_))
    ) || has_directive(attribute, "v-slot")
}

/// Returns `true` if `attribute` is the Vue directive `name`, such as `v-if`.
fn has_directive(attribute: &AnyHtmlAttribute, name: &str) -> bool {
    attribute
        .as_any_vue_directive()
        .and_then(AnyVueDirective::as_vue_directive)
        .and_then(|directive| directive.name_token().ok())
        .is_some_and(|token| token.text_trimmed() == name)
}

/// Builds a `v-slot` directive with the value of the `slot-scope` attribute.
fn v_slot_directive(initializer: Option<HtmlAttributeInitializerClause>) -> AnyHtmlAttribute {
    let mut directive = make::vue_directive(make::ident("v-slot"), make::vue_modifier_list([]));
    if let Some(initializer) = initializer {
        directive = directive.with_initializer(initializer);
    }
    AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueDirective(directive.build()))
}

fn template_tag_name(token: HtmlSyntaxToken) -> AnyHtmlTagName {
    AnyHtmlTagName::HtmlTagName(make::html_tag_name(token))
}

/// Returns a copy of `tag` without `attribute`. The whitespace that followed
/// `attribute` replaces the whitespace that preceded it, so `<a slot-scope="x" />`
/// becomes `<a />`.
fn without_attribute(tag: &AnyHtmlTagElement, attribute: &HtmlAttribute) -> Option<AnyHtmlTagElement> {
    let trailing = attribute.syntax().last_token()?.trailing_trivia();
    let mut name = tag.name().ok()?;
    let mut attributes: Vec<_> = tag.attributes().iter().collect();
    let index = attributes
        .iter()
        .position(|candidate| candidate.syntax() == attribute.syntax())?;
    attributes.remove(index);
    match index.checked_sub(1) {
        Some(previous) => {
            attributes[previous] = attributes[previous]
                .clone()
                .with_trailing_trivia_pieces(trailing.pieces())?;
        }
        None => name = name.with_trailing_trivia_pieces(trailing.pieces())?,
    }
    let attributes = make::html_attribute_list(attributes);
    Some(match tag {
        AnyHtmlTagElement::HtmlOpeningElement(opening) => {
            AnyHtmlTagElement::HtmlOpeningElement(make::html_opening_element(
                opening.l_angle_token().ok()?,
                name,
                attributes,
                opening.r_angle_token().ok()?,
            ))
        }
        AnyHtmlTagElement::HtmlSelfClosingElement(element) => {
            let mut builder = make::html_self_closing_element(
                element.l_angle_token().ok()?,
                name,
                attributes,
                element.r_angle_token().ok()?,
            );
            if let Some(slash) = element.slash_token() {
                builder = builder.with_slash_token(slash);
            }
            AnyHtmlTagElement::HtmlSelfClosingElement(builder.build())
        }
    })
}

/// The directives that make an element a branch of a conditional chain.
const CONDITIONAL_DIRECTIVES: &[&str] = &["v-if", "v-else-if", "v-else"];
