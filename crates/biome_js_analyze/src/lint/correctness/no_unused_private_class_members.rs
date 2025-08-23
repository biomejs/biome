use crate::JsRuleAction;
use crate::class_member_references::{
    ClassMemberReference, ClassMemberReferences, class_member_references,
};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsFormalParameter, JsClassDeclaration, JsSyntaxKind,
    TsAccessibilityModifier, TsPropertyParameter,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Text, TextRange, declare_node_union,
};
use biome_rule_options::no_unused_private_class_members::NoUnusedPrivateClassMembersOptions;
use rustc_hash::FxHashSet;
use std::collections::HashSet;

declare_lint_rule! {
    /// Disallow unused private class members
    ///
    /// Private class members that are declared and not used anywhere in the code are most likely an error due to incomplete refactoring.
    /// Such class members take up space in the code and can lead to confusion by readers.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class OnlyWrite {
    ///   #usedOnlyInWrite = 5;
    ///
    ///   method() {
    ///	    this.#usedOnlyInWrite = 212;
    ///   }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    ///  class TsBioo {
    ///    private unusedProperty = 5;
    ///  }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    ///  class TsBioo {
    ///    private unusedMethod() {}
    ///  }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class UsedMember {
    ///   #usedMember = 42;
    ///
    ///   method() {
    ///	    return this.#usedMember;
    ///   }
    /// }
    /// ```
    ///
    pub NoUnusedPrivateClassMembers {
        version: "1.3.3",
        name: "noUnusedPrivateClassMembers",
        language: "js",
        sources: &[RuleSource::Eslint("no-unused-private-class-members").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyMember = AnyJsClassMember | TsPropertyParameter
}

impl Rule for NoUnusedPrivateClassMembers {
    type Query = Ast<JsClassDeclaration>;
    type State = AnyMember;
    type Signals = Box<[Self::State]>;
    type Options = NoUnusedPrivateClassMembersOptions;

    /// Determines which private class members in the queried class are unused and returns them as signals.
    ///
    /// Inspects the class declaration returned by `ctx.query()` to:
    /// - collect all declared private members (including TypeScript private constructor parameters),
    /// - compute member reads and writes via `class_member_references` on the class body,
    /// - apply the unused-member rules:
    ///   - A private accessor (getter/setter) is considered unused if it has no reads and no writes.
    ///   - A write-only accessor (written but never read) is ignored (not reported).
    ///   - Non-accessor private members are reported if they are never read (writes alone do not count as a use).
    ///
    /// Returns a boxed slice of `AnyMember` signals representing the unused private members found for the rule to report and (optionally) fix.
    ///
    /// # Examples
    ///
    /// ```
    /// // Conceptual usage:
    /// // let signals = MyRule::run(&ctx);
    /// // `signals` will contain the private members that the rule should report as unused.
    /// ```
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let private_members = get_all_declared_private_members(node);

        let ClassMemberReferences { reads, writes } = class_member_references(&node.members());

        private_members
            .iter()
            .filter_map(|private_member| {
                let is_read = reads
                    .iter()
                    .any(|ClassMemberReference { name, .. }| private_member.match_js_name(name));

                let is_write = writes
                    .iter()
                    .any(|ClassMemberReference { name, .. }| private_member.match_js_name(name));

                let is_write_only = !is_read && is_write;

                if !is_read && !is_write && private_member.is_accessor() {
                    return Some(private_member.clone());
                }

                if is_write_only && private_member.is_accessor() {
                    return None;
                }

                if !is_read {
                    Some(private_member.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<AnyMember>>()
            .into()
    }

    /// Create a diagnostic for an unused private class member.
    ///
    /// The diagnostic is centered on the member's identifier range (from `state.property_range()`)
    /// and uses the message: "This private class member is defined but never used."
    ///
    /// # Parameters
    ///
    /// - `state`: the unused member to report (its `property_range()` provides the diagnostic span).
    ///
    /// # Returns
    ///
    /// `Some(RuleDiagnostic)` with the location and message for the unused private member.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given an `state` representing an unused private member:
    /// let diag = diagnostic(&ctx, &state);
    /// assert!(diag.is_some());
    /// ```
    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.property_range(),
            markup! {
                "This private class member is defined but never used."
            },
        ))
    }

    /// Creates a rule action that removes the unused private class member.
    ///
    /// Builds a mutation that removes the AST node represented by `state` and returns a
    /// `JsRuleAction` (with the label "Remove unused declaration.") that applies that mutation.
    /// The produced action implements the rule's unsafe auto-fix to delete the unused declaration.
    ///
    /// # Parameters
    ///
    /// `state` — the unused private class member (the node to be removed).
    ///
    /// # Returns
    ///
    /// `Some(JsRuleAction)` containing the mutation that removes the member, or `None` if no action
    /// can be produced.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given a rule context `ctx` and an identified unused member `unused_member`:
    /// let action = <impl Rule>::action(&ctx, &unused_member);
    /// assert!(action.is_some());
    /// ```
    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        mutation.remove_node(state.clone());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove unused declaration." }.to_owned(),
            mutation,
        ))
    }
}

/// Collects all declared private members of a class.
///
/// Returns a `HashSet<AnyMember>` containing private JS class members (fields, methods,
/// getters, setters) and private TypeScript constructor parameter properties declared on
/// `class_declaration`. Members are gathered from the class body and any private parameter
/// properties on the constructor and are deduplicated by the returned `HashSet`.
///
/// # Examples
///
/// ```no_run
/// // Given `class_decl: JsClassDeclaration`, get all private members:
/// let privates = get_all_declared_private_members(&class_decl);
/// assert!(privates.iter().all(|m| m.is_private()));
/// ```
fn get_all_declared_private_members(class_declaration: &JsClassDeclaration) -> HashSet<AnyMember> {
    class_declaration
        .members()
        .iter()
        .map(AnyMember::AnyJsClassMember)
        .chain(get_constructor_params(class_declaration))
        .filter(|member| member.is_private())
        .collect()
}

/// Collects TypeScript `private` constructor parameter properties from a class.
///
/// Returns a set of `AnyMember` values representing `TsPropertyParameter` entries
/// declared on the class constructor (e.g., `constructor(private x: number) {}`).
/// If the class has no constructor or there are no `TsPropertyParameter`s, an
/// empty set is returned.
///
/// # Examples
///
/// ```no_run
/// // Given a parsed `JsClassDeclaration` named `class_decl`:
/// // let private_params = get_constructor_params(&class_decl);
/// // `private_params` will contain AnyMember::TsPropertyParameter entries for
/// // each `private` constructor parameter property in the class.
/// ```
fn get_constructor_params(class_declaration: &JsClassDeclaration) -> FxHashSet<AnyMember> {
    let constructor_member = class_declaration
        .members()
        .iter()
        .find_map(|member| match member {
            AnyJsClassMember::JsConstructorClassMember(member) => Some(member),
            _ => None,
        });

    if let Some(constructor_member) = constructor_member
        && let Ok(constructor_params) = constructor_member.parameters()
    {
        return constructor_params
            .parameters()
            .iter()
            .filter_map(|param| match param.ok()? {
                biome_js_syntax::AnyJsConstructorParameter::TsPropertyParameter(ts_property) => {
                    Some(ts_property.into())
                }
                _ => None,
            })
            .collect();
    }

    FxHashSet::default()
}

impl AnyMember {
    /// Returns true if this member is an accessor (a getter or a setter).
    ///
    /// # Examples
    ///
    /// ```
    /// enum Kind { Setter, Getter, Other }
    /// fn is_accessor(kind: Kind) -> bool {
    ///     matches!(kind, Kind::Setter | Kind::Getter)
    /// }
    /// assert!(is_accessor(Kind::Getter));
    /// assert!(is_accessor(Kind::Setter));
    /// assert!(!is_accessor(Kind::Other));
    /// ```
    fn is_accessor(&self) -> bool {
        matches!(
            self.syntax().kind(),
            JsSyntaxKind::JS_SETTER_CLASS_MEMBER | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
        )
    }

    /// Returns true if this class member is declared private.
    ///
    /// This considers both ES private fields/methods (the `#` private name syntax)
    /// and TypeScript `private` accessibility modifiers. For `AnyJsClassMember`
    /// variants it checks for an ES-private name or for a `private` TS accessibility
    /// modifier on getters, setters, methods, and properties. For `TsPropertyParameter`
    /// it checks for a `private` accessibility modifier on the parameter.
    fn is_private(&self) -> bool {
        match self {
            Self::AnyJsClassMember(member) => {
                let name = member.name().ok().flatten();

                let is_es_private = matches!(
                    name,
                    Some(AnyJsClassMemberName::JsPrivateClassMemberName(_))
                );

                let is_ts_private = match member {
                    AnyJsClassMember::JsGetterClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    AnyJsClassMember::JsMethodClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    AnyJsClassMember::JsPropertyClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    AnyJsClassMember::JsSetterClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    _ => false,
                };

                is_es_private || is_ts_private
            }
            Self::TsPropertyParameter(param) => param
                .modifiers()
                .iter()
                .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                .any(|accessibility| accessibility.is_private()),
        }
    }

    /// Returns the text range of this member's identifier/name for diagnostics.
    ///
    /// For JS class members (getters, setters, methods, properties) this returns the
    /// range of the member's declared name. For a TypeScript constructor property
    /// parameter it returns the range of the parameter's identifier. Returns `None`
    /// for members that do not have a simple identifier (e.g., computed names,
    /// metavariables, or other unsupported parameter forms).
    ///
    /// # Examples
    ///
    /// ```
    /// // Given an `AnyMember` parsed from source, obtain the range to highlight
    /// // the member's name in a diagnostic.
    /// let range_opt = any_member.property_range();
    /// if let Some(range) = range_opt {
    ///     // use `range` for diagnostic span
    ///     assert!(range.len() > 0);
    /// }
    /// ```
    fn property_range(&self) -> Option<TextRange> {
        match self {
            Self::AnyJsClassMember(member) => match member {
                AnyJsClassMember::JsGetterClassMember(member) => Some(member.name().ok()?.range()),
                AnyJsClassMember::JsMethodClassMember(member) => Some(member.name().ok()?.range()),
                AnyJsClassMember::JsPropertyClassMember(member) => {
                    Some(member.name().ok()?.range())
                }
                AnyJsClassMember::JsSetterClassMember(member) => Some(member.name().ok()?.range()),
                _ => None,
            },
            Self::TsPropertyParameter(ts_property) => match ts_property.formal_parameter().ok()? {
                AnyJsFormalParameter::JsBogusParameter(_)
                | AnyJsFormalParameter::JsMetavariable(_) => None,
                AnyJsFormalParameter::JsFormalParameter(param) => Some(
                    param
                        .binding()
                        .ok()?
                        .as_any_js_binding()?
                        .as_js_identifier_binding()?
                        .name_token()
                        .ok()?
                        .text_range(),
                ),
            },
        }
    }

    /// Returns true if the provided `text` matches this member's declared name.
    ///
    /// The comparison trims surrounding whitespace and removes a leading `#` (private name syntax)
    /// from `text` before comparing it to the member's identifier. Works for JS class members
    /// (getters, setters, methods, properties) and TypeScript constructor `private` parameter properties.
    ///
    /// # Examples
    ///
    /// ```
    /// // pseudo-usage showing intent; types are simplified for illustration:
    /// let text = Text::from("#foo");
    /// assert!(member.match_js_name(&text));
    /// ```
    fn match_js_name(&self, text: &Text) -> bool {
        let token = text.text().trim();
        let token = token.strip_prefix('#').unwrap_or(token);

        match self {
            Self::AnyJsClassMember(member) => match member {
                AnyJsClassMember::JsGetterClassMember(member) => member
                    .name()
                    .ok()
                    .and_then(|n| n.name())
                    .is_some_and(|name| name.text().eq(token)),
                AnyJsClassMember::JsMethodClassMember(member) => member
                    .name()
                    .ok()
                    .and_then(|n| n.name())
                    .is_some_and(|name| name.text().eq(token)),
                AnyJsClassMember::JsPropertyClassMember(member) => member
                    .name()
                    .ok()
                    .and_then(|n| n.name())
                    .is_some_and(|name| name.text().eq(token)),
                AnyJsClassMember::JsSetterClassMember(member) => member
                    .name()
                    .ok()
                    .and_then(|n| n.name())
                    .is_some_and(|name| name.text().eq(token)),
                _ => false,
            },
            Self::TsPropertyParameter(ts_property) => ts_property
                .formal_parameter()
                .ok()
                .and_then(|param| match param {
                    AnyJsFormalParameter::JsBogusParameter(_)
                    | AnyJsFormalParameter::JsMetavariable(_) => None,
                    AnyJsFormalParameter::JsFormalParameter(param) => param
                        .binding()
                        .ok()?
                        .as_any_js_binding()?
                        .as_js_identifier_binding()?
                        .name_token()
                        .ok(),
                })
                .is_some_and(|name_token| name_token.text().eq(token)),
        }
    }
}
