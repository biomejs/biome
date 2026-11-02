use crate::services::semantic::Semantic;
use biome_js_semantic::{Binding, SemanticModel};
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsMemberExpression, JsArrowFunctionExpression,
    JsAssignmentExpression, JsClassMemberList, JsConstructorClassMember,
    JsConstructorParameterList, JsConstructorParameters, JsDoWhileStatement, JsForInStatement,
    JsForOfStatement, JsForStatement, JsFunctionBody, JsFunctionDeclaration, JsGetterClassMember,
    JsInitializerClause, JsParenthesizedExpression, TsAsExpression, TsNonNullAssertionExpression,
    TsSatisfiesExpression, TsTypeAssertionExpression,
    JsGetterObjectMember, JsLanguage, JsMethodClassMember, JsMethodObjectMember,
    JsPropertyClassMember, JsSetterClassMember, JsSetterObjectMember,
    JsStaticInitializationBlockClassMember, JsSyntaxKind, JsUnaryExpression, JsUnaryOperator,
    JsWhileStatement, T, TsPropertyParameter, assign_ext::AnyJsMemberAssignment,
    static_value::StaticValue,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{
    AstNode, AstNodeList, SyntaxKindSet, SyntaxResult, TextSize, WalkEvent, declare_node_union,
};
use biome_rule_options::no_useless_parameter_property_assignment::NoUselessParameterPropertyAssignmentOptions;

declare_lint_rule! {
    /// Disallow assigning a parameter property to itself in the constructor.
    ///
    /// A parameter property is a constructor parameter marked with `public`, `protected`,
    /// `private`, `readonly`, or `override`. TypeScript creates a class property with the
    /// same name and sets it to the parameter's value before the constructor body runs.
    ///
    /// Writing `this.name = name` in the constructor body sets the property to the value
    /// it already has. The line does nothing and can be removed.
    ///
    /// The rule doesn't report the assignment if the parameter or the property is
    /// changed before it, because then the assignment can change the value of the property.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// class User {
    ///     constructor(public name: string) {
    ///         this.name = name;
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class User {
    ///     constructor(private readonly id?: string) {
    ///         this.id ??= id;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// class User {
    ///     constructor(public name: string) {}
    /// }
    ///
    /// class Label {
    ///     constructor(public text: string) {
    ///         text = text.trim();
    ///         this.text = text;
    ///     }
    /// }
    ///
    /// class Point {
    ///     x: number;
    ///     constructor(x: number) {
    ///         this.x = x;
    ///     }
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow parameter properties entirely, see [`noParameterProperties`](https://biomejs.dev/linter/rules/no-parameter-properties/).
    ///
    pub NoUselessParameterPropertyAssignment {
        version: "next",
        name: "noUselessParameterPropertyAssignment",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-unnecessary-parameter-property-assignment").same()],
        recommended: false,
    }
}

impl Rule for NoUselessParameterPropertyAssignment {
    type Query = Semantic<TsPropertyParameter>;
    /// An assignment in the constructor that copies the parameter into the
    /// property it already initializes.
    type State = JsAssignmentExpression;
    type Signals = Box<[Self::State]>;
    type Options = NoUselessParameterPropertyAssignmentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        find_useless_copies(ctx.query(), ctx.model()).unwrap_or_default()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, assignment: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                assignment.range(),
                markup! {
                    "This assignment sets the property to the value it already has."
                },
            )
            .note(markup! {
                "The parameter property already sets this property to the parameter's value before the constructor body runs."
            })
            .note(markup! {
                "Remove the assignment."
            }),
        )
    }
}

/// Returns the assignments in the constructor body that copy the parameter of
/// `parameter` into the property it already initializes, when nothing can have
/// changed the parameter or the property before the copy runs.
fn find_useless_copies(
    parameter: &TsPropertyParameter,
    model: &SemanticModel,
) -> Option<Box<[JsAssignmentExpression]>> {
    let identifier = parameter
        .formal_parameter()
        .ok()?
        .as_js_formal_parameter()?
        .binding()
        .ok()?
        .as_any_js_binding()?
        .as_js_identifier_binding()?
        .clone();
    // Constructor overloads and constructors of `declare class` have no body,
    // so this fails for them.
    let constructor = parameter
        .parent::<JsConstructorParameterList>()?
        .parent::<JsConstructorParameters>()?
        .parent::<JsConstructorClassMember>()?;
    let body = AnyConstructorCode::from(constructor.body().ok()?);
    let name_token = identifier.name_token().ok()?;
    let name = name_token.text_trimmed();
    let binding = model.as_binding(&identifier);

    let mut copies = binding
        .all_reads()
        .filter_map(|reference| {
            let reference =
                AnyJsExpression::cast(reference.syntax_near(constructor.syntax()).parent()?)?;
            parameter_copy(&reference, name, &constructor)
        })
        .peekable();
    copies.peek()?;

    // Instance property initializers run before the constructor body.
    let changed_by_initializer = constructor
        .parent::<JsClassMemberList>()?
        .iter()
        .filter_map(|member| {
            let property = member.as_js_property_class_member()?;
            let is_static = property
                .modifiers()
                .iter()
                .any(|modifier| modifier.as_js_static_modifier().is_some());
            if is_static { None } else { property.value() }
        })
        .any(|initializer| has_this_member_change(&initializer.into(), name, |_| true, |_| false));
    if changed_by_initializer {
        return None;
    }

    // Function declarations are hoisted, so they can be called before a copy
    // wherever they appear.
    let changed_by_function_declaration = binding.all_writes().any(|reference| {
        reference
            .syntax_near(constructor.syntax())
            .ancestors()
            .take_while(|node| node != constructor.syntax())
            .any(|node| JsFunctionDeclaration::can_cast(node.kind()))
    });
    if changed_by_function_declaration {
        return None;
    }

    let useless_copies = copies
        .filter(|copy| {
            let start = copy.syntax().text_trimmed_range().start();
            // Inside a loop, the code after the copy also runs before the
            // copy's next run.
            let loop_range = copy
                .syntax()
                .ancestors()
                .take_while(|node| node != constructor.syntax())
                .filter(|node| LOOP_KINDS.matches(node.kind()))
                .last()
                .map(|node| node.text_trimmed_range());
            let runs_before = |position: TextSize| {
                position < start
                    || (position > start && loop_range.is_some_and(|range| range.contains(position)))
            };
            !binding
                .all_writes()
                .any(|reference| runs_before(reference.range_start()))
                // A copy doesn't change the property, so it doesn't make other
                // copies useful.
                && !has_this_member_change(&body, name, runs_before, |assignment| {
                    copies_binding(assignment, model, &binding)
                })
        })
        .collect();
    Some(useless_copies)
}

/// Returns the assignment that copies `value`, which reads the parameter `name`,
/// into `this.name`, as long as the assignment is in the body of `constructor`
/// and runs when `constructor` runs.
///
/// Type assertions and parentheses around the parameter are ignored, so
/// `this.name = name!` and `this.name = name as string` are copies too.
fn parameter_copy(
    value: &AnyJsExpression,
    name: &str,
    constructor: &JsConstructorClassMember,
) -> Option<JsAssignmentExpression> {
    let assignment = value
        .syntax()
        .ancestors()
        .skip(1)
        .find(|node| !TYPE_WRAPPER_KINDS.matches(node.kind()))
        .and_then(JsAssignmentExpression::cast)?;
    if !COPY_OPERATORS.contains(assignment.operator_token().ok()?.kind()) {
        return None;
    }
    let target = AnyJsMemberAssignment::cast(assignment.left().ok()?.into_syntax())?;
    if !is_this_member(target.object(), target.member_name(), name) {
        return None;
    }

    let scope = assignment
        .syntax()
        .ancestors()
        .skip(1)
        .find(|node| {
            // The body of an `async` arrow function can pause at `await` and
            // finish after the constructor returns.
            THIS_SCOPE_KINDS.matches(node.kind())
                && !JsArrowFunctionExpression::cast_ref(node)
                    .is_some_and(|arrow| arrow.async_token().is_none() && is_arrow_iife(arrow))
        })?;
    let in_body = constructor
        .body()
        .ok()?
        .syntax()
        .text_trimmed_range()
        .contains_range(assignment.syntax().text_trimmed_range());
    (scope == *constructor.syntax() && in_body).then_some(assignment)
}

/// Returns `true` if code inside `root` assigns or deletes `this.name` at a
/// position for which `runs_before` returns `true`.
///
/// Assignments for which `is_copy` returns `true` don't count, because they
/// assign the value the property already has.
///
/// Code inside nested functions and class members is skipped, because it may
/// run later or refer to a different `this`. Synchronous arrow functions that are
/// called right away, such as `(() => { this.name = 1; })()`, are searched.
fn has_this_member_change(
    root: &AnyConstructorCode,
    name: &str,
    runs_before: impl Fn(TextSize) -> bool,
    is_copy: impl Fn(&JsAssignmentExpression) -> bool,
) -> bool {
    let mut preorder = root
        .syntax()
        .preorder_of_kinds(THIS_MEMBER_WRITE_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        let is_change = if let Some(target) = AnyJsMemberAssignment::cast_ref(&node) {
            is_this_member(target.object(), target.member_name(), name)
                && runs_before(node.text_trimmed_range().start())
                && !target
                    .parent::<JsAssignmentExpression>()
                    .is_some_and(|assignment| is_copy(&assignment))
        } else if let Some(unary) = JsUnaryExpression::cast_ref(&node) {
            unary.operator() == Ok(JsUnaryOperator::Delete)
                && unary
                    .argument()
                    .ok()
                    .and_then(|argument| {
                        AnyJsMemberExpression::cast(argument.omit_parentheses().into_syntax())
                    })
                    .is_some_and(|member| {
                        is_this_member(member.object(), member.member_name(), name)
                    })
                && runs_before(node.text_trimmed_range().start())
        } else {
            if !JsArrowFunctionExpression::cast_ref(&node).is_some_and(is_arrow_iife) {
                preorder.skip_subtree();
            }
            false
        };
        if is_change {
            return true;
        }
    }
    false
}

/// Returns `true` if `assignment` sets a value to the current value of the
/// variable `binding`, with an operator from [`COPY_OPERATORS`].
fn copies_binding(
    assignment: &JsAssignmentExpression,
    model: &SemanticModel,
    binding: &Binding,
) -> bool {
    assignment
        .operator_token()
        .is_ok_and(|operator| COPY_OPERATORS.contains(operator.kind()))
        && assignment
            .right()
            .ok()
            .and_then(AnyJsExpression::omit_type_wrappers)
            .and_then(|value| value.as_js_identifier_expression()?.name().ok())
            .and_then(|reference| model.binding(&reference))
            .is_some_and(|value_binding| value_binding.range() == binding.range())
}

/// Returns `true` if `object` is `this` and `member` is `name`, as in
/// `this.name`, `this["name"]`, and ``this[`name`]``.
fn is_this_member(
    object: SyntaxResult<AnyJsExpression>,
    member: Option<StaticValue>,
    name: &str,
) -> bool {
    object.is_ok_and(|object| object.omit_parentheses().as_js_this_expression().is_some())
        && member.is_some_and(|member| member.text() == name)
}

/// Returns `true` for an arrow function that is called right where it's defined,
/// such as `(() => {})()`.
fn is_arrow_iife(arrow: JsArrowFunctionExpression) -> bool {
    AnyJsFunction::from(arrow).is_iife()
}

declare_node_union! {
    /// Code that runs while a class instance is created: the constructor body,
    /// or the initial value of an instance property.
    pub AnyConstructorCode = JsFunctionBody | JsInitializerClause
}

/// Assignment operators that leave the property equal to the parameter when the
/// property already holds the parameter's value.
const COPY_OPERATORS: TokenSet<JsSyntaxKind> = token_set![T![=], T![||=], T![&&=], T![??=]];

/// Nodes that start a new value of `this`, or that run at a different time than
/// the code around them.
const THIS_SCOPE_KINDS: SyntaxKindSet<JsLanguage> = AnyJsFunction::KIND_SET
    .union(JsConstructorClassMember::KIND_SET)
    .union(JsMethodClassMember::KIND_SET)
    .union(JsGetterClassMember::KIND_SET)
    .union(JsSetterClassMember::KIND_SET)
    .union(JsPropertyClassMember::KIND_SET)
    .union(JsStaticInitializationBlockClassMember::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);

/// The nodes the search for `this.name` writes stops at: member assignments,
/// unary expressions that may be `delete`, and the nodes whose code doesn't run
/// as part of the searched code.
const THIS_MEMBER_WRITE_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = AnyJsMemberAssignment::KIND_SET
    .union(JsUnaryExpression::KIND_SET)
    .union(THIS_SCOPE_KINDS);

/// Nodes that wrap an expression without changing its value.
const TYPE_WRAPPER_KINDS: SyntaxKindSet<JsLanguage> = JsParenthesizedExpression::KIND_SET
    .union(TsAsExpression::KIND_SET)
    .union(TsSatisfiesExpression::KIND_SET)
    .union(TsNonNullAssertionExpression::KIND_SET)
    .union(TsTypeAssertionExpression::KIND_SET);

const LOOP_KINDS: SyntaxKindSet<JsLanguage> = JsForStatement::KIND_SET
    .union(JsForInStatement::KIND_SET)
    .union(JsForOfStatement::KIND_SET)
    .union(JsWhileStatement::KIND_SET)
    .union(JsDoWhileStatement::KIND_SET);
