use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{TsMethodSignatureTypeMember, TsPropertySignatureTypeMember};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::use_consistent_method_signatures::{
    MethodSignatureStyle, UseConsistentMethodSignaturesOptions,
};

declare_lint_rule! {
    /// Enforce consistent use of either method signatures or function properties within interfaces and type aliases.
    ///
    /// TypeScript provides 2 different ways to declare methods within interfaces and object types:
    /// ```ts,ignore {2-3, 5-6}
    /// interface Example {
    ///   // method shorthand syntax
    ///   methodFunc(arg: string): void;
    ///
    ///   // regular property with function type
    ///   prop: (arg: string) => void;
    /// }
    ///
    /// // These forms correspond to the analogous JS object literal patterns:
    /// const obj = {
    ///   methodFunc(arg) {},
    ///   prop: (arg) => {},
    /// } satisfies Example;
    /// ```
    ///
    /// While mostly a matter of stylistic consistency, the two gain subtle differences in behavior when the
    /// [`strictFunctionTypes`](https://www.typescriptlang.org/tsconfig/#strictFunctionTypes) compiler option is enabled. \
    /// More specifically, its stricter contravariant checks will **only** apply to functions written in _property_ syntax —
    /// ones written as methods will remain with the weaker bivariant type checks.
    ///
    /// <details>
    /// <summary>What's the difference?</summary>
    ///
    /// To illustrate the differences between method bivariance and contravariance, consider the following snippet of code:
    /// ```ts,ignore
    /// interface Emitter {
    ///   methodFunc(arg: Event): void;
    ///   propFunc: (arg: Event) => void;
    /// }
    ///
    /// interface SpecialEvent extends Event {
    ///   isBirthday: boolean;
    /// }
    ///
    /// interface SpecialEmitter extends Emitter {
    ///   methodFunc(arg: SpecialEvent): void; // OK
    ///   propFunc: (arg: SpecialEvent) => void; // Error under `strictFunctionTypes`
    /// }
    /// ```
    /// In the above example, `SpecialEmitter.methodFunc` is compatible with `Emitter.methodFunc` under _bivariant_[^1] checks,
    /// as `SpecialEvent` is assignable to `Event` (i.e. all `SpecialEvent`s are guaranteed to be valid `Event`s). \
    /// On the other hand, the strict _contravariant_ checks for function properties produce errors on `propFunc` as the reverse is not guaranteed —
    /// `Event` is not assignable to `SpecialEvent` (i.e. not all `Event`s are guaranteed to be valid `SpecialEvent`s).
    ///
    /// The full rationale for this behavior can be found in the [TypeScript handbook](https://www.typescriptlang.org/docs/handbook/type-compatibility.html#function-parameter-bivariance).
    ///
    /// [^1]: From a purely type-theoretical perspective, bivariance technically refers to a type being _both_ covariant _and_ contravariant at once
    /// (`A` ⊆ `B` implies `T<A>` ≣ `T<B>`). \
    /// In practice, this is only true for pathological types like `type T<A> = number`,
    /// and so is often used to refer to a type being either covariant _or_ contravariant (which simply requires `T<A>` and `T<B>` to have some non-zero amount of overlap).
    ///
    /// </details>
    ///
    /// To avoid inconsistent type assignability issues and enforce stylistic consistency, this rule attempts to
    /// ensure either method- or property-style declarations are used consistently across a given codebase.
    ///
    /// :::info
    /// Without `strictFunctionTypes` enabled, method signatures and function properties become **functionally identical**.
    /// In this case, which option to use simply becomes a matter of personal preference.
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
    /// ```ts,use_options,expect_diagnostic
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
    /// Classes (as well as interfaces lacking function declarations) are not checked by either option:
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
    /// Default: `"property"`[^2]
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
    ///
    /// [^2]: Chosen to allow stricter type checks under the aforementioned `strictFunctionTypes`.
    pub UseConsistentMethodSignatures {
        version: "next",
        name: "useConsistentMethodSignatures",
        language: "ts",
        recommended: false,
        issue_number: Some("8780"),
        sources: &[RuleSource::EslintTypeScript("method-signature-style").same()],
        // TODO: Implement fix to convert between method/property
        // This will need to handle transforming overloads into intersections of function properties
        // fix_kind: FixKind::Unsafe,
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
