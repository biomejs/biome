use crate::services::semantic::Semantic;
use crate::utils::is_node_equal;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::static_value::StaticValue;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsExpression, AnyJsMemberExpression,
    AnyJsObjectMember, AnyJsObjectMemberName, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsClassMemberList, JsComputedMemberName, JsObjectExpression,
    JsObjectMemberList, JsPropertyObjectMember, JsSetterClassMember, JsSetterObjectMember,
    JsSyntaxToken, TextRange, TsSetterSignatureClassMember, global_identifier,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TokenText, declare_node_union};
use biome_rule_options::use_accessor_pairs::UseAccessorPairsOptions;

declare_lint_rule! {
    /// Require a getter for every setter in objects and classes.
    ///
    /// A property that has a setter but no getter can be written but not read:
    /// reading it always returns `undefined`. This is rarely what you want and
    /// usually means the getter was forgotten.
    ///
    /// This rule checks setters in object literals and classes. Static and
    /// instance members of a class are checked separately, so a `static` getter
    /// doesn't pair with an instance setter.
    ///
    /// The rule also checks property descriptors passed to `Object.create()`,
    /// `Object.defineProperty()`, `Object.defineProperties()`, and
    /// `Reflect.defineProperty()`: a descriptor with a `set` function must also
    /// have a `get` function.
    ///
    /// Getters without setters are allowed, because read-only properties are common.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const user = {
    ///     set name(value) {
    ///         this._name = value;
    ///     },
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class User {
    ///     set name(value) {
    ///         this._name = value;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.defineProperty(user, "name", {
    ///     set(value) {
    ///         this._name = value;
    ///     },
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const user = {
    ///     get name() {
    ///         return this._name;
    ///     },
    ///     set name(value) {
    ///         this._name = value;
    ///     },
    /// };
    ///
    /// class Counter {
    ///     get count() {
    ///         return this._count;
    ///     }
    /// }
    ///
    /// Object.defineProperty(user, "name", {
    ///     get() {
    ///         return this._name;
    ///     },
    ///     set(value) {
    ///         this._name = value;
    ///     },
    /// });
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want getters and setters for the same property to be next to each other, see [`useGroupedAccessorPairs`](https://biomejs.dev/linter/rules/use-grouped-accessor-pairs/).
    ///
    pub UseAccessorPairs {
        version: "2.6.5",
        name: "useAccessorPairs",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("accessor-pairs").same()],
    }
}

declare_node_union! {
    pub AnyUseAccessorPairsQuery =
        JsSetterObjectMember
        | JsSetterClassMember
        | TsSetterSignatureClassMember
        | JsObjectExpression
}

pub enum MissingGetter {
    /// A setter in an object literal or a class.
    Setter,
    /// A property descriptor, with the range of the name of its `set` property.
    Descriptor(TextRange),
}

impl Rule for UseAccessorPairs {
    type Query = Semantic<AnyUseAccessorPairsQuery>;
    type State = MissingGetter;
    type Signals = Option<Self::State>;
    type Options = UseAccessorPairsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyUseAccessorPairsQuery::JsSetterObjectMember(setter) => {
                let key = AccessorKey::from_object_member_name(&setter.name().ok()?)?;
                let members = setter.parent::<JsObjectMemberList>()?;
                let has_getter = members.iter().flatten().any(|member| {
                    member.as_js_getter_object_member().is_some_and(|getter| {
                        getter
                            .name()
                            .ok()
                            .and_then(|name| AccessorKey::from_object_member_name(&name))
                            .is_some_and(|getter_key| getter_key.is_same(&key))
                    })
                });
                (!has_getter).then_some(MissingGetter::Setter)
            }
            AnyUseAccessorPairsQuery::JsSetterClassMember(setter) => {
                let key = AccessorKey::from_class_member_name(&setter.name().ok()?)?;
                let is_static = setter
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                let members = setter.parent::<JsClassMemberList>()?;
                (!has_class_getter(&members, &key, is_static)).then_some(MissingGetter::Setter)
            }
            AnyUseAccessorPairsQuery::TsSetterSignatureClassMember(setter) => {
                let mut is_static = false;
                for modifier in setter.modifiers() {
                    // A subclass implements an abstract setter, so the pair is checked there.
                    if modifier.as_ts_abstract_modifier().is_some() {
                        return None;
                    }
                    is_static |= modifier.as_js_static_modifier().is_some();
                }
                let key = AccessorKey::from_class_member_name(&setter.name().ok()?)?;
                let members = setter.parent::<JsClassMemberList>()?;
                (!has_class_getter(&members, &key, is_static)).then_some(MissingGetter::Setter)
            }
            AnyUseAccessorPairsQuery::JsObjectExpression(object) => {
                if !is_property_descriptor(ctx.model(), object) {
                    return None;
                }
                let mut set_range = None;
                for member in object.members().iter().flatten() {
                    // The spread object may provide the getter.
                    if member.as_js_spread().is_some() {
                        return None;
                    }
                    let Some((name, range)) = descriptor_member_name(&member) else {
                        continue;
                    };
                    match name.text() {
                        "get" => return None,
                        "set" if set_range.is_none() => set_range = Some(range),
                        _ => {}
                    }
                }
                set_range.map(MissingGetter::Descriptor)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (range, message) = match state {
            MissingGetter::Setter => {
                let node = ctx.query();
                let name_end = match node {
                    AnyUseAccessorPairsQuery::JsSetterObjectMember(setter) => {
                        setter.name().ok()?.range().end()
                    }
                    AnyUseAccessorPairsQuery::JsSetterClassMember(setter) => {
                        setter.name().ok()?.range().end()
                    }
                    AnyUseAccessorPairsQuery::TsSetterSignatureClassMember(setter) => {
                        setter.name().ok()?.range().end()
                    }
                    AnyUseAccessorPairsQuery::JsObjectExpression(_) => return None,
                };
                (
                    TextRange::new(node.range().start(), name_end),
                    markup! { "This setter has no matching getter." },
                )
            }
            MissingGetter::Descriptor(range) => (
                *range,
                markup! { "This property descriptor has a setter but no getter." },
            ),
        };
        Some(
            RuleDiagnostic::new(rule_category!(), range, message)
                .note(markup! {
                    "Without a getter, reading the property always returns "<Emphasis>"undefined"</Emphasis>"."
                })
                .note(markup! {
                    "Add a getter for the property, or remove the setter."
                }),
        )
    }
}

/// Returns `true` if `members` contains a getter named `key` with the same `static` modifier.
fn has_class_getter(members: &JsClassMemberList, key: &AccessorKey, is_static: bool) -> bool {
    members.iter().any(|member| {
        let (getter_name, getter_is_static) = match member {
            AnyJsClassMember::JsGetterClassMember(getter) => (
                getter.name(),
                getter
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some()),
            ),
            AnyJsClassMember::TsGetterSignatureClassMember(getter) => (
                getter.name(),
                getter
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some()),
            ),
            _ => return false,
        };
        getter_is_static == is_static
            && getter_name
                .ok()
                .and_then(|name| AccessorKey::from_class_member_name(&name))
                .is_some_and(|getter_key| getter_key.is_same(key))
    })
}

/// The key of an accessor, used to find the getter that pairs with a setter.
enum AccessorKey {
    /// A key whose value is known, such as `foo`, `"foo"`, or `["foo"]`.
    Static(StaticValue),
    /// A private class member name, such as `#foo`.
    Private(JsSyntaxToken),
    /// A computed key whose value isn't known, such as `[foo]`.
    /// Two such keys are the same if their expressions are written the same way.
    Dynamic(AnyJsExpression),
}

impl AccessorKey {
    fn from_object_member_name(name: &AnyJsObjectMemberName) -> Option<Self> {
        match name {
            AnyJsObjectMemberName::JsLiteralMemberName(literal) => {
                Some(Self::Static(StaticValue::String(literal.value().ok()?)))
            }
            AnyJsObjectMemberName::JsComputedMemberName(computed) => Self::from_computed(computed),
            AnyJsObjectMemberName::JsMetavariable(_) => None,
        }
    }

    fn from_class_member_name(name: &AnyJsClassMemberName) -> Option<Self> {
        match name {
            AnyJsClassMemberName::JsLiteralMemberName(literal) => {
                Some(Self::Static(StaticValue::String(literal.value().ok()?)))
            }
            AnyJsClassMemberName::JsComputedMemberName(computed) => Self::from_computed(computed),
            AnyJsClassMemberName::JsPrivateClassMemberName(private) => {
                Some(Self::Private(private.id_token().ok()?))
            }
            AnyJsClassMemberName::JsMetavariable(_) => None,
        }
    }

    fn from_computed(computed: &JsComputedMemberName) -> Option<Self> {
        let expression = computed.expression().ok()?;
        Some(match expression.as_static_value() {
            Some(value) => Self::Static(value),
            None => Self::Dynamic(expression),
        })
    }

    fn is_same(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Static(left), Self::Static(right)) => left.text() == right.text(),
            (Self::Private(left), Self::Private(right)) => {
                left.text_trimmed() == right.text_trimmed()
            }
            (Self::Dynamic(left), Self::Dynamic(right)) => {
                is_node_equal(left.syntax(), right.syntax())
            }
            _ => false,
        }
    }
}

/// Returns the name of a property or method in a property descriptor, and the range of that name.
/// Returns `None` for computed names, such as `["set"]`, and for getters and setters.
fn descriptor_member_name(member: &AnyJsObjectMember) -> Option<(TokenText, TextRange)> {
    let name = match member {
        AnyJsObjectMember::JsPropertyObjectMember(property) => property.name().ok()?,
        AnyJsObjectMember::JsMethodObjectMember(method) => method.name().ok()?,
        AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => {
            let reference = shorthand.name().ok()?;
            return Some((reference.name().ok()?, reference.range()));
        }
        _ => return None,
    };
    let literal = name.as_js_literal_member_name()?;
    Some((literal.name().ok()?, literal.range()))
}

/// Returns `true` if `object` is used as a property descriptor:
/// - the third argument of `Object.defineProperty()` or `Reflect.defineProperty()`;
/// - a property value in the second argument of `Object.create()` or `Object.defineProperties()`.
fn is_property_descriptor(model: &SemanticModel, object: &JsObjectExpression) -> bool {
    let Some(expression) = AnyJsExpression::from(object.clone()).outer_expression() else {
        return false;
    };
    if is_argument_of_global_method_call(model, &expression, 2, |object, method| {
        method == "defineProperty" && (object == "Object" || object == "Reflect")
    }) {
        return true;
    }
    let Some(descriptors) = expression
        .parent::<JsPropertyObjectMember>()
        .and_then(|property| property.parent::<JsObjectMemberList>())
        .and_then(|members| members.parent::<JsObjectExpression>())
        .and_then(|descriptors| AnyJsExpression::from(descriptors).outer_expression())
    else {
        return false;
    };
    is_argument_of_global_method_call(model, &descriptors, 1, |object, method| {
        object == "Object" && (method == "create" || method == "defineProperties")
    })
}

/// Returns `true` if `argument` is the argument at `index` of a call to a method
/// of a global object, and `is_method(object_name, method_name)` returns `true`.
fn is_argument_of_global_method_call(
    model: &SemanticModel,
    argument: &AnyJsExpression,
    index: usize,
    is_method: impl Fn(&str, &str) -> bool,
) -> bool {
    let Some(arguments) = argument.parent::<JsCallArgumentList>() else {
        return false;
    };
    let is_at_index = arguments
        .iter()
        .nth(index)
        .and_then(|node| node.ok())
        .is_some_and(|node| node.syntax() == argument.syntax());
    if !is_at_index {
        return false;
    }
    let Some(call) = arguments
        .parent::<JsCallArguments>()
        .and_then(|arguments| arguments.parent::<JsCallExpression>())
    else {
        return false;
    };
    let Some(callee) = call
        .callee()
        .ok()
        .map(AnyJsExpression::omit_parentheses)
        .and_then(|callee| AnyJsMemberExpression::cast(callee.into_syntax()))
    else {
        return false;
    };
    let Some(method) = callee.member_name() else {
        return false;
    };
    let Some((reference, object)) = callee
        .object()
        .ok()
        .map(AnyJsExpression::omit_parentheses)
        .and_then(|object| object.as_any_global_identifier_expression())
        .and_then(|object| global_identifier(&object))
    else {
        return false;
    };
    is_method(object.text(), method.text()) && model.binding(&reference).is_none()
}
