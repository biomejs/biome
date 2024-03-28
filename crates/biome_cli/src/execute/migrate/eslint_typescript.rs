/// Configuration related to [TypeScript Eslint](https://typescript-eslint.io/).
///
/// ALso, the module includes implementation to convert rule options to Biome's rule options.
use biome_deserialize_macros::Deserializable;
use biome_js_analyze::lint::style::{use_consistent_array_type, use_naming_convention};

use super::eslint;

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

#[derive(Debug)]
pub(crate) struct NamingConventionOptions(Vec<NamingConventionSelection>);
impl NamingConventionOptions {
    pub(crate) fn override_default(
        overrides: impl IntoIterator<Item = NamingConventionSelection>,
    ) -> Self {
        let mut result = Self::default();
        result.0.extend(overrides);
        result
    }
}
impl Default for NamingConventionOptions {
    fn default() -> Self {
        Self(vec![
            NamingConventionSelection {
                selector: Selector::Default.into(),
                format: Some(vec![NamingConventionCase::Camel]),
                leading_underscore: Some(Underscore::Allow),
                trailing_underscore: Some(Underscore::Allow),
                ..Default::default()
            },
            NamingConventionSelection {
                selector: Selector::Import.into(),
                format: Some(vec![
                    NamingConventionCase::Camel,
                    NamingConventionCase::Pascal,
                ]),
                ..Default::default()
            },
            NamingConventionSelection {
                selector: Selector::Variable.into(),
                format: Some(vec![
                    NamingConventionCase::Camel,
                    NamingConventionCase::Upper,
                ]),
                leading_underscore: Some(Underscore::Allow),
                trailing_underscore: Some(Underscore::Allow),
                ..Default::default()
            },
            NamingConventionSelection {
                selector: Selector::TypeLike.into(),
                format: Some(vec![NamingConventionCase::Pascal]),
                leading_underscore: Some(Underscore::Allow),
                trailing_underscore: Some(Underscore::Allow),
                ..Default::default()
            },
        ])
    }
}
impl From<NamingConventionOptions> for use_naming_convention::NamingConventionOptions {
    fn from(val: NamingConventionOptions) -> Self {
        let mut enum_member_format = None;
        for selection in val.0 {
            if selection.selector.contains(&Selector::EnumMember) {
                // We only extract the first format because Biome doesn't allow for now multiple cases.
                enum_member_format = selection
                    .format
                    .and_then(|format| format.into_iter().next());
            }
        }
        use_naming_convention::NamingConventionOptions {
            strict_case: matches!(
                enum_member_format,
                Some(NamingConventionCase::StrictCamel | NamingConventionCase::StrictPascal)
            ),
            require_ascii: false,
            enum_member_case: enum_member_format
                .and_then(|format| {
                    match format {
                        NamingConventionCase::Camel | NamingConventionCase::StrictCamel => {
                            Some(use_naming_convention::EnumMemberCase::Camel)
                        }
                        NamingConventionCase::Pascal | NamingConventionCase::StrictPascal => {
                            Some(use_naming_convention::EnumMemberCase::Pascal)
                        }
                        NamingConventionCase::Upper => {
                            Some(use_naming_convention::EnumMemberCase::Constant)
                        }
                        // Biome doesn't support `snake_case` for enum member
                        NamingConventionCase::Snake => None,
                    }
                })
                .unwrap_or_default(),
        }
    }
}
#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct NamingConventionSelection {
    pub(crate) selector: eslint::Shorthand<Selector>,
    pub(crate) modifiers: Option<Vec<Modifier>>,
    pub(crate) types: Option<Vec<Type>>,
    //pub(crate) custom: Option<Custom>,
    pub(crate) format: Option<Vec<NamingConventionCase>>,
    pub(crate) leading_underscore: Option<Underscore>,
    pub(crate) trailing_underscore: Option<Underscore>,
    //pub(crate) prefix: Option<Vec<String>>,
    //pub(crate) suffix: Option<Vec<String>>,
    //pub(crate) filter: Option<Filter>,
}
//#[derive(Debug, Default, Deserializable)]
//pub(crate) struct Custom {
//    regex: String,
//    #[deserializable(rename = "match")]
//    matches: bool,
//}
//#[derive(Debug, Clone)]
//pub(crate) enum Filter {
//    Regex(String),
//    Custom(Custom),
//}
//impl Deserializable for Filter {
//    fn deserialize(
//        value: &impl biome_deserialize::DeserializableValue,
//        name: &str,
//        diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
//    ) -> Option<Self> {
//        if value.is_type(VisitableType::STR) {
//            Deserializable::deserialize(value, name, diagnostics).map(Filter::Regex)
//        } else {
//            Deserializable::deserialize(value, name, diagnostics).map(Filter::Custom)
//        }
//    }
//}
#[derive(Debug, Deserializable)]
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
#[derive(Debug, Default, Eq, PartialEq, Deserializable)]
pub(crate) enum Selector {
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
    #[default]
    Default,
    Accessor,
    MemberLike,
    Method,
    Property,
    TypeLike,
    VariableLike,
}
#[derive(Debug, Deserializable)]
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
#[derive(Debug, Deserializable)]
pub(crate) enum Type {
    Array,
    Boolean,
    Function,
    Number,
    String,
}
#[derive(Debug, Deserializable)]
pub(crate) enum Underscore {
    Forbid,
    Require,
    RequireDouble,
    Allow,
    AllowDouble,
    AllowSingleOrDouble,
}
