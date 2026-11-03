use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::{ReferencesExtensions, SemanticModel};
use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsArrayElement, AnyJsArrowFunctionParameters,
    AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression,
    AnyJsForInOrOfInitializer, AnyJsFunctionBody, AnyJsObjectMember, AnyJsObjectMemberName,
    AnyJsParameter, AnyJsStatement, AnyTsType, JsArrowFunctionExpression, JsAssignmentOperator,
    JsCallExpression, JsForOfStatement, JsIdentifierBinding, JsSyntaxKind, JsSyntaxToken,
    JsVariableStatement, T, TriviaPieceKind, TsTypeAnnotation,
    binding_ext::AnyJsBindingDeclaration, global_identifier,
};
use biome_rowan::{
    AstNode, AstNodeExt, AstNodeList, AstSeparatedList, BatchMutationExt, declare_node_union,
};
use biome_rule_options::use_object_from_entries::UseObjectFromEntriesOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Enforce using `Object.fromEntries()` to build an object from a list of key-value pairs.
    ///
    /// `Object.fromEntries()` turns a list of `[key, value]` pairs into an object in a single call.
    /// Building the same object with `Array#reduce()`, a `for...of` loop, or a helper function
    /// such as Lodash's `_.fromPairs()` takes more code and hides what the code is doing.
    /// Spreading the previous object on each step of `reduce()` also copies the whole object
    /// every time, which gets slow for long lists.
    ///
    /// This rule reports:
    ///
    /// - `reduce()` calls that start from `{}` and add one property on each step,
    ///   either with object spread or with `Object.assign()`;
    /// - `for...of` loops that only copy each pair of a list into an object declared
    ///   as `{}` right before the loop;
    /// - calls to `_.fromPairs()`, `lodash.fromPairs()`, and the functions listed in the
    ///   [`functions`](#functions) option.
    ///
    /// `reduce()` calls on typed arrays, such as `new Uint8Array(values)`, aren't reported,
    /// because a typed array can't hold key-value pairs.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const object = pairs.reduce((object, [key, value]) => ({ ...object, [key]: value }), {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const object = pairs.reduce((object, [key, value]) => Object.assign(object, { [key]: value }), {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const object = {};
    /// for (const [key, value] of pairs) {
    ///     object[key] = value;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const object = _.fromPairs(pairs);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const object = Object.fromEntries(pairs);
    ///
    /// // `Object.fromEntries()` always creates an object that inherits from `Object.prototype`.
    /// const dictionary = pairs.reduce((object, [key, value]) => ({ ...object, [key]: value }), Object.create(null));
    /// ```
    ///
    /// ## Options
    ///
    /// ### `functions`
    ///
    /// Type: `string[]`
    ///
    /// Default: `[]`
    ///
    /// Other functions that build an object from a list of key-value pairs.
    /// A name can contain dots, such as `utils.fromPairs`, to match a function that is accessed through an object.
    /// `_.fromPairs()` and `lodash.fromPairs()` are always reported.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "functions": ["utils.fromPairs"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// const object = utils.fromPairs(pairs);
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// const object = Object.fromEntries(pairs);
    /// ```
    ///
    pub UseObjectFromEntries {
        version: "next",
        name: "useObjectFromEntries",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-object-from-entries").same()],
        recommended: true,
        severity: Severity::Warning,
        // Unsafe because the receiver of `reduce()` or the list in a `for...of` loop may not be an
        // array, and helper functions may accept inputs that `Object.fromEntries()` rejects.
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyUseObjectFromEntriesQuery = JsCallExpression | JsForOfStatement
}

impl Rule for UseObjectFromEntries {
    type Query = Semantic<AnyUseObjectFromEntriesQuery>;
    type State = UseObjectFromEntriesState;
    type Signals = Option<Self::State>;
    type Options = UseObjectFromEntriesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        match ctx.query() {
            AnyUseObjectFromEntriesQuery::JsCallExpression(call) => {
                if let Some(entry) = reduce_entry(call, model) {
                    return Some(UseObjectFromEntriesState::Reduce(entry));
                }
                pairs_function_index(call, ctx.options()).map(UseObjectFromEntriesState::Function)
            }
            AnyUseObjectFromEntriesQuery::JsForOfStatement(for_of) => {
                loop_object_statement(for_of, model).map(UseObjectFromEntriesState::Loop)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match (ctx.query(), state) {
            (
                AnyUseObjectFromEntriesQuery::JsCallExpression(call),
                UseObjectFromEntriesState::Reduce(_),
            ) => {
                let member = call.callee().ok()?;
                let member = member.as_js_static_member_expression()?.member().ok()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    member.range(),
                    markup! {
                        "This "<Emphasis>"reduce()"</Emphasis>" call builds an object one property at a time."
                    },
                )
                .note(markup! {
                    <Emphasis>"Object.fromEntries()"</Emphasis>" builds the same object from a list of "<Emphasis>"[key, value]"</Emphasis>" pairs in a single step, which is shorter and easier to read."
                })
                .note(markup! {
                    "Turn each item into a "<Emphasis>"[key, value]"</Emphasis>" pair with "<Emphasis>"map()"</Emphasis>", then pass the result to "<Emphasis>"Object.fromEntries()"</Emphasis>"."
                })
            }
            (
                AnyUseObjectFromEntriesQuery::JsCallExpression(call),
                UseObjectFromEntriesState::Function(index),
            ) => {
                let name = checked_function_names(ctx.options()).nth(*index)?;
                RuleDiagnostic::new(
                    rule_category!(),
                    call.callee().ok()?.range(),
                    markup! {
                        "This call to "<Emphasis>{name}"()"</Emphasis>" builds an object from a list of key-value pairs."
                    },
                )
                .note(markup! {
                    <Emphasis>"Object.fromEntries()"</Emphasis>" does the same thing and is built into JavaScript, so no helper function is needed."
                })
                .note(markup! {
                    "Call "<Emphasis>"Object.fromEntries()"</Emphasis>" instead."
                })
            }
            (
                AnyUseObjectFromEntriesQuery::JsForOfStatement(for_of),
                UseObjectFromEntriesState::Loop(_),
            ) => RuleDiagnostic::new(
                rule_category!(),
                for_of.range(),
                markup! {
                    "This "<Emphasis>"for...of"</Emphasis>" loop builds an object one property at a time."
                },
            )
            .note(markup! {
                <Emphasis>"Object.fromEntries()"</Emphasis>" builds the same object from a list of "<Emphasis>"[key, value]"</Emphasis>" pairs in a single step, which is shorter and easier to read."
            })
            .note(markup! {
                "Pass the list of pairs to "<Emphasis>"Object.fromEntries()"</Emphasis>" instead of looping over it."
            }),
            _ => return None,
        };
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        // Every fix writes `Object.fromEntries`, which would call a local `Object` variable instead.
        if ctx
            .model()
            .scope(ctx.query().syntax())
            .ancestors()
            .any(|scope| scope.get_binding("Object").is_some())
        {
            return None;
        }
        let mut mutation = ctx.root().begin();
        match (ctx.query(), state) {
            (
                AnyUseObjectFromEntriesQuery::JsCallExpression(call),
                UseObjectFromEntriesState::Reduce(entry),
            ) => {
                let replacement = reduce_replacement(call, entry, ctx)?;
                mutation.replace_node(AnyJsExpression::from(call.clone()), replacement);
            }
            (
                AnyUseObjectFromEntriesQuery::JsCallExpression(call),
                UseObjectFromEntriesState::Function(_),
            ) => {
                let callee = call.callee().ok()?;
                if callee.syntax().has_inner_comments() {
                    return None;
                }
                mutation.replace_node(callee, object_from_entries_callee());
            }
            (
                AnyUseObjectFromEntriesQuery::JsForOfStatement(for_of),
                UseObjectFromEntriesState::Loop(statement),
            ) => {
                if statement.syntax().has_inner_comments()
                    || for_of.syntax().has_comments_descendants()
                {
                    return None;
                }
                let declarator = statement.declaration().ok()?.declarators().first()?.ok()?;
                let object = declarator.initializer()?.expression().ok()?;
                let pairs = for_of.expression().ok()?.trim_trivia()?;
                let declaration = statement
                    .declaration()
                    .ok()?
                    .replace_node(object, object_from_entries_call(pairs).into())?;
                // The loop no longer separates the declaration from the next statement, so a
                // missing semicolon could make the next line continue the declaration.
                let semicolon = statement
                    .semicolon_token()
                    .unwrap_or_else(|| make::token(T![;]));
                let replacement = statement
                    .clone()
                    .with_declaration(declaration)
                    .with_semicolon_token(Some(semicolon));
                mutation.replace_node(statement.clone(), replacement);
                mutation.remove_node(for_of.clone());
            }
            _ => return None,
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"Object.fromEntries()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

pub enum UseObjectFromEntriesState {
    /// A `reduce()` call that adds one property per step. Holds the object member that adds it.
    Reduce(AnyJsObjectMember),
    /// A `for...of` loop that copies pairs into an object. Holds the statement that declares the object.
    Loop(JsVariableStatement),
    /// A call to a function that builds an object from pairs.
    /// Holds the index of the function's name in [`checked_function_names`].
    Function(usize),
}

/// Helper functions that are always reported, in addition to the configured ones.
const BUILT_IN_PAIRS_FUNCTIONS: &[&str] = &["_.fromPairs", "lodash.fromPairs"];

/// Typed array constructors. A typed array holds numbers, so it can't hold key-value pairs.
///
/// Sorted for binary search.
const TYPED_ARRAY_CONSTRUCTORS: &[&str] = &[
    "BigInt64Array",
    "BigUint64Array",
    "Float16Array",
    "Float32Array",
    "Float64Array",
    "Int16Array",
    "Int32Array",
    "Int8Array",
    "Uint16Array",
    "Uint32Array",
    "Uint8Array",
    "Uint8ClampedArray",
];

/// How many variables [`is_typed_array`] follows to find where a value comes from.
const MAX_ALIAS_DEPTH: usize = 4;

/// Returns the names of the functions that the rule reports: the configured ones, then the built-in ones.
fn checked_function_names(options: &UseObjectFromEntriesOptions) -> impl Iterator<Item = &str> {
    options
        .functions()
        .iter()
        .map(|name| name.trim())
        .chain(BUILT_IN_PAIRS_FUNCTIONS.iter().copied())
}

/// Returns the index in [`checked_function_names`] of the function that `call` calls with a single argument.
fn pairs_function_index(
    call: &JsCallExpression,
    options: &UseObjectFromEntriesOptions,
) -> Option<usize> {
    if call.is_optional() {
        return None;
    }
    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 1 || arguments.first()?.ok()?.as_any_js_expression().is_none() {
        return None;
    }
    let callee = call.callee().ok()?;
    checked_function_names(options).position(|name| matches_path(&callee, name))
}

/// Returns `true` if `expression` is the identifier or the chain of static member accesses
/// spelled by `path`, such as `utils.fromPairs`.
///
/// Optional member accesses, like `utils?.fromPairs`, never match.
fn matches_path(expression: &AnyJsExpression, path: &str) -> bool {
    let mut segments = path.rsplit('.');
    let mut expression = expression.clone();
    while let Some(segment) = segments.next() {
        match expression.omit_parentheses() {
            AnyJsExpression::JsStaticMemberExpression(member) => {
                if member.is_optional()
                    || !member
                        .member()
                        .ok()
                        .and_then(|name| name.as_js_name()?.value_token().ok())
                        .is_some_and(|name| name.text_trimmed() == segment)
                {
                    return false;
                }
                let Ok(object) = member.object() else {
                    return false;
                };
                expression = object;
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                return segments.next().is_none()
                    && identifier.name().is_ok_and(|name| name.has_name(segment));
            }
            _ => return false,
        }
    }
    false
}

/// Returns the object member added on each step if `call` is a `reduce()` call that builds an
/// object from pairs, like `pairs.reduce((object, [key, value]) => ({...object, [key]: value}), {})`.
fn reduce_entry(call: &JsCallExpression, model: &SemanticModel) -> Option<AnyJsObjectMember> {
    if call.is_optional() {
        return None;
    }
    let member = call.callee().ok()?;
    let member = member.as_js_static_member_expression()?;
    if member.is_optional()
        || member
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .text_trimmed()
            != "reduce"
    {
        return None;
    }

    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 2 {
        return None;
    }
    let mut arguments = arguments.iter();
    let callback = arguments.next()?.ok()?;
    let initial_value = arguments.next()?.ok()?;
    if !is_empty_object(initial_value.as_any_js_expression()?) {
        return None;
    }
    let callback = callback.as_any_js_expression()?.clone().omit_parentheses();
    let callback = callback.as_js_arrow_function_expression()?;
    if callback.async_token().is_some() {
        return None;
    }

    let accumulator = accumulator_parameter(callback)?;
    let accumulator_name = accumulator.name_token().ok()?;
    let accumulator_name = accumulator_name.text_trimmed();
    let AnyJsFunctionBody::AnyJsExpression(body) = callback.body().ok()? else {
        return None;
    };
    let entry = match body.omit_parentheses() {
        // `({...object, key: value})`
        AnyJsExpression::JsObjectExpression(object) => {
            let members = object.members();
            if members.len() != 2 {
                return None;
            }
            let mut members = members.iter();
            let AnyJsObjectMember::JsSpread(spread) = members.next()?.ok()? else {
                return None;
            };
            if !is_reference_named(&spread.argument().ok()?, accumulator_name) {
                return None;
            }
            members.next()?.ok()?
        }
        // `Object.assign(object, {key: value})`
        AnyJsExpression::JsCallExpression(assign) => {
            if !is_global_object_assign(&assign, model) {
                return None;
            }
            let arguments = assign.arguments().ok()?.args();
            if arguments.len() != 2 {
                return None;
            }
            let mut arguments = arguments.iter();
            let target = arguments.next()?.ok()?;
            if !is_reference_named(target.as_any_js_expression()?, accumulator_name) {
                return None;
            }
            let source = arguments.next()?.ok()?;
            let source = source.as_any_js_expression()?.clone().omit_parentheses();
            let members = source.as_js_object_expression()?.members();
            if members.len() != 1 {
                return None;
            }
            members.first()?.ok()?
        }
        _ => return None,
    };

    if !is_plain_entry(&entry) {
        return None;
    }
    // The accumulator must only be used as the spread or `Object.assign()` target, because
    // `Object.fromEntries()` doesn't expose the object while it is being built.
    if accumulator.all_references(model).count() != 1 {
        return None;
    }
    if is_typed_array(&member.object().ok()?, model, MAX_ALIAS_DEPTH) {
        return None;
    }
    Some(entry)
}

/// Returns the first parameter of `callback` if it is a plain identifier, like `object` in `(object, pair) => …`.
fn accumulator_parameter(callback: &JsArrowFunctionExpression) -> Option<JsIdentifierBinding> {
    match callback.parameters().ok()? {
        AnyJsArrowFunctionParameters::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) => {
            Some(binding)
        }
        AnyJsArrowFunctionParameters::JsParameters(parameters) => {
            let AnyJsParameter::AnyJsFormalParameter(parameter) =
                parameters.items().first()?.ok()?
            else {
                return None;
            };
            let parameter = parameter.as_js_formal_parameter()?;
            if parameter.initializer().is_some() {
                return None;
            }
            match parameter.binding().ok()? {
                AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) => {
                    Some(binding)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Returns `true` if `member` is a property that `Object.fromEntries()` can create, like `key`,
/// `key: value`, or `[key]: value`.
///
/// A literal `__proto__: value` member sets the prototype of the object instead of creating a
/// property, so it doesn't count.
fn is_plain_entry(member: &AnyJsObjectMember) -> bool {
    match member {
        AnyJsObjectMember::JsShorthandPropertyObjectMember(_) => true,
        AnyJsObjectMember::JsPropertyObjectMember(property) => match property.name() {
            Ok(AnyJsObjectMemberName::JsLiteralMemberName(name)) => {
                name.name().is_ok_and(|name| name.text() != "__proto__")
            }
            Ok(AnyJsObjectMemberName::JsComputedMemberName(_)) => true,
            _ => false,
        },
        _ => false,
    }
}

/// Returns `true` if `call` is a non-optional call to the global `Object.assign()`.
fn is_global_object_assign(call: &JsCallExpression, model: &SemanticModel) -> bool {
    if call.is_optional() {
        return false;
    }
    let Some(callee) = call.callee().ok().map(AnyJsExpression::omit_parentheses) else {
        return false;
    };
    let Some(member) = callee.as_js_static_member_expression() else {
        return false;
    };
    if member.is_optional()
        || !member
            .member()
            .ok()
            .and_then(|name| name.as_js_name()?.value_token().ok())
            .is_some_and(|name| name.text_trimmed() == "assign")
    {
        return false;
    }
    member
        .object()
        .ok()
        .map(AnyJsExpression::omit_parentheses)
        .and_then(|object| global_identifier(&object.as_any_global_identifier_expression()?))
        .is_some_and(|(reference, name)| {
            name.text() == "Object" && model.binding(&reference).is_none()
        })
}

/// Returns `true` if `expression` is known to be a typed array, like `new Uint8Array(values)`,
/// `Uint8Array.from(values)`, or a variable that holds one.
///
/// `depth` limits how many variables are followed.
fn is_typed_array(expression: &AnyJsExpression, model: &SemanticModel, depth: usize) -> bool {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::JsNewExpression(new) => new
            .callee()
            .ok()
            .and_then(|callee| callee.omit_parentheses().as_js_reference_identifier())
            .is_some_and(|reference| is_typed_array_name(reference.name().ok().as_deref())),
        // `Uint8Array.from(values)` and `Uint8Array.of(...values)`
        AnyJsExpression::JsCallExpression(call) => call
            .callee()
            .ok()
            .map(AnyJsExpression::omit_parentheses)
            .and_then(|callee| {
                let member = callee.as_js_static_member_expression()?;
                let method = member.member().ok()?.as_js_name()?.value_token().ok()?;
                if !matches!(method.text_trimmed(), "from" | "of") {
                    return None;
                }
                member
                    .object()
                    .ok()?
                    .omit_parentheses()
                    .as_js_reference_identifier()
            })
            .is_some_and(|reference| is_typed_array_name(reference.name().ok().as_deref())),
        AnyJsExpression::TsAsExpression(expression) => {
            expression.ty().is_ok_and(|ty| is_typed_array_type(&ty))
                || expression
                    .expression()
                    .is_ok_and(|inner| is_typed_array(&inner, model, depth))
        }
        AnyJsExpression::TsSatisfiesExpression(expression) => {
            expression.ty().is_ok_and(|ty| is_typed_array_type(&ty))
                || expression
                    .expression()
                    .is_ok_and(|inner| is_typed_array(&inner, model, depth))
        }
        AnyJsExpression::TsTypeAssertionExpression(expression) => {
            expression.ty().is_ok_and(|ty| is_typed_array_type(&ty))
                || expression
                    .expression()
                    .is_ok_and(|inner| is_typed_array(&inner, model, depth))
        }
        AnyJsExpression::TsNonNullAssertionExpression(expression) => expression
            .expression()
            .is_ok_and(|inner| is_typed_array(&inner, model, depth)),
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            if depth == 0 {
                return false;
            }
            let Some(binding) = identifier
                .name()
                .ok()
                .and_then(|reference| model.binding(&reference))
            else {
                return false;
            };
            match binding.tree().declaration() {
                Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) => {
                    is_typed_array_annotation(
                        declarator
                            .variable_annotation()
                            .and_then(|annotation| annotation.as_ts_type_annotation().cloned()),
                    ) || (declarator
                        .declaration()
                        .is_some_and(|declaration| declaration.is_const())
                        && declarator
                            .initializer()
                            .and_then(|initializer| initializer.expression().ok())
                            .is_some_and(|initializer| {
                                is_typed_array(&initializer, model, depth - 1)
                            }))
                }
                Some(AnyJsBindingDeclaration::JsFormalParameter(parameter)) => {
                    is_typed_array_annotation(parameter.type_annotation())
                }
                _ => false,
            }
        }
        _ => false,
    }
}

fn is_typed_array_annotation(annotation: Option<TsTypeAnnotation>) -> bool {
    annotation
        .and_then(|annotation| annotation.ty().ok())
        .is_some_and(|ty| is_typed_array_type(&ty))
}

/// Returns `true` if `ty` names a typed array, like `Uint8Array`.
fn is_typed_array_type(ty: &AnyTsType) -> bool {
    ty.as_ts_reference_type()
        .and_then(|reference| reference.name().ok())
        .and_then(|name| name.as_js_reference_identifier()?.name().ok())
        .is_some_and(|name| is_typed_array_name(Some(name.text())))
}

fn is_typed_array_name(name: Option<&str>) -> bool {
    name.is_some_and(|name| TYPED_ARRAY_CONSTRUCTORS.binary_search(&name).is_ok())
}

/// Returns the statement that declares the object if `for_of` only copies each pair into an
/// object declared as `{}` by the statement right before the loop.
fn loop_object_statement(
    for_of: &JsForOfStatement,
    model: &SemanticModel,
) -> Option<JsVariableStatement> {
    if for_of.await_token().is_some() {
        return None;
    }

    let statement = JsVariableStatement::cast(for_of.syntax().prev_sibling()?)?;
    let declarators = statement.declaration().ok()?.declarators();
    if declarators.len() != 1 {
        return None;
    }
    let declarator = declarators.first()?.ok()?;
    let AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(object)) =
        declarator.id().ok()?
    else {
        return None;
    };
    if !is_empty_object(&declarator.initializer()?.expression().ok()?) {
        return None;
    }
    let object_token = object.name_token().ok()?;
    let object_name = object_token.text_trimmed();

    // `const [key, value]`. With `var`, the key and value would still be readable after the loop.
    let AnyJsForInOrOfInitializer::JsForVariableDeclaration(pair) = for_of.initializer().ok()?
    else {
        return None;
    };
    if pair.is_var() || pair.variable_kind().ok()? == biome_js_syntax::JsVariableKind::Using {
        return None;
    }
    let AnyJsBindingPattern::JsArrayBindingPattern(pair) = pair.declarator().ok()?.id().ok()?
    else {
        return None;
    };
    let elements = pair.elements();
    if elements.len() != 2 {
        return None;
    }
    let mut elements = elements.iter();
    let key = pair_element_name(elements.next()?.ok()?)?;
    let value = pair_element_name(elements.next()?.ok()?)?;
    let (key, value) = (key.text_trimmed(), value.text_trimmed());
    if key == object_name || value == object_name {
        return None;
    }

    // `object[key] = value;`, with or without braces around it.
    let body = match for_of.body().ok()? {
        AnyJsStatement::JsBlockStatement(block) => {
            let statements = block.statements();
            if statements.len() != 1 {
                return None;
            }
            statements.first()?
        }
        statement => statement,
    };
    let assignment = body.as_js_expression_statement()?.expression().ok()?;
    let assignment = assignment.as_js_assignment_expression()?;
    if assignment.operator().ok()? != JsAssignmentOperator::Assign
        || !is_reference_named(&assignment.right().ok()?, value)
    {
        return None;
    }
    let AnyJsAssignmentPattern::AnyJsAssignment(target) = assignment.left().ok()? else {
        return None;
    };
    let target = target.as_js_computed_member_assignment()?;
    if !is_reference_named(&target.object().ok()?, object_name)
        || !is_reference_named(&target.member().ok()?, key)
    {
        return None;
    }

    // `for (const [key, value] of object)` reads the object while the loop fills it.
    let pairs_range = for_of.expression().ok()?.range();
    if object
        .all_references(model)
        .any(|reference| pairs_range.contains(reference.range_start()))
    {
        return None;
    }
    Some(statement)
}

/// Returns the name of a plain identifier in an array pattern, like `key` in `[key, value]`.
fn pair_element_name(element: AnyJsArrayBindingPatternElement) -> Option<JsSyntaxToken> {
    let AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(element) = element else {
        return None;
    };
    if element.init().is_some() {
        return None;
    }
    match element.pattern().ok()? {
        AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) => {
            binding.name_token().ok()
        }
        _ => None,
    }
}

/// Returns `true` if `expression` is `{}`, ignoring parentheses.
fn is_empty_object(expression: &AnyJsExpression) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_js_object_expression()
        .is_some_and(|object| object.members().is_empty())
}

/// Returns `true` if `expression` is a reference to the variable called `name`, ignoring parentheses.
fn is_reference_named(expression: &AnyJsExpression, name: &str) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_js_reference_identifier()
        .is_some_and(|reference| reference.has_name(name))
}

/// Builds `Object.fromEntries(pairs.map((…) => [key, value]))` from a `reduce()` call.
///
/// Returns `None` if the call has type arguments or the callback has a return type, which
/// don't apply to `map()`, if the receiver contains an optional chain, which would make
/// `Object.fromEntries()` receive `undefined`, or if the rewrite would drop a comment.
fn reduce_replacement(
    call: &JsCallExpression,
    entry: &AnyJsObjectMember,
    ctx: &RuleContext<UseObjectFromEntries>,
) -> Option<AnyJsExpression> {
    if call.type_arguments().is_some() || call.is_optional_chain() {
        return None;
    }
    let arguments = call.arguments().ok()?;
    if arguments.syntax().has_inner_comments() {
        return None;
    }
    let callback = arguments.args().first()?.ok()?;
    let callback = callback.as_any_js_expression()?;
    let arrow = callback.clone().omit_parentheses();
    let arrow = arrow.as_js_arrow_function_expression()?;
    if arrow.return_type_annotation().is_some() {
        return None;
    }

    let parameters = match arrow.parameters().ok()? {
        // `object => …` becomes `() => …`, keeping the space before the arrow.
        AnyJsArrowFunctionParameters::AnyJsBinding(binding) => make::js_parameters(
            make::token(T!['(']),
            make::js_parameter_list([], []),
            make::token(T![')']).with_trailing_trivia_pieces(
                binding.syntax().last_token()?.trailing_trivia().pieces(),
            ),
        ),
        AnyJsArrowFunctionParameters::JsParameters(parameters) => {
            let rest = parameters.items().elements().skip(1);
            let mut items = Vec::new();
            let mut separators = Vec::new();
            for element in rest {
                items.push(element.node().ok()?.clone());
                if let Some(separator) = element.trailing_separator().ok()? {
                    separators.push(separator.clone());
                }
            }
            parameters.with_items(make::js_parameter_list(items, separators))
        }
    };

    let (key, value) = entry_key_value(entry, ctx)?;
    let pair = make::js_array_expression(
        make::token(T!['[']),
        make::js_array_element_list(
            [
                AnyJsArrayElement::AnyJsExpression(key),
                AnyJsArrayElement::AnyJsExpression(value),
            ],
            [make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])],
        ),
        make::token(T![']']),
    );
    let new_arrow = arrow
        .clone()
        .with_parameters(parameters.into())
        .with_body(AnyJsFunctionBody::AnyJsExpression(pair.into()));
    // Keeps the parentheses around the callback, if any.
    let new_callback = if callback.syntax() == arrow.syntax() {
        new_arrow.into()
    } else {
        callback
            .clone()
            .replace_node(AnyJsExpression::from(arrow.clone()), new_arrow.into())?
    };

    let member = call.callee().ok()?;
    let member = member.as_js_static_member_expression()?;
    let reduce = member.member().ok()?.as_js_name()?.value_token().ok()?;
    let map = make::ident("map");
    let callee = member.clone().replace_token(reduce, map)?;
    let map_call = call
        .clone()
        .with_callee(callee.into())
        .with_arguments(arguments.with_args(make::js_call_argument_list(
            [AnyJsCallArgument::AnyJsExpression(new_callback)],
            [],
        )))
        // `replace_node()` moves the trivia around the call to the outer `Object.fromEntries()`.
        .with_leading_trivia_pieces([])?
        .with_trailing_trivia_pieces([])?;
    Some(object_from_entries_call(map_call.into()).into())
}

/// Returns the key and the value of the property that `member` creates, as expressions that can
/// go in a `[key, value]` pair.
fn entry_key_value(
    member: &AnyJsObjectMember,
    ctx: &RuleContext<UseObjectFromEntries>,
) -> Option<(AnyJsExpression, AnyJsExpression)> {
    match member {
        AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => {
            let name = shorthand.name().ok()?.trim_trivia()?;
            let key = string_literal(name.value_token().ok()?.text_trimmed(), ctx);
            Some((key, make::js_identifier_expression(name).into()))
        }
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            let key =
                match property.name().ok()? {
                    AnyJsObjectMemberName::JsLiteralMemberName(name) => {
                        let token = name.value().ok()?;
                        let text = token.text_trimmed();
                        match token.kind() {
                            JsSyntaxKind::IDENT => string_literal(text, ctx),
                            JsSyntaxKind::JS_STRING_LITERAL => {
                                AnyJsExpression::AnyJsLiteralExpression(
                                    make::js_string_literal_expression(
                                        JsSyntaxToken::new_detached(token.kind(), text, [], []),
                                    )
                                    .into(),
                                )
                            }
                            JsSyntaxKind::JS_NUMBER_LITERAL => {
                                AnyJsExpression::AnyJsLiteralExpression(
                                    make::js_number_literal_expression(
                                        JsSyntaxToken::new_detached(token.kind(), text, [], []),
                                    )
                                    .into(),
                                )
                            }
                            _ => return None,
                        }
                    }
                    AnyJsObjectMemberName::JsComputedMemberName(name) => {
                        name.expression().ok()?.trim_trivia()?
                    }
                    _ => return None,
                };
            Some((key, property.value().ok()?.trim_trivia()?))
        }
        _ => None,
    }
}

fn string_literal(text: &str, ctx: &RuleContext<UseObjectFromEntries>) -> AnyJsExpression {
    let token = if ctx.preferred_quote().is_double() {
        make::js_string_literal(text)
    } else {
        make::js_string_literal_single_quotes(text)
    };
    AnyJsExpression::AnyJsLiteralExpression(make::js_string_literal_expression(token).into())
}

/// Builds `Object.fromEntries`.
fn object_from_entries_callee() -> AnyJsExpression {
    make::js_static_member_expression(
        make::js_identifier_expression(make::js_reference_identifier(make::ident("Object"))).into(),
        make::token(T![.]),
        make::js_name(make::ident("fromEntries")).into(),
    )
    .into()
}

/// Builds `Object.fromEntries(pairs)`.
fn object_from_entries_call(pairs: AnyJsExpression) -> JsCallExpression {
    make::js_call_expression(
        object_from_entries_callee(),
        make::js_call_arguments(
            make::token(T!['(']),
            make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(pairs)], []),
            make::token(T![')']),
        ),
    )
    .build()
}

#[cfg(test)]
mod tests {
    use super::TYPED_ARRAY_CONSTRUCTORS;

    #[test]
    fn typed_array_constructors_are_sorted() {
        for pair in TYPED_ARRAY_CONSTRUCTORS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} should come after {}",
                pair[0],
                pair[1]
            );
        }
    }
}
