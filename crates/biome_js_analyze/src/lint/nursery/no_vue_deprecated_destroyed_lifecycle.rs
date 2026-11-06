use crate::JsRuleAction;
use crate::services::vue::VueComponent;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectMember, AnyJsObjectMemberName, JsObjectMemberList,
    JsShorthandPropertyObjectMember, JsSyntaxKind, JsSyntaxToken, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange, TriviaPieceKind};
use biome_rule_options::no_vue_deprecated_destroyed_lifecycle::NoVueDeprecatedDestroyedLifecycleOptions;

declare_lint_rule! {
    /// Disallow the deprecated `destroyed` and `beforeDestroy` lifecycle hooks in Vue components.
    ///
    /// Vue 3 renamed the `beforeDestroy` hook to `beforeUnmount`, and the `destroyed` hook to `unmounted`.
    /// Vue 3 doesn't call hooks that use the old names, so the code inside them never runs.
    ///
    /// See the [Vue 3 migration guide](https://v3-migration.vuejs.org/breaking-changes/vnode-lifecycle-events.html#migration-strategy) for more information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   beforeDestroy() {},
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   destroyed() {},
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   beforeMount() {},
    ///   mounted() {},
    ///   beforeUnmount() {},
    ///   unmounted() {},
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueDeprecatedDestroyedLifecycle {
        version: "2.6.5",
        name: "noVueDeprecatedDestroyedLifecycle",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-destroyed-lifecycle").same()],
    }
}

impl Rule for NoVueDeprecatedDestroyedLifecycle {
    type Query = VueComponent;
    type State = DeprecatedHookUsage;
    type Signals = Box<[Self::State]>;
    type Options = NoVueDeprecatedDestroyedLifecycleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        let snippet = component.snippet().map(|snippet| snippet.id());
        let root = ctx.root();
        component
            .hooks()
            // Ranges are local to the tree that declares the hook, so only
            // hooks declared in the analyzed tree can be mapped to its syntax.
            .filter(|hook| hook.snippet().map(|snippet| snippet.id()) == snippet)
            .filter_map(|hook| {
                let kind = match hook.name() {
                    "beforeDestroy" => DeprecatedHook::BeforeDestroy,
                    "destroyed" => DeprecatedHook::Destroyed,
                    _ => return None,
                };
                let range = hook.range();
                let member = root
                    .syntax()
                    .covering_element(range)
                    .ancestors()
                    .find_map(AnyJsObjectMember::cast)?;
                let replacement = kind.replacement();
                let has_replacement = member
                    .parent::<JsObjectMemberList>()
                    .is_some_and(|members| {
                        members.iter().flatten().any(|sibling| {
                            sibling.name().is_some_and(|name| name.text() == replacement)
                        })
                    });
                Some(DeprecatedHookUsage {
                    kind,
                    range,
                    member,
                    has_replacement,
                })
            })
            .collect()
    }

    fn text_range(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<TextRange> {
        Some(state.range)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let hook = state.kind;
        let replacement = hook.replacement();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range,
            markup! {
                "The "<Emphasis>{hook}</Emphasis>" hook was renamed to "<Emphasis>{replacement}</Emphasis>" in Vue 3."
            },
        )
        .note(markup! {
            "Vue 3 doesn't call the "<Emphasis>{hook}</Emphasis>" hook, so the code inside it never runs."
        });
        Some(if state.has_replacement {
            diagnostic.note(markup! {
                "Move the code into the existing "<Emphasis>{replacement}</Emphasis>" hook, and remove this one."
            })
        } else {
            diagnostic.note(markup! {
                "Rename the hook to "<Emphasis>{replacement}</Emphasis>"."
            })
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        // Renaming the hook next to an existing hook with the new name would
        // create a duplicate key, and only one of the two hooks would remain.
        if state.has_replacement {
            return None;
        }
        let replacement = state.kind.replacement();

        let mut mutation = ctx.root().begin();
        match &state.member {
            AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => {
                mutation.replace_node(
                    state.member.clone(),
                    expand_shorthand(shorthand, replacement)?,
                );
            }
            member => {
                let token = member_name(member)?.name_token()?;
                let text = if token.kind() == JsSyntaxKind::JS_STRING_LITERAL {
                    // Keep the quotes the key was written with.
                    let quote = token.text_trimmed().chars().next()?;
                    format!("{quote}{replacement}{quote}")
                } else {
                    replacement.to_string()
                };
                let new_token = JsSyntaxToken::new_detached(token.kind(), &text, [], []);
                mutation.replace_token_transfer_trivia(token, new_token);
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Rename the hook to "<Emphasis>{replacement}</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

pub struct DeprecatedHookUsage {
    kind: DeprecatedHook,
    /// The range of the hook's name.
    range: TextRange,
    /// The object member that declares the hook.
    member: AnyJsObjectMember,
    /// Whether the same object already declares the hook under its Vue 3 name.
    has_replacement: bool,
}

#[derive(Clone, Copy)]
pub enum DeprecatedHook {
    BeforeDestroy,
    Destroyed,
}

impl DeprecatedHook {
    /// The name Vue 3 uses for this hook.
    const fn replacement(self) -> &'static str {
        match self {
            Self::BeforeDestroy => "beforeUnmount",
            Self::Destroyed => "unmounted",
        }
    }
}

impl biome_console::fmt::Display for DeprecatedHook {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::BeforeDestroy => "beforeDestroy",
            Self::Destroyed => "destroyed",
        })
    }
}

fn member_name(member: &AnyJsObjectMember) -> Option<AnyJsObjectMemberName> {
    match member {
        AnyJsObjectMember::JsGetterObjectMember(member) => member.name().ok(),
        AnyJsObjectMember::JsMethodObjectMember(member) => member.name().ok(),
        AnyJsObjectMember::JsPropertyObjectMember(member) => member.name().ok(),
        AnyJsObjectMember::JsSetterObjectMember(member) => member.name().ok(),
        _ => None,
    }
}

/// Turns `destroyed` into `unmounted: destroyed`, so the key changes while
/// the value stays the same variable.
fn expand_shorthand(
    shorthand: &JsShorthandPropertyObjectMember,
    key: &str,
) -> Option<AnyJsObjectMember> {
    let reference = shorthand.name().ok()?;
    let value_token = reference.value_token().ok()?;
    // The key takes the place of the shorthand, so it takes its leading
    // trivia too, such as the line break and indentation before it.
    let key_token = make::ident(key).with_leading_trivia_pieces(value_token.leading_trivia().pieces());
    let value_token = value_token.with_leading_trivia([]);
    Some(AnyJsObjectMember::JsPropertyObjectMember(
        make::js_property_object_member(
            AnyJsObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(key_token)),
            make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            AnyJsExpression::JsIdentifierExpression(make::js_identifier_expression(
                make::js_reference_identifier(value_token),
            )),
        ),
    ))
}
