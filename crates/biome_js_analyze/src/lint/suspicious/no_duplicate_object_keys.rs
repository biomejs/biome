use crate::utils::batch::JsBatchMutation;
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsObjectMember, JsGetterObjectMember, JsObjectExpression, JsSetterObjectMember,
};
use biome_js_syntax::{
    JsMethodObjectMember, JsPropertyObjectMember, JsShorthandPropertyObjectMember, TextRange,
};
use biome_rowan::{AstNode, BatchMutationExt, TokenText};
use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::fmt::Display;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow two keys with the same name inside objects.
    ///
    /// If an object property with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored, which is likely a mistake.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const obj = {
    ///		a: 1,
    ///		a: 2,
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const obj = {
    ///		set a(v) {},
    ///		a: 2,
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const obj = {
    ///		a: 1,
    ///		b: 2,
    /// }
    /// ```
    ///
    /// ```js
    /// const obj = {
    ///		get a() { return 1; },
    ///		set a(v) {},
    /// }
    /// ```
    ///
    pub NoDuplicateObjectKeys {
        version: "1.0.0",
        name: "noDuplicateObjectKeys",
        language: "js",
        sources: &[RuleSource::Eslint("no-dupe-keys")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

/// An object member defining a single object property.
enum MemberDefinition {
    Getter(JsGetterObjectMember),
    Setter(JsSetterObjectMember),
    Method(JsMethodObjectMember),
    Property(JsPropertyObjectMember),
    ShorthandProperty(JsShorthandPropertyObjectMember),
}
impl MemberDefinition {
    fn name(&self) -> Option<TokenText> {
        match self {
            Self::Getter(getter) => getter.name().ok()?.as_js_literal_member_name()?.name().ok(),
            Self::Setter(setter) => setter.name().ok()?.as_js_literal_member_name()?.name().ok(),
            Self::Method(method) => method.name().ok()?.as_js_literal_member_name()?.name().ok(),
            Self::Property(property) => property
                .name()
                .ok()?
                .as_js_literal_member_name()?
                .name()
                .ok(),
            Self::ShorthandProperty(shorthand_property) => Some(
                shorthand_property
                    .name()
                    .ok()?
                    .value_token()
                    .ok()?
                    .token_text_trimmed(),
            ),
        }
    }

    fn range(&self) -> TextRange {
        match self {
            Self::Getter(getter) => getter.range(),
            Self::Setter(setter) => setter.range(),
            Self::Method(method) => method.range(),
            Self::Property(property) => property.range(),
            Self::ShorthandProperty(shorthand_property) => shorthand_property.range(),
        }
    }

    fn node(&self) -> AnyJsObjectMember {
        match self {
            Self::Getter(getter) => getter.clone().into(),
            Self::Setter(setter) => setter.clone().into(),
            Self::Method(method) => method.clone().into(),
            Self::Property(property) => property.clone().into(),
            Self::ShorthandProperty(shorthand_property) => shorthand_property.clone().into(),
        }
    }
}
impl Display for MemberDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Getter(_) => "getter",
            Self::Setter(_) => "setter",
            Self::Method(_) => "method",
            Self::Property(_) => "property value",
            Self::ShorthandProperty(_) => "shorthand property",
        })?;
        if let Some(name) = self.name() {
            f.write_str(" named ")?;
            f.write_str(&name)?;
        }
        Ok(())
    }
}
enum MemberDefinitionError {
    NotASinglePropertyMember,
    BogusMemberType,
}
impl TryFrom<AnyJsObjectMember> for MemberDefinition {
    type Error = MemberDefinitionError;

    fn try_from(member: AnyJsObjectMember) -> Result<Self, Self::Error> {
        match member {
            AnyJsObjectMember::JsGetterObjectMember(member) => Ok(Self::Getter(member)),
            AnyJsObjectMember::JsSetterObjectMember(member) => Ok(Self::Setter(member)),
            AnyJsObjectMember::JsMethodObjectMember(member) => Ok(Self::Method(member)),
            AnyJsObjectMember::JsPropertyObjectMember(member) => Ok(Self::Property(member)),
            AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => {
                Ok(Self::ShorthandProperty(member))
            }
            AnyJsObjectMember::JsSpread(_) => Err(MemberDefinitionError::NotASinglePropertyMember),
            AnyJsObjectMember::JsBogusMember(_) => Err(MemberDefinitionError::BogusMemberType),
        }
    }
}

/// A descriptor for a property that is, as far as we can tell from statically analyzing the object expression,
/// not overwritten by another object member and will make it into the object.
#[derive(Clone)]
enum DefinedProperty {
    Get(TextRange),
    Set(TextRange),
    GetSet(TextRange, TextRange),
    Value(TextRange),
}
impl From<MemberDefinition> for DefinedProperty {
    fn from(definition: MemberDefinition) -> Self {
        match definition {
            MemberDefinition::Getter(getter) => Self::Get(getter.range()),
            MemberDefinition::Setter(setter) => Self::Set(setter.range()),
            MemberDefinition::Method(method) => Self::Value(method.range()),
            MemberDefinition::Property(property) => Self::Value(property.range()),
            MemberDefinition::ShorthandProperty(shorthand_property) => {
                Self::Value(shorthand_property.range())
            }
        }
    }
}

pub struct PropertyConflict(DefinedProperty, MemberDefinition);
impl DefinedProperty {
    fn extend_with(&self, member_definition: MemberDefinition) -> Result<Self, PropertyConflict> {
        match (self, member_definition) {
            // Add missing get/set counterpart
            (Self::Set(set_range), MemberDefinition::Getter(getter)) => {
                Ok(Self::GetSet(getter.range(), *set_range))
            }

            (Self::Get(get_range), MemberDefinition::Setter(setter)) => {
                Ok(Self::GetSet(*get_range, setter.range()))
            }
            // Else conflict
            (defined_property, member_definition) => Err(PropertyConflict(
                defined_property.clone(),
                member_definition,
            )),
        }
    }
}

impl Rule for NoDuplicateObjectKeys {
    type Query = Ast<JsObjectExpression>;
    type State = PropertyConflict;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut defined_properties = FxHashMap::default();
        let mut signals = Vec::new();

        for member_definition in node
            .members()
            .into_iter()
            .flatten()
            .filter_map(|member| MemberDefinition::try_from(member).ok())
            // Note that we iterate from last to first property, so that we highlight properties being overwritten as problems and not those that take effect.
            .rev()
        {
            if let Some(member_name) = member_definition.name() {
                match defined_properties.remove(&member_name) {
                    None => {
                        defined_properties
                            .insert(member_name, DefinedProperty::from(member_definition));
                    }
                    Some(defined_property) => {
                        match defined_property.extend_with(member_definition) {
                            Ok(new_defined_property) => {
                                defined_properties.insert(member_name, new_defined_property);
                            }
                            Err(conflict) => {
                                signals.push(conflict);
                                defined_properties.insert(member_name, defined_property);
                            }
                        }
                    }
                }
            }
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        PropertyConflict(defined_property, member_definition): &Self::State,
    ) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            member_definition.range(),
            format!(
                "This {member_definition} is later overwritten by an object member with the same name."
            ),
        );
        diagnostic = match defined_property {
            DefinedProperty::Get(range) => {
                diagnostic.detail(range, "Overwritten with this getter.")
            }
            DefinedProperty::Set(range) => {
                diagnostic.detail(range, "Overwritten with this setter.")
            }
            DefinedProperty::Value(range) => {
                diagnostic.detail(range, "Overwritten with this value.")
            }
            DefinedProperty::GetSet(get_range, set_range) => match member_definition {
                MemberDefinition::Getter(_) => {
                    diagnostic.detail(get_range, "Overwritten with this getter.")
                }
                MemberDefinition::Setter(_) => {
                    diagnostic.detail(set_range, "Overwritten with this setter.")
                }
                MemberDefinition::Method(_)
                | MemberDefinition::Property(_)
                | MemberDefinition::ShorthandProperty(_) => match get_range.ordering(*set_range) {
                    Ordering::Less => diagnostic.detail(set_range, "Overwritten with this setter."),
                    Ordering::Greater => {
                        diagnostic.detail(get_range, "Overwritten with this getter.")
                    }
                    Ordering::Equal => {
                        panic!(
                            "The ranges of the property getter and property setter cannot overlap."
                        )
                    }
                },
            },
        };
        diagnostic = diagnostic.note("If an object property with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored.");

        Some(diagnostic)
    }

    fn action(
        ctx: &RuleContext<Self>,
        PropertyConflict(_, member_definition): &Self::State,
    ) -> Option<JsRuleAction> {
        let mut batch = ctx.root().begin();
        batch.remove_js_object_member(member_definition.node());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            // The property initialization could contain side effects
            ctx.metadata().applicability(),
            markup!("Remove this " {member_definition.to_string()}).to_owned(),
            batch,
        ))
    }
}
