use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnyHtmlElement, AnyVueDirective,
    AnyVueDirectiveArgument, HtmlAttributeInitializerClause, HtmlAttributeList, HtmlElement,
    HtmlSyntaxKind, T, VueDirective, VueModifierList, VueVSlotShorthandDirective,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstNodeList, TextRange, TokenText, declare_node_union};
use biome_rule_options::use_vue_valid_v_slot::UseVueValidVSlotOptions;

declare_lint_rule! {
    /// Enforce valid `v-slot` directives in Vue templates.
    ///
    /// A `v-slot` directive (or its `#` shorthand) passes content to a slot of a component.
    /// Vue only accepts it in specific places, and a misplaced or duplicated `v-slot`
    /// is either rejected when the template is compiled or silently drops content.
    ///
    /// This rule reports `v-slot` directives in the following cases:
    ///
    /// - The directive isn't on a component, or on a `<template>` placed directly inside a component.
    ///   For example, `<div v-slot="props"></div>`.
    /// - A named slot is written on the component itself instead of on a `<template>` inside it.
    ///   For example, `<MyComponent v-slot:header></MyComponent>`.
    /// - The default slot is written on the component itself while the component also has
    ///   `<template>` elements with `v-slot`.
    /// - An element has more than one `v-slot` directive.
    /// - Two `<template>` elements inside the same component fill the same slot.
    ///   Elements in the same `v-if`, `v-else-if`, `v-else` sequence are allowed to fill the same slot,
    ///   because only one of them is shown at a time.
    /// - A `<template>` with `v-for` fills a slot whose name doesn't change between repetitions.
    /// - The directive has modifiers, such as `v-slot.foo`.
    /// - The default slot is written on the component itself without a value.
    ///   For example, `<MyComponent v-slot></MyComponent>`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-slot="{ item }">{{ item }}</div>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent v-slot:header>Title</MyComponent>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent v-slot="{ item }">
    ///     {{ item }}
    ///     <template #header>Title</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent>
    ///     <template #header>First title</template>
    ///     <template #header>Second title</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent>
    ///     <template v-for="item in items" #header>{{ item }}</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent v-slot>Content</MyComponent>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <MyComponent v-slot="{ item }">{{ item }}</MyComponent>
    ///   <MyComponent>
    ///     <template #default="{ item }">{{ item }}</template>
    ///     <template #header>Title</template>
    ///   </MyComponent>
    ///   <MyComponent>
    ///     <template v-if="isAdmin" #header>Admin title</template>
    ///     <template v-else #header>Title</template>
    ///   </MyComponent>
    ///   <MyComponent>
    ///     <template v-for="item in items" #[item.slot]>{{ item.text }}</template>
    ///   </MyComponent>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue slots](https://vuejs.org/guide/components/slots.html)
    /// - [Vue `v-slot` directive](https://vuejs.org/api/built-in-directives.html#v-slot)
    pub UseVueValidVSlot {
        version: "next",
        name: "useVueValidVSlot",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-slot").same()],
    }
}

declare_node_union! {
    pub AnyVueSlotDirective = VueDirective | VueVSlotShorthandDirective
}

pub enum ViolationKind {
    /// The element that receives the slot content isn't a component.
    OwnerNotComponent(AnyHtmlTagElement),
    NamedSlotOnComponent,
    DefaultSlotOnComponentWithTemplates,
    MultipleSlotsOnElement,
    /// The range of the earlier `v-slot` directive that fills the same slot.
    DuplicateSlot(TextRange),
    SameSlotInVFor,
    Modifiers,
    MissingValue,
}

impl Rule for UseVueValidVSlot {
    type Query = Ast<AnyVueSlotDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVSlotOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let directive = ctx.query();
        if !directive.is_v_slot() {
            return None;
        }
        let name = SlotName::from_directive(directive)?;
        let element = directive
            .syntax()
            .grand_parent()
            .and_then(AnyHtmlTagElement::cast)?;
        let element_node = html_element(&element)?;
        let is_template = element.tag_name_kind() == Some(T![template]);

        // A `<template>` hands its content to the element it is placed in. A
        // `<template>` at the top level of the file has no such element.
        let owner = if is_template {
            Some(
                element_node
                    .syntax()
                    .grand_parent()
                    .and_then(HtmlElement::cast)?,
            )
        } else {
            None
        };
        let owner_tag = match &owner {
            Some(owner) => owner.opening_element().ok()?.into(),
            None => element.clone(),
        };

        if !is_component(&owner_tag) {
            return Some(ViolationKind::OwnerNotComponent(owner_tag));
        }

        if !is_template && !name.is_default() {
            return Some(ViolationKind::NamedSlotOnComponent);
        }

        if !is_template
            && let Some(element) = element_node.as_html_element()
            && has_template_slot_children(element)
        {
            return Some(ViolationKind::DefaultSlotOnComponentWithTemplates);
        }

        if slot_directives(&element.attributes())
            .next()
            .is_some_and(|first| &first != directive)
        {
            return Some(ViolationKind::MultipleSlotsOnElement);
        }

        if let Some(owner) = owner {
            let has_v_for = find_directive(&element.attributes(), "v-for").is_some();
            // A dynamic slot name inside `v-for` can change between repetitions
            // when it uses the `v-for` variables. Template expressions aren't
            // analyzed here, so these slots are never reported as duplicates.
            if !(has_v_for && name.is_dynamic()) {
                if let Some(range) = find_earlier_duplicate(&owner, &element_node, directive, &name)
                {
                    return Some(ViolationKind::DuplicateSlot(range));
                }
                if has_v_for {
                    return Some(ViolationKind::SameSlotInVFor);
                }
            }
        }

        if !directive.modifiers().is_empty() {
            return Some(ViolationKind::Modifiers);
        }

        if !is_template
            && name.is_default()
            && directive
                .initializer()
                .is_none_or(|initializer| is_empty_value(&initializer))
        {
            return Some(ViolationKind::MissingValue);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let directive = ctx.query();
        let range = directive.range();
        Some(match state {
            ViolationKind::OwnerNotComponent(owner) => {
                let owner_name = owner.tag_name()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "The "<Emphasis>"v-slot"</Emphasis>" directive must be used on a component, but "<Emphasis>"<"{owner_name.text()}">"</Emphasis>" is not a component."
                    },
                )
                .note(markup! {
                    "Slots belong to components. Vue rejects "<Emphasis>"v-slot"</Emphasis>" anywhere else."
                })
                .note(markup! {
                    "Move the directive to a component, or to a "<Emphasis>"<template>"</Emphasis>" placed directly inside a component."
                })
            }
            ViolationKind::NamedSlotOnComponent => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Named slots must use a "<Emphasis>"<template>"</Emphasis>" inside the component."
                },
            )
            .note(markup! {
                "Only the default slot is meant to be written on the component itself. A named slot there is easy to mistake for the default slot."
            })
            .note(markup! {
                "Wrap the content in a "<Emphasis>"<template>"</Emphasis>" with this directive, and place it inside the component."
            }),
            ViolationKind::DefaultSlotOnComponentWithTemplates => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The default slot must use a "<Emphasis>"<template>"</Emphasis>" when the component has other "<Emphasis>"<template>"</Emphasis>" slots."
                },
            )
            .note(markup! {
                "Vue doesn't allow "<Emphasis>"v-slot"</Emphasis>" on the component itself when "<Emphasis>"<template>"</Emphasis>" elements inside it also use "<Emphasis>"v-slot"</Emphasis>"."
            })
            .note(markup! {
                "Move the default slot content into a "<Emphasis>"<template #default>"</Emphasis>" inside the component."
            }),
            ViolationKind::MultipleSlotsOnElement => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "An element can have only one "<Emphasis>"v-slot"</Emphasis>" directive."
                },
            )
            .note(markup! {
                "Each element fills a single slot, so Vue ignores the extra directives."
            })
            .note(markup! {
                "Remove this directive, or move it to its own "<Emphasis>"<template>"</Emphasis>" inside the component."
            }),
            ViolationKind::DuplicateSlot(first_range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This slot is already filled by another "<Emphasis>"<template>"</Emphasis>" in the same component."
                },
            )
            .detail(first_range, markup! { "The slot is first filled here." })
            .note(markup! {
                "A slot can receive content from only one "<Emphasis>"<template>"</Emphasis>"."
            })
            .note(markup! {
                "Remove one of the "<Emphasis>"<template>"</Emphasis>" elements, or give it another slot name. If only one of them should be shown at a time, use "<Emphasis>"v-if"</Emphasis>" and "<Emphasis>"v-else"</Emphasis>"."
            }),
            ViolationKind::SameSlotInVFor => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"<template>"</Emphasis>" is repeated with "<Emphasis>"v-for"</Emphasis>", but every repetition fills the same slot."
                },
            )
            .note(markup! {
                "A slot can receive content from only one "<Emphasis>"<template>"</Emphasis>", so only one of the repetitions is shown."
            })
            .note(markup! {
                "Use a slot name that changes with each item, such as "<Emphasis>"#[item.name]"</Emphasis>", or remove "<Emphasis>"v-for"</Emphasis>"."
            }),
            ViolationKind::Modifiers => {
                let modifiers = directive.modifiers();
                let first = modifiers.first()?;
                let last = modifiers.last()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    first.range().cover(last.range()),
                    markup! {
                        "The "<Emphasis>"v-slot"</Emphasis>" directive doesn't support modifiers."
                    },
                )
                .note(markup! {
                    <Emphasis>"v-slot"</Emphasis>" has no modifiers, so Vue either ignores the text after the dot or reads it as part of the slot name."
                })
                .note(markup! {
                    "Remove the modifiers."
                })
            }
            ViolationKind::MissingValue => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The "<Emphasis>"v-slot"</Emphasis>" directive on a component needs a value."
                },
            )
            .note(markup! {
                "Without a value, the directive has no effect: the content of the component already goes to its default slot."
            })
            .note(markup! {
                "Add the slot props you want to use, such as "<Emphasis>"v-slot=\"{ item }\""</Emphasis>", or remove the directive."
            }),
        })
    }
}

impl AnyVueSlotDirective {
    fn is_v_slot(&self) -> bool {
        match self {
            Self::VueDirective(directive) => directive
                .name_token()
                .is_ok_and(|name| name.text_trimmed() == "v-slot"),
            Self::VueVSlotShorthandDirective(_) => true,
        }
    }

    fn modifiers(&self) -> VueModifierList {
        match self {
            Self::VueDirective(directive) => directive.modifiers(),
            Self::VueVSlotShorthandDirective(directive) => directive.modifiers(),
        }
    }

    fn initializer(&self) -> Option<HtmlAttributeInitializerClause> {
        match self {
            Self::VueDirective(directive) => directive.initializer(),
            Self::VueVSlotShorthandDirective(directive) => directive.initializer(),
        }
    }
}

/// The slot a `v-slot` directive fills, without its modifiers.
#[derive(PartialEq, Eq)]
enum SlotName {
    /// `v-slot`, `v-slot:default`, or `#default`.
    Default,
    /// `#header` holds `header`.
    Static(TokenText),
    /// `#[name]` holds `name`.
    Dynamic(TokenText),
}

impl SlotName {
    /// Returns `None` when the argument can't be read, as in a `#` without a name.
    fn from_directive(directive: &AnyVueSlotDirective) -> Option<Self> {
        let argument = match directive {
            AnyVueSlotDirective::VueDirective(directive) => {
                directive.arg().and_then(|argument| argument.arg())
            }
            AnyVueSlotDirective::VueVSlotShorthandDirective(directive) => {
                Some(directive.arg().ok()?)
            }
        };
        Some(match argument {
            None => Self::Default,
            Some(AnyVueDirectiveArgument::VueStaticArgument(argument)) => {
                let name = argument.name_token().ok()?.token_text_trimmed();
                if name.text() == "default" {
                    Self::Default
                } else {
                    Self::Static(name)
                }
            }
            Some(AnyVueDirectiveArgument::VueDynamicArgument(argument)) => {
                Self::Dynamic(argument.name_token().ok()?.token_text_trimmed())
            }
            Some(AnyVueDirectiveArgument::VueBogusDirectiveArgument(_)) => return None,
        })
    }

    fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }

    fn is_dynamic(&self) -> bool {
        matches!(self, Self::Dynamic(_))
    }
}

/// Tag names that Vue resolves as components even though they are written in
/// lowercase: `<component>` and tags that aren't known HTML or SVG elements.
const COMPONENT_TAG_KINDS: TokenSet<HtmlSyntaxKind> =
    token_set![T![component], HtmlSyntaxKind::HTML_UNKNOWN_TAG];

fn is_component(element: &AnyHtmlTagElement) -> bool {
    element.is_custom_component()
        || element
            .tag_name_kind()
            .is_some_and(|kind| COMPONENT_TAG_KINDS.contains(kind))
        || element.find_attribute_or_vue_binding("is").is_some()
}

/// Returns the element as it appears in the children of its parent element:
/// the `HtmlElement` of an opening tag, or the self-closing element itself.
fn html_element(element: &AnyHtmlTagElement) -> Option<AnyHtmlElement> {
    match element {
        AnyHtmlTagElement::HtmlOpeningElement(opening) => {
            opening.parent::<HtmlElement>().map(AnyHtmlElement::from)
        }
        AnyHtmlTagElement::HtmlSelfClosingElement(element) => Some(element.clone().into()),
    }
}

fn slot_directives(attributes: &HtmlAttributeList) -> impl Iterator<Item = AnyVueSlotDirective> {
    attributes.iter().filter_map(|attribute| match attribute {
        AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueDirective(directive)) => {
            Some(AnyVueSlotDirective::from(directive)).filter(AnyVueSlotDirective::is_v_slot)
        }
        AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueVSlotShorthandDirective(
            directive,
        )) => Some(directive.into()),
        _ => None,
    })
}

fn find_directive(attributes: &HtmlAttributeList, name: &str) -> Option<VueDirective> {
    attributes.iter().find_map(|attribute| match attribute {
        AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueDirective(directive))
            if directive
                .name_token()
                .is_ok_and(|token| token.text_trimmed() == name) =>
        {
            Some(directive)
        }
        _ => None,
    })
}

/// Returns the attributes of `child` when it is a `<template>` element.
fn template_attributes(child: AnyHtmlElement) -> Option<HtmlAttributeList> {
    let tag = child.as_any_html_tag_element()?;
    (tag.tag_name_kind() == Some(T![template])).then(|| tag.attributes())
}

fn has_template_slot_children(element: &HtmlElement) -> bool {
    element.children().into_iter().any(|child| {
        template_attributes(child)
            .is_some_and(|attributes| slot_directives(&attributes).next().is_some())
    })
}

/// Finds a `<template>` before `element` inside `owner` that fills the same
/// slot as `directive`, and returns the range of its `v-slot` directive.
///
/// Elements of the same `v-if`, `v-else-if`, `v-else` sequence are never
/// duplicates of each other, because only one of them is shown.
fn find_earlier_duplicate(
    owner: &HtmlElement,
    element: &AnyHtmlElement,
    directive: &AnyVueSlotDirective,
    name: &SlotName,
) -> Option<TextRange> {
    let modifiers = directive.modifiers();
    let mut first_match = None;
    for child in owner.children() {
        let Some(tag) = child.clone().as_any_html_tag_element() else {
            continue;
        };
        let attributes = tag.attributes();
        let continues_sequence = find_directive(&attributes, "v-else-if").is_some()
            || find_directive(&attributes, "v-else").is_some();
        if &child == element {
            return first_match
                .filter(|(_, in_sequence)| !(continues_sequence && *in_sequence))
                .map(|(range, _)| range);
        }
        match &mut first_match {
            // The match stays in the same sequence as `element` only while
            // every element in between continues that sequence.
            Some((_, in_sequence)) => *in_sequence &= continues_sequence,
            None => {
                if tag.tag_name_kind() != Some(T![template]) {
                    continue;
                }
                let has_v_for = find_directive(&attributes, "v-for").is_some();
                first_match = slot_directives(&attributes)
                    .find(|other| {
                        SlotName::from_directive(other).is_some_and(|other_name| {
                            other_name == *name
                                && !(has_v_for && other_name.is_dynamic())
                                && same_modifiers(&other.modifiers(), &modifiers)
                        })
                    })
                    .map(|other| (other.range(), true));
            }
        }
    }
    None
}

fn same_modifiers(left: &VueModifierList, right: &VueModifierList) -> bool {
    left.len() == right.len()
        && left.iter().zip(right.iter()).all(|(left, right)| {
            match (left.modifier_token(), right.modifier_token()) {
                (Ok(left), Ok(right)) => left.text_trimmed() == right.text_trimmed(),
                _ => false,
            }
        })
}

/// Returns `true` for `v-slot=""` and for values that contain only whitespace.
fn is_empty_value(initializer: &HtmlAttributeInitializerClause) -> bool {
    match initializer.value() {
        Ok(AnyHtmlAttributeInitializer::HtmlString(value)) => value
            .inner_string_text()
            .is_ok_and(|text| text.text().trim().is_empty()),
        _ => false,
    }
}
