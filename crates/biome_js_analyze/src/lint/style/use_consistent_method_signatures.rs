use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyTsType, AnyTsTypeMember, T, TsMethodSignatureTypeMember, TsPropertySignatureTypeMember,
    TsTypeMemberList,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind, declare_node_union};
use biome_rule_options::use_consistent_method_signatures::{
    MethodSignatureStyle, UseConsistentMethodSignaturesOptions,
};

// TODO: Highlight lines 2-3 and 4-6 of the codeblock if/when the doctest parser learns to ignore such patterns 
// ```ts,ignore {2-3, 4,6}

declare_lint_rule! {
    /// Enforce one syntax for functions declared in interfaces and type aliases.
    ///
    /// TypeScript supports method syntax and property syntax:
    ///
    /// ```ts,ignore
    /// interface Example {
    ///   method(arg: string): void;
    ///   property: (arg: string) => void;
    /// }
    /// ```
    ///
    /// The two forms can behave differently when the
    /// [`strictFunctionTypes`](https://www.typescriptlang.org/tsconfig/#strictFunctionTypes)
    /// compiler option is enabled. Function properties receive stricter parameter checks, while
    /// methods keep TypeScript's more permissive compatibility behavior.
    ///
    /// In this example, callers of `Emitter` may pass any `Event`. Narrowing the property parameter
    /// to `SpecialEvent` is therefore rejected, while TypeScript still accepts the method form:
    ///
    /// ```ts,ignore
    /// interface Emitter {
    ///   method(arg: Event): void;
    ///   property: (arg: Event) => void;
    /// }
    ///
    /// interface SpecialEvent extends Event {
    ///   isBirthday: boolean;
    /// }
    ///
    /// interface SpecialEmitter extends Emitter {
    ///   method(arg: SpecialEvent): void;
    ///   property: (arg: SpecialEvent) => void; // Error with `strictFunctionTypes`
    /// }
    /// ```
    ///
    /// <details>
    /// <summary>Type-system terminology</summary>
    ///
    /// With `strictFunctionTypes`, function properties check parameters *contravariantly*: a
    /// replacement function must accept every value accepted by the original function. Methods use
    /// TypeScript's more permissive *bivariant* parameter checking, which allows the narrower
    /// `SpecialEvent` parameter above. See the
    /// [TypeScript handbook](https://www.typescriptlang.org/docs/handbook/type-compatibility.html#function-parameter-bivariance)
    /// for the formal rationale.
    ///
    /// </details>
    ///
    /// Using one form consistently avoids unexpected differences in type checking.
    ///
    /// :::info
    /// Without `strictFunctionTypes`, the two forms have the same parameter-checking behavior, so
    /// the choice is only stylistic.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface Example {
    ///   methodFunc(arg: string): number;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type Generic<T, U> = {
    ///   methodFunc(arg: T): U;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type Union =
    ///   | {
    ///     foo(bar: number): number;
    ///   }
    ///   | 4;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type Intersection =
    ///   {
    ///     qux(quux: number): "quuux";
    ///   } & { foo: string };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// interface Prop {
    ///   propFunc: (arg: string) => number;
    /// }
    /// ```
    ///
    /// ```ts
    /// type Thing<T> = {
    ///   genericProp: <U>(arg: U) => T;
    /// }
    /// ```
    ///
    /// ```ts
    /// type Callback = () => void;
    /// ```
    ///
    /// Classes (as well as interfaces lacking function declarations) are always ignored:
    /// ```ts
    /// interface Example {
    ///   notAFunc: number;
    /// }
    /// ```
    ///
    /// ```ts
    /// class Foo {
    ///   methodFunc(arg: string): number;
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `style`
    /// The desired method signature style to enforce. \
    /// Possible values are either `"method"` or `"property"`.
    ///
    /// Default: `"property"`, which enables stricter parameter checking with
    /// `strictFunctionTypes`.
    ///
    /// #### Examples for `"style": "method"`
    ///
    /// ```json,options
    /// {
    ///  "options": {
    ///    "style": "method"
    ///  }
    /// }
    /// ```
    ///
    /// ```ts,use_options,expect_diagnostic
    /// interface Blah {
    ///   propFunc: (arg: string) => void;
    /// }
    /// ```
    ///
    /// ```ts,use_options,expect_diagnostic
    /// type Generic = {
    ///   propFunc: <T, U>(arg: T) => U;
    /// }
    /// ```
    ///
    /// ```ts,use_options
    /// type OK = {
    ///   flubber(arg: number): number;
    /// }
    /// ```
    pub UseConsistentMethodSignatures {
        version: "2.3.14",
        name: "useConsistentMethodSignatures",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("method-signature-style").same()],
        fix_kind: FixKind::Unsafe,
    }
}

// Struct containing info about an inconsistent method signature diagnostic.
pub struct InconsistentMethodSignatureState {
    target_style: MethodSignatureStyle,
    node_style: MethodSignatureStyle,
}

impl Rule for UseConsistentMethodSignatures {
    type Query = Ast<AnyTsMethodSignatureLike>;
    type State = InconsistentMethodSignatureState;
    type Signals = Option<Self::State>;
    type Options = UseConsistentMethodSignaturesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let target_style = ctx.options().style.unwrap_or_default();
        let node_style = binding.get_signature_style()?;
        if target_style == node_style {
            return None;
        }

        Some(InconsistentMethodSignatureState {
            target_style,
            node_style,
        })
    }

    fn diagnostic(
        ctx: &RuleContext<Self>,
        state: &InconsistentMethodSignatureState,
    ) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let InconsistentMethodSignatureState {
            target_style,
            node_style,
        } = *state;

        let mut diagnostic = RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Prefer using "<Emphasis>{target_style}</Emphasis>"-style over "<Emphasis>{node_style}</Emphasis>"-style method signatures."
                },
            )
            .note("Consistently using a single style of method signatures helps improve readability and consistency.");

        if target_style == MethodSignatureStyle::Property {
            diagnostic = diagnostic.note(markup! {
                "Property-style function declarations also allow for stricter type checking when the "<Emphasis>"strictFunctionTypes"</Emphasis>" compiler option is enabled."
            })
        }

        diagnostic = diagnostic
            .note(markup! {
                "If this isn't what you want, consider changing the "<Emphasis>"style"</Emphasis>" option in the rule's settings."
            });

        Some(diagnostic)
    }

    fn action(
        ctx: &RuleContext<Self>,
        state: &Self::State,
    ) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let (prev, next) = match node {
            AnyTsMethodSignatureLike::TsMethodSignatureTypeMember(method) => {
                let new_node = method_to_property(method)?;
                (
                    AnyTsTypeMember::from(method.clone()),
                    AnyTsTypeMember::from(new_node),
                )
            }
            AnyTsMethodSignatureLike::TsPropertySignatureTypeMember(prop) => {
                let new_node = property_to_method(prop)?;
                (
                    AnyTsTypeMember::from(prop.clone()),
                    AnyTsTypeMember::from(new_node),
                )
            }
        };

        mutation.replace_node(prev, next);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Convert to "<Emphasis>{state.target_style}</Emphasis>"-style signature." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Converts a method-style signature into a property-style one.
///
/// Returns `None` if:
/// - The method has no explicit return type annotation (can't build a valid function type).
/// - The method is one of several overloads (would need intersection types — handled separately).
fn method_to_property(
    node: &TsMethodSignatureTypeMember,
) -> Option<TsPropertySignatureTypeMember> {
    let return_type_annotation = node.return_type_annotation()?;

    if has_method_overloads(node) {
        return None;
    }

    let name = node.name().ok()?;
    let orig_params = node.parameters().ok()?;
    // Rebuild parameters with a `)` that has trailing whitespace so the output is
    // `(arg: string) => void` instead of `(arg: string)=> void`.
    let parameters = make::js_parameters(
        orig_params.l_paren_token().ok()?,
        orig_params.items(),
        make::token(T![')']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    );
    let return_type = return_type_annotation.ty().ok()?;

    let function_type = make::ts_function_type(
        parameters,
        make::token(T![=>]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        return_type,
    )
    .build()
    .with_type_parameters(node.type_parameters());

    let type_annotation = make::ts_type_annotation(
        make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        AnyTsType::from(function_type),
    );

    let mut builder = make::ts_property_signature_type_member(name)
        .with_type_annotation(type_annotation);

    if let Some(opt) = node.optional_token() {
        builder = builder.with_optional_token(opt);
    }
    if let Some(sep) = node.separator_token() {
        builder = builder.with_separator_token_token(sep);
    }

    Some(builder.build())
}

/// Converts a property-style signature into a method-style one.
///
/// Returns `None` if the property has a `readonly` modifier (methods can't be readonly).
fn property_to_method(
    node: &TsPropertySignatureTypeMember,
) -> Option<TsMethodSignatureTypeMember> {
    if node.readonly_token().is_some() {
        return None;
    }

    let name = node.name().ok()?;
    let type_annotation = node.type_annotation()?;
    let function_type = type_annotation.ty().ok()?.as_ts_function_type()?.clone();

    let orig_params = function_type.parameters().ok()?;
    // Rebuild parameters with a clean `)` — the original `)` carries trailing whitespace
    // from the source (e.g. the space before `=>` in `(arg: string) => void`), which would
    // produce `method() : void` instead of the desired `method(): void`.
    let parameters = make::js_parameters(
        orig_params.l_paren_token().ok()?,
        orig_params.items(),
        make::token(T![')']),
    );
    let return_type = function_type.return_type().ok()?;

    let return_type_annotation = make::ts_return_type_annotation(
        make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        return_type,
    );

    let mut builder = make::ts_method_signature_type_member(name, parameters)
        .with_return_type_annotation(return_type_annotation);

    if let Some(opt) = node.optional_token() {
        builder = builder.with_optional_token(opt);
    }
    if let Some(type_params) = function_type.type_parameters() {
        builder = builder.with_type_parameters(type_params);
    }
    if let Some(sep) = node.separator_token() {
        builder = builder.with_separator_token_token(sep);
    }

    Some(builder.build())
}

/// Returns `true` if `node` is one of multiple overloads — i.e. the parent type member list
/// contains more than one method signature with the same name.
fn has_method_overloads(node: &TsMethodSignatureTypeMember) -> bool {
    check_method_overloads(node).unwrap_or(false)
}

fn check_method_overloads(node: &TsMethodSignatureTypeMember) -> Option<bool> {
    let name_text = node.name().ok()?.name()?;
    let member_list = node.parent::<TsTypeMemberList>()?;

    let overload_count = member_list
        .into_iter()
        .filter_map(|m| m.as_ts_method_signature_type_member().cloned())
        .filter(|m| m.name().ok().and_then(|n| n.name()).is_some_and(|n| n == name_text))
        .count();

    Some(overload_count > 1)
}

declare_node_union! {
    /// Node union representing anything that _might_ be a method signature within a type alias or interface.
    ///
    /// (In reality, most property signatures aren't actually function declarations, depending on the type annotation in question.)
    pub AnyTsMethodSignatureLike = TsMethodSignatureTypeMember | TsPropertySignatureTypeMember
}

impl AnyTsMethodSignatureLike {
    /// Return the style of this node's function declaration.
    /// Returns `None` if this node is a property signature that either lacks a type annotation
    /// or is not a function type.
    pub fn get_signature_style(&self) -> Option<MethodSignatureStyle> {
        match self {
            Self::TsMethodSignatureTypeMember(_) => Some(MethodSignatureStyle::Method),
            Self::TsPropertySignatureTypeMember(prop) => prop
                .type_annotation()
                .and_then(|annotation| annotation.ty().ok())
                .and_then(|ty| {
                    ty.as_ts_function_type()
                        .map(|_| MethodSignatureStyle::Property)
                }),
        }
    }
}
