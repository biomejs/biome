use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsAssignment, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument,
    AnyJsExpression, AnyJsFormalParameter, AnyJsLiteralExpression, AnyJsName, AnyJsStatement,
    AnyJsTemplateElement, AnyTsType, JsBinaryOperator, JsCallExpression, JsGetterClassMember,
    JsGetterObjectMember, JsIdentifierBinding, JsIfStatement, JsLanguage, JsLogicalOperator,
    JsModuleItemList, JsSetterClassMember, JsSetterObjectMember, JsStatementList,
    JsUnaryExpression, JsUnaryOperator, JsVariableDeclarator, T,
    binding_ext::AnyJsBindingDeclaration,
    numbers::{canonicalize_js_bigint_literal, parse_js_number_with_single_rounding},
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet, Text, TextRange, TriviaPieceKind,
    WalkEvent,
};
use biome_rule_options::use_else_if::UseElseIfOptions;

use crate::JsRuleAction;
use crate::services::semantic::Semantic;
use crate::utils::condition::split_by_logical_operator;
use crate::utils::early_exit::breaks_early;
use crate::utils::indexed_collection::binding_type_annotation;
use crate::utils::same_reference::{is_same_assignment_target, is_same_reference, string_value};

declare_lint_rule! {
    /// Require `else if` when adjacent `if` statements compare the same value against different values.
    ///
    /// When two `if` statements in a row compare the same variable or property with `===`
    /// against different values, only one of them can match. Joining them with `else if`
    /// makes this clear, and skips the second check once the first one matched.
    ///
    /// Conditions like `if (flag)` and `if (!flag)` count as comparisons with `true` and
    /// `false` when `flag` is declared as a `boolean`.
    ///
    /// The rule ignores an `if` statement when a previous branch ends with `return`, `throw`,
    /// `break`, or `continue`, or assigns a new value to the compared variable or property.
    ///
    /// The fix is unsafe, because a function called in a previous branch could change the
    /// compared value.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (status === "loading") {
    ///     showSpinner();
    /// }
    /// if (status === "error") {
    ///     showError();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
    ///     previous();
    /// } else if (event.key === "ArrowDown") {
    ///     next();
    /// }
    /// if (event.key === "Escape") {
    ///     close();
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function toggle(enabled: boolean) {
    ///     if (enabled) {
    ///         start();
    ///     }
    ///     if (enabled === false) {
    ///         stop();
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (status === "loading") {
    ///     showSpinner();
    /// } else if (status === "error") {
    ///     showError();
    /// }
    ///
    /// // The conditions compare different variables.
    /// if (status === "loading") {}
    /// if (mode === "dark") {}
    ///
    /// // Both conditions can be true at the same time.
    /// if (count === 1 || count === 2) {}
    /// if (count === 2) {}
    ///
    /// // The first branch changes the compared value.
    /// if (step === 1) {
    ///     step = 2;
    /// }
    /// if (step === 2) {}
    ///
    /// function handle(signal) {
    ///     // The first branch always leaves the function.
    ///     if (signal === "SIGINT") {
    ///         return;
    ///     }
    ///     if (signal === "SIGTERM") {}
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to remove `else` after a branch that ends with `return` or `throw`, see [`noUselessElse`](https://biomejs.dev/linter/rules/no-useless-else/).
    /// - If you want to turn `else { if (...) {} }` into `else if (...) {}`, see [`useCollapsedElseIf`](https://biomejs.dev/linter/rules/use-collapsed-else-if/).
    pub UseElseIf {
        version: "2.6.10",
        name: "useElseIf",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-else-if").inspired()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseElseIf {
    type Query = Semantic<JsIfStatement>;
    /// The last `if` statement of the previous `if`/`else if` chain. The fix adds the
    /// reported `if` statement as its `else` branch.
    type State = JsIfStatement;
    type Signals = Option<Self::State>;
    type Options = UseElseIfOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let if_statement = ctx.query();
        let model = ctx.model();
        if !STATEMENT_LIST_KINDS.matches(if_statement.syntax().parent()?.kind()) {
            return None;
        }
        let previous = JsIfStatement::cast(if_statement.syntax().prev_sibling()?)?;

        let previous_chain = if_chain(previous)?;
        let chain = if_chain(if_statement.clone())?;
        let consequents = previous_chain
            .iter()
            .map(|if_statement| if_statement.consequent().ok())
            .collect::<Option<Vec<_>>>()?;
        if consequents
            .iter()
            .any(|consequent| breaks_early(consequent.clone()).is_some())
        {
            return None;
        }

        let previous_comparison = Comparison::from_chain(model, &previous_chain)?;
        let comparison = Comparison::from_chain(model, &chain)?;
        if !is_same_reference(
            previous_comparison.discriminant.clone(),
            comparison.discriminant,
        )? || previous_comparison
            .values
            .iter()
            .any(|value| comparison.values.contains(value))
        {
            return None;
        }

        let references = reference_prefixes(previous_comparison.discriminant)?;
        if consequents
            .iter()
            .any(|consequent| changes_reference(consequent, &references))
        {
            return None;
        }

        previous_chain.into_iter().next_back()
    }

    fn diagnostic(ctx: &RuleContext<Self>, last_if: &Self::State) -> Option<RuleDiagnostic> {
        let if_statement = ctx.query();
        let range = TextRange::new(
            if_statement.if_token().ok()?.text_trimmed_range().start(),
            if_statement
                .r_paren_token()
                .ok()?
                .text_trimmed_range()
                .end(),
        );
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"if"</Emphasis>" statement compares the same value as the previous one, but against a different value."
                },
            )
            .detail(
                last_if.test().ok()?.range(),
                markup! { "The previous condition is here." },
            )
            .note(markup! {
                "Only one of these conditions can be true, but separate "<Emphasis>"if"</Emphasis>" statements hide this, and the second condition is still checked when the first one matched."
            })
            .note(markup! {
                "Add "<Emphasis>"else"</Emphasis>" before this "<Emphasis>"if"</Emphasis>" statement to join it to the previous one."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, last_if: &Self::State) -> Option<JsRuleAction> {
        // An `else` after an `if` that has no `else` of its own belongs to that inner `if`.
        if ends_with_dangling_if(last_if.consequent().ok()?) {
            return None;
        }
        let if_statement = ctx.query();

        // Keep comments between the two statements in place, and otherwise write `} else if`
        // on a single line.
        let keep_trivia = if_statement.syntax().has_leading_comments()
            || last_if.syntax().has_trailing_comments();
        let (else_token, alternate) = if keep_trivia {
            let leading_trivia = if_statement.syntax().first_leading_trivia()?;
            (
                make::token(T![else])
                    .with_leading_trivia_pieces(leading_trivia.pieces())
                    .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                if_statement.clone().with_leading_trivia_pieces([])?,
            )
        } else {
            (
                make::token_decorated_with_space(T![else]),
                if_statement.clone().trim_leading_trivia()?,
            )
        };
        let else_clause = make::js_else_clause(else_token, AnyJsStatement::from(alternate));

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(
            last_if.clone(),
            last_if.clone().with_else_clause(Some(else_clause)),
        );
        mutation.remove_node(if_statement.clone());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add "<Emphasis>"else"</Emphasis>" before this "<Emphasis>"if"</Emphasis>" statement." }
                .to_owned(),
            mutation,
        ))
    }
}

/// The value that every condition of an `if`/`else if` chain compares, and the values it is
/// compared against.
struct Comparison {
    discriminant: AnyJsExpression,
    values: Vec<StaticValue>,
}

impl Comparison {
    /// Returns `None` unless every condition of `chain` compares the same variable or property
    /// with `===` against values known without running the code.
    fn from_chain(model: &SemanticModel, chain: &[JsIfStatement]) -> Option<Self> {
        let mut discriminant: Option<AnyJsExpression> = None;
        let mut values = Vec::new();
        for if_statement in chain {
            let test = if_statement.test().ok()?;
            let comparisons: Vec<_> = match boolean_test(model, &test) {
                Some(comparison) => vec![comparison],
                None => split_by_logical_operator(JsLogicalOperator::LogicalOr, test)
                    .map(|operand| strict_equality(model, &operand))
                    .collect::<Option<_>>()?,
            };
            for (candidate, value) in comparisons {
                match &discriminant {
                    Some(discriminant) => {
                        if !is_same_reference(discriminant.clone(), candidate)? {
                            return None;
                        }
                    }
                    None => discriminant = Some(candidate),
                }
                values.push(value);
            }
        }
        Some(Self {
            discriminant: discriminant?,
            values,
        })
    }
}

/// A value that a variable or property can be compared against with `===`.
#[derive(PartialEq)]
enum StaticValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    /// The canonical text of a `bigint` literal, so `0x10n` and `16n` are equal.
    BigInt(Text),
    String(Text),
}

impl StaticValue {
    /// Returns `None` if `expression` isn't a literal, a template literal without
    /// placeholders, or the global `undefined`, or if its value can't be decoded reliably.
    fn from_expression(model: &SemanticModel, expression: &AnyJsExpression) -> Option<Self> {
        match expression.clone().omit_parentheses() {
            AnyJsExpression::AnyJsLiteralExpression(literal) => match literal {
                AnyJsLiteralExpression::JsBooleanLiteralExpression(literal) => Some(Self::Boolean(
                    literal.value_token().ok()?.text_trimmed() == "true",
                )),
                AnyJsLiteralExpression::JsNullLiteralExpression(_) => Some(Self::Null),
                AnyJsLiteralExpression::JsNumberLiteralExpression(literal) => {
                    Some(Self::Number(parse_js_number_with_single_rounding(
                        literal.value_token().ok()?.text_trimmed(),
                    )?))
                }
                AnyJsLiteralExpression::JsBigintLiteralExpression(literal) => {
                    Some(Self::BigInt(Text::from(
                        canonicalize_js_bigint_literal(literal.value_token().ok()?.text_trimmed())?
                            .into_owned(),
                    )))
                }
                AnyJsLiteralExpression::JsStringLiteralExpression(literal) => Some(Self::String(
                    string_value(literal.inner_string_text().ok()?)?,
                )),
                AnyJsLiteralExpression::JsRegexLiteralExpression(_) => None,
            },
            AnyJsExpression::JsTemplateExpression(template) if template.tag().is_none() => {
                let mut elements = template.elements().into_iter();
                match (elements.next(), elements.next()) {
                    (None, _) => Some(Self::String(Text::new_static(""))),
                    (Some(AnyJsTemplateElement::JsTemplateChunkElement(chunk)), None) => {
                        Some(Self::String(string_value(
                            chunk.template_chunk_token().ok()?.token_text_trimmed(),
                        )?))
                    }
                    _ => None,
                }
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                let reference = identifier.name().ok()?;
                (reference.value_token().ok()?.text_trimmed() == "undefined"
                    && model.binding(&reference).is_none())
                .then_some(Self::Undefined)
            }
            _ => None,
        }
    }
}

/// Returns the compared reference and value of `expression` when it has the form
/// `reference === value` or `value === reference`.
fn strict_equality(
    model: &SemanticModel,
    expression: &AnyJsExpression,
) -> Option<(AnyJsExpression, StaticValue)> {
    let binary = expression.as_js_binary_expression()?;
    if binary.operator().ok()? != JsBinaryOperator::StrictEquality {
        return None;
    }
    let left = binary.left().ok()?;
    let right = binary.right().ok()?;
    let (reference, value) = match (
        StaticValue::from_expression(model, &left),
        StaticValue::from_expression(model, &right),
    ) {
        (Some(value), None) => (right, value),
        (None, Some(value)) => (left, value),
        _ => return None,
    };
    is_static_reference(&reference).then_some((reference, value))
}

/// Returns the compared variable and value of a condition such as `flag`, `!flag`, or
/// `Boolean(flag)`, when `flag` is known to hold a boolean.
fn boolean_test(
    model: &SemanticModel,
    test: &AnyJsExpression,
) -> Option<(AnyJsExpression, StaticValue)> {
    let mut expression = test.clone().omit_type_wrappers()?;
    let mut value = true;
    while let Some(unary) = expression.as_js_unary_expression()
        && unary.operator().ok()? == JsUnaryOperator::LogicalNot
    {
        value = !value;
        expression = unary.argument().ok()?.omit_type_wrappers()?;
    }
    if let Some(call) = expression.as_js_call_expression()
        && let Some(argument) = global_boolean_call_argument(model, call)
    {
        expression = argument.omit_type_wrappers()?;
    }

    let reference = expression.as_js_identifier_expression()?.name().ok()?;
    let binding = model.binding(&reference)?;
    let declaration = binding.tree().declaration()?;
    let is_boolean = match binding_type_annotation(&binding) {
        Some(annotation) => {
            !is_optional_parameter(&declaration) && is_boolean_type(annotation.ty().ok()?)
        }
        None => match &declaration {
            AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
                is_const_with_boolean_literal(declarator)
            }
            _ => false,
        },
    };
    is_boolean.then_some((expression, StaticValue::Boolean(value)))
}

/// Returns the argument of `call` if it is a call of the global `Boolean` function with
/// a single argument.
fn global_boolean_call_argument(
    model: &SemanticModel,
    call: &JsCallExpression,
) -> Option<AnyJsExpression> {
    if call.is_optional() {
        return None;
    }
    let callee = call.callee().ok()?.omit_parentheses();
    let reference = callee.as_js_identifier_expression()?.name().ok()?;
    if reference.value_token().ok()?.text_trimmed() != "Boolean"
        || model.binding(&reference).is_some()
    {
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

fn is_optional_parameter(declaration: &AnyJsBindingDeclaration) -> bool {
    let parameter = match declaration {
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => Some(parameter.clone()),
        AnyJsBindingDeclaration::TsPropertyParameter(parameter) => {
            match parameter.formal_parameter() {
                Ok(AnyJsFormalParameter::JsFormalParameter(parameter)) => Some(parameter),
                _ => None,
            }
        }
        _ => None,
    };
    parameter.is_some_and(|parameter| parameter.question_mark_token().is_some())
}

/// Returns whether `ty` only allows `true` and `false`, such as `boolean` or `true | false`.
fn is_boolean_type(ty: AnyTsType) -> bool {
    let mut types = vec![ty];
    while let Some(ty) = types.pop() {
        match ty {
            AnyTsType::TsBooleanType(_) | AnyTsType::TsBooleanLiteralType(_) => {}
            AnyTsType::TsParenthesizedType(ty) => match ty.ty() {
                Ok(ty) => types.push(ty),
                Err(_) => return false,
            },
            AnyTsType::TsUnionType(union) => {
                for ty in union.types() {
                    match ty {
                        Ok(ty) => types.push(ty),
                        Err(_) => return false,
                    }
                }
            }
            _ => return false,
        }
    }
    true
}

/// Returns whether `declarator` has the form `const name = true` or `const name = false`.
/// A destructuring pattern such as `const { name } = true` doesn't match.
fn is_const_with_boolean_literal(declarator: &JsVariableDeclarator) -> bool {
    matches!(
        declarator.id(),
        Ok(AnyJsBindingPattern::AnyJsBinding(
            AnyJsBinding::JsIdentifierBinding(_)
        ))
    ) && declarator
        .declaration()
        .is_some_and(|declaration| declaration.is_const())
        && declarator
            .initializer()
            .and_then(|initializer| initializer.expression().ok())
            .is_some_and(|expression| {
                matches!(
                    expression.omit_parentheses(),
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
                    )
                )
            })
}

/// Returns whether `expression` is a variable, `this`, or `super`, or a chain of properties
/// on one of them whose names are known without running the code, such as `a.b["c"]` or
/// `this.#d`.
fn is_static_reference(expression: &AnyJsExpression) -> bool {
    let mut expression = expression.clone();
    loop {
        let Some(current) = expression.omit_type_wrappers() else {
            return false;
        };
        let object = match current {
            AnyJsExpression::JsIdentifierExpression(_)
            | AnyJsExpression::JsThisExpression(_)
            | AnyJsExpression::JsSuperExpression(_) => return true,
            AnyJsExpression::JsStaticMemberExpression(member) => {
                if member.is_optional()
                    || !matches!(
                        member.member(),
                        Ok(AnyJsName::JsName(_) | AnyJsName::JsPrivateName(_))
                    )
                {
                    return false;
                }
                member.object()
            }
            AnyJsExpression::JsComputedMemberExpression(member) => {
                let is_static_key = member
                    .member()
                    .ok()
                    .and_then(AnyJsExpression::omit_type_wrappers)
                    .is_some_and(|key| match key {
                        AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(_)
                            | AnyJsLiteralExpression::JsNumberLiteralExpression(_),
                        ) => true,
                        AnyJsExpression::JsTemplateExpression(template) => {
                            template.tag().is_none()
                                && template.elements().into_iter().all(|element| {
                                    matches!(
                                        element,
                                        AnyJsTemplateElement::JsTemplateChunkElement(_)
                                    )
                                })
                        }
                        _ => false,
                    });
                if member.is_optional() || !is_static_key {
                    return false;
                }
                member.object()
            }
            _ => return false,
        };
        match object {
            Ok(object) => expression = object,
            Err(_) => return false,
        }
    }
}

/// Returns `reference` followed by the objects it reads properties from: `a.b.c` gives
/// `a.b.c`, `a.b`, and `a`. Assigning to any of them changes the value of `reference`.
fn reference_prefixes(reference: AnyJsExpression) -> Option<Vec<AnyJsExpression>> {
    let mut prefixes = Vec::new();
    let mut reference = reference.omit_type_wrappers()?;
    loop {
        let object = match &reference {
            AnyJsExpression::JsStaticMemberExpression(member) => member.object().ok()?,
            AnyJsExpression::JsComputedMemberExpression(member) => member.object().ok()?,
            _ => {
                prefixes.push(reference);
                return Some(prefixes);
            }
        };
        prefixes.push(reference);
        reference = object.omit_type_wrappers()?;
    }
}

/// Returns whether `statement` may assign to, delete, or declare with `var` one of
/// `references`. Code inside nested functions is ignored, since it doesn't run as part of
/// `statement`.
fn changes_reference(statement: &AnyJsStatement, references: &[AnyJsExpression]) -> bool {
    let root_name = references
        .last()
        .and_then(|root| root.as_js_identifier_expression()?.name().ok())
        .and_then(|name| name.value_token().ok());

    let mut preorder = statement.syntax().preorder_of_kinds(MUTATION_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if FUNCTION_KINDS.matches(node.kind()) {
            preorder.skip_subtree();
        } else if let Some(target) = AnyJsAssignment::cast_ref(&node) {
            if references.iter().any(|reference| {
                is_same_assignment_target(target.clone(), reference.clone()) != Some(false)
            }) {
                return true;
            }
        } else if let Some(unary) = JsUnaryExpression::cast_ref(&node) {
            if unary.operator().ok() == Some(JsUnaryOperator::Delete)
                && let Ok(argument) = unary.argument()
                && references.iter().any(|reference| {
                    is_same_reference(argument.clone(), reference.clone()) != Some(false)
                })
            {
                return true;
            }
        } else if let Some(declarator) = JsVariableDeclarator::cast_ref(&node)
            && let Some(root_name) = &root_name
            && declarator
                .declaration()
                .is_some_and(|declaration| declaration.is_var())
            && let Ok(id) = declarator.id()
            && id
                .syntax()
                .descendants()
                .filter_map(JsIdentifierBinding::cast)
                .any(|binding| {
                    binding
                        .name_token()
                        .is_ok_and(|name| name.text_trimmed() == root_name.text_trimmed())
                })
        {
            return true;
        }
    }
    false
}

/// Returns the `if` statements of the chain that starts with `if_statement`, following its
/// `else if` branches.
///
/// Returns `None` if the chain ends with a plain `else`, since that branch runs whenever no
/// other branch matched, so no `if` statement can be joined after it.
fn if_chain(if_statement: JsIfStatement) -> Option<Vec<JsIfStatement>> {
    let mut chain = vec![if_statement];
    while let Some(else_clause) = chain.last()?.else_clause() {
        match else_clause.alternate().ok()? {
            AnyJsStatement::JsIfStatement(alternate) => chain.push(alternate),
            _ => return None,
        }
    }
    Some(chain)
}

/// Returns whether `statement` ends with an `if` statement that has no `else`, such as
/// `while (a) if (b) c();`. An `else` written after `statement` would belong to that `if`.
fn ends_with_dangling_if(statement: AnyJsStatement) -> bool {
    let mut statement = statement;
    loop {
        let body = match statement {
            AnyJsStatement::JsIfStatement(if_statement) => match if_statement.else_clause() {
                Some(else_clause) => else_clause.alternate(),
                None => return true,
            },
            AnyJsStatement::JsForStatement(for_statement) => for_statement.body(),
            AnyJsStatement::JsForInStatement(for_statement) => for_statement.body(),
            AnyJsStatement::JsForOfStatement(for_statement) => for_statement.body(),
            AnyJsStatement::JsWhileStatement(while_statement) => while_statement.body(),
            AnyJsStatement::JsLabeledStatement(labeled) => labeled.body(),
            AnyJsStatement::JsWithStatement(with_statement) => with_statement.body(),
            _ => return false,
        };
        match body {
            Ok(body) => statement = body,
            // Don't offer a fix for an incomplete syntax tree.
            Err(_) => return true,
        }
    }
}

/// The lists of statements in which an `if` statement can follow another one.
const STATEMENT_LIST_KINDS: SyntaxKindSet<JsLanguage> =
    JsStatementList::KIND_SET.union(JsModuleItemList::KIND_SET);

/// The nodes whose code doesn't run when the surrounding code runs.
const FUNCTION_KINDS: SyntaxKindSet<JsLanguage> = AnyFunctionLike::KIND_SET
    .union(JsGetterClassMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterClassMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);

/// The nodes that [`changes_reference`] stops at: the nodes that can change a variable or
/// property, and the nodes in [`FUNCTION_KINDS`].
const MUTATION_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = AnyJsAssignment::KIND_SET
    .union(JsUnaryExpression::KIND_SET)
    .union(JsVariableDeclarator::KIND_SET)
    .union(FUNCTION_KINDS);
