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
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPiece};
use biome_rule_options::no_disabled_button::NoDisabledButtonOptions;
use biome_vue_semantic::{AnalyzedVueModel, RefBase, Resolution, RootShape, Snippet};

declare_lint_rule! {
    /// Discourages `disabled` on elements that act as buttons.
    ///
    /// This is an opinionated usability rule, not a Web Content Accessibility Guidelines (WCAG)
    /// requirement. Native `disabled` controls are removed from sequential focus navigation and
    /// suppress activation. Keeping an unavailable button focusable can make it easier to discover
    /// with a keyboard or assistive technology.
    ///
    /// The rule reports native `button` elements, `input` elements with the `button`, `submit`,
    /// `reset`, or `image` type, and built-in elements whose first valid explicit role is
    /// `button`. A native `button` is still reported when another role is assigned because
    /// `disabled` continues to change its native behavior.
    ///
    /// The unsafe fix replaces `disabled` with `aria-disabled`. The ARIA attribute only exposes
    /// the disabled state to assistive technology. It does not prevent pointer or keyboard
    /// activation, change form submission or validation, omit values from submitted data, or add
    /// disabled styling. Implement those behaviors and provide disabled styling, including for
    /// forced-colors mode.
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
    /// <div role="button" disabled>Save</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <button aria-disabled="true">Save</button>
    /// ```
    ///
    /// ## References
    ///
    /// - [MDN: `aria-disabled`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-disabled)
    /// - [Making Disabled Buttons More Inclusive](https://css-tricks.com/making-disabled-buttons-more-inclusive/)
    pub NoDisabledButton {
        version: "next",
        name: "noDisabledButton",
        language: "html",
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    attribute: AnyHtmlAttribute,
    kind: ButtonKind,
}

#[derive(Clone, Copy)]
enum ButtonKind {
    Native,
    Role,
}

const NATIVE_DISABLED_CONTROLS: TokenSet<HtmlSyntaxKind> = token_set![
    T![input],
    T![select],
    T![textarea],
    T![fieldset],
    T![optgroup],
    T![option]
];

impl Rule for NoDisabledButton {
    type Query = VueSemantic<AnyHtmlTagElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoDisabledButtonOptions;

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

        let kind = if is_native_button(element, ctx.model())? {
            ButtonKind::Native
        } else {
            if is_native_disabled_control(element) {
                return None;
            }
            let role_attribute = element.find_attribute_or_vue_binding("role")?;
            if has_trailing_dynamic_spread(element, &role_attribute)
                || AriaRole::from_roles(&static_attribute_text(&role_attribute, ctx.model())?)?
                    != AriaRole::Button
            {
                return None;
            }
            ButtonKind::Role
        };

        Some(RuleState { attribute, kind })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state.kind {
            ButtonKind::Native => RuleDiagnostic::new(
                rule_category!(),
                state.attribute.range(),
                markup! {
                    "Avoid the native "<Emphasis>"disabled"</Emphasis>" attribute on this button."
                },
            )
            .note(markup! {
                "The native disabled state removes the button from sequential focus navigation, suppresses activation, and changes form behavior."
            }),
            ButtonKind::Role => RuleDiagnostic::new(
                rule_category!(),
                state.attribute.range(),
                markup! {
                    "The "<Emphasis>"disabled"</Emphasis>" attribute has no effect on this element."
                },
            )
            .note(markup! {
                "The "<Emphasis>"button"</Emphasis>" role changes accessibility semantics but does not add native button behavior."
            }),
        };

        Some(diagnostic.note(markup! {
            <Emphasis>"aria-disabled"</Emphasis>" only exposes the disabled state to assistive technology. It does not suppress pointer or keyboard activation, form submission or validation, or apply disabled styling. Implement those behaviors, including forced-colors styling."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        if ctx
            .query()
            .find_attribute_or_vue_binding("aria-disabled")
            .is_some()
        {
            return None;
        }

        let mut mutation = ctx.root().begin();
        match &state.attribute {
            AnyHtmlAttribute::HtmlAttribute(attribute) => {
                if attribute.initializer().is_some() && attribute.as_static_value().is_none() {
                    let old_name = attribute.name().ok()?.value_token().ok()?;
                    let new_name =
                        HtmlSyntaxToken::new_detached(old_name.kind(), "aria-disabled", [], []);
                    mutation.replace_token_transfer_trivia(old_name, new_name);
                } else {
                    if attribute.syntax().has_comments_descendants() {
                        return None;
                    }
                    let replacement = make_aria_disabled_true_attribute(&state.attribute)?;
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
                        "aria-disabled",
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
                            make_vue_argument(&argument, &old_name)?,
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
                        let replacement = make_aria_disabled_true_attribute(&state.attribute)?;
                        mutation.replace_node(
                            state.attribute.clone(),
                            AnyHtmlAttribute::HtmlAttribute(replacement),
                        );
                    } else {
                        let (old_name, new_name) =
                            vue_argument_rename_tokens(&directive.arg().ok()?)?;
                        mutation.replace_token_transfer_trivia(old_name, new_name);
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
                        .with_arg(make_vue_argument(&argument, &old_name)?)
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
                        let replacement = make_aria_disabled_true_attribute(&state.attribute)?;
                        mutation.replace_node(
                            state.attribute.clone(),
                            AnyHtmlAttribute::HtmlAttribute(replacement),
                        );
                    } else {
                        let (old_name, new_name) = vue_argument_rename_tokens(&directive.arg()?)?;
                        mutation.replace_token_transfer_trivia(old_name, new_name);
                    }
                }
                _ => return None,
            },
            _ => return None,
        }

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"aria-disabled"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

fn make_aria_disabled_true_attribute(
    attribute: &AnyHtmlAttribute,
) -> Option<biome_html_syntax::HtmlAttribute> {
    let first_token = attribute.syntax().first_token()?;
    let last_token = attribute.syntax().last_token()?;
    let name = token_with_leading_trivia(
        HtmlSyntaxKind::HTML_LITERAL,
        "aria-disabled",
        &first_token,
    );
    let value = token_with_trailing_trivia(
        HtmlSyntaxKind::HTML_STRING_LITERAL,
        "\"true\"",
        &last_token,
    );
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
) -> Option<biome_html_syntax::VueDirectiveArgument> {
    let new_name = token_with_leading_trivia(old_name.kind(), "aria-disabled", old_name);
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
) -> Option<(HtmlSyntaxToken, HtmlSyntaxToken)> {
    let old_name = argument
        .arg()?
        .as_vue_static_argument()?
        .name_token()
        .ok()?;
    let new_name = HtmlSyntaxToken::new_detached(old_name.kind(), "aria-disabled", [], []);
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

fn is_native_button(
    element: &AnyHtmlTagElement,
    model: &AnalyzedVueModel<HtmlLanguage>,
) -> Option<bool> {
    if element.tag_name_kind() == Some(T![button]) {
        return Some(true);
    }
    if element.tag_name_kind() != Some(T![input]) {
        return Some(false);
    }

    let type_attribute = element.find_attribute_or_vue_binding("type")?;
    if has_trailing_dynamic_spread(element, &type_attribute) {
        return Some(false);
    }
    let input_type = static_attribute_text(&type_attribute, model)?;
    Some(
        ["button", "submit", "reset", "image"]
            .into_iter()
            .any(|button_type| input_type.eq_ignore_ascii_case(button_type)),
    )
}

fn is_native_disabled_control(element: &AnyHtmlTagElement) -> bool {
    element
        .tag_name_kind()
        .is_some_and(|kind| NATIVE_DISABLED_CONTROLS.contains(kind))
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
        if attribute.range().start() < current_attribute.range().end() {
            return false;
        }
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
    })
}
