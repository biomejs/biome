use crate::{JsRuleAction, ast_utils::starts_unterminated_statement, services::semantic::Semantic};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsCallArgument, AnyJsExpression, AnyTsName, AnyTsType,
    AnyTsVariableAnnotation, JsCallExpression, JsStaticMemberExpression,
    JsSyntaxToken, JsUnaryExpression, JsUnaryOperator, T, binding_ext::AnyJsBindingDeclaration,
    global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, Direction, TriviaPieceKind};
use biome_rule_options::use_set_size::UseSetSizeOptions;

declare_lint_rule! {
    /// Prefer reading the `size` property of a `Set` over converting the `Set` to an array to read its `length`.
    ///
    /// A `Set` already keeps track of how many values it contains in its `size` property.
    /// Copying the values into a new array, with the spread syntax `[...set]` or with `Array.from(set)`, only to read the `length` of that array, does extra work and creates an array that is thrown away right after.
    ///
    /// The rule reports `[...set].length` and `Array.from(set).length` when it can tell that `set` is a `Set`. This is the case when the value is:
    ///
    /// - created with `new Set()`;
    /// - a `const` variable initialized with `new Set()`;
    /// - a variable or a function parameter with the type `Set` or `ReadonlySet`;
    /// - marked as a `Set` or a `ReadonlySet` with `as` or `satisfies`.
    ///
    /// :::caution
    /// The fix is unsafe because the rule trusts type annotations and `as` expressions without inferring them.
    /// If the value isn't a `Set` when the code runs, for example an array that was cast with `as Set<string>`, the value has no `size` property, and the fixed code returns `undefined` instead of a number.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function isUnique(array) {
    ///     return [...new Set(array)].length === array.length;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const items = new Set([1, 2, 3, 4, 5]);
    /// if ([...items].length > 3) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const uniqueCount = Array.from(new Set(userIds)).length;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function countTags(tags: Set<string>) {
    ///     return [...tags].length;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function isUnique(array) {
    ///     return new Set(array).size === array.length;
    /// }
    ///
    /// const items = new Set([1, 2, 3, 4, 5]);
    /// if (items.size > 3) {}
    ///
    /// // The rule can't tell whether `values` is a `Set`.
    /// const count = [...values].length;
    ///
    /// // The array contains more than the values of the `Set`.
    /// const total = [...new Set(array), extra].length;
    /// ```
    ///
    pub UseSetSize {
        version: "2.6.5",
        name: "useSetSize",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-set-size").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseSetSize {
    type Query = Semantic<JsStaticMemberExpression>;
    /// The expression that evaluates to the `Set`.
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = UseSetSizeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        if node.operator_token().ok()?.kind() != T![.]
            || node
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?
                .text_trimmed()
                != "length"
            || is_deleted(node)
        {
            return None;
        }

        let set = match node.object().ok()?.omit_parentheses() {
            AnyJsExpression::JsArrayExpression(array) => {
                let mut elements = array.elements().iter();
                let AnyJsArrayElement::JsSpread(spread) = elements.next()?.ok()? else {
                    return None;
                };
                if elements.next().is_some() {
                    return None;
                }
                spread.argument().ok()?
            }
            AnyJsExpression::JsCallExpression(call) => array_from_argument(&call, model)?,
            _ => return None,
        };

        is_set(&set, model, true).then_some(set)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This code copies a "<Emphasis>"Set"</Emphasis>" into an array only to count its values."
                },
            )
            .note(markup! {
                "Copying every value takes extra time and memory, while the "<Emphasis>"size"</Emphasis>" property of the "<Emphasis>"Set"</Emphasis>" already holds the count."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, set: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if has_comments_outside(node, set) {
            return None;
        }

        let object = set.clone().trim_trivia()?;
        let object = if needs_parentheses(&object) {
            make::parenthesized(object).into()
        } else {
            object
        };
        let starts_with_parenthesis = object
            .syntax()
            .first_token()
            .is_some_and(|token| token.kind() == T!['(']);
        if starts_with_parenthesis
            && starts_unterminated_statement(&AnyJsExpression::from(node.clone()))
        {
            return None;
        }

        let replacement = make::js_static_member_expression(
            object,
            make::token(T![.]),
            make::js_name(make::ident("size")).into(),
        );

        let mut mutation = ctx.root().begin();
        // In `return[...set].length`, the keyword is directly followed by `[`. The replacement
        // starts with a name such as `set`, which must not be joined to the keyword.
        if !starts_with_parenthesis
            && let Some(keyword) = keyword_directly_before(node)
        {
            mutation.replace_token_discard_trivia(
                keyword.clone(),
                keyword.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            );
        }
        mutation.replace_node(node.clone(), replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"size"</Emphasis>" instead." },
            mutation,
        ))
    }
}

/// Returns `true` if `node` is the operand of `delete`, as in `delete [...set].length`.
fn is_deleted(node: &JsStaticMemberExpression) -> bool {
    AnyJsExpression::from(node.clone())
        .outer_expression()
        .and_then(|outer| outer.parent::<JsUnaryExpression>())
        .is_some_and(|unary| unary.operator() == Ok(JsUnaryOperator::Delete))
}

/// Returns the only argument of `call` if `call` is `Array.from(argument)`, where `Array` is
/// the built-in global.
fn array_from_argument(call: &JsCallExpression, model: &SemanticModel) -> Option<AnyJsExpression> {
    if call.optional_chain_token().is_some() {
        return None;
    }

    let callee = call.callee().ok()?.omit_parentheses();
    let callee = callee.as_js_static_member_expression()?;
    if callee.operator_token().ok()?.kind() != T![.]
        || callee
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .text_trimmed()
            != "from"
    {
        return None;
    }

    let object = callee.object().ok()?.omit_parentheses();
    let (reference, name) = global_identifier(&object.as_any_global_identifier_expression()?)?;
    if name.text() != "Array" || model.binding(&reference).is_some() {
        return None;
    }

    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 1 {
        return None;
    }
    match arguments.first()?.ok()? {
        AnyJsCallArgument::AnyJsExpression(argument) => Some(argument),
        AnyJsCallArgument::JsSpread(_) => None,
    }
}

/// Returns `true` if the rule can tell that `expression` evaluates to a `Set`.
///
/// Variables are only followed when `follow_variables` is `true`, so that the initializer of a
/// variable isn't checked through another variable.
fn is_set(expression: &AnyJsExpression, model: &SemanticModel, follow_variables: bool) -> bool {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::JsNewExpression(new) => new.callee().is_ok_and(|callee| {
            callee
                .omit_parentheses()
                .as_any_global_identifier_expression()
                .and_then(|callee| global_identifier(&callee))
                .is_some_and(|(reference, name)| {
                    name.text() == "Set" && model.binding(&reference).is_none()
                })
        }),
        AnyJsExpression::TsAsExpression(assertion) => {
            assertion.ty().is_ok_and(|ty| is_set_type(&ty, model))
                || assertion
                    .expression()
                    .is_ok_and(|inner| is_set(&inner, model, follow_variables))
        }
        AnyJsExpression::TsSatisfiesExpression(assertion) => {
            assertion.ty().is_ok_and(|ty| is_set_type(&ty, model))
                || assertion
                    .expression()
                    .is_ok_and(|inner| is_set(&inner, model, follow_variables))
        }
        AnyJsExpression::TsTypeAssertionExpression(assertion) => {
            assertion.ty().is_ok_and(|ty| is_set_type(&ty, model))
                || assertion
                    .expression()
                    .is_ok_and(|inner| is_set(&inner, model, follow_variables))
        }
        AnyJsExpression::TsNonNullAssertionExpression(assertion) => assertion
            .expression()
            .is_ok_and(|inner| is_set(&inner, model, follow_variables)),
        AnyJsExpression::JsIdentifierExpression(identifier) if follow_variables => identifier
            .name()
            .ok()
            .and_then(|reference| model.binding(&reference))
            .and_then(|binding| binding.tree().declaration())
            .is_some_and(|declaration| is_set_declaration(&declaration, model)),
        _ => false,
    }
}

/// Returns `true` if the variable or parameter declared by `declaration` holds a `Set`.
///
/// A `let` or `var` variable without a type annotation isn't recognized, because it can be
/// given another value later.
fn is_set_declaration(declaration: &AnyJsBindingDeclaration, model: &SemanticModel) -> bool {
    match declaration {
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => parameter
            .type_annotation()
            .and_then(|annotation| annotation.ty().ok())
            .is_some_and(|ty| is_set_type(&ty, model)),
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            if let Some(AnyTsVariableAnnotation::TsTypeAnnotation(annotation)) =
                declarator.variable_annotation()
            {
                return annotation.ty().is_ok_and(|ty| is_set_type(&ty, model));
            }

            declarator
                .declaration()
                .is_some_and(|declaration| declaration.is_const())
                && declarator
                    .initializer()
                    .and_then(|initializer| initializer.expression().ok())
                    .is_some_and(|value| is_set(&value, model, false))
        }
        _ => false,
    }
}

/// Returns `true` if `ty` is the built-in `Set` or `ReadonlySet` type.
fn is_set_type(ty: &AnyTsType, model: &SemanticModel) -> bool {
    ty.clone()
        .omit_parentheses()
        .as_ts_reference_type()
        .and_then(|reference| reference.name().ok())
        .and_then(|name| match name {
            AnyTsName::JsReferenceIdentifier(name) => Some(name),
            AnyTsName::TsQualifiedName(_) => None,
        })
        .is_some_and(|name| {
            name.value_token()
                .is_ok_and(|token| matches!(token.text_trimmed(), "Set" | "ReadonlySet"))
                && model.binding(&name).is_none()
        })
}

/// Returns `true` if a comment inside `node` is outside of `set`, so the fix would remove it.
fn has_comments_outside(node: &JsStaticMemberExpression, set: &AnyJsExpression) -> bool {
    let node_range = node.syntax().text_trimmed_range();
    let set_range = set.syntax().text_trimmed_range();
    node.syntax()
        .descendants_tokens(Direction::Next)
        .flat_map(|token| {
            token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
        })
        .any(|piece| {
            piece.is_comments()
                && node_range.contains_range(piece.text_range())
                && !set_range.contains_range(piece.text_range())
        })
}

/// Returns `true` if `object` must be wrapped in parentheses before `.size` can be read from it.
///
/// `new Set` without arguments would read `size` from `Set` itself, and `set as Set<string>`
/// would apply `as` to the whole `set as Set<string>.size` expression.
fn needs_parentheses(object: &AnyJsExpression) -> bool {
    match object {
        AnyJsExpression::JsIdentifierExpression(_)
        | AnyJsExpression::JsParenthesizedExpression(_) => false,
        AnyJsExpression::JsNewExpression(new) => new.arguments().is_none(),
        _ => true,
    }
}

/// Returns the keyword right before `node` when no whitespace separates them, as in
/// `return[...set].length`.
fn keyword_directly_before(node: &JsStaticMemberExpression) -> Option<JsSyntaxToken> {
    let first_token = node.syntax().first_token()?;
    let previous = first_token.prev_token()?;
    (previous.kind().is_keyword()
        && previous.text_trimmed_range().end() == first_token.text_trimmed_range().start())
    .then_some(previous)
}
