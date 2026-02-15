use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, JsClassMemberList, JsSyntaxKind,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_member_ordering::{MemberGroup, UseMemberOrderingOptions};

declare_lint_rule! {
    /// Enforce a consistent ordering of class members.
    ///
    /// A consistent ordering of class members improves readability and maintainability.
    /// This rule enforces that class members are ordered according to a configurable order.
    ///
    /// By default, the order is:
    /// 1. Index signatures
    /// 2. Static properties (public, protected, private, #private)
    /// 3. Instance properties (public, protected, private, #private)
    /// 4. Static accessors
    /// 5. Instance accessors
    /// 6. Static methods
    /// 7. Constructor
    /// 8. Instance methods
    /// 9. Static blocks
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///     method() {}
    ///     name;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class Foo {
    ///     name;
    ///     constructor() {}
    ///     method() {}
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// The rule accepts an object with a `groups` property that determines the ordering of class members.
    ///
    /// ### `groups`
    ///
    /// An array of member group names that defines the order. Members not listed in the configured order
    /// are allowed in any position.
    ///
    /// Available member groups:
    /// - `index-signature`, `static-index-signature`
    /// - `property`, `static-property`, `protected-property`, `protected-static-property`,
    ///   `private-property`, `private-static-property`, `#private-property`, `#private-static-property`
    /// - `accessor`, `static-accessor`, `protected-accessor`, `protected-static-accessor`,
    ///   `private-accessor`, `private-static-accessor`, `#private-accessor`, `#private-static-accessor`
    ///   (combined accessor groups are reserved for the TC39 auto-accessor proposal and are
    ///   not currently produced by the classifier — use `get-accessor` / `set-accessor` variants instead)
    /// - `get-accessor`, `static-get-accessor`, `protected-get-accessor`, `protected-static-get-accessor`,
    ///   `private-get-accessor`, `private-static-get-accessor`, `#private-get-accessor`, `#private-static-get-accessor`
    /// - `set-accessor`, `static-set-accessor`, `protected-set-accessor`, `protected-static-set-accessor`,
    ///   `private-set-accessor`, `private-static-set-accessor`, `#private-set-accessor`, `#private-static-set-accessor`
    /// - `method`, `static-method`, `protected-method`, `protected-static-method`,
    ///   `private-method`, `private-static-method`, `#private-method`, `#private-static-method`
    /// - `constructor`
    /// - `static-block`
    ///
    /// Example custom order:
    /// ```json
    /// {
    ///   "linter": {
    ///     "rules": {
    ///       "nursery": {
    ///         "useMemberOrdering": {
    ///           "options": {
    ///             "groups": ["constructor", "method", "property", "static-block"]
    ///           }
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    pub UseMemberOrdering {
        version: "next",
        name: "useMemberOrdering",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("member-ordering").inspired()],
    }
}

/// Represents a violation where a class member is out of order.
pub struct MemberOrderViolation {
    /// The range of the member that is out of order.
    member_range: TextRange,
    /// The label of the group that this member should be placed before.
    expected_before_name: &'static str,
}

impl Rule for UseMemberOrdering {
    type Query = Ast<JsClassMemberList>;
    type State = MemberOrderViolation;
    type Signals = Box<[Self::State]>;
    type Options = UseMemberOrderingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let member_list = ctx.query();
        let options = ctx.options();
        let order = options.groups();

        let classified: Vec<(TextRange, Option<MemberGroup>)> = member_list
            .into_iter()
            .map(|member| (member.range(), classify_member(&member)))
            .collect();

        find_order_violations(&classified, &order)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let options = ctx.options();
        let order = options.groups();
        let mut seen = Vec::new();
        for g in &order {
            let broad = g.broad_category();
            if !seen.contains(&broad) {
                seen.push(broad);
            }
        }
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.member_range,
                markup! {
                    "This member should be placed before the "{state.expected_before_name}" member."
                },
            )
            .footer_list(
                "Members should be ordered by:",
                &seen,
            ),
        )
    }
}

/// Classify a class member into its `MemberGroup`.
fn classify_member(member: &AnyJsClassMember) -> Option<MemberGroup> {
    match member {
        AnyJsClassMember::JsConstructorClassMember(_) => Some(MemberGroup::Constructor),
        AnyJsClassMember::JsPropertyClassMember(prop) => {
            let is_static = has_static_modifier_property(&prop.modifiers());
            let is_hash_private = prop
                .name()
                .ok()
                .is_some_and(|n| is_hash_private_name(&n));
            let accessibility = get_property_accessibility(&prop.modifiers());
            Some(resolve_property_group(
                is_static,
                &accessibility,
                is_hash_private,
            ))
        }
        AnyJsClassMember::JsMethodClassMember(method) => {
            let is_static = has_static_modifier_method(&method.modifiers());
            let is_hash_private = method
                .name()
                .ok()
                .is_some_and(|n| is_hash_private_name(&n));
            let accessibility = get_method_accessibility(&method.modifiers());
            Some(resolve_method_group(
                is_static,
                &accessibility,
                is_hash_private,
            ))
        }
        AnyJsClassMember::JsGetterClassMember(getter) => {
            let is_static = has_static_modifier_method(&getter.modifiers());
            let is_hash_private = getter
                .name()
                .ok()
                .is_some_and(|n| is_hash_private_name(&n));
            let accessibility = get_method_accessibility(&getter.modifiers());
            Some(resolve_get_accessor_group(
                is_static,
                &accessibility,
                is_hash_private,
            ))
        }
        AnyJsClassMember::JsSetterClassMember(setter) => {
            let is_static = has_static_modifier_method(&setter.modifiers());
            let is_hash_private = setter
                .name()
                .ok()
                .is_some_and(|n| is_hash_private_name(&n));
            let accessibility = get_method_accessibility(&setter.modifiers());
            Some(resolve_set_accessor_group(
                is_static,
                &accessibility,
                is_hash_private,
            ))
        }
        AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => {
            Some(MemberGroup::StaticBlock)
        }
        AnyJsClassMember::TsIndexSignatureClassMember(idx) => {
            let is_static = has_static_modifier_index(&idx.modifiers());
            if is_static {
                Some(MemberGroup::StaticIndexSignature)
            } else {
                Some(MemberGroup::IndexSignature)
            }
        }
        AnyJsClassMember::TsPropertySignatureClassMember(prop) => {
            let is_static = has_static_modifier_ts_property(&prop.modifiers());
            let is_hash_private = prop
                .name()
                .ok()
                .is_some_and(|n| is_hash_private_name(&n));
            let accessibility = get_ts_property_accessibility(&prop.modifiers());
            Some(resolve_property_group(
                is_static,
                &accessibility,
                is_hash_private,
            ))
        }
        AnyJsClassMember::TsMethodSignatureClassMember(method) => {
            let is_static = has_static_modifier_ts_method(&method.modifiers());
            let is_hash_private = method
                .name()
                .ok()
                .is_some_and(|n| is_hash_private_name(&n));
            let accessibility = get_ts_method_accessibility(&method.modifiers());
            Some(resolve_method_group(
                is_static,
                &accessibility,
                is_hash_private,
            ))
        }
        AnyJsClassMember::TsGetterSignatureClassMember(getter) => {
            let is_static = has_static_modifier_ts_method(&getter.modifiers());
            let is_hash_private = getter
                .name()
                .ok()
                .is_some_and(|n| is_hash_private_name(&n));
            let accessibility = get_ts_method_accessibility(&getter.modifiers());
            Some(resolve_get_accessor_group(
                is_static,
                &accessibility,
                is_hash_private,
            ))
        }
        AnyJsClassMember::TsSetterSignatureClassMember(setter) => {
            let is_static = has_static_modifier_ts_method(&setter.modifiers());
            let is_hash_private = setter
                .name()
                .ok()
                .is_some_and(|n| is_hash_private_name(&n));
            let accessibility = get_ts_method_accessibility(&setter.modifiers());
            Some(resolve_set_accessor_group(
                is_static,
                &accessibility,
                is_hash_private,
            ))
        }
        AnyJsClassMember::TsConstructorSignatureClassMember(_) => Some(MemberGroup::Constructor),
        AnyJsClassMember::TsInitializedPropertySignatureClassMember(prop) => {
            let is_static = has_static_modifier_ts_property(&prop.modifiers());
            let is_hash_private = prop
                .name()
                .ok()
                .is_some_and(|n| is_hash_private_name(&n));
            let accessibility = get_ts_property_accessibility(&prop.modifiers());
            Some(resolve_property_group(
                is_static,
                &accessibility,
                is_hash_private,
            ))
        }
        // Empty members, bogus members, metavariables: skip
        AnyJsClassMember::JsEmptyClassMember(_)
        | AnyJsClassMember::JsBogusMember(_)
        | AnyJsClassMember::JsMetavariable(_) => None,
    }
}

/// Accessibility level of a class member.
enum Accessibility {
    Public,
    Protected,
    Private,
}

fn is_hash_private_name(name: &AnyJsClassMemberName) -> bool {
    matches!(
        name,
        AnyJsClassMemberName::JsPrivateClassMemberName(_)
    )
}

// Helper functions for JS modifiers

fn has_static_modifier_property(
    modifiers: &biome_js_syntax::JsPropertyModifierList,
) -> bool {
    use biome_js_syntax::AnyJsPropertyModifier;
    modifiers
        .into_iter()
        .any(|m| matches!(m, AnyJsPropertyModifier::JsStaticModifier(_)))
}

fn get_property_accessibility(
    modifiers: &biome_js_syntax::JsPropertyModifierList,
) -> Accessibility {
    use biome_js_syntax::AnyJsPropertyModifier;
    for m in modifiers {
        if let AnyJsPropertyModifier::TsAccessibilityModifier(acc) = m {
            return ts_accessibility_to_enum(&acc);
        }
    }
    Accessibility::Public
}

fn has_static_modifier_method(
    modifiers: &biome_js_syntax::JsMethodModifierList,
) -> bool {
    use biome_js_syntax::AnyJsMethodModifier;
    modifiers
        .into_iter()
        .any(|m| matches!(m, AnyJsMethodModifier::JsStaticModifier(_)))
}

fn get_method_accessibility(
    modifiers: &biome_js_syntax::JsMethodModifierList,
) -> Accessibility {
    use biome_js_syntax::AnyJsMethodModifier;
    for m in modifiers {
        if let AnyJsMethodModifier::TsAccessibilityModifier(acc) = m {
            return ts_accessibility_to_enum(&acc);
        }
    }
    Accessibility::Public
}

// Helper functions for TS modifiers

fn has_static_modifier_index(
    modifiers: &biome_js_syntax::TsIndexSignatureModifierList,
) -> bool {
    use biome_js_syntax::AnyTsIndexSignatureModifier;
    modifiers
        .into_iter()
        .any(|m| matches!(m, AnyTsIndexSignatureModifier::JsStaticModifier(_)))
}

fn has_static_modifier_ts_property(
    modifiers: &biome_js_syntax::TsPropertySignatureModifierList,
) -> bool {
    use biome_js_syntax::AnyTsPropertySignatureModifier;
    modifiers
        .into_iter()
        .any(|m| matches!(m, AnyTsPropertySignatureModifier::JsStaticModifier(_)))
}

fn get_ts_property_accessibility(
    modifiers: &biome_js_syntax::TsPropertySignatureModifierList,
) -> Accessibility {
    use biome_js_syntax::AnyTsPropertySignatureModifier;
    for m in modifiers {
        if let AnyTsPropertySignatureModifier::TsAccessibilityModifier(acc) = m {
            return ts_accessibility_to_enum(&acc);
        }
    }
    Accessibility::Public
}

fn has_static_modifier_ts_method(
    modifiers: &biome_js_syntax::TsMethodSignatureModifierList,
) -> bool {
    use biome_js_syntax::AnyTsMethodSignatureModifier;
    modifiers
        .into_iter()
        .any(|m| matches!(m, AnyTsMethodSignatureModifier::JsStaticModifier(_)))
}

fn get_ts_method_accessibility(
    modifiers: &biome_js_syntax::TsMethodSignatureModifierList,
) -> Accessibility {
    use biome_js_syntax::AnyTsMethodSignatureModifier;
    for m in modifiers {
        if let AnyTsMethodSignatureModifier::TsAccessibilityModifier(acc) = m {
            return ts_accessibility_to_enum(&acc);
        }
    }
    Accessibility::Public
}

fn ts_accessibility_to_enum(
    acc: &biome_js_syntax::TsAccessibilityModifier,
) -> Accessibility {
    if let Ok(token) = acc.modifier_token() {
        match token.kind() {
            JsSyntaxKind::PROTECTED_KW => Accessibility::Protected,
            JsSyntaxKind::PRIVATE_KW => Accessibility::Private,
            _ => Accessibility::Public,
        }
    } else {
        Accessibility::Public
    }
}

// Resolve helpers

fn resolve_property_group(
    is_static: bool,
    accessibility: &Accessibility,
    is_hash_private: bool,
) -> MemberGroup {
    match (is_static, accessibility, is_hash_private) {
        (true, _, true) => MemberGroup::HashPrivateStaticProperty,
        (false, _, true) => MemberGroup::HashPrivateProperty,
        (true, Accessibility::Protected, _) => MemberGroup::ProtectedStaticProperty,
        (true, Accessibility::Private, _) => MemberGroup::PrivateStaticProperty,
        (true, Accessibility::Public, _) => MemberGroup::StaticProperty,
        (false, Accessibility::Protected, _) => MemberGroup::ProtectedProperty,
        (false, Accessibility::Private, _) => MemberGroup::PrivateProperty,
        (false, Accessibility::Public, _) => MemberGroup::Property,
    }
}

fn resolve_method_group(
    is_static: bool,
    accessibility: &Accessibility,
    is_hash_private: bool,
) -> MemberGroup {
    match (is_static, accessibility, is_hash_private) {
        (true, _, true) => MemberGroup::HashPrivateStaticMethod,
        (false, _, true) => MemberGroup::HashPrivateMethod,
        (true, Accessibility::Protected, _) => MemberGroup::ProtectedStaticMethod,
        (true, Accessibility::Private, _) => MemberGroup::PrivateStaticMethod,
        (true, Accessibility::Public, _) => MemberGroup::StaticMethod,
        (false, Accessibility::Protected, _) => MemberGroup::ProtectedMethod,
        (false, Accessibility::Private, _) => MemberGroup::PrivateMethod,
        (false, Accessibility::Public, _) => MemberGroup::Method,
    }
}

fn resolve_get_accessor_group(
    is_static: bool,
    accessibility: &Accessibility,
    is_hash_private: bool,
) -> MemberGroup {
    match (is_static, accessibility, is_hash_private) {
        (true, _, true) => MemberGroup::HashPrivateStaticGetAccessor,
        (false, _, true) => MemberGroup::HashPrivateGetAccessor,
        (true, Accessibility::Protected, _) => MemberGroup::ProtectedStaticGetAccessor,
        (true, Accessibility::Private, _) => MemberGroup::PrivateStaticGetAccessor,
        (true, Accessibility::Public, _) => MemberGroup::StaticGetAccessor,
        (false, Accessibility::Protected, _) => MemberGroup::ProtectedGetAccessor,
        (false, Accessibility::Private, _) => MemberGroup::PrivateGetAccessor,
        (false, Accessibility::Public, _) => MemberGroup::GetAccessor,
    }
}

fn resolve_set_accessor_group(
    is_static: bool,
    accessibility: &Accessibility,
    is_hash_private: bool,
) -> MemberGroup {
    match (is_static, accessibility, is_hash_private) {
        (true, _, true) => MemberGroup::HashPrivateStaticSetAccessor,
        (false, _, true) => MemberGroup::HashPrivateSetAccessor,
        (true, Accessibility::Protected, _) => MemberGroup::ProtectedStaticSetAccessor,
        (true, Accessibility::Private, _) => MemberGroup::PrivateStaticSetAccessor,
        (true, Accessibility::Public, _) => MemberGroup::StaticSetAccessor,
        (false, Accessibility::Protected, _) => MemberGroup::ProtectedSetAccessor,
        (false, Accessibility::Private, _) => MemberGroup::PrivateSetAccessor,
        (false, Accessibility::Public, _) => MemberGroup::SetAccessor,
    }
}

/// Find violations by tracking the maximum rank seen so far.
/// A member is out of order if its rank is less than the maximum rank seen.
/// Members with groups not listed in the order are skipped (not tracked).
fn find_order_violations(
    classified: &[(TextRange, Option<MemberGroup>)],
    order: &[MemberGroup],
) -> Box<[MemberOrderViolation]> {
    let mut violations = Vec::new();
    let mut max_rank: Option<(usize, &str)> = None;
    let sentinel = order.len();

    for (range, group) in classified {
        let Some(group) = group else {
            continue;
        };
        let rank = group.rank(order);
        // Skip members whose group is not listed in the configured order.
        // When rank == sentinel (order.len()), the group was not found in the order
        // and should be allowed in any position without triggering violations.
        if rank == sentinel {
            continue;
        }
        if let Some((max_r, max_label)) = max_rank {
            if rank < max_r {
                violations.push(MemberOrderViolation {
                    member_range: *range,
                    expected_before_name: max_label,
                });
            } else {
                max_rank = Some((rank, group.label()));
            }
        } else {
            max_rank = Some((rank, group.label()));
        }
    }

    violations.into_boxed_slice()
}
