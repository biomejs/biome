use crate::context::Semicolons;
use crate::prelude::*;
use crate::utils::{AnyJsAssignmentLike, FormatSemicolon};
use biome_formatter::write;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, JsInitializerClause, JsPropertyClassMember,
    JsSyntaxToken, TsInitializedPropertySignatureClassMember, TsPropertySignatureClassMember,
};
use biome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsPropertyClassMember;

impl FormatNodeRule<JsPropertyClassMember> for FormatJsPropertyClassMember {
    fn fmt_fields(&self, node: &JsPropertyClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let semicolon_token = node.semicolon_token();
        write!(
            f,
            [
                &AnyJsAssignmentLike::from(node.clone()),
                FormatClassPropertySemicolon {
                    semicolon: semicolon_token.as_ref(),
                    property: &AnyJsPropertyClassMember::from(node.clone())
                }
            ]
        )
    }
}

declare_node_union! {
    pub(crate) AnyJsPropertyClassMember = JsPropertyClassMember | TsPropertySignatureClassMember | TsInitializedPropertySignatureClassMember
}

impl AnyJsPropertyClassMember {
    fn name(&self) -> SyntaxResult<AnyJsClassMemberName> {
        match self {
            AnyJsPropertyClassMember::JsPropertyClassMember(property) => property.name(),
            AnyJsPropertyClassMember::TsPropertySignatureClassMember(property) => property.name(),
            AnyJsPropertyClassMember::TsInitializedPropertySignatureClassMember(property) => {
                property.name()
            }
        }
    }

    fn value(&self) -> Option<JsInitializerClause> {
        match self {
            AnyJsPropertyClassMember::JsPropertyClassMember(property) => property.value(),
            AnyJsPropertyClassMember::TsPropertySignatureClassMember(_) => None,
            AnyJsPropertyClassMember::TsInitializedPropertySignatureClassMember(property) => {
                property.value().ok()
            }
        }
    }

    fn has_property_annotation(&self) -> bool {
        match self {
            AnyJsPropertyClassMember::JsPropertyClassMember(property) => {
                property.property_annotation().is_some()
            }
            AnyJsPropertyClassMember::TsPropertySignatureClassMember(property) => {
                property.property_annotation().is_some()
            }
            AnyJsPropertyClassMember::TsInitializedPropertySignatureClassMember(_) => false,
        }
    }
}

pub(crate) struct FormatClassPropertySemicolon<'a> {
    property: &'a AnyJsPropertyClassMember,
    semicolon: Option<&'a JsSyntaxToken>,
}

impl<'a> FormatClassPropertySemicolon<'a> {
    pub fn new(
        property: &'a AnyJsPropertyClassMember,
        semicolon: Option<&'a JsSyntaxToken>,
    ) -> Self {
        Self {
            property,
            semicolon,
        }
    }
}

impl Format<JsFormatContext> for FormatClassPropertySemicolon<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match f.options().semicolons() {
            Semicolons::Always => FormatSemicolon::new(self.semicolon).fmt(f),
            Semicolons::AsNeeded => match (self.semicolon, needs_semicolon(self.property)?) {
                (None, false) => Ok(()),
                (Some(semicolon), true) => semicolon.format().fmt(f),

                (Some(semicolon), false) => format_removed(semicolon).fmt(f),
                (None, true) => FormatSemicolon::new(self.semicolon).fmt(f),
            },
        }
    }
}

fn needs_semicolon(property: &AnyJsPropertyClassMember) -> SyntaxResult<bool> {
    if let AnyJsClassMemberName::JsLiteralMemberName(name) = property.name()? {
        // `get;`, `set;` or `static`
        if matches!(name.value()?.text_trimmed(), "static" | "get" | "set")
            && property.value().is_none()
            && !property.has_property_annotation()
        {
            return Ok(true);
        }
    }

    let Some(next_member) = property
        .syntax()
        .next_sibling()
        .and_then(AnyJsClassMember::cast)
    else {
        return Ok(false);
    };

    // a;
    // static b;
    if has_modifiers(&next_member) {
        return Ok(false);
    }

    // a = b;
    // instanceof;
    if property.value().is_some()
        && (next_member.has_name("instanceof")? || next_member.has_name("in")?)
    {
        return Ok(true);
    }

    Ok(match next_member {
        AnyJsClassMember::TsConstructorSignatureClassMember(_)
        | AnyJsClassMember::JsConstructorClassMember(_)
        | AnyJsClassMember::JsEmptyClassMember(_)
        // `get`, `set`, and `static` start with a keyword -> no need for semi
        | AnyJsClassMember::JsStaticInitializationBlockClassMember(_)
        | AnyJsClassMember::JsGetterClassMember(_)
        | AnyJsClassMember::TsGetterSignatureClassMember(_)
        | AnyJsClassMember::TsSetterSignatureClassMember(_)
        | AnyJsClassMember::JsSetterClassMember(_)
        | AnyJsClassMember::JsMetavariable(_) => false,

        // Computed members may be misinterpreted as array accessors/array types
        member @
(AnyJsClassMember::JsPropertyClassMember(_) |
AnyJsClassMember::TsPropertySignatureClassMember(_) |
AnyJsClassMember::TsInitializedPropertySignatureClassMember(_)) => match member.name()? {
            Some(name) => name.is_computed(),
            None => false,
        },

        // When the name starts with the generator token or `[`
        AnyJsClassMember::JsMethodClassMember(method) => {
            method.async_token().is_none()
                && (method.name()?.is_computed() || method.star_token().is_some())
        }

        // When the name starts with a `[`
        AnyJsClassMember::TsMethodSignatureClassMember(method) => {
            method.async_token().is_none() && method.name()?.is_computed()
        }

        // Keep it, just to be safe
        AnyJsClassMember::JsBogusMember(_) => true,

        AnyJsClassMember::TsIndexSignatureClassMember(_) => true,
    })
}

/// Tests if `member` has any modifiers
fn has_modifiers(member: &AnyJsClassMember) -> bool {
    let is_empty = match member {
        AnyJsClassMember::JsConstructorClassMember(constructor) => {
            constructor.modifiers().is_empty()
        }
        AnyJsClassMember::JsEmptyClassMember(_) => true,
        AnyJsClassMember::JsGetterClassMember(getter) => getter.modifiers().is_empty(),
        AnyJsClassMember::JsMethodClassMember(method) => method.modifiers().is_empty(),
        AnyJsClassMember::JsPropertyClassMember(property) => property.modifiers().is_empty(),
        AnyJsClassMember::JsSetterClassMember(setter) => setter.modifiers().is_empty(),
        AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => true,
        AnyJsClassMember::JsBogusMember(_) | AnyJsClassMember::JsMetavariable(_) => true,
        AnyJsClassMember::TsConstructorSignatureClassMember(constructor) => {
            constructor.modifiers().is_empty()
        }
        AnyJsClassMember::TsGetterSignatureClassMember(getter) => getter.modifiers().is_empty(),
        AnyJsClassMember::TsIndexSignatureClassMember(index) => index.modifiers().is_empty(),
        AnyJsClassMember::TsMethodSignatureClassMember(method) => method.modifiers().is_empty(),
        AnyJsClassMember::TsPropertySignatureClassMember(property) => {
            property.modifiers().is_empty()
        }
        AnyJsClassMember::TsInitializedPropertySignatureClassMember(property) => {
            property.modifiers().is_empty()
        }
        AnyJsClassMember::TsSetterSignatureClassMember(setter) => setter.modifiers().is_empty(),
    };

    !is_empty
}
