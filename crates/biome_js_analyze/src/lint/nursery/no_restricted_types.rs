use std::fmt::Display;

use ::serde::{Deserialize, Serialize};
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_factory::make;
use biome_js_syntax::{
    JsReferenceIdentifier, TextRange, TsIntersectionTypeElementList, TsObjectType, TsReferenceType,
    TsTypeConstraintClause,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, BatchMutationExt};
use rustc_hash::FxHashMap;

use crate::services::semantic::Semantic;
use crate::JsRuleAction;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_lint_rule! {
    /// Disallow primitive type aliases, misleading or user defined types.
    ///
    /// - Enforce consistent names for primitive types
    ///
    ///   Primitive types have aliases.
    ///   For example, `Number` is an alias of `number`.
    ///   The rule recommends the lowercase primitive type names.
    ///
    /// - Disallow the `Function` type
    ///
    ///   The `Function` type is loosely typed and is thus considered dangerous or harmful.
    ///   `Function` is equivalent to the type `(...rest: any[]) => any` that uses the unsafe `any` type.
    ///
    /// - Disallow the misleading non-nullable type `{}`
    ///
    ///   In TypeScript, the type `{}` doesn't represent an empty object.
    ///   It represents any value except `null` and `undefined`.
    ///   The following TypeScript example is perfectly valid:
    ///
    ///   ```ts,expect_diagnostic
    ///   const n: {} = 0
    ///   ```
    ///
    ///   To represent an empty object, you should use `{ [k: string]: never }` or `Record<string, never>`.
    ///
    ///   To avoid any confusion, the rule forbids the use of the type `{}`, except in two situations:
    ///
    ///   1. In type constraints to restrict a generic type to non-nullable types:
    ///
    ///   ```ts
    ///   function f<T extends {}>(x: T) {
    ///       assert(x != null);
    ///   }
    ///   ```
    ///
    ///   2. In a type intersection to narrow a type to its non-nullable equivalent type:
    ///
    ///   ```ts
    ///   type NonNullableMyType = MyType & {};
    ///   ```
    ///
    ///   In this last case, you can also use the `NonNullable` utility type:
    ///
    ///   ```ts
    ///   type NonNullableMyType = NonNullable<MyType>;
    ///   ```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let foo: String = "bar";
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let bool = true as Boolean;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let invalidTuple: [string, Boolean] = ["foo", false];
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// let foo: string = "bar";
    /// ```
    ///
    /// ```ts
    /// let tuple: [boolean, string] = [false, "foo"];
    /// ```
    ///
    /// ## Options
    ///
    /// Use the options to specify additional types that you want to restrict in your
    /// source code.
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "types": {
    ///            "Foo": {
    ///               "message": "Only bar is allowed",
    ///               "fixWith": "bar"
    ///             },
    ///             "OldAPI": {
    ///                 "message": "Use NewAPI instead"
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// In the example above, the rule will emit a diagnostics if tried to use `Foo` or `OldAPI` are used.
    ///
    /// This rule provides predefined list of restricted types, however if you do not want to use the predefined list
    /// you can disable it by setting option `extendDefaults` to `false`.
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///        "extendDefaults": false,
    ///         "types": {}
    ///     }
    /// }
    ///
    pub NoRestrictedTypes {
        version: "next",
        name: "noRestrictedTypes",
        language: "ts",
        sources: &[
            RuleSource::EslintTypeScript("no-restricted-types"),
            // TODO: Add this source once `noBannedTypes` is deprecated
            // RuleSource::EslintTypeScript("ban-types")
        ],
        recommended: false,
        fix_kind: FixKind::Safe,

    }
}

impl Rule for NoRestrictedTypes {
    type Query = Semantic<TsRestrictedType>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoRestrictedTypesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let model = ctx.model();
        let options = ctx.options();
        let should_extend_defaults = options.extend_defaults.unwrap_or(true);

        match query {
            TsRestrictedType::TsObjectType(ts_object_type) => {
                if !should_extend_defaults {
                    return None;
                }

                // Allow empty object type for type constraint and intersections.
                // ```js
                // type AssertNonNullGeneric<T extends {}> = T
                // type NonNull<T> = T & {}
                // ```
                if ts_object_type.members().is_empty()
                    && (ts_object_type.parent::<TsTypeConstraintClause>().is_none()
                        && ts_object_type
                            .parent::<TsIntersectionTypeElementList>()
                            .is_none())
                {
                    return Some(State {
                        restricted_type: RestrictedType::EmptyObject,
                        range: ts_object_type.range(),
                        reference_identifier: None,
                    });
                }
            }
            TsRestrictedType::TsReferenceType(ts_reference_type) => {
                let ts_any_name = ts_reference_type.name().ok()?;
                let reference_identifier = ts_any_name.as_js_reference_identifier()?;
                let identifier_token = reference_identifier.value_token().ok()?;
                let token_name = identifier_token.text_trimmed();

                let restricted_type =
                    if should_extend_defaults && model.binding(reference_identifier).is_none() {
                        RestrictedType::from_str(token_name)
                    } else {
                        options.types.get(token_name).map(|custom_restricted_type| {
                            RestrictedType::Custom((
                                token_name.to_string(),
                                custom_restricted_type.clone(),
                            ))
                        })
                    }?;

                return Some(State {
                    restricted_type,
                    range: identifier_token.text_trimmed_range(),
                    reference_identifier: Some(reference_identifier.clone()),
                });
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let restricted_type = &state.restricted_type;

        let message = match restricted_type {
            RestrictedType::Custom((_, data)) => &data.message,
            _ => &format!("Don't use '{}' as a type.", restricted_type),
        };

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range,
            markup! { {message} }.to_owned(),
        );

        if let Some(additional_note) = restricted_type.additional_note() {
            diagnostic = diagnostic.note(markup! { {additional_note} }.to_owned());
        }

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let suggested_type = state.restricted_type.suggested_fix_type()?;

        let prev_token = state.reference_identifier.clone()?.value_token().ok()?;
        let new_token = make::ident(suggested_type.as_str());

        mutation.replace_element(prev_token.into(), new_token.into());

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Use '"{suggested_type}"' instead" }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Clone, Debug, Default, Deserializable, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoRestrictedTypesOptions {
    extend_defaults: Option<bool>,
    types: FxHashMap<String, CustomRestrictedType>,
}

#[derive(Debug, Clone, Default, Deserializable, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomRestrictedType {
    message: String,
    fix_with: Option<String>,
}

declare_node_union! {
    pub TsRestrictedType = TsReferenceType | TsObjectType
}

pub struct State {
    /// Name of the restricted type.
    restricted_type: RestrictedType,
    /// Text range used to diagnostic the banned type.
    range: TextRange,
    /// Reference to the node to be replaced in the action.
    /// This is optional because we don't replace empty objects references.
    reference_identifier: Option<JsReferenceIdentifier>,
}

#[derive(Debug)]
pub enum RestrictedType {
    BigInt,
    Boolean,
    Function,
    Number,
    Object,
    String,
    Symbol,
    /// {}
    EmptyObject,
    /// User provided custom restricted type
    Custom((String, CustomRestrictedType)),
}

impl RestrictedType {
    /// construct a [RestrictedType] from the textual name of a JavaScript type
    fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "BigInt" => Self::BigInt,
            "Boolean" => Self::Boolean,
            "Function" => Self::Function,
            "Number" => Self::Number,
            "Object" => Self::Object,
            "String" => Self::String,
            "Symbol" => Self::Symbol,
            "{}" => Self::EmptyObject,
            _ => return None,
        })
    }

    /// Retrieves a diagnostic message from a [RestrictedType]
    fn additional_note(&self) -> Option<&str> {
        Some(match *self {
			| Self::BigInt
			| Self::Boolean
			| Self::Number
			| Self::String
			| Self::Symbol => "Use lowercase primitives for consistency.",
			Self::Function =>
				"Prefer explicitly define the function shape. This type accepts any function-like value, which can be a common source of bugs.",
			Self::Object =>
				"Prefer explicitly define the object shape. This type means \"any non-nullable value\", which is slightly better than 'unknown', but it's still a broad type.",
			Self::EmptyObject => "Prefer explicitly define the object shape. '{}' means \"any non-nullable value\".",
            _ => return None,
        })
    }

    fn suggested_fix_type(&self) -> Option<String> {
        Some(match self {
            Self::BigInt => "bigint".to_string(),
            Self::Boolean => "boolean".to_string(),
            Self::Number => "number".to_string(),
            Self::String => "string".to_string(),
            Self::Symbol => "symbol".to_string(),
            Self::Custom((_, data)) => data.fix_with.clone()?,
            _ => return None,
        })
    }
}

impl Display for RestrictedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let representation = match self {
            Self::BigInt => "BigInt",
            Self::Boolean => "Boolean",
            Self::Function => "Function",
            Self::Number => "Number",
            Self::Object => "Object",
            Self::String => "String",
            Self::Symbol => "Symbol",
            Self::EmptyObject => "{}",
            Self::Custom((name, _)) => name,
        };
        write!(f, "{representation}")
    }
}
