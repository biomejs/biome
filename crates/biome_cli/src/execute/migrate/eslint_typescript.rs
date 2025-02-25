use std::{cmp::Ordering, str::FromStr};

/// Configuration related to [TypeScript Eslint](https://typescript-eslint.io/).
///
/// Also, the module includes implementation to convert rule options to Biome's rule options.
use biome_deserialize::Deserializable;
use biome_deserialize_macros::Deserializable;
use biome_js_analyze::{
    lint::nursery::use_consistent_member_accessibility,
    lint::style::{use_consistent_array_type, use_naming_convention},
    utils::restricted_regex::RestrictedRegex,
};

use super::eslint_eslint;

#[derive(Debug, Default, Deserializable)]
pub(crate) struct ArrayTypeOptions {
    default: ArrayType,
    readonly: Option<ArrayType>,
}
impl From<ArrayTypeOptions> for use_consistent_array_type::ConsistentArrayTypeOptions {
    fn from(val: ArrayTypeOptions) -> Self {
        use_consistent_array_type::ConsistentArrayTypeOptions {
            syntax: val.default.into(),
        }
    }
}
#[derive(Debug, Default, Deserializable)]
pub(crate) enum ArrayType {
    #[default]
    Array,
    #[deserializable(rename = "array-simple")]
    ArraySimple,
    Generic,
}
impl From<ArrayType> for use_consistent_array_type::ConsistentArrayType {
    fn from(val: ArrayType) -> Self {
        match val {
            // NOTE: we translate `array-simple` to `array`.
            ArrayType::Array | ArrayType::ArraySimple => {
                use_consistent_array_type::ConsistentArrayType::Shorthand
            }
            ArrayType::Generic => use_consistent_array_type::ConsistentArrayType::Generic,
        }
    }
}

#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct ExplicitMemberAccessibilityOptions {
    accessibility: Option<AccessibilityLevel>,
}
impl From<ExplicitMemberAccessibilityOptions>
    for use_consistent_member_accessibility::ConsistentMemberAccessibilityOptions
{
    fn from(value: ExplicitMemberAccessibilityOptions) -> Self {
        use_consistent_member_accessibility::ConsistentMemberAccessibilityOptions {
            accessibility: value.accessibility.map(|x| x.into()).unwrap_or_default(),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, Deserializable)]
pub(crate) enum AccessibilityLevel {
    #[default]
    #[deserializable(rename = "no-public")]
    NoPublic,
    Explicit,
    None,
}
impl From<AccessibilityLevel> for use_consistent_member_accessibility::Accessibility {
    fn from(value: AccessibilityLevel) -> Self {
        match value {
            AccessibilityLevel::NoPublic => Self::NoPublic,
            AccessibilityLevel::Explicit => Self::Explicit,
            AccessibilityLevel::None => Self::None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct NamingConventionOptions(Vec<NamingConventionSelection>);
impl NamingConventionOptions {
    pub(crate) fn new(overrides: impl IntoIterator<Item = NamingConventionSelection>) -> Self {
        let mut inner: Vec<_> = overrides.into_iter().collect();
        // Order of the least general selection to the most geenral selection
        inner.sort_by(|a, b| a.precedence(b));
        Self(inner)
    }
}
impl From<NamingConventionOptions> for use_naming_convention::NamingConventionOptions {
    fn from(val: NamingConventionOptions) -> Self {
        let mut conventions = Vec::new();
        for selection in val.0 {
            if selection.types.is_some() || selection.filter.is_some() || selection.custom.is_some()
            {
                // We don't support types/filter/custom
                continue;
            }
            let matching = if selection.leading_underscore.is_some()
                || selection.trailing_underscore.is_some()
            {
                let leading_underscore = selection
                    .leading_underscore
                    .map_or("", |underscore| underscore.as_regex_part());
                let trailing_underscore = selection
                    .trailing_underscore
                    .map_or("", |underscore| underscore.as_regex_part());
                let regex = format!("{leading_underscore}([^_]*){trailing_underscore}");
                RestrictedRegex::from_str(&regex).ok()
            } else {
                None
            };
            let prefix = selection
                .prefix
                .iter()
                .map(|p| regex::escape(p))
                .collect::<Vec<_>>()
                .join("|");
            let suffix = selection
                .suffix
                .iter()
                .map(|p| regex::escape(p))
                .collect::<Vec<_>>()
                .join("|");
            let prefix = if prefix.is_empty() {
                prefix
            } else {
                format!("(?:{prefix})")
            };
            let suffix = if suffix.is_empty() {
                suffix
            } else {
                format!("(?:{suffix})")
            };
            let matching = if !prefix.is_empty() || !suffix.is_empty() {
                if matching.is_some() {
                    continue;
                }
                RestrictedRegex::try_from(format!("{prefix}(.*){suffix}")).ok()
            } else {
                matching
            };
            let selectors = selection.selectors();
            let formats = if let Some(format) = selection.format {
                format
                    .into_iter()
                    .map(use_naming_convention::Format::from)
                    .collect()
            } else {
                use_naming_convention::Formats::default()
            };
            for selector in selectors {
                conventions.push(use_naming_convention::Convention {
                    selector,
                    matching: matching.clone(),
                    formats,
                });
            }
        }
        use_naming_convention::NamingConventionOptions {
            strict_case: false,
            require_ascii: false,
            conventions: conventions.into_boxed_slice(),
        }
    }
}
#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct NamingConventionSelection {
    pub(crate) selector: eslint_eslint::ShorthandVec<Selector>,
    pub(crate) modifiers: Option<Vec<Modifier>>,
    pub(crate) types: Option<Vec<Type>>,
    pub(crate) custom: Option<Anything>,
    pub(crate) format: Option<Vec<NamingConventionCase>>,
    pub(crate) leading_underscore: Option<Underscore>,
    pub(crate) trailing_underscore: Option<Underscore>,
    pub(crate) prefix: Vec<String>,
    pub(crate) suffix: Vec<String>,
    pub(crate) filter: Option<Anything>,
}
impl NamingConventionSelection {
    fn precedence(&self, other: &Self) -> Ordering {
        // Simplification: We compare only the first selectors.
        let selector = self.selector.iter().next();
        let other_selector = other.selector.iter().next();
        match selector.cmp(&other_selector) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match (&self.types, &other.types) {
            (None, None) | (Some(_), Some(_)) => {}
            (None, Some(_)) => return Ordering::Greater,
            (Some(_), None) => return Ordering::Less,
        }
        match (&self.modifiers, &other.modifiers) {
            (None, None) | (Some(_), Some(_)) => {}
            (None, Some(_)) => return Ordering::Greater,
            (Some(_), None) => return Ordering::Less,
        }
        Ordering::Equal
    }

    fn selectors(&self) -> Vec<use_naming_convention::Selector> {
        let mut result = Vec::new();
        let modifiers: use_naming_convention::Modifiers = self
            .modifiers
            .iter()
            .flatten()
            .filter_map(|m| m.as_modifier())
            .collect();
        let has_class_modifier =
            modifiers.contains(use_naming_convention::RestrictedModifier::Abstract);
        let has_class_member_modifier = modifiers
            .contains(use_naming_convention::RestrictedModifier::Private)
            || modifiers.contains(use_naming_convention::RestrictedModifier::Protected);
        let has_property_modifier =
            modifiers.contains(use_naming_convention::RestrictedModifier::Readonly);
        modifiers.contains(use_naming_convention::RestrictedModifier::Private);
        let scope = self
            .modifiers
            .iter()
            .flatten()
            .find_map(|m| m.as_scope())
            .unwrap_or_default();
        for selector in self.selector.iter() {
            match selector {
                Selector::AutoAccessor => {
                    // currently unsupported by Biome
                    continue;
                }
                Selector::Class => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::Class,
                        modifiers,
                        scope,
                    });
                }
                Selector::ClassMethod => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ClassMethod,
                        modifiers,
                        scope,
                    });
                }
                Selector::ClassProperty => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ClassProperty,
                        modifiers,
                        scope,
                    });
                }
                Selector::Enum => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::Enum,
                        modifiers,
                        scope,
                    });
                }
                Selector::EnumMember => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::EnumMember,
                        modifiers,
                        scope,
                    });
                }
                Selector::Function => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::Function,
                        modifiers,
                        scope,
                    });
                }
                Selector::Import => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ImportNamespace,
                        modifiers,
                        scope,
                    });
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ImportAlias,
                        modifiers,
                        scope,
                    });
                }
                Selector::Interface => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::Interface,
                        modifiers,
                        scope,
                    });
                }
                Selector::ObjectLiteralMethod => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ObjectLiteralMethod,
                        modifiers,
                        scope,
                    });
                }
                Selector::ObjectLiteralProperty => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ObjectLiteralProperty,
                        modifiers,
                        scope,
                    });
                }
                Selector::Parameter => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::FunctionParameter,
                        modifiers,
                        scope,
                    });
                }
                Selector::ParameterProperty => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ClassProperty,
                        modifiers,
                        scope,
                    });
                }
                Selector::TypeAlias => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::TypeAlias,
                        modifiers,
                        scope,
                    });
                }
                Selector::TypeMethod => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::TypeMethod,
                        modifiers,
                        scope,
                    });
                }
                Selector::TypeParameter => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::TypeParameter,
                        modifiers,
                        scope,
                    });
                }
                Selector::TypeProperty => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::TypeProperty,
                        modifiers,
                        scope,
                    });
                }
                Selector::Variable => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::Variable,
                        modifiers,
                        scope,
                    });
                }
                Selector::Default => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::Any,
                        modifiers,
                        scope,
                    });
                }
                Selector::ClassicAccessor | Selector::Accessor => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ClassGetter,
                        modifiers,
                        scope,
                    });
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ClassSetter,
                        modifiers,
                        scope,
                    });
                    if !has_class_member_modifier {
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::ObjectLiteralGetter,
                            modifiers,
                            scope,
                        });
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::ObjectLiteralSetter,
                            modifiers,
                            scope,
                        });
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::TypeGetter,
                            modifiers,
                            scope,
                        });
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::TypeSetter,
                            modifiers,
                            scope,
                        });
                    }
                }
                Selector::MemberLike => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ClassMember,
                        modifiers,
                        scope,
                    });
                    if !has_class_member_modifier {
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::ObjectLiteralMember,
                            modifiers,
                            scope,
                        });
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::TypeMember,
                            modifiers,
                            scope,
                        });
                    }
                }
                Selector::Method => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ClassMethod,
                        modifiers,
                        scope,
                    });
                    if !has_class_member_modifier {
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::ObjectLiteralMethod,
                            modifiers,
                            scope,
                        });
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::TypeMethod,
                            modifiers,
                            scope,
                        });
                    }
                }
                Selector::Property => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::ClassProperty,
                        modifiers,
                        scope,
                    });
                    if !has_class_member_modifier {
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::TypeProperty,
                            modifiers,
                            scope,
                        });
                        if !has_property_modifier {
                            result.push(use_naming_convention::Selector {
                                kind: use_naming_convention::Kind::ObjectLiteralProperty,
                                modifiers,
                                scope,
                            });
                        }
                    }
                }
                Selector::TypeLike => {
                    if has_class_modifier {
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::Class,
                            modifiers,
                            scope,
                        });
                    } else {
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::TypeLike,
                            modifiers,
                            scope,
                        });
                    }
                }
                Selector::VariableLike => {
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::Variable,
                        modifiers,
                        scope,
                    });
                    result.push(use_naming_convention::Selector {
                        kind: use_naming_convention::Kind::Function,
                        modifiers,
                        scope,
                    });
                    if scope != use_naming_convention::Scope::Global {
                        result.push(use_naming_convention::Selector {
                            kind: use_naming_convention::Kind::FunctionParameter,
                            modifiers,
                            scope,
                        });
                    }
                }
            }
        }
        // Remove invalid selectors.
        // This avoids to generate errors when loading the Biome configuration.
        result.retain(|selector| selector.check().is_ok());
        result
    }
}
#[derive(Debug)]
pub(crate) struct Anything;
impl Deserializable for Anything {
    fn deserialize(
        _ctx: &mut impl biome_deserialize::DeserializationContext,
        _value: &impl biome_deserialize::DeserializableValue,
        _name: &str,
    ) -> Option<Self> {
        Some(Anything)
    }
}
#[derive(Copy, Clone, Debug, Deserializable)]
pub(crate) enum NamingConventionCase {
    #[deserializable(rename = "camelCase")]
    Camel,
    #[deserializable(rename = "strictCamelCase")]
    StrictCamel,
    #[deserializable(rename = "PascalCase")]
    Pascal,
    #[deserializable(rename = "StrictPascalCase")]
    StrictPascal,
    #[deserializable(rename = "snake_case")]
    Snake,
    #[deserializable(rename = "UPPER_CASE")]
    Upper,
}
impl From<NamingConventionCase> for use_naming_convention::Format {
    fn from(value: NamingConventionCase) -> Self {
        match value {
            NamingConventionCase::Camel | NamingConventionCase::StrictCamel => Self::Camel,
            NamingConventionCase::Pascal | NamingConventionCase::StrictPascal => Self::Pascal,
            NamingConventionCase::Snake => Self::Snake,
            NamingConventionCase::Upper => Self::Constant,
        }
    }
}
#[derive(Debug, Default, Deserializable, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) enum Selector {
    // Order is important, it reflects the precedence relation between selectors
    // Individual selectors
    ClassicAccessor,
    AutoAccessor,
    Class,
    ClassMethod,
    ClassProperty,
    Enum,
    EnumMember,
    Function,
    Import,
    Interface,
    ObjectLiteralMethod,
    ObjectLiteralProperty,
    Parameter,
    ParameterProperty,
    TypeAlias,
    TypeMethod,
    TypeParameter,
    TypeProperty,
    Variable,
    // group selector
    Accessor,
    Method,
    Property,
    TypeLike,
    VariableLike,
    MemberLike,
    #[default]
    Default,
}
#[derive(Copy, Clone, Debug, Deserializable)]
pub(crate) enum Modifier {
    Abstract,
    Async,
    Const,
    Destructured,
    Exported,
    Global,
    Override,
    Private,
    Protected,
    Public,
    Readonly,
    RequiresQuotes,
    #[deserializable(rename = "#private")]
    SharpPrivate,
    Static,
    Unused,
}
impl Modifier {
    fn as_modifier(self) -> Option<use_naming_convention::RestrictedModifier> {
        match self {
            Modifier::Abstract => Some(use_naming_convention::RestrictedModifier::Abstract),
            Modifier::Private => Some(use_naming_convention::RestrictedModifier::Private),
            Modifier::Protected => Some(use_naming_convention::RestrictedModifier::Protected),
            Modifier::Readonly => Some(use_naming_convention::RestrictedModifier::Readonly),
            Modifier::Static => Some(use_naming_convention::RestrictedModifier::Static),
            _ => None,
        }
    }
    fn as_scope(self) -> Option<use_naming_convention::Scope> {
        match self {
            Modifier::Global => Some(use_naming_convention::Scope::Global),
            _ => None,
        }
    }
}
#[derive(Debug, Deserializable)]
pub(crate) enum Type {
    Array,
    Boolean,
    Function,
    Number,
    String,
}
#[derive(Clone, Copy, Debug, Deserializable)]
pub(crate) enum Underscore {
    Forbid,
    Require,
    RequireDouble,
    Allow,
    AllowDouble,
    AllowSingleOrDouble,
}
impl Underscore {
    fn as_regex_part(self) -> &'static str {
        match self {
            Self::Forbid => "",
            Self::Require => "_",
            Self::RequireDouble => "__",
            Self::Allow => "_?",
            Self::AllowDouble => "(?:__)?",
            Self::AllowSingleOrDouble => "_?_?",
        }
    }
}
