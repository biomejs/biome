use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsObjectMember, JsObjectExpression, JsSyntaxKind};
use biome_rowan::{AstNode, BatchMutationExt, NodeOrToken, TokenText};
use biome_rule_options::no_duplicate_object_keys::NoDuplicateObjectKeysOptions;
use rustc_hash::FxHashMap;
use std::collections::hash_map;
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
        sources: &[RuleSource::Eslint("no-dupe-keys").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoDuplicateObjectKeys {
    type Query = Ast<JsObjectExpression>;
    type State = PropertyConflict;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateObjectKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut defined_properties: FxHashMap<TokenText, DefinedProperties> = FxHashMap::default();
        let mut signals = Vec::new();

        for (member_name, defined_property) in node
            .members()
            .into_iter()
            .flatten()
            .filter_map(|member| Some((member.name()?, DefinedProperty::try_from(&member).ok()?)))
            // Note that we iterate from last to first property, so that we highlight properties being overwritten as problems and not those that take effect.
            .rev()
        {
            match defined_properties.entry(member_name) {
                hash_map::Entry::Occupied(entry) => {
                    if let Some(conflict) = entry.into_mut().extend_with(defined_property) {
                        signals.push(conflict);
                    }
                }
                hash_map::Entry::Vacant(entry) => {
                    entry.insert(defined_property.into());
                }
            }
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(
        ctx: &RuleContext<Self>,
        PropertyConflict(overridden_properties, defined_property): &Self::State,
    ) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let members = node.members().into_syntax();
        let range = members
            .slots()
            .nth(defined_property.index() as usize)?
            .into_node()?
            .text_trimmed_range();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            range,
            format!(
                "This {defined_property} is later overwritten by an object member with the same name."
            ),
        );
        let diagnostic = match overridden_properties {
            DefinedProperties::Get(index)
            | DefinedProperties::Set(index)
            | DefinedProperties::Method(index)
            | DefinedProperties::Property(index) => {
                let range = members
                    .slots()
                    .nth(*index as usize)?
                    .into_node()?
                    .text_trimmed_range();
                diagnostic.detail(range, markup! { "Overwritten with this "{format_args!("{}", overridden_properties)}"." })
            }
            DefinedProperties::GetSet(getter_index, setter_index) => {
                let getter_range = members
                    .slots()
                    .nth(*getter_index as usize)?
                    .into_node()?
                    .text_trimmed_range();
                let setter_range = members
                    .slots()
                    .nth(*setter_index as usize)?
                    .into_node()?
                    .text_trimmed_range();
                match defined_property {
                    DefinedProperty::Get(_) => {
                        diagnostic.detail(getter_range, "Overwritten with this getter.")
                    }
                    DefinedProperty::Set(_) => {
                        diagnostic.detail(setter_range, "Overwritten with this setter.")
                    }
                    DefinedProperty::Method(_) | DefinedProperty::Property(_) => {
                        if getter_range < setter_range {
                            diagnostic.detail(setter_range, "Overwritten with this setter.")
                        } else {
                            diagnostic.detail(getter_range, "Overwritten with this getter.")
                        }
                    }
                }
            }
        };
        Some(diagnostic.note("If an object property with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored."))
    }

    fn action(
        ctx: &RuleContext<Self>,
        PropertyConflict(overridden_properties, defined_property): &Self::State,
    ) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query();
        let members = node.members().into_syntax();
        let member = members
            .slots()
            .nth(defined_property.index() as usize)?
            .into_node()?;
        if member.has_leading_comments() {
            match overridden_properties {
                DefinedProperties::Get(index)
                | DefinedProperties::Set(index)
                | DefinedProperties::Method(index)
                | DefinedProperties::Property(index) => {
                    let node = members.slots().nth(*index as usize)?.into_node()?;
                    let new_node = node
                        .clone()
                        .prepend_trivia_pieces(member.first_leading_trivia()?.pieces())?;
                    mutation.replace_element_discard_trivia(node.into(), new_node.into());
                }
                DefinedProperties::GetSet(getter_index, setter_index) => {
                    let getter = members.slots().nth(*getter_index as usize)?.into_node()?;
                    let setter = members.slots().nth(*setter_index as usize)?.into_node()?;
                    if matches!(defined_property, DefinedProperty::Set(_))
                        || getter.text_trimmed_range() < setter.text_trimmed_range()
                    {
                        let new_setter = setter
                            .clone()
                            .prepend_trivia_pieces(member.first_leading_trivia()?.pieces())?;
                        mutation.replace_element_discard_trivia(setter.into(), new_setter.into());
                    } else {
                        let new_getter = getter
                            .clone()
                            .prepend_trivia_pieces(member.first_leading_trivia()?.pieces())?;
                        mutation.replace_element_discard_trivia(getter.into(), new_getter.into());
                    }
                }
            }
        }
        if let Some(NodeOrToken::Token(next_token)) = member.next_sibling_or_token()
            && next_token.kind() == JsSyntaxKind::COMMA
        {
            mutation.remove_token(next_token);
        }
        mutation.remove_element(member.into());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            // The property initialization could contain side effects
            ctx.metadata().applicability(),
            markup!("Remove this " {format_args!("{}", defined_property)} ".").to_owned(),
            mutation,
        ))
    }
}

pub struct PropertyConflict(DefinedProperties, DefinedProperty);

/// A descriptor for a property that is, as far as we can tell from statically analyzing the object expression,
/// not overwritten by another object member and will make it into the object.
#[derive(Clone)]
enum DefinedProperty {
    Get(u32),
    Set(u32),
    Method(u32),
    Property(u32),
}
impl DefinedProperty {
    fn index(&self) -> u32 {
        match self {
            Self::Get(index) | Self::Set(index) | Self::Method(index) | Self::Property(index) => {
                *index
            }
        }
    }
}
impl TryFrom<&AnyJsObjectMember> for DefinedProperty {
    type Error = ();
    fn try_from(value: &AnyJsObjectMember) -> Result<Self, Self::Error> {
        match value {
            AnyJsObjectMember::JsGetterObjectMember(member) => {
                Ok(Self::Get(member.syntax().index() as u32))
            }
            AnyJsObjectMember::JsMethodObjectMember(member) => {
                Ok(Self::Method(member.syntax().index() as u32))
            }
            AnyJsObjectMember::JsPropertyObjectMember(member) => {
                Ok(Self::Property(member.syntax().index() as u32))
            }
            AnyJsObjectMember::JsSetterObjectMember(member) => {
                Ok(Self::Set(member.syntax().index() as u32))
            }
            AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => {
                Ok(Self::Property(member.syntax().index() as u32))
            }
            AnyJsObjectMember::JsBogusMember(_)
            | AnyJsObjectMember::JsSpread(_)
            | AnyJsObjectMember::JsMetavariable(_) => Err(()),
        }
    }
}
impl Display for DefinedProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Get(_) => "getter",
            Self::Set(_) => "setter",
            Self::Method(_) => "method",
            Self::Property(_) => "property",
        })
    }
}

#[derive(Clone)]
enum DefinedProperties {
    Get(u32),
    Set(u32),
    GetSet(u32, u32),
    Method(u32),
    Property(u32),
}
impl DefinedProperties {
    fn extend_with(&mut self, other: DefinedProperty) -> Option<PropertyConflict> {
        match (&self, other) {
            // Add missing get/set counterpart
            (Self::Set(setter_index), DefinedProperty::Get(getter_index)) => {
                *self = Self::GetSet(getter_index, *setter_index);
                None
            }
            (Self::Get(getter_index), DefinedProperty::Set(setter_index)) => {
                *self = Self::GetSet(*getter_index, setter_index);
                None
            }
            // Else conflict
            (_, other) => Some(PropertyConflict(self.clone(), other)),
        }
    }
}
impl From<DefinedProperty> for DefinedProperties {
    fn from(value: DefinedProperty) -> Self {
        match value {
            DefinedProperty::Get(index) => Self::Get(index),
            DefinedProperty::Set(index) => Self::Set(index),
            DefinedProperty::Method(index) => Self::Method(index),
            DefinedProperty::Property(index) => Self::Property(index),
        }
    }
}
impl Display for DefinedProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Get(_) => "getter",
            Self::Set(_) => "setter",
            Self::GetSet(_, _) => "getter and setter",
            Self::Method(_) => "method",
            Self::Property(_) => "property",
        })
    }
}
