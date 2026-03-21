use crate::JsRuleAction;
use crate::services::typed::Typed;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyTsType, JsCallExpression, JsSyntaxKind, TsAsExpression,
    TsTypeAssertionExpression,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_reduce_type_parameter::UseReduceTypeParameterOptions;

declare_lint_rule! {
    /// Enforce using a type parameter on `Array#reduce` instead of casting the initial value.
    ///
    /// When using `Array#reduce`, the type of the accumulator is inferred from the initial value.
    /// If you use a type assertion (`as` or angle bracket `<T>`) on the initial value, the type
    /// is not checked against the accumulator usage in the callback. Using a type parameter on
    /// `reduce` instead is more type-safe because TypeScript will verify that the callback's
    /// return type matches the declared type.
    ///
    /// This rule uses type information to ensure it only triggers on actual array or tuple
    /// `reduce`/`reduceRight` calls, not on custom objects with a method of the same name.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,file=invalid.ts,expect_diagnostic
    /// const arr: number[] = [1, 2, 3];
    /// arr.reduce((sum, num) => sum.concat(num * 2), [] as number[]);
    /// ```
    ///
    /// ```ts,file=invalid2.ts,expect_diagnostic
    /// const arr: string[] = ['a', 'b'];
    /// arr.reduce((acc, name) => ({ ...acc, [name]: true }), {} as Record<string, boolean>);
    /// ```
    ///
    /// ```ts,file=invalid3.ts,expect_diagnostic
    /// const arr: number[] = [1, 2, 3];
    /// arr.reduceRight((sum, num) => sum.concat(num * 2), [] as number[]);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// const arr: number[] = [1, 2, 3];
    /// arr.reduce<number[]>((sum, num) => sum.concat(num * 2), []);
    /// ```
    ///
    /// ```ts,file=valid2.ts
    /// const arr: number[] = [1, 2, 3];
    /// arr.reduce((a, b) => a + b);
    /// ```
    ///
    /// ```ts,file=valid3.ts
    /// const arr: number[] = [1, 2, 3];
    /// arr.reduce((sum, n) => sum + n, 0);
    /// ```
    ///
    pub UseReduceTypeParameter {
        version: "next",
        name: "useReduceTypeParameter",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("prefer-reduce-type-parameter").inspired()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Types],
    }
}

/// Extracted assertion info from the second argument.
pub struct UseReduceTypeParameterState {
    /// The asserted type to move into the type parameter.
    asserted_type: AnyTsType,
    /// The inner expression without the assertion wrapper.
    inner_expression: AnyJsExpression,
}

impl Rule for UseReduceTypeParameter {
    type Query = Typed<JsCallExpression>;
    type State = UseReduceTypeParameterState;
    type Signals = Option<Self::State>;
    type Options = UseReduceTypeParameterOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();

        // already has type arguments like .reduce<Foo>(...)
        if call.type_arguments().is_some() {
            return None;
        }

        let callee = call.callee().ok()?.omit_parentheses();
        let member_expr = callee.as_js_static_member_expression()?;

        let method_name = member_expr.member().ok()?.as_js_name()?.to_trimmed_text();
        if method_name != "reduce" && method_name != "reduceRight" {
            return None;
        }

        // Use type information to verify the receiver is an array or tuple
        let call_object = member_expr.object().ok()?;
        let ty = ctx.type_of_expression(&call_object);
        if !is_array_or_tuple(&ty) {
            return None;
        }

        let args = call.arguments().ok()?.args();

        // must have exactly 2 arguments (callback + initial value)
        if args.len() != 2 {
            return None;
        }

        // second argument must have a type assertion (as or angle bracket)
        let second_arg = args.iter().nth(1)?.ok()?;
        let AnyJsCallArgument::AnyJsExpression(second_expr) = second_arg else {
            return None;
        };

        extract_assertion(second_expr)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let call = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                call.syntax().text_trimmed_range(),
                markup! {
                    "Use a type parameter on "<Emphasis>"Array#reduce"</Emphasis>" instead of casting the initial value."
                },
            )
            .note(markup! {
                "Type assertions on the initial value can mask type errors in the reducer callback."
            })
            .note(markup! {
                "Pass the type as a generic parameter to the reduce call for better type safety."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        let mut mutation = ctx.root().begin();

        // Build type arguments: <AssertedType>
        let type_arguments = make::ts_type_arguments(
            make::token(JsSyntaxKind::L_ANGLE),
            make::ts_type_argument_list([state.asserted_type.clone()], []),
            make::token(JsSyntaxKind::R_ANGLE),
        );

        // Rebuild argument list with the unwrapped inner expression (no trivia from `as`/`<>`)
        let inner = state.inner_expression.clone().trim_trivia().unwrap_or(state.inner_expression.clone());
        let old_args = call.arguments().ok()?;
        let old_arg_list = old_args.args();
        let first_arg = old_arg_list.iter().next()?.ok()?;
        let new_arg_list = make::js_call_argument_list(
            [first_arg, AnyJsCallArgument::AnyJsExpression(inner)],
            [old_arg_list.separators().next()?.ok()?],
        );
        let new_args = make::js_call_arguments(
            old_args.l_paren_token().ok()?,
            new_arg_list,
            old_args.r_paren_token().ok()?,
        );

        // Build the new call with type arguments and updated arguments
        let mut builder = make::js_call_expression(call.callee().ok()?, new_args);
        if let Some(chain_token) = call.optional_chain_token() {
            builder = builder.with_optional_chain_token(chain_token);
        }
        let new_call = builder.with_type_arguments(type_arguments).build();

        mutation.replace_node(call.clone(), new_call);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use a type parameter instead of type assertion." }.to_owned(),
            mutation,
        ))
    }
}

/// Extract assertion type and inner expression from either `expr as Type` or `<Type>expr`.
/// Handles parenthesized expressions like `(expr as Type)` and `(<Type>expr)`.
fn extract_assertion(expr: AnyJsExpression) -> Option<UseReduceTypeParameterState> {
    let unwrapped = expr.omit_parentheses();
    let syntax = unwrapped.into_syntax();

    if let Some(as_expr) = TsAsExpression::cast_ref(&syntax) {
        return Some(UseReduceTypeParameterState {
            asserted_type: as_expr.ty().ok()?,
            inner_expression: as_expr.expression().ok()?,
        });
    }

    if let Some(angle_expr) = TsTypeAssertionExpression::cast_ref(&syntax) {
        return Some(UseReduceTypeParameterState {
            asserted_type: angle_expr.ty().ok()?,
            inner_expression: angle_expr.expression().ok()?,
        });
    }

    None
}

/// Check if a type is an array or tuple.
fn is_array_or_tuple(ty: &biome_js_type_info::Type) -> bool {
    if ty.is_array_of(|_| true) {
        return true;
    }
    // Check for tuple types via resolved data
    if let Some(data) = ty.resolved_data() {
        if matches!(data.as_raw_data(), biome_js_type_info::TypeData::Tuple(_)) {
            return true;
        }
    }
    false
}
