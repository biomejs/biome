use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyVueDirective, AnyVueDirectiveArgument, HtmlSyntaxKind, HtmlSyntaxToken, VueModifierList,
    element_ext::AnyHtmlTagElement,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_vue_v_on_exact::UseVueVOnExactOptions;
use enumflags2::{BitFlags, bitflags};

declare_lint_rule! {
    /// Enforce the `.exact` modifier on `v-on` directives when another listener for the same event needs more modifier keys.
    ///
    /// In Vue, a listener with modifier keys such as `.ctrl` or `.shift` runs when those keys are
    /// held down, even if other keys are held down too. A listener without modifier keys runs no
    /// matter which keys are held down. So when an element has both `@click="select"` and
    /// `@click.ctrl="open"`, a Ctrl+click runs both `select` and `open`.
    ///
    /// The [`.exact` modifier](https://vuejs.org/guide/essentials/event-handling.html#exact-modifier)
    /// makes a listener run only when exactly its modifier keys are held down. This rule reports
    /// `v-on` directives that also run for the modifier keys of another listener for the same
    /// event on the same element.
    ///
    /// The modifier keys are `.ctrl`, `.shift`, `.alt` and `.meta`.
    ///
    /// Components, such as `<MyComponent>`, `<component>` and `<tr is="vue:my-row">`, are ignored,
    /// because their listeners usually handle events emitted by the component.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <button @click="select" @click.ctrl="open"></button>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <input @keydown.enter="submit" @keydown.enter.shift="addLine" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <button @click.exact="select" @click.ctrl="open"></button>
    ///   <input @keydown.enter.exact="submit" @keydown.enter.shift="addLine" />
    ///   <button @click="select" @click.stop="track"></button>
    ///   <button @click="select"></button>
    ///   <button @click.ctrl="open"></button>
    /// </template>
    /// ```
    ///
    pub UseVueVOnExact {
        version: "2.6.3",
        name: "useVueVOnExact",
        language: "html",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("use-v-on-exact").same()],
    }
}

/// A `v-on` directive that runs for the modifier keys of another listener for the same event.
pub struct ConflictingListener {
    /// The directive name, argument and modifiers of the listener that needs `.exact`.
    listener: TextRange,
    /// The directive name, argument and modifiers of the listener with more modifier keys.
    other: TextRange,
}

impl Rule for UseVueVOnExact {
    type Query = Ast<AnyHtmlTagElement>;
    type State = ConflictingListener;
    type Signals = Box<[Self::State]>;
    type Options = UseVueVOnExactOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        if is_component(element) {
            return Box::default();
        }

        let listeners: Vec<_> = element
            .attributes()
            .iter()
            .filter_map(|attribute| EventListener::new(attribute.as_any_vue_directive()?))
            .collect();
        if listeners
            .iter()
            .all(|listener| listener.modifier_keys.is_empty())
        {
            return Box::default();
        }

        listeners
            .iter()
            .enumerate()
            .filter_map(|(index, listener)| {
                let other = listeners
                    .iter()
                    .enumerate()
                    .find_map(|(other_index, other)| {
                        (other_index != index && listener.conflicts_with(other)).then_some(other)
                    })?;
                Some(ConflictingListener {
                    listener: listener.key_range,
                    other: other.key_range,
                })
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.listener,
                markup! {
                    "This event listener also runs when the modifier keys of another listener are held down."
                },
            )
            .detail(
                state.other,
                markup! {
                    "When these modifier keys are held down, both listeners run."
                },
            )
            .note(markup! {
                "Add the "<Emphasis>".exact"</Emphasis>" modifier so this listener only runs when no other modifier keys are held down."
            }),
        )
    }
}

/// Returns `true` if Vue renders the element as a component instead of a native element.
///
/// Tags that aren't known HTML or SVG elements, such as `<card>`, `<component>` or `<MyComponent>`,
/// are components. A native element becomes a component when its `is` attribute starts with
/// `vue:`, as in `<tr is="vue:my-row">`.
fn is_component(element: &AnyHtmlTagElement) -> bool {
    element
        .tag_name_kind()
        .is_none_or(|kind| COMPONENT_TAG_NAMES.contains(kind))
        || element
            .find_attribute_by_name("is")
            .and_then(|attribute| attribute.as_html_attribute()?.initializer()?.value().ok())
            .and_then(|value| value.as_static_value())
            .is_some_and(|value| value.text().starts_with("vue:"))
}

/// Tag name kinds that Vue resolves to a component: `<component>` and tags that aren't known HTML
/// or SVG elements.
const COMPONENT_TAG_NAMES: TokenSet<HtmlSyntaxKind> = token_set![
    HtmlSyntaxKind::COMPONENT_KW,
    HtmlSyntaxKind::HTML_UNKNOWN_TAG
];

/// A `v-on` directive with an event name, along with the facts about its modifiers that the
/// rule compares.
struct EventListener {
    /// The range of the directive name, argument and modifiers, without the value.
    key_range: TextRange,
    /// The token holding the event name, such as `click` in `@click` or `event` in `@[event]`.
    event_name: HtmlSyntaxToken,
    /// Whether the event name is dynamic, as in `@[event]`.
    is_dynamic: bool,
    /// The event that Vue listens to instead of `click` when the listener has the `.right` or
    /// `.middle` modifier.
    renamed_click: Option<&'static str>,
    modifiers: VueModifierList,
    modifier_keys: BitFlags<ModifierKey>,
    has_exact: bool,
    /// Whether every modifier is an event modifier, such as `.stop` or `.once`.
    has_only_event_modifiers: bool,
    has_stop: bool,
    has_handler: bool,
}

impl EventListener {
    fn new(directive: &AnyVueDirective) -> Option<Self> {
        let (argument, modifiers, has_handler) = match directive {
            AnyVueDirective::VueDirective(directive) if directive.is_event_listener() => (
                directive.arg()?.arg()?,
                directive.modifiers(),
                directive.initializer().is_some(),
            ),
            AnyVueDirective::VueVOnShorthandDirective(directive) => (
                directive.arg().ok()?,
                directive.modifiers(),
                directive.initializer().is_some(),
            ),
            _ => return None,
        };
        let (event_name, is_dynamic) = match &argument {
            AnyVueDirectiveArgument::VueStaticArgument(argument) => {
                (argument.name_token().ok()?, false)
            }
            AnyVueDirectiveArgument::VueDynamicArgument(argument) => {
                (argument.name_token().ok()?, true)
            }
            AnyVueDirectiveArgument::VueBogusDirectiveArgument(_) => return None,
        };
        let key_end = modifiers
            .last()
            .map_or_else(|| argument.range().end(), |modifier| modifier.range().end());

        let mut modifier_keys = BitFlags::empty();
        let mut has_exact = false;
        let mut has_only_event_modifiers = true;
        let mut has_stop = false;
        let mut has_right = false;
        let mut has_middle = false;
        for modifier in modifiers.iter() {
            let Ok(token) = modifier.modifier_token() else {
                continue;
            };
            match ModifierKind::from_name(token.text_trimmed()) {
                Some(ModifierKind::Key(key)) => {
                    modifier_keys |= key;
                    has_only_event_modifiers = false;
                }
                Some(ModifierKind::Event { is_stop }) => has_stop |= is_stop,
                Some(ModifierKind::Exact) => {
                    has_exact = true;
                    has_only_event_modifiers = false;
                }
                None => {
                    has_only_event_modifiers = false;
                    has_right |= token.text_trimmed() == "right";
                    has_middle |= token.text_trimmed() == "middle";
                }
            }
        }
        let renamed_click =
            if is_dynamic || !event_name.text_trimmed().eq_ignore_ascii_case("click") {
                None
            } else if has_right {
                Some("contextmenu")
            } else if has_middle {
                Some("mouseup")
            } else {
                None
            };

        Some(Self {
            key_range: TextRange::new(directive.range().start(), key_end),
            event_name,
            is_dynamic,
            renamed_click,
            modifiers,
            modifier_keys,
            has_exact,
            has_only_event_modifiers,
            has_stop,
            has_handler,
        })
    }

    /// Returns the name of the event that Vue listens to.
    fn event(&self) -> &str {
        self.renamed_click
            .unwrap_or_else(|| self.event_name.text_trimmed())
    }

    /// Returns `true` if this listener runs whenever `other` runs, because `other` listens to the
    /// same event and needs more modifier keys, and this listener has no `.exact` modifier.
    fn conflicts_with(&self, other: &Self) -> bool {
        if self.has_exact || self.is_dynamic != other.is_dynamic || self.event() != other.event() {
            return false;
        }
        // `@keydown.stop` stops the propagation of every key event, and an `.exact` listener
        // next to it is expected to still run.
        if other.has_exact && self.is_handlerless_stop_on_key_event() {
            return false;
        }
        // Listeners for different keys, such as `@keydown.enter` and `@keydown.ctrl.tab`, never
        // run for the same key press.
        if has_different_other_modifiers(&self.modifiers, &other.modifiers) {
            return false;
        }
        other.modifier_keys != self.modifier_keys
            && other.modifier_keys.contains(self.modifier_keys)
    }

    fn is_handlerless_stop_on_key_event(&self) -> bool {
        !self.is_dynamic
            && matches!(self.event(), "keydown" | "keypress" | "keyup")
            && self.has_stop
            && self.has_only_event_modifiers
            && !self.has_handler
    }
}

/// A modifier key of a `v-on` directive.
#[derive(Clone, Copy)]
#[bitflags]
#[repr(u8)]
enum ModifierKey {
    Alt = 1 << 0,
    Ctrl = 1 << 1,
    Meta = 1 << 2,
    Shift = 1 << 3,
}

#[derive(Clone, Copy)]
enum ModifierKind {
    /// A modifier key, such as `.ctrl`.
    Key(ModifierKey),
    /// A modifier that changes how the event is handled, such as `.prevent` or `.once`.
    Event { is_stop: bool },
    /// The `.exact` modifier.
    Exact,
}

impl ModifierKind {
    /// Returns the kind of a modifier key, an event modifier or `.exact`. Returns `None` for any
    /// other modifier, such as `.enter` or `.left`.
    fn from_name(name: &str) -> Option<Self> {
        MODIFIER_KINDS
            .binary_search_by(|(modifier, _)| modifier.cmp(&name))
            .ok()
            .map(|index| MODIFIER_KINDS[index].1)
    }
}

/// Modifier keys, event modifiers and `.exact`, sorted by name.
const MODIFIER_KINDS: &[(&str, ModifierKind)] = &[
    ("alt", ModifierKind::Key(ModifierKey::Alt)),
    ("capture", ModifierKind::Event { is_stop: false }),
    ("ctrl", ModifierKind::Key(ModifierKey::Ctrl)),
    ("exact", ModifierKind::Exact),
    ("meta", ModifierKind::Key(ModifierKey::Meta)),
    ("once", ModifierKind::Event { is_stop: false }),
    ("passive", ModifierKind::Event { is_stop: false }),
    ("prevent", ModifierKind::Event { is_stop: false }),
    ("self", ModifierKind::Event { is_stop: false }),
    ("shift", ModifierKind::Key(ModifierKey::Shift)),
    ("stop", ModifierKind::Event { is_stop: true }),
];

/// Returns `true` if both lists have modifiers with no [`ModifierKind`], such as `.enter`, and
/// those modifiers differ between the lists.
fn has_different_other_modifiers(left: &VueModifierList, right: &VueModifierList) -> bool {
    let left_count = other_modifier_names(left).count();
    let right_count = other_modifier_names(right).count();
    if left_count == 0 || right_count == 0 {
        return false;
    }
    left_count != right_count
        || other_modifier_names(left).any(|name| {
            other_modifier_names(left)
                .filter(|other| other.text_trimmed() == name.text_trimmed())
                .count()
                != other_modifier_names(right)
                    .filter(|other| other.text_trimmed() == name.text_trimmed())
                    .count()
        })
}

fn other_modifier_names(modifiers: &VueModifierList) -> impl Iterator<Item = HtmlSyntaxToken> {
    modifiers.iter().filter_map(|modifier| {
        let token = modifier.modifier_token().ok()?;
        ModifierKind::from_name(token.text_trimmed())
            .is_none()
            .then_some(token)
    })
}

#[cfg(test)]
mod tests {
    use super::MODIFIER_KINDS;

    #[test]
    fn modifier_kinds_are_sorted() {
        for window in MODIFIER_KINDS.windows(2) {
            assert!(
                window[0].0 < window[1].0,
                "{} should come after {}",
                window[0].0,
                window[1].0
            );
        }
    }
}
