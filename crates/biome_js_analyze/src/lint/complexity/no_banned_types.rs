use std::fmt::Display;

use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    JsReferenceIdentifier, JsSyntaxKind, TextRange, TsIntersectionTypeElementList, TsObjectType,
    TsReferenceType, TsTypeConstraintClause,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, BatchMutationExt};

use crate::services::semantic::Semantic;
use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow primitive type aliases and misleading types.
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
    pub NoBannedTypes {
        version: "1.0.0",
        name: "noBannedTypes",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("ban-types")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoBannedTypes {
    type Query = Semantic<TsBannedType>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let model = ctx.model();
        match query {
            TsBannedType::TsObjectType(ts_object_type) => {
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
                        banned_type: BannedType::EmptyObject,
                        banned_type_range: ts_object_type.range(),
                        reference_identifier: None,
                    });
                }
            }
            TsBannedType::TsReferenceType(ts_reference_type) => {
                let ts_any_name = ts_reference_type.name().ok()?;
                let reference_identifier = ts_any_name.as_js_reference_identifier()?;
                if model.binding(reference_identifier).is_none() {
                    // if the dientifier is global
                    let identifier_token = reference_identifier.value_token().ok()?;
                    if let Some(banned_type) = BannedType::from_str(identifier_token.text_trimmed())
                    {
                        return Some(State {
                            banned_type,
                            banned_type_range: identifier_token.text_trimmed_range(),
                            reference_identifier: Some(reference_identifier.clone()),
                        });
                    }
                }
            }
        }

        None
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        State {
            banned_type,
            banned_type_range,
            ..
        }: &Self::State,
    ) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            banned_type_range,
            markup! {"Don't use '"{banned_type.to_string()}"' as a type."}.to_owned(),
        )
        .note(markup! { {banned_type.message()} }.to_owned());
        Some(diagnostic)
    }

    fn action(
        ctx: &RuleContext<Self>,
        State {
            banned_type,
            reference_identifier,
            ..
        }: &Self::State,
    ) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let suggested_type = banned_type.as_js_syntax_kind()?.to_string()?;
        mutation.replace_node(reference_identifier.clone()?, banned_type.fix_with()?);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use '"{suggested_type}"' instead" }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    pub TsBannedType = TsReferenceType | TsObjectType
}

pub struct State {
    /// Reference to the enum item containing the banned type.
    /// Used for both diagnostic and action.
    banned_type: BannedType,
    /// Text range used to diagnostic the banned type.
    banned_type_range: TextRange,
    /// Reference to the node to be replaced in the action.
    /// This is optional because we don't replace empty objects references.
    reference_identifier: Option<JsReferenceIdentifier>,
}

#[derive(Debug)]
pub enum BannedType {
    BigInt,
    Boolean,
    Function,
    Number,
    Object,
    String,
    Symbol,
    /// {}
    EmptyObject,
}

impl BannedType {
    /// construct a [BannedType] from the textual name of a JavaScript type
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

    /// Retrieves a diagnostic message from a [BannedType]
    fn message(&self) -> &str {
        match *self {
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
		}
    }

    /// Converts a [BannedType] to a [JsSyntaxKind]
    fn as_js_syntax_kind(&self) -> Option<JsSyntaxKind> {
        Some(match *self {
            Self::BigInt => JsSyntaxKind::BIGINT_KW,
            Self::Boolean => JsSyntaxKind::BOOLEAN_KW,
            Self::Number => JsSyntaxKind::NUMBER_KW,
            Self::String => JsSyntaxKind::STRING_KW,
            Self::Symbol => JsSyntaxKind::SYMBOL_KW,
            _ => return None,
        })
    }

    /// Retrieves a [JsReferenceIdentifier] from a [BannedType] that will be used to
    /// replace it on the rule action
    fn fix_with(&self) -> Option<JsReferenceIdentifier> {
        Some(match *self {
            Self::BigInt | Self::Boolean | Self::Number | Self::String | Self::Symbol => {
                make::js_reference_identifier(make::token(Self::as_js_syntax_kind(self)?))
            }
            _ => return None,
        })
    }
}

impl Display for BannedType {
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
        };
        write!(f, "{representation}")
    }
}
