use crate::{HtmlRuleAction, VueSemantic};
use biome_analyze::{FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnyVueDirective, AnyVueDirectiveArgument,
    HtmlLanguage, HtmlSyntaxKind, HtmlSyntaxToken, T, element_ext::AnyHtmlTagElement,
    static_value::StaticValue,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPiece};
use biome_rule_options::no_disabled_attribute::NoDisabledAttributeOptions;
use biome_vue_semantic::{AnalyzedVueModel, RefBase, Resolution, RootShape, Snippet};

declare_lint_rule! {
    /// Discourages the `disabled` attribute on form controls and on elements that act as buttons.
    ///
    /// This is an opinionated usability rule, not a Web Content Accessibility Guidelines (WCAG)
    /// requirement. A form control with the native `disabled` attribute can't receive keyboard
    /// focus, so people who move through a page with the Tab key skip over it. Keeping an
    /// unavailable control focusable can make it easier to discover with a keyboard or assistive
    /// technology.
    ///
    /// The rule reports the following elements when `disabled` is set:
    ///
    /// - Buttons: `button` elements, and `input` elements with the `button`, `submit`, `reset`,
    ///   or `image` type. A `button` is still reported when a role other than `button` is
    ///   assigned, because `disabled` continues to change its native behavior.
    /// - Fields that support the `readonly` attribute: `textarea` elements, and `input` elements
    ///   whose type accepts typed input, such as `text`, `email`, `number`, or `date`. An `input`
    ///   with no `type`, or with a `type` that browsers don't recognize, is a text field.
    /// - Other form controls: `select` elements, and `input` elements with the `checkbox`,
    ///   `radio`, `range`, `color`, or `file` type.
    /// - `fieldset` elements, because a disabled `fieldset` disables every form control inside it.
    /// - Built-in elements that aren't form controls and whose first valid explicit role is
    ///   `button`. The `disabled` attribute has no effect on these elements.
    ///
    /// The rule doesn't report `option` and `optgroup` elements, `input` elements with the
    /// `hidden` type, or `input` elements whose `type` is a dynamic expression or can come from
    /// a spread attribute.
    ///
    /// ## Replacing `disabled`
    ///
    /// The unsafe fix depends on the reported element:
    ///
    /// - For fields that support it, the fix replaces `disabled` with `readonly`. A read-only
    ///   field can receive focus and can't be edited. Unlike a disabled field, its value is
    ///   still submitted with the form, and browsers don't apply disabled styling to it.
    /// - For buttons, other form controls, and elements with the `button` role, the fix replaces
    ///   `disabled` with `aria-disabled`. The ARIA attribute only exposes the disabled state to
    ///   assistive technology. It does not prevent pointer or keyboard activation, change form
    ///   submission or validation, omit values from submitted data, or add disabled styling.
    ///   Implement those behaviors and provide disabled styling, including for forced-colors
    ///   mode.
    /// - A `fieldset` has no fix. Mark each form control inside it instead. Disabling a
    ///   `fieldset` can be appropriate when the whole group doesn't apply to the user, for
    ///   example the details of a second applicant when there is only one applicant. In that
    ///   case, suppress the diagnostic.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <button disabled>Save</button>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <input type="email" disabled>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <select disabled><option>Monthly</option></select>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <fieldset disabled><input type="text"></fieldset>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div role="button" disabled>Save</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <button aria-disabled="true">Save</button>
    /// <input type="email" readonly>
    /// <select aria-disabled="true"><option>Monthly</option></select>
    /// <input type="hidden" disabled>
    /// ```
    ///
    /// ## References
    ///
    /// - [MDN: `aria-disabled`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-disabled)
    /// - [MDN: `readonly`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/readonly)
    /// - [Making Disabled Buttons More Inclusive](https://css-tricks.com/making-disabled-buttons-more-inclusive/)
    pub NoDisabledAttribute {
        version: "next",
        name: "noDisabledAttribute",
        language: "html",
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    attribute: AnyHtmlAttribute,
    kind: ControlKind,
}

/// What an element with a `disabled` attribute is, which decides the advice and the fix.
#[derive(Clone, Copy)]
enum ControlKind {
    /// A `button` element, or an `input` element with a button type.
    Button,
    /// A form control that supports the `readonly` attribute.
    EditableField,
    /// A form control that supports neither button activation nor `readonly`.
    FormControl,
    Fieldset,
    /// An element that isn't a form control and has the `button` role.
    ButtonRole,
}

impl Rule for NoDisabledAttribute {
    type Query = VueSemantic<AnyHtmlTagElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoDisabledAttributeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        if element.is_custom_component()
            || element.is_svelte_special_element()
            || element.tag_name_kind() == Some(HtmlSyntaxKind::HTML_UNKNOWN_TAG)
        {
            return None;
        }

        let attribute = element.find_attribute_or_vue_binding("disabled")?;
        if is_inactive_disabled_attribute(&attribute, ctx.model())?
            || has_trailing_dynamic_spread(element, &attribute)
        {
            return None;
        }

        let kind = ControlKind::from_element(element, ctx.model())?;
        Some(RuleState { attribute, kind })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.attribute.range(),
                match state.kind {
                    ControlKind::ButtonRole => markup! {
                        "The "<Emphasis>"disabled"</Emphasis>" attribute has no effect on this element."
                    },
                    ControlKind::Button => markup! {
                        "Avoid the native "<Emphasis>"disabled"</Emphasis>" attribute on this button."
                    },
                    ControlKind::EditableField => markup! {
                        "Avoid the native "<Emphasis>"disabled"</Emphasis>" attribute on this field."
                    },
                    ControlKind::FormControl => markup! {
                        "Avoid the native "<Emphasis>"disabled"</Emphasis>" attribute on this form control."
                    },
                    ControlKind::Fieldset => markup! {
                        "Avoid the native "<Emphasis>"disabled"</Emphasis>" attribute on this fieldset."
                    },
                },
            )
            .note(match state.kind {
                ControlKind::ButtonRole => markup! {
                    "The "<Emphasis>"button"</Emphasis>" role changes accessibility semantics but does not add native button behavior."
                },
                ControlKind::Fieldset => markup! {
                    "A disabled fieldset disables every form control inside it, so none of them can be reached with the Tab key."
                },
                ControlKind::Button | ControlKind::EditableField | ControlKind::FormControl => markup! {
                    "A disabled form control can't receive keyboard focus, so it can't be reached with the Tab key."
                },
            })
            .note(match state.kind {
                ControlKind::EditableField => markup! {
                    "Replace it with "<Emphasis>"readonly"</Emphasis>", which keeps the field focusable and prevents edits. Unlike a disabled field, a read-only field is still submitted with its form and doesn't get disabled styling."
                },
                ControlKind::Fieldset => markup! {
                    "Mark each form control inside the fieldset instead: use "<Emphasis>"readonly"</Emphasis>" on fields that support it and "<Emphasis>"aria-disabled"</Emphasis>" on other controls."
                },
                ControlKind::Button | ControlKind::FormControl | ControlKind::ButtonRole => markup! {
                    "Replace it with "<Emphasis>"aria-disabled"</Emphasis>", which only exposes the disabled state to assistive technology. It does not suppress pointer or keyboard activation, form submission or validation, or apply disabled styling. Implement those behaviors, including forced-colors styling."
                },
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        // The static value is what replaces a `disabled` attribute that is always active.
        let (new_name, static_value) = match state.kind {
            ControlKind::Fieldset => return None,
            ControlKind::EditableField => ("readonly", None),
            ControlKind::Button | ControlKind::FormControl | ControlKind::ButtonRole => {
                ("aria-disabled", Some("\"true\""))
            }
        };
        if ctx
            .query()
            .find_attribute_or_vue_binding(new_name)
            .is_some()
        {
            return None;
        }

        let mut mutation = ctx.root().begin();
        match &state.attribute {
            AnyHtmlAttribute::HtmlAttribute(attribute) => {
                if attribute.initializer().is_some() && attribute.as_static_value().is_none() {
                    let old_name = attribute.name().ok()?.value_token().ok()?;
                    let new_name_token =
                        HtmlSyntaxToken::new_detached(old_name.kind(), new_name, [], []);
                    mutation.replace_token_transfer_trivia(old_name, new_name_token);
                } else {
                    if attribute.syntax().has_comments_descendants() {
                        return None;
                    }
                    let replacement = make_static_attribute(&state.attribute, new_name, static_value)?;
                    mutation.replace_node(
                        state.attribute.clone(),
                        AnyHtmlAttribute::HtmlAttribute(replacement),
                    );
                }
            }
            AnyHtmlAttribute::HtmlAttributeSingleTextExpression(attribute) => {
                let replacement =
                    make::html_attribute(make::html_attribute_name(HtmlSyntaxToken::new_detached(
                        HtmlSyntaxKind::HTML_LITERAL,
                        new_name,
                        [],
                        [],
                    )))
                    .with_initializer(make::html_attribute_initializer_clause(
                        make::token(T![=]),
                        AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(
                            make::html_attribute_single_text_expression(
                                attribute.l_curly_token().ok()?,
                                attribute.expression().ok()?,
                                attribute.r_curly_token().ok()?,
                            ),
                        ),
                    ))
                    .build();
                mutation.replace_node(
                    state.attribute.clone(),
                    AnyHtmlAttribute::HtmlAttribute(replacement),
                );
            }
            AnyHtmlAttribute::AnyVueDirective(directive) => match directive {
                AnyVueDirective::VueVBindShorthandDirective(directive) => {
                    if !directive.modifiers().is_empty() {
                        return None;
                    }
                    if directive.initializer().is_none() {
                        if directive.syntax().has_comments_descendants() {
                            return None;
                        }
                        let argument = directive.arg().ok()?;
                        let old_name = argument
                            .arg()?
                            .as_vue_static_argument()?
                            .name_token()
                            .ok()?;
                        let replacement = make::vue_v_bind_shorthand_directive(
                            make_vue_argument(&argument, &old_name, new_name)?,
                            directive.modifiers(),
                        )
                        .with_initializer(make_vue_same_name_initializer(&old_name))
                        .build();
                        mutation.replace_node(
                            state.attribute.clone(),
                            AnyHtmlAttribute::AnyVueDirective(
                                AnyVueDirective::VueVBindShorthandDirective(replacement),
                            ),
                        );
                    } else if is_static_active_vue_binding(&state.attribute, ctx.model()) {
                        if directive.syntax().has_comments_descendants() {
                            return None;
                        }
                        let replacement = make_static_attribute(&state.attribute, new_name, static_value)?;
                        mutation.replace_node(
                            state.attribute.clone(),
                            AnyHtmlAttribute::HtmlAttribute(replacement),
                        );
                    } else {
                        let (old_name, new_name_token) =
                            vue_argument_rename_tokens(&directive.arg().ok()?, new_name)?;
                        mutation.replace_token_transfer_trivia(old_name, new_name_token);
                    }
                }
                AnyVueDirective::VueDirective(directive) if directive.is_binding() => {
                    if !directive.modifiers().is_empty() {
                        return None;
                    }
                    if directive.initializer().is_none() {
                        if directive.syntax().has_comments_descendants() {
                            return None;
                        }
                        let argument = directive.arg()?;
                        let old_name = argument
                            .arg()?
                            .as_vue_static_argument()?
                            .name_token()
                            .ok()?;
                        let replacement = make::vue_directive(
                            directive.name_token().ok()?,
                            directive.modifiers(),
                        )
                        .with_arg(make_vue_argument(&argument, &old_name, new_name)?)
                        .with_initializer(make_vue_same_name_initializer(&old_name))
                        .build();
                        mutation.replace_node(
                            state.attribute.clone(),
                            AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueDirective(
                                replacement,
                            )),
                        );
                    } else if is_static_active_vue_binding(&state.attribute, ctx.model()) {
                        if directive.syntax().has_comments_descendants() {
                            return None;
                        }
                        let replacement = make_static_attribute(&state.attribute, new_name, static_value)?;
                        mutation.replace_node(
                            state.attribute.clone(),
                            AnyHtmlAttribute::HtmlAttribute(replacement),
                        );
                    } else {
                        let (old_name, new_name_token) =
                            vue_argument_rename_tokens(&directive.arg()?, new_name)?;
                        mutation.replace_token_transfer_trivia(old_name, new_name_token);
                    }
                }
                _ => return None,
            },
            _ => return None,
        }

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{new_name}</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

/// Creates the attribute `name`, or `name=value` when `value` is set, with the outer trivia of
/// `attribute`. `value` includes its quotes.
fn make_static_attribute(
    attribute: &AnyHtmlAttribute,
    name: &str,
    value: Option<&str>,
) -> Option<biome_html_syntax::HtmlAttribute> {
    let first_token = attribute.syntax().first_token()?;
    let last_token = attribute.syntax().last_token()?;
    let name = token_with_leading_trivia(HtmlSyntaxKind::HTML_LITERAL, name, &first_token);
    let Some(value) = value else {
        let name = name.with_trailing_trivia_pieces(last_token.trailing_trivia().pieces());
        return Some(make::html_attribute(make::html_attribute_name(name)).build());
    };
    let value = token_with_trailing_trivia(HtmlSyntaxKind::HTML_STRING_LITERAL, value, &last_token);
    Some(
        make::html_attribute(make::html_attribute_name(name))
            .with_initializer(make::html_attribute_initializer_clause(
                make::token(T![=]),
                AnyHtmlAttributeInitializer::HtmlString(make::html_string(value)),
            ))
            .build(),
    )
}

fn make_vue_argument(
    argument: &biome_html_syntax::VueDirectiveArgument,
    old_name: &HtmlSyntaxToken,
    new_name: &str,
) -> Option<biome_html_syntax::VueDirectiveArgument> {
    let new_name = token_with_leading_trivia(old_name.kind(), new_name, old_name);
    Some(
        make::vue_directive_argument(argument.colon_token().ok()?)
            .with_arg(AnyVueDirectiveArgument::VueStaticArgument(
                make::vue_static_argument(new_name),
            ))
            .build(),
    )
}

fn make_vue_same_name_initializer(
    old_name: &HtmlSyntaxToken,
) -> biome_html_syntax::HtmlAttributeInitializerClause {
    let value = token_with_trailing_trivia(
        HtmlSyntaxKind::HTML_STRING_LITERAL,
        &format!("\"{}\"", old_name.text_trimmed()),
        old_name,
    );
    make::html_attribute_initializer_clause(
        make::token(T![=]),
        AnyHtmlAttributeInitializer::HtmlString(make::html_string(value)),
    )
}

fn token_with_leading_trivia(
    kind: HtmlSyntaxKind,
    replacement: &str,
    token: &HtmlSyntaxToken,
) -> HtmlSyntaxToken {
    let leading = token.leading_trivia();
    let mut text = String::with_capacity(leading.text().len() + replacement.len());
    text.push_str(leading.text());
    text.push_str(replacement);
    HtmlSyntaxToken::new_detached(
        kind,
        &text,
        leading
            .pieces()
            .map(|piece| TriviaPiece::new(piece.kind(), piece.text_len())),
        [],
    )
}

fn token_with_trailing_trivia(
    kind: HtmlSyntaxKind,
    replacement: &str,
    token: &HtmlSyntaxToken,
) -> HtmlSyntaxToken {
    let trailing = token.trailing_trivia();
    let mut text = String::with_capacity(replacement.len() + trailing.text().len());
    text.push_str(replacement);
    text.push_str(trailing.text());
    HtmlSyntaxToken::new_detached(
        kind,
        &text,
        [],
        trailing
            .pieces()
            .map(|piece| TriviaPiece::new(piece.kind(), piece.text_len())),
    )
}

fn vue_argument_rename_tokens(
    argument: &biome_html_syntax::VueDirectiveArgument,
    new_name: &str,
) -> Option<(HtmlSyntaxToken, HtmlSyntaxToken)> {
    let old_name = argument
        .arg()?
        .as_vue_static_argument()?
        .name_token()
        .ok()?;
    let new_name = HtmlSyntaxToken::new_detached(old_name.kind(), new_name, [], []);
    Some((old_name, new_name))
}

fn is_static_active_vue_binding(
    attribute: &AnyHtmlAttribute,
    model: &AnalyzedVueModel<HtmlLanguage>,
) -> bool {
    vue_binding_snippet(attribute, model).is_some_and(|snippet| {
        matches!(snippet.root(), RootShape::Bool(true) | RootShape::String(_))
    })
}

fn vue_binding_snippet(
    attribute: &AnyHtmlAttribute,
    model: &AnalyzedVueModel<HtmlLanguage>,
) -> Option<Snippet> {
    attribute.as_any_vue_directive()?;
    model.attr_at(attribute.range())?.value_snippet()
}

fn static_attribute_text(
    attribute: &AnyHtmlAttribute,
    model: &AnalyzedVueModel<HtmlLanguage>,
) -> Option<String> {
    if attribute.as_any_vue_directive().is_some() {
        return vue_binding_snippet(attribute, model)?
            .string_value()
            .map(|value| value.into_owned());
    }

    if let AnyHtmlAttribute::HtmlAttribute(attribute) = attribute {
        let initializer = attribute.initializer()?.value().ok()?;
        if let AnyHtmlAttributeInitializer::HtmlString(string) = initializer {
            return Some(string.inner_string_text().ok()?.text().to_string());
        }
    }

    attribute
        .as_static_value()
        .map(|value| value.text().to_string())
}

impl ControlKind {
    /// Classifies an element that has a `disabled` attribute.
    ///
    /// Returns `None` for elements the rule doesn't report, and for `input` elements whose
    /// `type` isn't a static value.
    fn from_element(
        element: &AnyHtmlTagElement,
        model: &AnalyzedVueModel<HtmlLanguage>,
    ) -> Option<Self> {
        match element.tag_name_kind()? {
            T![button] => Some(Self::Button),
            T![input] => Self::from_input(element, model),
            T![textarea] => Some(Self::EditableField),
            T![select] => Some(Self::FormControl),
            T![fieldset] => Some(Self::Fieldset),
            // These elements are never reached with the Tab key, whether disabled or not.
            T![optgroup] | T![option] => None,
            _ => {
                let role_attribute = element.find_attribute_or_vue_binding("role")?;
                if has_trailing_dynamic_spread(element, &role_attribute) {
                    return None;
                }
                let role = AriaRole::from_roles(&static_attribute_text(&role_attribute, model)?)?;
                (role == AriaRole::Button).then_some(Self::ButtonRole)
            }
        }
    }

    /// Classifies an `input` element by its `type` attribute.
    fn from_input(
        element: &AnyHtmlTagElement,
        model: &AnalyzedVueModel<HtmlLanguage>,
    ) -> Option<Self> {
        let Some(type_attribute) = element.find_attribute_or_vue_binding("type") else {
            // An `input` without a `type` is a text field, unless a spread attribute sets one.
            let has_dynamic_spread = element
                .attributes()
                .into_iter()
                .any(|attribute| is_dynamic_spread(&attribute));
            return (!has_dynamic_spread).then_some(Self::EditableField);
        };
        if has_trailing_dynamic_spread(element, &type_attribute) {
            return None;
        }
        let input_type = static_attribute_text(&type_attribute, model)?;
        if input_type.eq_ignore_ascii_case("hidden") {
            // A hidden input is never focusable. `disabled` only keeps it out of the submitted data.
            None
        } else if is_one_of(&input_type, &BUTTON_INPUT_TYPES) {
            Some(Self::Button)
        } else if is_one_of(&input_type, &INPUT_TYPES_WITHOUT_READONLY) {
            Some(Self::FormControl)
        } else {
            // Browsers treat every other type, including unknown ones, as a field that
            // supports `readonly`.
            Some(Self::EditableField)
        }
    }
}

const BUTTON_INPUT_TYPES: [&str; 4] = ["button", "submit", "reset", "image"];

/// The `input` types, other than the button types and `hidden`, that don't support `readonly`.
const INPUT_TYPES_WITHOUT_READONLY: [&str; 5] = ["checkbox", "radio", "range", "color", "file"];

/// Returns `true` if `input_type` is one of `types`, ignoring ASCII case.
fn is_one_of(input_type: &str, types: &[&str]) -> bool {
    types
        .iter()
        .any(|candidate| input_type.eq_ignore_ascii_case(candidate))
}

fn is_inactive_disabled_attribute(
    attribute: &AnyHtmlAttribute,
    model: &AnalyzedVueModel<HtmlLanguage>,
) -> Option<bool> {
    if attribute.as_any_vue_directive().is_some() {
        let model_attribute = model.attr_at(attribute.range())?;
        let Some(snippet) = model_attribute.value_snippet() else {
            return Some(false);
        };
        return Some(
            matches!(snippet.root(), RootShape::Bool(false) | RootShape::Null)
                || is_global_undefined(&snippet),
        );
    }

    Some(
        matches!(
            attribute.as_static_value(),
            Some(StaticValue::Boolean(token)) if token.text_trimmed() == "false"
        ) || matches!(
            attribute.as_static_value(),
            Some(StaticValue::Null(_) | StaticValue::Undefined(_))
        ),
    )
}

fn is_global_undefined(snippet: &Snippet) -> bool {
    if !matches!(snippet.root(), RootShape::Identifier) {
        return false;
    }

    let mut references = snippet.references();
    references.next().is_some_and(|reference| {
        reference.name() == "undefined"
            && reference.base() == RefBase::Bare
            && reference.resolution() == Resolution::Global
    }) && references.next().is_none()
}

fn has_trailing_dynamic_spread(
    element: &AnyHtmlTagElement,
    current_attribute: &AnyHtmlAttribute,
) -> bool {
    element.attributes().into_iter().any(|attribute| {
        attribute.range().start() >= current_attribute.range().end()
            && is_dynamic_spread(&attribute)
    })
}

/// Returns `true` for an attribute that can set any attribute of its element: a spread
/// attribute, or a Vue `v-bind` without a static argument.
fn is_dynamic_spread(attribute: &AnyHtmlAttribute) -> bool {
    let has_static_name = attribute.name().is_some();
    match attribute {
        AnyHtmlAttribute::HtmlSpreadAttribute(_) => true,
        AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueVBindShorthandDirective(_)) => {
            !has_static_name
        }
        AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueDirective(directive)) => {
            directive.is_binding() && !has_static_name
        }
        _ => false,
    }
}
