use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::static_value::StaticValue;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsObjectMember, AnyJsObjectMemberName,
    AnyTsTypeMember, JsClassMemberList, JsComputedMemberName, JsGetterClassMember,
    JsGetterObjectMember, JsLiteralMemberName, JsMetavariable, JsObjectMemberList,
    JsPrivateClassMemberName, JsSetterClassMember, JsSetterObjectMember,
    TsGetterSignatureClassMember, TsGetterSignatureTypeMember, TsSetterSignatureClassMember,
    TsSetterSignatureTypeMember, TsTypeMemberList,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxResult, declare_node_union};
use biome_rule_options::use_grouped_accessor_pairs::UseGroupedAccessorPairsOptions;

declare_lint_rule! {
    /// Enforce that getters and setters for the same property are adjacent in class and object definitions.
    ///
    /// When defining a property in a class or object, it's common to have both a getter and a setter.
    /// This rule enforces that getter is defined right before the setter,
    /// making the code more maintainable and easier to read.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// Name getter and setter are not adjacent:
    ///
    /// ```js,expect_diagnostic
    /// class User {
    ///   get name() { return this._name; }
    ///   constructor() {}
    ///   set name(value) { this._name = value; }
    /// }
    /// ```
    ///
    /// Getter should go before the setter.
    ///
    /// ```js,expect_diagnostic
    /// const user = {
    ///   set name(value) { this._name = value; },
    ///   get name() { return this._name; }
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class User {
    ///   get name() { return this._name; }
    ///   set name(value) { this._name = value; }
    ///   get age() { return this._age; }
    ///   set age(age) { this._age = age; }
    /// }
    /// ```
    ///
    /// This rule does not enforce the existence of both getter and setter for a property.
    /// Single getters without setters and setters without getters are ignored.
    ///
    pub UseGroupedAccessorPairs {
        version: "2.0.0",
        name: "useGroupedAccessorPairs",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("grouped-accessor-pairs").same()],
    }
}

impl Rule for UseGroupedAccessorPairs {
    type Query = Ast<AnySetter>;
    type State = MatchingPropertyAccessors;
    type Signals = Option<Self::State>;
    type Options = UseGroupedAccessorPairsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let accessors = get_matching_property_accessors(node)?;
        let is_adjacent = accessors
            .getter
            .position
            .abs_diff(accessors.setter.position)
            == 1;

        if !is_adjacent || accessors.getter.position > accessors.setter.position {
            Some(accessors)
        } else {
            None
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, accessors: &Self::State) -> Option<RuleDiagnostic> {
        let getter_name = accessors.getter.property_accessor.name().ok()?;
        let setter_name = accessors.setter.property_accessor.name().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                getter_name.range(),
                markup! {
                    "Getter should be defined right before the setter."
                },
            )
            .detail(
                setter_name.range(),
                markup! {
                    "Move this setter after the getter."
                },
            ),
        )
    }
}

/// A helper function to get the property accessors pair for a given setter.
fn get_matching_property_accessors(setter: &AnySetter) -> Option<MatchingPropertyAccessors> {
    let accessors =
        reduce_container_property_accessors(setter, (None, None), |member, accessors| {
            match member {
                PropertyAccessor::Setter(another_setter_info) => {
                    if another_setter_info.property_accessor.eq(setter) {
                        return (accessors.0, Some(another_setter_info));
                    }
                }
                PropertyAccessor::Getter(getter_info) => {
                    if getter_info
                        .property_accessor
                        .name()
                        .is_name_equal(&setter.name())
                    {
                        return (Some(getter_info), accessors.1);
                    }
                }
            }
            accessors
        })?;

    if let (Some(getter_property), Some(setter_property)) = accessors {
        return Some(MatchingPropertyAccessors {
            getter: getter_property,
            setter: setter_property,
        });
    }

    None
}

/// A helper function to reduce the property accessors in a container of any setter types.
fn reduce_container_property_accessors<R>(
    node: &AnySetter,
    initial: R,
    cb: impl Fn(PropertyAccessor, R) -> R,
) -> Option<R> {
    match node {
        AnySetter::JsSetterObjectMember(setter) => setter
            .parent::<JsObjectMemberList>()
            .map(|members| members.reduce_property_accessors(initial, cb)),
        AnySetter::JsSetterClassMember(setter) => setter
            .parent::<JsClassMemberList>()
            .map(|members| members.reduce_property_accessors(initial, cb)),
        AnySetter::TsSetterSignatureClassMember(setter) => setter
            .parent::<JsClassMemberList>()
            .map(|members| members.reduce_property_accessors(initial, cb)),
        AnySetter::TsSetterSignatureTypeMember(setter) => setter
            .parent::<TsTypeMemberList>()
            .map(|members| members.reduce_property_accessors(initial, cb)),
    }
}

/// A pair of property accessors (getter and setter).
pub struct MatchingPropertyAccessors {
    getter: PropertyAccessorInfo<AnyGetter>,
    setter: PropertyAccessorInfo<AnySetter>,
}

/// A helper struct to store the property accessor and its position in the list of members.
pub struct PropertyAccessorInfo<T> {
    property_accessor: T,
    position: usize,
}

pub enum PropertyAccessor {
    Getter(PropertyAccessorInfo<AnyGetter>),
    Setter(PropertyAccessorInfo<AnySetter>),
}

/// This trait abstracts the logic of iterating over property accessors
/// across 3 different property accessor containers:
/// - `JsObjectMemberList`
/// - `JsClassMemberList`
/// - `TsTypeMemberList`
trait ForEachPropertyAccessors {
    fn reduce_property_accessors<R>(&self, initial: R, cb: impl Fn(PropertyAccessor, R) -> R) -> R;
}

impl ForEachPropertyAccessors for JsObjectMemberList {
    fn reduce_property_accessors<R>(&self, initial: R, cb: impl Fn(PropertyAccessor, R) -> R) -> R {
        let mut result = initial;
        for (position, member) in self.iter().enumerate() {
            if let Ok(member) = member {
                match member {
                    AnyJsObjectMember::JsGetterObjectMember(getter) => {
                        result = cb(
                            PropertyAccessor::Getter(PropertyAccessorInfo {
                                property_accessor: AnyGetter::JsGetterObjectMember(getter),
                                position,
                            }),
                            result,
                        )
                    }
                    AnyJsObjectMember::JsSetterObjectMember(setter) => {
                        result = cb(
                            PropertyAccessor::Setter(PropertyAccessorInfo {
                                property_accessor: AnySetter::JsSetterObjectMember(setter),
                                position,
                            }),
                            result,
                        )
                    }
                    _ => {}
                }
            }
        }
        result
    }
}

impl ForEachPropertyAccessors for JsClassMemberList {
    fn reduce_property_accessors<R>(&self, initial: R, cb: impl Fn(PropertyAccessor, R) -> R) -> R {
        let mut result = initial;
        for (position, member) in self.iter().enumerate() {
            match member {
                AnyJsClassMember::JsGetterClassMember(getter) => {
                    result = cb(
                        PropertyAccessor::Getter(PropertyAccessorInfo {
                            property_accessor: AnyGetter::JsGetterClassMember(getter),
                            position,
                        }),
                        result,
                    )
                }
                AnyJsClassMember::TsGetterSignatureClassMember(getter) => {
                    result = cb(
                        PropertyAccessor::Getter(PropertyAccessorInfo {
                            property_accessor: AnyGetter::TsGetterSignatureClassMember(getter),
                            position,
                        }),
                        result,
                    )
                }
                AnyJsClassMember::JsSetterClassMember(setter) => {
                    result = cb(
                        PropertyAccessor::Setter(PropertyAccessorInfo {
                            property_accessor: AnySetter::JsSetterClassMember(setter),
                            position,
                        }),
                        result,
                    )
                }
                AnyJsClassMember::TsSetterSignatureClassMember(setter) => {
                    result = cb(
                        PropertyAccessor::Setter(PropertyAccessorInfo {
                            property_accessor: AnySetter::TsSetterSignatureClassMember(setter),
                            position,
                        }),
                        result,
                    )
                }
                _ => {}
            }
        }
        result
    }
}

impl ForEachPropertyAccessors for TsTypeMemberList {
    fn reduce_property_accessors<R>(&self, initial: R, cb: impl Fn(PropertyAccessor, R) -> R) -> R {
        let mut result = initial;
        for (position, member) in self.iter().enumerate() {
            match member {
                AnyTsTypeMember::TsGetterSignatureTypeMember(getter) => {
                    result = cb(
                        PropertyAccessor::Getter(PropertyAccessorInfo {
                            property_accessor: AnyGetter::TsGetterSignatureTypeMember(getter),
                            position,
                        }),
                        result,
                    )
                }
                AnyTsTypeMember::TsSetterSignatureTypeMember(setter) => {
                    result = cb(
                        PropertyAccessor::Setter(PropertyAccessorInfo {
                            property_accessor: AnySetter::TsSetterSignatureTypeMember(setter),
                            position,
                        }),
                        result,
                    )
                }
                _ => {}
            }
        }
        result
    }
}

/// This trait abstracts the logic of comparing two property accessor names.
/// We can't just get names as strings and compare them, because:
/// - `JsComputedMemberName` can be a static name or a dynamic name. We can only compare static.
/// - `JsPrivateClassMemberName` is a special case, we need to compare them by their identity.
/// - `JsMetavariable` is not supported, because it doesn't work with property accessors.
trait IsNameEqual {
    fn is_name_equal(&self, another_name: &Self) -> bool;
}

impl<T: IsNameEqual> IsNameEqual for SyntaxResult<T> {
    fn is_name_equal(&self, another_name: &Self) -> bool {
        match (&self, another_name) {
            (Ok(left), Ok(right)) => left.is_name_equal(right),
            _ => false,
        }
    }
}

impl IsNameEqual for JsPrivateClassMemberName {
    fn is_name_equal(&self, another_private_name: &Self) -> bool {
        match (self.id_token(), another_private_name.id_token()) {
            (Ok(left), Ok(right)) => left.text().eq(right.text()),
            _ => false,
        }
    }
}

impl IsNameEqual for AnyJsObjectMemberName {
    fn is_name_equal(&self, another_name: &Self) -> bool {
        match self {
            Self::JsMetavariable(_) => return false,
            _ => {
                if let Some(name) = self.as_static_name()
                    && let Some(another_name) = another_name.as_static_name()
                {
                    return name.text().eq(another_name.text());
                }
            }
        }
        false
    }
}

impl IsNameEqual for AnyJsClassMemberName {
    fn is_name_equal(&self, another_name: &Self) -> bool {
        match self {
            Self::JsPrivateClassMemberName(private) => {
                if let Self::JsPrivateClassMemberName(another_private) = another_name {
                    return private.is_name_equal(another_private);
                }
            }
            Self::JsMetavariable(_) => return false,
            _ => {
                if let Some(name) = self.as_static_name()
                    && let Some(another_name) = another_name.as_static_name()
                {
                    return name.text().eq(another_name.text());
                }
            }
        }
        false
    }
}

impl IsNameEqual for AnyPropertyAccessorName {
    fn is_name_equal(&self, another_name: &Self) -> bool {
        match (self, another_name) {
            (Self::AnyJsObjectMemberName(left), Self::AnyJsObjectMemberName(right)) => {
                left.is_name_equal(right)
            }
            (Self::AnyJsClassMemberName(left), Self::AnyJsClassMemberName(right)) => {
                left.is_name_equal(right)
            }
            _ => false,
        }
    }
}

/// This trait abstracts the logic of getting a static name from a property accessor.
/// We can only get a static name from `JsLiteralMemberName` and `JsComputedMemberName` (if static).
trait AsStaticName {
    fn as_static_name(&self) -> Option<StaticValue>;
}

impl AsStaticName for JsComputedMemberName {
    fn as_static_name(&self) -> Option<StaticValue> {
        self.expression()
            .ok()
            .and_then(|expr| expr.as_static_value())
    }
}

impl AsStaticName for JsLiteralMemberName {
    fn as_static_name(&self) -> Option<StaticValue> {
        self.value().ok().map(StaticValue::String)
    }
}

impl AsStaticName for JsPrivateClassMemberName {
    fn as_static_name(&self) -> Option<StaticValue> {
        None
    }
}

impl AsStaticName for JsMetavariable {
    fn as_static_name(&self) -> Option<StaticValue> {
        None
    }
}

impl AsStaticName for AnyJsObjectMemberName {
    fn as_static_name(&self) -> Option<StaticValue> {
        match self {
            Self::JsComputedMemberName(computed) => computed.as_static_name(),
            Self::JsLiteralMemberName(literal) => literal.as_static_name(),
            Self::JsMetavariable(_) => None,
        }
    }
}

impl AsStaticName for AnyJsClassMemberName {
    fn as_static_name(&self) -> Option<StaticValue> {
        match self {
            Self::JsComputedMemberName(computed) => computed.as_static_name(),
            Self::JsLiteralMemberName(literal) => literal.as_static_name(),
            Self::JsMetavariable(_) => None,
            Self::JsPrivateClassMemberName(_) => None,
        }
    }
}

declare_node_union! {
    pub AnyPropertyAccessorName = AnyJsObjectMemberName
        | AnyJsClassMemberName
}

declare_node_union! {
    pub AnyGetter = JsGetterObjectMember
        | JsGetterClassMember
        | TsGetterSignatureClassMember
        | TsGetterSignatureTypeMember
}

impl AnyGetter {
    fn name(&self) -> SyntaxResult<AnyPropertyAccessorName> {
        match self {
            Self::JsGetterObjectMember(getter) => Ok(
                AnyPropertyAccessorName::AnyJsObjectMemberName(getter.name()?),
            ),
            Self::JsGetterClassMember(getter) => Ok(AnyPropertyAccessorName::AnyJsClassMemberName(
                getter.name()?,
            )),
            Self::TsGetterSignatureClassMember(getter) => Ok(
                AnyPropertyAccessorName::AnyJsClassMemberName(getter.name()?),
            ),
            Self::TsGetterSignatureTypeMember(getter) => Ok(
                AnyPropertyAccessorName::AnyJsObjectMemberName(getter.name()?),
            ),
        }
    }
}

declare_node_union! {
    pub AnySetter = JsSetterObjectMember
        | JsSetterClassMember
        | TsSetterSignatureClassMember
        | TsSetterSignatureTypeMember
}

impl AnySetter {
    fn name(&self) -> SyntaxResult<AnyPropertyAccessorName> {
        match self {
            Self::JsSetterObjectMember(setter) => Ok(
                AnyPropertyAccessorName::AnyJsObjectMemberName(setter.name()?),
            ),
            Self::JsSetterClassMember(setter) => Ok(AnyPropertyAccessorName::AnyJsClassMemberName(
                setter.name()?,
            )),
            Self::TsSetterSignatureClassMember(setter) => Ok(
                AnyPropertyAccessorName::AnyJsClassMemberName(setter.name()?),
            ),
            Self::TsSetterSignatureTypeMember(setter) => Ok(
                AnyPropertyAccessorName::AnyJsObjectMemberName(setter.name()?),
            ),
        }
    }
}
