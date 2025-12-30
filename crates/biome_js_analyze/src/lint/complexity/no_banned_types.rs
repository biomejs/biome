use std::fmt::Display;

use biome_analyze::context::RuleContext;
use biome_analyze::{FixKind, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    JsReferenceIdentifier, JsSyntaxKind, TextRange, TsIntersectionTypeElementList, TsObjectType,
    TsReferenceType, TsTypeConstraintClause,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, declare_node_union};
use biome_rule_options::no_banned_types::NoBannedTypesOptions;

use crate::JsRuleAction;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow primitive type aliases and misleading types.
    ///
    /// This rule aims to prevent usage of potentially "misleading" types and type aliases
    /// which may behave unexpectedly.
    ///
    /// ### Disallow "boxed object" types like `Boolean` and `Number`
    ///
    /// JavaScript's 8 data types are described in TypeScript by the lowercase types
    /// `undefined`, `null`, `boolean`, `number`, `string`, `bigint`, `symbol`, and `object`.
    ///
    /// The latter 6 also have uppercase variants, which instead represent _interfaces_ with the shared properties of their primitive counterparts.
    /// Due to the nature of structural typing, these uppercase types accept both primitive values and non-primitive "boxed object"s
    /// like `new Boolean(true)`, despite the two behaving differently in many circumstances like equality and truthiness.
    ///
    /// It is thus considered best practice to avoid these "boxed types" in favor of their lowercase
    /// primitive counterparts.
    ///
    /// ### Disallow the unsafe `Function` type
    ///
    /// TypeScript's built-in `Function` type is capable of accepting callbacks of any shape or form,
    /// behaving equivalent to `(...rest: any[]) => any` (which uses the unsafe `any` type) when called directly. \
    /// It also accepts classes or plain objects that happen to possess all properties of the `Function` class,
    /// which is likewise a potential source of confusion.
    ///
    /// As such, it is almost always preferable to explicitly specify function parameters and return types where possible. \
    /// When a generic "catch-all" callback type is required, one of the following can be used instead:
    /// - `() => void`: A function that accepts no parameters and whose return value is ignored
    /// - `(...args: never) => unknown`: A "top type" for functions that can be _assigned_ any function type,
    ///    but can't be called directly
    ///
    /// ### Disallow the misleading empty object type `{}`
    /// `{}`, also known as the "empty object" type, _doesn't_ actually represent an empty object (despite what many new to TypeScript may assume). \
    /// Due to TypeScript's type system being _structural_ instead of nominal, it actually accepts _any non-nullish value_,
    // including non-object primitives like numbers and strings[^1]. \
    /// The following example is thus perfectly valid TypeScript:
    ///
    /// ```ts,ignore
    /// const n: {} = 0;
    /// ```
    ///
    /// Often, developers writing `{}` actually mean one of the following:
    /// - `object`: Represents any object value
    /// - `unknown`: Represents any value at all, including `null` and `undefined`
    /// - `{ [k: keyof any]: never }` or `Record<keyof any, never>`: Represent object types whose properties are all of type `never` (and cannot be used)
    /// - `{ [myUniqueInternalSymbol]?: never }`: Represents an object type whose only "property" is an unexported `unique symbol`, thereby forcing external consumers to omit it[^2]. \
    ///   This can be used as a type guard for use in `extends` clauses or a type annotation for use in [excess property checks](https://www.typescriptlang.org/docs/handbook/2/objects.html#excess-property-checks),
    ///   both with their own respective use cases and pitfalls.
    ///
    /// To avoid confusion, this rule forbids the use of the type `{}`, except in two situations:
    ///
    /// 1. In type constraints to restrict a generic type to non-nullable types:
    ///
    /// ```ts
    /// function f<T extends {}>(x: T) {
    ///     assert(x != null);
    /// }
    /// ```
    ///
    /// 2. In a type intersection to narrow a type to its non-nullable equivalent type:
    ///
    /// ```ts
    /// type NonNullableMyType = MyType & {};
    /// ```
    ///
    /// In this last case, you can also use the `NonNullable` utility type to the same effect:
    ///
    /// ```ts
    /// // equivalent to `{}`
    /// type AnythingNotNullish = NonNullable<unknown>;
    /// ```
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
    /// const bool = true as Boolean;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let invalidTuple: [string, Number] = ["foo", 12];
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function badFunction(cb: Function) {
    ///   cb(12);
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// const notEmpty: {} = {prop: 12};
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// const alsoNotAnObj: Object = "foo";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// const foo: string = "bar";
    /// ```
    ///
    /// ```ts
    /// let tuple: [boolean, string] = [false, "foo"];
    /// ```
    ///
    /// ```ts
    /// function betterFunction(cb: (n: number) => string) {
    ///   return cb(12);
    /// }
    /// ```
    ///
    /// ```ts
    /// type wrapFn<T extends (...args: never) => unknown> = { func: T }
    /// ```
    ///
    /// ```ts
    /// const goodObj: object = {foo: 12};
    /// ```
    ///
    /// ```ts
    /// type emptyObj = Record<string, never>;
    /// ```
    ///
    /// Exceptions for `{}`:
    /// ```ts
    /// declare function foo<T extends {}>(x: T): void;
    /// ```
    ///
    /// ```ts
    /// type notNull<T> = T & {};
    /// ```
    ///
    /// [^1]: This is the exact same mechanism that allows passing `{ foo: number, bar: string }`
    /// to a function expecting `{ bar: string }`.
    /// Specifying `{}` doesn't restrict compatible types to ones with _exactly_ 0 properties;
    /// it simply requires they have _at least_ 0 properties.
    /// [^2]: In this case, you'd write `declare const myUniqueInternalSymbol: unique symbol` somewhere in the same file.
    pub NoBannedTypes {
        version: "1.0.0",
        name: "noBannedTypes",
        language: "ts",
        sources: &[
            RuleSource::EslintTypeScript("ban-types").same(),
            RuleSource::EslintTypeScript("no-empty-object-type").inspired(),
            RuleSource::EslintTypeScript("no-wrapper-object-types").inspired(),
            RuleSource::EslintTypeScript("no-unsafe-function-type").inspired(),
        ],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoBannedTypes {
    type Query = Semantic<TsBannedType>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoBannedTypesOptions;

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
            markup! {"Don't use '"<Emphasis>{banned_type.to_string()}</Emphasis>"' as a type."}
                .to_owned(),
        )
        .note(banned_type.message())
        // TODO: Update this if/when the rule gets split up or has individual disabling options added
        .note("If that's really what you want, use an inline disable comment.");

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
            markup! { "Use '"{suggested_type}"' instead." }.to_owned(),
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
    /// Construct a [BannedType] from the textual name of a JavaScript type.
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

    /// Retrieve a diagnostic message from a [BannedType].
    fn message(&self) -> impl biome_console::fmt::Display {
        match *self {
            Self::BigInt | Self::Boolean | Self::Number | Self::String | Self::Symbol => {
                let primitive_str = self.as_js_syntax_kind().and_then(|syntax| syntax.to_string())
                    .expect("BannedType should be coercible to its lowercase primitive as a string");

                markup! {
                    "Prefer using lowercase primitive types instead of uppercase \"boxed object\" types."
                    "\n'"<Emphasis>{ self.to_string() }</Emphasis>"' accepts "<Emphasis>"anything"</Emphasis>" that implements the corresponding interface "
                    "- both primitives and \"primitive-like\" objects."
                    "\nIt is considered best practice to use '"<Emphasis>{ primitive_str }</Emphasis>"' instead in nearly all circumstances."
                }.to_owned()
            }
            Self::Function => {
                markup! {
                    "The '"<Emphasis>"Function"</Emphasis>"' type is unsafe and accepts any arbitrary function or \"function-like\" value."
                    "\nExplicitly defining the function's shape helps prevent mismatching argument types and return values."
                    "\nIf a generic \"catch-all\" callback type is required, consider using a \"top type\" like '"<Emphasis>"(...args: never) => unknown"</Emphasis>"' instead."
                }.to_owned()
            }
            Self::Object | Self::EmptyObject => {
                markup! {
                    "'"<Emphasis>{ self.to_string() }</Emphasis>"' accepts "<Emphasis>"any"</Emphasis>" non-nullish value, including non-object primitives like "
                    "'"<Emphasis>"123"</Emphasis>"' and '"<Emphasis>"true"</Emphasis>"'."
                    "\n- If you want a type meaning \"any arbitrary object\", use '"<Emphasis>"object"</Emphasis>"' instead."
                    "\n- If you want a type meaning \"any value\", use '"<Emphasis>"unknown"</Emphasis>"' instead."
                    "\n- If you want a type meaning \"an object whose properties cannot be used\", use "
                    "'"<Emphasis>"{ [k: keyof any]: never }"</Emphasis>"' or '"<Emphasis>"Record<keyof any, never>"</Emphasis>"' instead."
                    "\n- If you want a type meaning \"an object that cannot contain any properties whatsoever\", use "
                    "'"<Emphasis>"{ [uniqueSymbol]?: never }"</Emphasis>"' with an unexported "<Emphasis>"unique symbol"</Emphasis>" in the same file."
                }.to_owned()
            }
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
