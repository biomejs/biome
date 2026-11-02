use crate::JsRuleAction;
use crate::services::typed::Typed;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsExpression, AnyTsType, AnyTsVariableAnnotation, JsAssignmentExpression,
    JsAssignmentOperator, JsCallExpression, JsIdentifierExpression, JsLanguage,
    JsParenthesizedExpression, JsSyntaxKind, JsVariableDeclarator, OperatorPrecedence, T,
    TsAsExpression, TsDeclareStatement, TsNonNullAssertionAssignment, TsNonNullAssertionExpression,
    TsTypeAssertionExpression,
};
use biome_js_type_info::InferredType;
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, chain_trivia_pieces, declare_node_union};
use biome_rule_options::no_useless_type_assertion::NoUselessTypeAssertionOptions;

declare_lint_rule! {
    /// Disallow type assertions that don't change the type of an expression.
    ///
    /// A type assertion tells TypeScript to treat a value as a different type.
    /// This includes `as` assertions (`value as string`), angle-bracket
    /// assertions (`<string>value`), and non-null assertions (`value!`), which
    /// tell TypeScript that a value isn't `null` or `undefined`.
    ///
    /// When the value already has the asserted type, the assertion does nothing.
    /// It only adds noise, and it makes the assertions that do matter harder to
    /// notice.
    ///
    /// This rule reports:
    ///
    /// - non-null assertions on values that can't be `null` or `undefined`;
    /// - non-null assertions on the left side of an `=` assignment, because
    ///   assigning a value never reads the old one;
    /// - `as` and angle-bracket assertions to a type that the value already has.
    ///   This check only covers `string`, `number`, `bigint`, `boolean`,
    ///   `symbol`, `null`, `undefined`, and unions of them, such as
    ///   `string | undefined`. It only checks function calls, awaited function
    ///   calls, `+`, comparisons, and other type assertions. It doesn't check
    ///   variables and properties, because TypeScript can give them a more
    ///   specific type where the assertion is written, for example inside
    ///   `if (typeof value === "string")`.
    ///
    /// The rule doesn't report `as const` assertions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-non-null.ts
    /// const foo = 3;
    /// const bar = foo!;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-parameter.ts
    /// function foo(x: number): number {
    ///     return x!;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-angle-bracket.ts
    /// const foo = <number>(3 + 5);
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-alias.ts
    /// type Foo = number;
    /// const foo = (3 + 5) as Foo;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-double.ts
    /// declare function getName(): string;
    /// const foo = getName() as unknown as string;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-assignment.ts
    /// let foo: number | undefined;
    /// foo! = 1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// const a = <number>3;
    /// const b = 3 as number;
    /// let c = "foo" as const;
    ///
    /// function foo(x: number | undefined): number {
    ///     return x!;
    /// }
    /// ```
    ///
    pub NoUselessTypeAssertion {
        version: "2.6.5",
        name: "noUselessTypeAssertion",
        language: "ts",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintTypeScript("no-unnecessary-type-assertion").same()],
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Types],
    }
}

declare_node_union! {
    pub AnyTsUselessTypeAssertionCandidate =
        TsAsExpression
        | TsNonNullAssertionAssignment
        | TsNonNullAssertionExpression
        | TsTypeAssertionExpression
}

pub enum UselessTypeAssertion {
    /// `value!`, where `value` can't be `null` or `undefined`.
    NonNull,
    /// `value! = other`.
    NonNullAssignmentTarget,
    /// `value as T` or `<T>value`, where `value` already has the type `T`.
    SameType,
    /// `value as unknown as T`, where `value` already has the type `T`.
    ///
    /// Stores the operand of the innermost assertion, including its parentheses.
    Chained(AnyJsExpression),
}

impl Rule for NoUselessTypeAssertion {
    type Query = Typed<AnyTsUselessTypeAssertionCandidate>;
    type State = UselessTypeAssertion;
    type Signals = Option<Self::State>;
    type Options = NoUselessTypeAssertionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyTsUselessTypeAssertionCandidate::TsNonNullAssertionAssignment(assignment) => {
                let parent = assignment.parent::<JsAssignmentExpression>()?;
                let is_target = parent.left().ok()?.syntax() == assignment.syntax();
                (is_target && parent.operator().ok()? == JsAssignmentOperator::Assign)
                    .then_some(UselessTypeAssertion::NonNullAssignmentTarget)
            }
            AnyTsUselessTypeAssertionCandidate::TsNonNullAssertionExpression(expression) => {
                run_non_null_assertion(ctx, expression)
            }
            AnyTsUselessTypeAssertionCandidate::TsAsExpression(expression) => run_type_assertion(
                ctx,
                &AnyJsExpression::from(expression.clone()),
                &expression.ty().ok()?,
                expression.expression().ok()?,
            ),
            AnyTsUselessTypeAssertionCandidate::TsTypeAssertionExpression(expression) => {
                run_type_assertion(
                    ctx,
                    &AnyJsExpression::from(expression.clone()),
                    &expression.ty().ok()?,
                    expression.expression().ok()?,
                )
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diagnostic = match state {
            UselessTypeAssertion::NonNull => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "This non-null assertion is unnecessary." },
            )
            .note(markup! {
                "This value can't be "<Emphasis>"null"</Emphasis>" or "<Emphasis>"undefined"</Emphasis>", so the assertion has no effect."
            }),
            UselessTypeAssertion::NonNullAssignmentTarget => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "This non-null assertion is unnecessary." },
            )
            .note(markup! {
                "An assignment replaces the current value without reading it, so the value doesn't need to be non-null."
            }),
            UselessTypeAssertion::SameType => RuleDiagnostic::new(
                rule_category!(),
                node.assertion_range()?,
                markup! { "This type assertion is unnecessary." },
            )
            .note(markup! {
                "The expression already has this type, so the assertion has no effect."
            }),
            UselessTypeAssertion::Chained(_) => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "These type assertions are unnecessary." },
            )
            .note(markup! {
                "The original expression already has the final type, so the assertions have no effect."
            }),
        };

        Some(diagnostic.note(markup! {
            "Unnecessary assertions add noise and make the assertions that matter harder to notice."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let (expression, replacement) = match (node, state) {
            (AnyTsUselessTypeAssertionCandidate::TsNonNullAssertionAssignment(assignment), _) => {
                let target = AnyJsAssignment::from(assignment.clone());
                mutation.replace_node_discard_trivia(
                    target.clone(),
                    with_transferred_trivia(&target, assignment.assignment().ok()?)?,
                );
                return Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove the non-null assertion." }.to_owned(),
                    mutation,
                ));
            }
            (_, UselessTypeAssertion::Chained(original)) => (
                AnyJsExpression::cast(node.syntax().clone())?,
                original.clone(),
            ),
            (AnyTsUselessTypeAssertionCandidate::TsNonNullAssertionExpression(expression), _) => {
                (expression.clone().into(), expression.expression().ok()?)
            }
            (AnyTsUselessTypeAssertionCandidate::TsAsExpression(expression), _) => {
                (expression.clone().into(), expression.expression().ok()?)
            }
            (AnyTsUselessTypeAssertionCandidate::TsTypeAssertionExpression(expression), _) => {
                (expression.clone().into(), expression.expression().ok()?)
            }
        };

        let mut replacement = with_transferred_trivia(&expression, replacement)?;
        if needs_parentheses(&expression, &replacement) {
            replacement = parenthesized(replacement)?;
        }
        mutation.replace_node_discard_trivia(expression, replacement);

        let message = match state {
            UselessTypeAssertion::NonNull | UselessTypeAssertion::NonNullAssignmentTarget => {
                markup! { "Remove the non-null assertion." }
            }
            UselessTypeAssertion::SameType => markup! { "Remove the type assertion." },
            UselessTypeAssertion::Chained(_) => markup! { "Remove the type assertions." },
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message.to_owned(),
            mutation,
        ))
    }
}

impl AnyTsUselessTypeAssertionCandidate {
    /// Returns the range of `as T` or `<T>`, excluding the asserted expression.
    fn assertion_range(&self) -> Option<TextRange> {
        Some(match self {
            Self::TsAsExpression(expression) => expression
                .as_token()
                .ok()?
                .text_trimmed_range()
                .cover(expression.ty().ok()?.range()),
            Self::TsTypeAssertionExpression(expression) => expression
                .l_angle_token()
                .ok()?
                .text_trimmed_range()
                .cover(expression.r_angle_token().ok()?.text_trimmed_range()),
            Self::TsNonNullAssertionAssignment(_) | Self::TsNonNullAssertionExpression(_) => {
                self.range()
            }
        })
    }
}

fn run_non_null_assertion(
    ctx: &RuleContext<NoUselessTypeAssertion>,
    node: &TsNonNullAssertionExpression,
) -> Option<UselessTypeAssertion> {
    // The assertion narrows the assigned variable for the code that follows,
    // even when the right side alone doesn't need it.
    if node
        .syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !JsParenthesizedExpression::can_cast(ancestor.kind()))
        .and_then(JsAssignmentExpression::cast)
        .is_some_and(|assignment| assignment.operator() == Ok(JsAssignmentOperator::Assign))
    {
        return None;
    }

    let operand = node.expression().ok()?.omit_parentheses();
    // `value!!` is reported by `noExtraNonNullAssertion`.
    if matches!(operand, AnyJsExpression::TsNonNullAssertionExpression(_)) {
        return None;
    }

    if ctx.type_of_expression(&operand)?.has_nullish_variant()? {
        return None;
    }

    // Reading a key through an index signature may return `undefined` when
    // the `noUncheckedIndexedAccess` compiler option is on.
    if let Some(object) = member_object(&operand)
        && ctx.type_of_expression(&object)?.has_index_signature()?
    {
        return None;
    }

    if let AnyJsExpression::JsIdentifierExpression(identifier) = &operand
        && may_hold_undefined(ctx, identifier)
    {
        return None;
    }

    Some(UselessTypeAssertion::NonNull)
}

fn run_type_assertion(
    ctx: &RuleContext<NoUselessTypeAssertion>,
    node: &AnyJsExpression,
    ty: &AnyTsType,
    operand: AnyJsExpression,
) -> Option<UselessTypeAssertion> {
    if is_const_reference_type(ty) {
        return None;
    }

    let cast_type = ctx.type_of_expression(node)?;
    let inner = operand.clone().omit_parentheses();
    if is_unchanged_by_assertion(ctx, cast_type, &inner) {
        return Some(UselessTypeAssertion::SameType);
    }
    if !is_type_assertion(&inner) {
        return None;
    }

    let mut original = operand;
    loop {
        original = match original.clone().omit_parentheses() {
            AnyJsExpression::TsAsExpression(assertion) => assertion.expression().ok()?,
            AnyJsExpression::TsTypeAssertionExpression(assertion) => assertion.expression().ok()?,
            _ => break,
        };
    }

    is_unchanged_by_assertion(ctx, cast_type, &original.clone().omit_parentheses())
        .then_some(UselessTypeAssertion::Chained(original))
}

/// Returns whether asserting `expression` to `cast_type` keeps the type that
/// `expression` already has.
fn is_unchanged_by_assertion(
    ctx: &RuleContext<NoUselessTypeAssertion>,
    cast_type: InferredType,
    expression: &AnyJsExpression,
) -> bool {
    ctx.type_of_expression(expression)
        .is_some_and(|ty| cast_type.is_same_primitive_type_as(ty))
        && has_type_independent_of_location(ctx, expression)
}

/// Returns whether TypeScript gives `expression` the same type wherever it's
/// written.
///
/// Inference only knows the declared type of variables and properties, while
/// TypeScript narrows them at each use, for example to `"a"` inside
/// `if (value === "a")`. Expressions built from them, such as `a ?? b`, inherit
/// the narrowed type. Comparing the declared type with the asserted type could
/// then report an assertion that widens the narrowed type.
fn has_type_independent_of_location(
    ctx: &RuleContext<NoUselessTypeAssertion>,
    expression: &AnyJsExpression,
) -> bool {
    match expression {
        AnyJsExpression::JsAwaitExpression(expression) => expression
            .argument()
            .ok()
            .and_then(|argument| argument.omit_parentheses().as_js_call_expression().cloned())
            .is_some_and(|call| !has_inferred_type_arguments(ctx, &call)),
        // Inference gives other arithmetic operators the type `number`, even for
        // `bigint` operands.
        AnyJsExpression::JsBinaryExpression(expression) => expression
            .operator_token()
            .is_ok_and(|operator| ADDITION_OR_COMPARISON_OPERATORS.contains(operator.kind())),
        AnyJsExpression::JsCallExpression(call) => !has_inferred_type_arguments(ctx, call),
        AnyJsExpression::TsAsExpression(_) | AnyJsExpression::TsTypeAssertionExpression(_) => true,
        _ => false,
    }
}

const ADDITION_OR_COMPARISON_OPERATORS: TokenSet<JsSyntaxKind> = token_set![
    T![+],
    T![==],
    T![!=],
    T![===],
    T![!==],
    T![<],
    T![<=],
    T![>],
    T![>=]
];

fn is_type_assertion(expression: &AnyJsExpression) -> bool {
    matches!(
        expression,
        AnyJsExpression::TsAsExpression(_) | AnyJsExpression::TsTypeAssertionExpression(_)
    )
}

fn member_object(expression: &AnyJsExpression) -> Option<AnyJsExpression> {
    match expression {
        AnyJsExpression::JsComputedMemberExpression(member) => member.object().ok(),
        AnyJsExpression::JsStaticMemberExpression(member) => member.object().ok(),
        _ => None,
    }
}

/// Returns whether `identifier` refers to a variable whose value may be
/// `undefined` even though its type doesn't include `undefined`:
///
/// - a variable that may be read before it receives a value, such as
///   `let value: number;`;
/// - a variable that holds a value read through an index signature, such as
///   `const value = record.key;` or `const { key } = record;`. This value may be
///   `undefined` when the `noUncheckedIndexedAccess` compiler option is on.
fn may_hold_undefined(
    ctx: &RuleContext<NoUselessTypeAssertion>,
    identifier: &JsIdentifierExpression,
) -> bool {
    let Some(model) = ctx.get_service::<SemanticModel>() else {
        return true;
    };
    let Ok(reference) = identifier.name() else {
        return true;
    };
    let Some(binding) = model.binding(&reference) else {
        return true;
    };
    let Some(declaration) = binding.tree().declaration() else {
        return false;
    };
    let pattern_declaration = declaration.parent_binding_pattern_declaration();
    let is_destructured = pattern_declaration.is_some();
    // Destructured parameters and `for...of` variables may also read through an
    // index signature, but their source isn't known here.
    let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) =
        pattern_declaration.or(Some(declaration))
    else {
        return is_destructured;
    };

    // A `var` declared in a block is also visible outside of that block, where
    // it may be unassigned.
    if declarator
        .declaration()
        .is_some_and(|declaration| declaration.is_var())
        && !model
            .scope(declarator.syntax())
            .is_ancestor_of(&model.scope(reference.syntax()))
    {
        return true;
    }

    if is_declared_without_value(&declarator) {
        return true;
    }
    if !is_destructured && declarator.variable_annotation().is_some() {
        return false;
    }

    let Some(initializer) = declarator
        .initializer()
        .and_then(|initializer| initializer.expression().ok())
        .map(AnyJsExpression::omit_parentheses)
    else {
        return is_destructured;
    };
    let source = if is_destructured {
        Some(initializer)
    } else {
        member_object(&initializer)
    };
    source.is_some_and(|source| {
        ctx.type_of_expression(&source)
            .and_then(|ty| ty.has_index_signature())
            != Some(false)
    })
}

/// Returns whether `declarator` has a type annotation but neither an
/// initializer nor a definite assignment assertion (`let value!: number`).
/// Ambient `declare` declarations never get a value in the source, so they're
/// excluded.
fn is_declared_without_value(declarator: &JsVariableDeclarator) -> bool {
    declarator.initializer().is_none()
        && matches!(
            declarator.variable_annotation(),
            Some(AnyTsVariableAnnotation::TsTypeAnnotation(_))
        )
        && !declarator
            .syntax()
            .ancestors()
            .any(|ancestor| TsDeclareStatement::can_cast(ancestor.kind()))
}

/// Returns whether `call` may infer its type arguments from the asserted type,
/// such as `parse() as number` with `declare function parse<T>(): T`.
/// Removing the assertion could change the inferred type arguments.
fn has_inferred_type_arguments(
    ctx: &RuleContext<NoUselessTypeAssertion>,
    call: &JsCallExpression,
) -> bool {
    call.type_arguments().is_none()
        && !call
            .callee()
            .ok()
            .and_then(|callee| ctx.type_of_expression(&callee))
            .is_some_and(|callee| callee.is_non_generic_function())
}

fn is_const_reference_type(ty: &AnyTsType) -> bool {
    let Some(reference_type) = ty.as_ts_reference_type() else {
        return false;
    };

    reference_type.type_arguments().is_none()
        && reference_type.name().is_ok_and(|name| {
            name.as_js_reference_identifier()
                .and_then(|identifier| identifier.value_token().ok())
                .is_some_and(|token| token.text_trimmed() == "const")
        })
}

/// Returns whether `replacement` needs parentheses to keep its meaning when it
/// takes the place of `expression`.
///
/// An assertion like `a + b as number` can be the operand of a tighter
/// operator, as in `a + b as number * 2`, which means `(a + b) * 2`. A unary
/// operand, such as `await value`, can't be the left side of `**` without
/// parentheses.
fn needs_parentheses(expression: &AnyJsExpression, replacement: &AnyJsExpression) -> bool {
    replacement
        .precedence()
        .is_ok_and(|precedence| precedence <= OperatorPrecedence::Unary)
        && expression
            .parent::<AnyJsExpression>()
            .is_some_and(|parent| !matches!(parent, AnyJsExpression::JsParenthesizedExpression(_)))
}

/// Wraps `expression` in parentheses, moving its outer trivia outside of them.
fn parenthesized(expression: AnyJsExpression) -> Option<AnyJsExpression> {
    let leading_trivia: Vec<_> = expression
        .syntax()
        .first_token()?
        .leading_trivia()
        .pieces()
        .collect();
    let trailing_trivia: Vec<_> = expression
        .syntax()
        .last_token()?
        .trailing_trivia()
        .pieces()
        .collect();
    let expression = expression.trim_trivia()?;
    Some(
        make::js_parenthesized_expression(
            make::token(T!['(']).with_leading_trivia_pieces(leading_trivia),
            expression,
            make::token(T![')']).with_trailing_trivia_pieces(trailing_trivia),
        )
        .into(),
    )
}

/// Returns `replacement` with the trivia that `node` has outside of
/// `replacement`, so that removing the assertion keeps surrounding comments.
fn with_transferred_trivia<N: AstNode<Language = JsLanguage>>(
    node: &N,
    replacement: N,
) -> Option<N> {
    let expression = node.syntax();
    let mut replacement = replacement.into_syntax();
    let first_token = expression.first_token()?;
    let last_token = expression.last_token()?;
    let replacement_first_token = replacement.first_token()?;
    let replacement_last_token = replacement.last_token()?;

    if first_token != replacement_first_token {
        replacement = replacement.with_leading_trivia_pieces(chain_trivia_pieces(
            first_token.leading_trivia().pieces(),
            replacement_first_token.leading_trivia().pieces(),
        ))?;
    }

    if last_token != replacement_last_token {
        // Whitespace between the last comment and `as` or `!` is dropped.
        let mut trailing_trivia: Vec<_> =
            replacement_last_token.trailing_trivia().pieces().collect();
        let kept = trailing_trivia
            .iter()
            .rposition(|piece| piece.is_comments())
            .map_or(0, |index| index + 1);
        trailing_trivia.truncate(kept);
        trailing_trivia.extend(last_token.trailing_trivia().pieces());
        replacement = replacement.with_trailing_trivia_pieces(trailing_trivia)?;
    }

    N::cast(replacement)
}
