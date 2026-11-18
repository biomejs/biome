use crate::services::control_flow::AnyJsControlFlowRoot;
use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::{Reference, SemanticModel};
use biome_js_syntax::binding_ext::AnyJsIdentifierBinding;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsLiteralExpression,
    AnyJsMemberExpression, AnyTsName, AnyTsReturnType, AnyTsType, JsAssignmentExpression,
    JsAssignmentOperator, JsCallExpression, JsConstructorClassMember, JsConstructorParameters,
    JsForInStatement, JsFormalParameter, JsFunctionBody, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsIdentifierBinding, JsIdentifierExpression, JsLanguage,
    JsMethodClassMember, JsMethodObjectMember, JsModuleItemList, JsParameters,
    JsPostUpdateExpression, JsPreUpdateExpression, JsReturnStatement, JsStatementList,
    JsUnaryOperator, JsVariableDeclarator, TextRange, TextSize, TsDeclareFunctionDeclaration,
    TsDeclareFunctionExportDefaultDeclaration, TsFunctionType, TsReferenceType,
    TsReturnTypeAnnotation, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, WalkEvent, declare_node_union};
use biome_rule_options::use_consistent_boolean_name::UseConsistentBooleanNameOptions;

declare_lint_rule! {
    /// Require boolean names to start with a prefix such as `is` or `has`, and reserve those prefixes for booleans.
    ///
    /// A name like `isReady` or `hasAccess` tells the reader that the value is
    /// either `true` or `false`. When a boolean has a name like `ready`, readers
    /// have to look at the code to learn what kind of value it holds. When a
    /// name like `hasName` holds a string, it misleads readers.
    ///
    /// This rule checks variables, function parameters, and function names.
    /// It reports a name that holds a boolean, or a function that returns one,
    /// when the name doesn't start with a boolean prefix. It also reports a
    /// name that starts with a boolean prefix when its value isn't a boolean.
    ///
    /// The rule decides whether a value is a boolean from what the code shows:
    /// type annotations, literals like `true`, comparisons like `a === b`,
    /// negations like `!value`, calls like `Boolean(value)` or
    /// `Array.isArray(value)`, and the values that a function returns. When the
    /// rule can't tell whether a value is a boolean, it doesn't report the name.
    ///
    /// A prefix counts only when it's a separate word in the name. `isReady`,
    /// `is_ready`, `IS_READY`, and `_isReady` all start with the prefix `is`,
    /// but `island` doesn't. For React hooks, the rule checks the part of the
    /// name after `use`, so `useIsReady` is a boolean name.
    ///
    /// The default prefixes are `are`, `can`, `did`, `does`, `has`, `have`,
    /// `is`, `requires`, `should`, `was`, `were`, and `will`.
    ///
    /// Destructured variables, imports, class names, and `catch` parameters
    /// aren't checked. Object properties, class properties, and methods
    /// aren't checked either.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const completed = true;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const completed = progress === 100;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const hasName = "Sindre";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function hasTitle() {
    ///     return "Unicorn";
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function completed() {
    ///     return true;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function download(showProgress: boolean) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// const isCompleted = true;
    /// const hasCompleted = progress === 100;
    /// const name = "Sindre";
    /// function getTitle() {
    ///     return "Unicorn";
    /// }
    /// function isCompleted() {
    ///     return true;
    /// }
    /// function download(shouldShowProgress: boolean) {}
    /// const task = { completed: progress === 100 };
    /// ```
    ///
    /// ## Options
    ///
    /// ### `prefixes`
    ///
    /// Default: `{}`
    ///
    /// Prefixes to add to the defaults, or to turn off. Set a prefix to `true`
    /// to accept it on boolean names and to require a boolean value for names
    /// that start with it. Set a prefix to `false` to turn it off: the rule no
    /// longer accepts it on boolean names, and no longer requires a boolean
    /// value for names that start with it.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "prefixes": {
    ///             "needs": true,
    ///             "did": false
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// const didUpdate = true;
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// const needsUpdate = "yes";
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// const needsUpdate = true;
    /// const didUpdate = "yes";
    /// ```
    ///
    /// ### `ignore`
    ///
    /// Default: `[]`
    ///
    /// Names that the rule doesn't check. A name must match an entry exactly.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignore": ["value", "completed"]
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// const value = true;
    /// const completed = true;
    /// ```
    ///
    pub UseConsistentBooleanName {
        version: "2.6.7",
        name: "useConsistentBooleanName",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("consistent-boolean-name").inspired()],
    }
}

impl Rule for UseConsistentBooleanName {
    type Query = Semantic<JsIdentifierBinding>;
    type State = BooleanNameIssue;
    type Signals = Option<Self::State>;
    type Options = UseConsistentBooleanNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let model = ctx.model();
        let options = ctx.options();
        let token = binding.name_token().ok()?;
        let name = token.text_trimmed();
        if options
            .ignore()
            .iter()
            .any(|ignored| ignored.as_ref() == name)
        {
            return None;
        }

        let (state, is_function) = binding_state(binding, model)?;
        if state == BooleanState::Unknown {
            return None;
        }

        let mut offset = name.len() - name.trim_start_matches('_').len();
        let mut rest = &name[offset..];
        let is_hook = is_function && is_hook_name(rest);
        let name_kind = if is_hook {
            NameKind::Hook
        } else if is_function {
            NameKind::Function
        } else {
            NameKind::Value
        };
        if is_hook {
            offset += HOOK_PREFIX.len();
            rest = &rest[HOOK_PREFIX.len()..];
        }
        let prefix_len = options
            .enabled_prefixes()
            .find(|prefix| starts_with_prefix(rest, prefix, is_hook))
            .map(str::len);

        match (state, prefix_len) {
            (BooleanState::Boolean, None) => Some(BooleanNameIssue {
                kind: BooleanNameIssueKind::MissingPrefix,
                name_kind,
            }),
            (BooleanState::NonBoolean, Some(len)) => {
                let start = token.text_trimmed_range().start() + TextSize::try_from(offset).ok()?;
                Some(BooleanNameIssue {
                    kind: BooleanNameIssueKind::NonBooleanWithPrefix(TextRange::at(
                        start,
                        TextSize::try_from(len).ok()?,
                    )),
                    name_kind,
                })
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let token = ctx.query().name_token().ok()?;
        let name = token.text_trimmed();
        let range = token.text_trimmed_range();
        let verb = Verb(state.name_kind);
        Some(match state.kind {
            BooleanNameIssueKind::MissingPrefix => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    <Emphasis>{name}</Emphasis>" "{verb}"s a boolean, but its name doesn't start with a boolean prefix."
                },
            )
            .note(markup! {
                "A boolean prefix tells readers that the value is either true or false."
            })
            .note(markup! {
                "Rename it so that "{NamePart(state.name_kind)}" starts with one of these prefixes: "{PrefixList(ctx.options())}"."
            }),
            BooleanNameIssueKind::NonBooleanWithPrefix(prefix_range) => {
                let prefix = &name[usize::from(prefix_range.start() - range.start())
                    ..usize::from(prefix_range.end() - range.start())];
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        <Emphasis>{name}</Emphasis>" starts with the boolean prefix "<Emphasis>{prefix}</Emphasis>", but doesn't "{verb}" a boolean."
                    },
                )
                .note(markup! {
                    "Readers expect a name that starts with "<Emphasis>{prefix}</Emphasis>" to be either true or false."
                })
                .note(markup! {
                    "Rename it so that it doesn't start with "<Emphasis>{prefix}</Emphasis>"."
                })
            }
        })
    }
}

declare_node_union! {
    /// A function declaration without a body: an overload signature or a
    /// `declare function` statement.
    pub AnyTsDeclareFunction = TsDeclareFunctionDeclaration | TsDeclareFunctionExportDefaultDeclaration
}

impl AnyTsDeclareFunction {
    fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        match self {
            Self::TsDeclareFunctionDeclaration(declaration) => declaration.return_type_annotation(),
            Self::TsDeclareFunctionExportDefaultDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
        }
    }
}

/// A name that doesn't follow the boolean naming convention.
pub struct BooleanNameIssue {
    kind: BooleanNameIssueKind,
    name_kind: NameKind,
}

/// What a checked name refers to.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum NameKind {
    /// A variable or parameter, whose value was checked.
    Value,
    /// A function, whose return value was checked.
    Function,
    /// A React hook, whose return value and the part of whose name after
    /// `use` were checked.
    Hook,
}

pub enum BooleanNameIssueKind {
    /// The value is a boolean, but the name doesn't start with an enabled prefix.
    MissingPrefix,
    /// The name starts with the enabled prefix at this range, but the value isn't a boolean.
    NonBooleanWithPrefix(TextRange),
}

/// Writes how a name relates to the checked value: a function `return`s
/// it, and a variable or parameter `hold`s it.
struct Verb(NameKind);

impl biome_console::fmt::Display for Verb {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self.0 {
            NameKind::Value => "hold",
            NameKind::Function | NameKind::Hook => "return",
        })
    }
}

/// Writes the part of a name that has to start with a boolean prefix.
struct NamePart(NameKind);

impl biome_console::fmt::Display for NamePart {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self.0 {
            NameKind::Value | NameKind::Function => fmt.write_str("its name"),
            NameKind::Hook => fmt
                .write_markup(markup! { "the part of its name after "<Emphasis>"use"</Emphasis> }),
        }
    }
}

/// What the rule knows about whether a value is a boolean.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BooleanState {
    Boolean,
    NonBoolean,
    Unknown,
}

impl BooleanState {
    /// Returns the state of a value that is either `self` or `other`.
    fn either(self, other: Self) -> Self {
        if self == other { self } else { Self::Unknown }
    }

    /// Returns the state of a variable that receives both `self` and `other`.
    ///
    /// Receiving a non-boolean value makes the variable non-boolean, even if
    /// it also receives booleans.
    fn with_assignment(self, other: Self) -> Self {
        match (self, other) {
            (Self::NonBoolean, _) | (_, Self::NonBoolean) => Self::NonBoolean,
            (Self::Boolean, Self::Boolean) => Self::Boolean,
            _ => Self::Unknown,
        }
    }
}

/// Writes the enabled prefixes as a comma-separated list.
struct PrefixList<'a>(&'a UseConsistentBooleanNameOptions);

impl biome_console::fmt::Display for PrefixList<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        for (index, prefix) in self.0.enabled_prefixes().enumerate() {
            if index > 0 {
                fmt.write_str(", ")?;
            }
            fmt.write_markup(markup! { <Emphasis>{prefix}</Emphasis> })?;
        }
        Ok(())
    }
}

const HOOK_PREFIX: &str = "use";

/// The maximum number of references to variables and parameters followed
/// to find a value, which also stops cycles such as `const a = b, b = a`.
const MAX_RESOLUTION_DEPTH: u8 = 3;

/// The nodes that the search for `return` statements stops at: the statements
/// themselves, and the functions whose `return` statements belong to them.
const RETURN_SEARCH_KINDS: SyntaxKindSet<JsLanguage> =
    JsReturnStatement::KIND_SET.union(AnyJsControlFlowRoot::KIND_SET);

/// The functions whose parameters the rule checks. Parameters of setters and
/// of type-only signatures aren't checked.
const CHECKED_PARAMETER_OWNERS: SyntaxKindSet<JsLanguage> = AnyJsFunction::KIND_SET
    .union(JsMethodClassMember::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsConstructorClassMember::KIND_SET);

/// The parameter lists of functions and constructors.
const PARAMETERS_KINDS: SyntaxKindSet<JsLanguage> =
    JsParameters::KIND_SET.union(JsConstructorParameters::KIND_SET);

/// The parents of a variable write that always assigns a non-boolean value.
const NON_BOOLEAN_WRITE_KINDS: SyntaxKindSet<JsLanguage> = JsPreUpdateExpression::KIND_SET
    .union(JsPostUpdateExpression::KIND_SET)
    .union(JsForInStatement::KIND_SET);

/// The lists that contain statements.
const STATEMENT_LIST_KINDS: SyntaxKindSet<JsLanguage> =
    JsModuleItemList::KIND_SET.union(JsStatementList::KIND_SET);

/// The declarations of named functions, with or without a body.
const FUNCTION_DECLARATION_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionDeclaration::KIND_SET
    .union(JsFunctionExportDefaultDeclaration::KIND_SET)
    .union(TsDeclareFunctionDeclaration::KIND_SET)
    .union(TsDeclareFunctionExportDefaultDeclaration::KIND_SET);

/// Methods that return a boolean when called on a string or an array.
const BOOLEAN_INSTANCE_METHODS: &[&str] = &["endsWith", "every", "includes", "some", "startsWith"];

/// Static methods of global objects that return a boolean, sorted by object
/// name, then by method name.
const BOOLEAN_STATIC_METHODS: &[(&str, &str)] = &[
    ("Array", "isArray"),
    ("ArrayBuffer", "isView"),
    ("Number", "isFinite"),
    ("Number", "isInteger"),
    ("Number", "isNaN"),
    ("Number", "isSafeInteger"),
    ("Object", "is"),
    ("Object", "isExtensible"),
    ("Object", "isFrozen"),
    ("Object", "isSealed"),
    ("Reflect", "has"),
];

/// Returns the boolean state of the value that `binding` names, and whether
/// that value is a function, in which case the state describes what the
/// function returns.
///
/// Returns `None` when the rule doesn't check this kind of name.
fn binding_state(
    binding: &JsIdentifierBinding,
    model: &SemanticModel,
) -> Option<(BooleanState, bool)> {
    let parent = binding.syntax().parent()?;
    if let Some(declarator) = JsVariableDeclarator::cast_ref(&parent) {
        let (declared, is_function) = declarator_state(&declarator, model);
        let state = match declared {
            Some(state) if declarator.variable_annotation().is_some() => state,
            declared => with_writes(declared, is_function, binding, model),
        };
        return Some((state, is_function));
    }
    if let Some(parameter) = JsFormalParameter::cast_ref(&parent) {
        if !is_checked_parameter(&parameter) {
            return None;
        }
        let (declared, is_function) = parameter_state(&parameter, model);
        let state = if parameter.type_annotation().is_some() {
            declared
        } else {
            // A parameter without a default value receives unknown arguments.
            with_writes(Some(declared), is_function, binding, model)
        };
        return Some((state, is_function));
    }
    if let Some(function) = AnyJsFunction::cast_ref(&parent) {
        return match function {
            // The only binding directly inside an arrow function is a
            // parameter without parentheses, as in `value => value`, which
            // receives unknown arguments.
            AnyJsFunction::JsArrowFunctionExpression(_) => Some((
                with_writes(Some(BooleanState::Unknown), false, binding, model),
                false,
            )),
            // Function expression names are checked through the variable
            // that holds the function, if any.
            AnyJsFunction::JsFunctionExpression(_) => None,
            _ => Some((function_state(&function, model), true)),
        };
    }
    if let Some(declaration) = AnyTsDeclareFunction::cast_ref(&parent) {
        if is_overload_signature(&declaration, binding) {
            return None;
        }
        let state = declaration
            .return_type_annotation()
            .and_then(|annotation| annotation.ty().ok())
            .map_or(BooleanState::Unknown, |ty| return_type_state(&ty));
        return Some((state, true));
    }
    None
}

/// Combines `declared`, the state of the declared value, with the states of
/// the values assigned to `binding` later. `None` means that the declaration
/// doesn't give the variable a value. `holds_function` tells whether the
/// declared value is a function, in which case the states describe what the
/// functions return.
fn with_writes(
    declared: Option<BooleanState>,
    holds_function: bool,
    binding: &JsIdentifierBinding,
    model: &SemanticModel,
) -> BooleanState {
    model
        .as_binding(binding)
        .all_writes()
        .fold(declared, |state, write| {
            let write = write_state(&write, holds_function, model);
            Some(state.map_or(write, |state| state.with_assignment(write)))
        })
        .unwrap_or(BooleanState::Unknown)
}

/// Returns the state of the value that the write `reference` assigns. When
/// `holds_function` is `true`, the state describes what the assigned
/// function returns.
fn write_state(reference: &Reference, holds_function: bool, model: &SemanticModel) -> BooleanState {
    let Some(parent) = reference.syntax().parent() else {
        return BooleanState::Unknown;
    };
    if let Some(assignment) = JsAssignmentExpression::cast_ref(&parent) {
        let (Ok(operator), Ok(right)) = (assignment.operator(), assignment.right()) else {
            return BooleanState::Unknown;
        };
        return match operator {
            JsAssignmentOperator::Assign => {
                // A variable that holds both functions and other values has
                // no single kind of value to name.
                if is_function_expression(&right) == holds_function {
                    value_state(&right, model).0
                } else {
                    BooleanState::Unknown
                }
            }
            _ if holds_function => BooleanState::Unknown,
            // `&&=`, `||=`, and `??=` keep the current value in some cases,
            // so a non-boolean right side doesn't prove the variable is non-boolean.
            JsAssignmentOperator::LogicalAndAssign
            | JsAssignmentOperator::LogicalOrAssign
            | JsAssignmentOperator::NullishCoalescingAssign => {
                match expressions_state(vec![right], model) {
                    BooleanState::NonBoolean => BooleanState::Unknown,
                    state => state,
                }
            }
            _ => BooleanState::NonBoolean,
        };
    }
    if NON_BOOLEAN_WRITE_KINDS.matches(parent.kind()) {
        return BooleanState::NonBoolean;
    }
    BooleanState::Unknown
}

/// Returns the state of the value that `declarator` declares, and whether
/// that value is a function. The state is `None` when the declarator has
/// neither a type annotation nor an initializer.
fn declarator_state(
    declarator: &JsVariableDeclarator,
    model: &SemanticModel,
) -> (Option<BooleanState>, bool) {
    if let Some(ty) = declarator
        .variable_annotation()
        .and_then(|annotation| annotation.type_annotation().ok().flatten())
        .and_then(|annotation| annotation.ty().ok())
    {
        let (state, is_function) = annotation_state(&ty);
        return (Some(state), is_function);
    }
    match declarator
        .initializer()
        .and_then(|initializer| initializer.expression().ok())
    {
        Some(expression) => {
            let (state, is_function) = value_state(&expression, model);
            (Some(state), is_function)
        }
        None => (None, false),
    }
}

/// Returns the state of the value of `parameter`, and whether that value is
/// a function.
fn parameter_state(parameter: &JsFormalParameter, model: &SemanticModel) -> (BooleanState, bool) {
    if let Some(ty) = parameter
        .type_annotation()
        .and_then(|annotation| annotation.ty().ok())
    {
        return annotation_state(&ty);
    }
    parameter
        .initializer()
        .and_then(|initializer| initializer.expression().ok())
        .map_or((BooleanState::Unknown, false), |expression| {
            value_state(&expression, model)
        })
}

/// Returns whether the rule checks `parameter`, which is the case for the
/// parameters of functions, methods, and constructors that have a body.
fn is_checked_parameter(parameter: &JsFormalParameter) -> bool {
    parameter
        .syntax()
        .ancestors()
        .skip(1)
        .find(|node| PARAMETERS_KINDS.matches(node.kind()))
        .and_then(|parameters| parameters.parent())
        .is_some_and(|owner| CHECKED_PARAMETER_OWNERS.matches(owner.kind()))
}

/// Returns whether `declaration`, a function declaration without a body, is
/// an overload signature: the next statement declares a function with the
/// same name.
fn is_overload_signature(
    declaration: &AnyTsDeclareFunction,
    binding: &JsIdentifierBinding,
) -> bool {
    let Some(statement) = declaration.syntax().ancestors().find(|node| {
        node.parent()
            .is_some_and(|parent| STATEMENT_LIST_KINDS.matches(parent.kind()))
    }) else {
        return false;
    };
    let Some(next) = statement.next_sibling() else {
        return false;
    };
    // The first binding of a function declaration is the function name.
    next.descendants()
        .find_map(JsIdentifierBinding::cast)
        .is_some_and(|next_binding| {
            next_binding
                .syntax()
                .parent()
                .is_some_and(|parent| FUNCTION_DECLARATION_KINDS.matches(parent.kind()))
                && next_binding
                    .name_token()
                    .ok()
                    .zip(binding.name_token().ok())
                    .is_some_and(|(next, current)| next.text_trimmed() == current.text_trimmed())
        })
}

/// Returns the state of a value with the type annotation `ty`, and whether
/// the annotation is a function type, in which case the state describes
/// what the function returns.
fn annotation_state(ty: &AnyTsType) -> (BooleanState, bool) {
    match function_type(ty) {
        Some(function) => (
            function
                .return_type()
                .map_or(BooleanState::Unknown, |ty| return_type_state(&ty)),
            true,
        ),
        None => (type_state(ty.clone(), false), false),
    }
}

/// Returns the function type that `ty` describes, when `ty` is a function
/// type, possibly in parentheses or in a union with `null` or `undefined`,
/// as in `(() => boolean) | undefined`.
fn function_type(ty: &AnyTsType) -> Option<TsFunctionType> {
    let mut ty = ty.clone();
    loop {
        ty = match ty {
            AnyTsType::TsFunctionType(function) => return Some(function),
            AnyTsType::TsParenthesizedType(ty) => ty.ty().ok()?,
            AnyTsType::TsUnionType(union) => {
                let mut members = union.types().iter().filter_map(Result::ok).filter(|ty| {
                    !matches!(
                        ty,
                        AnyTsType::TsNullLiteralType(_) | AnyTsType::TsUndefinedType(_)
                    )
                });
                let member = members.next()?;
                if members.next().is_some() {
                    return None;
                }
                member
            }
            _ => return None,
        };
    }
}

/// Returns the state of a function that returns `ty`. A function that
/// returns a promise of a boolean counts as returning a boolean.
fn return_type_state(ty: &AnyTsReturnType) -> BooleanState {
    match ty {
        AnyTsReturnType::AnyTsType(ty) => type_state(ty.clone(), true),
        AnyTsReturnType::TsPredicateReturnType(_) => BooleanState::Boolean,
        AnyTsReturnType::TsAssertsReturnType(_) => BooleanState::NonBoolean,
    }
}

/// Returns the state of a value of type `ty`. When `unwrap_promise` is
/// `true`, `Promise<T>` and `PromiseLike<T>` have the state of `T`.
///
/// The members of a union combine as the values assigned to a variable do,
/// and `null` and `undefined` members are skipped, so `boolean | undefined`
/// is a boolean and `string | false` isn't.
fn type_state(ty: AnyTsType, unwrap_promise: bool) -> BooleanState {
    // Each pending type comes with whether promises in it are unwrapped, and
    // whether it's a union member.
    let mut pending = vec![(ty, unwrap_promise, false)];
    let mut result: Option<BooleanState> = None;
    while let Some((ty, unwrap_promise, is_union_member)) = pending.pop() {
        let state = match ty {
            AnyTsType::TsBooleanType(_) | AnyTsType::TsBooleanLiteralType(_) => {
                BooleanState::Boolean
            }
            AnyTsType::TsParenthesizedType(ty) => {
                let Ok(ty) = ty.ty() else {
                    return BooleanState::Unknown;
                };
                pending.push((ty, unwrap_promise, is_union_member));
                continue;
            }
            AnyTsType::TsUnionType(union) => {
                for member in union.types().iter() {
                    let Ok(member) = member else {
                        return BooleanState::Unknown;
                    };
                    pending.push((member, unwrap_promise, true));
                }
                continue;
            }
            AnyTsType::TsNullLiteralType(_) | AnyTsType::TsUndefinedType(_) if is_union_member => {
                continue;
            }
            AnyTsType::TsReferenceType(reference) => {
                if !is_promise_reference(&reference) {
                    BooleanState::Unknown
                } else if !unwrap_promise {
                    BooleanState::NonBoolean
                } else {
                    let Some(argument) = reference
                        .type_arguments()
                        .and_then(|arguments| arguments.ts_type_argument_list().iter().next())
                        .and_then(Result::ok)
                    else {
                        return BooleanState::Unknown;
                    };
                    pending.push((argument, false, false));
                    continue;
                }
            }
            AnyTsType::TsAnyType(_)
            | AnyTsType::TsUnknownType(_)
            | AnyTsType::TsNeverType(_)
            | AnyTsType::TsNullLiteralType(_)
            | AnyTsType::TsUndefinedType(_)
            | AnyTsType::TsConditionalType(_)
            | AnyTsType::TsIntersectionType(_)
            | AnyTsType::TsIndexedAccessType(_)
            | AnyTsType::TsInferType(_)
            | AnyTsType::TsImportType(_)
            | AnyTsType::TsThisType(_)
            | AnyTsType::TsTypeofType(_)
            | AnyTsType::TsBogusType(_)
            | AnyTsType::JsMetavariable(_) => BooleanState::Unknown,
            AnyTsType::TsArrayType(_)
            | AnyTsType::TsBigintLiteralType(_)
            | AnyTsType::TsBigintType(_)
            | AnyTsType::TsConstructorType(_)
            | AnyTsType::TsFunctionType(_)
            | AnyTsType::TsMappedType(_)
            | AnyTsType::TsNonPrimitiveType(_)
            | AnyTsType::TsNumberLiteralType(_)
            | AnyTsType::TsNumberType(_)
            | AnyTsType::TsObjectType(_)
            | AnyTsType::TsStringLiteralType(_)
            | AnyTsType::TsStringType(_)
            | AnyTsType::TsSymbolType(_)
            | AnyTsType::TsTemplateLiteralType(_)
            | AnyTsType::TsTupleType(_)
            | AnyTsType::TsTypeOperatorType(_)
            | AnyTsType::TsVoidType(_) => BooleanState::NonBoolean,
        };
        if state == BooleanState::Unknown {
            return state;
        }
        result = Some(result.map_or(state, |result| result.with_assignment(state)));
    }
    result.unwrap_or(BooleanState::Unknown)
}

/// Returns whether `reference` names `Promise` or `PromiseLike`.
fn is_promise_reference(reference: &TsReferenceType) -> bool {
    let Ok(AnyTsName::JsReferenceIdentifier(name)) = reference.name() else {
        return false;
    };
    name.value_token()
        .is_ok_and(|name| matches!(name.text_trimmed(), "Promise" | "PromiseLike"))
}

/// Returns the state of `expression` when it's assigned to a name, and
/// whether it's a function, in which case the state describes what the
/// function returns.
fn value_state(expression: &AnyJsExpression, model: &SemanticModel) -> (BooleanState, bool) {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::JsArrowFunctionExpression(function) => {
            (function_state(&function.into(), model), true)
        }
        AnyJsExpression::JsFunctionExpression(function) => {
            (function_state(&function.into(), model), true)
        }
        expression => (expressions_state(vec![expression], model), false),
    }
}

/// Returns the state of what `function` returns.
fn function_state(function: &AnyJsFunction, model: &SemanticModel) -> BooleanState {
    if function.is_generator() {
        return BooleanState::NonBoolean;
    }
    if let Some(ty) = function
        .return_type_annotation()
        .and_then(|annotation| annotation.ty().ok())
    {
        let state = return_type_state(&ty);
        if state != BooleanState::Unknown {
            return state;
        }
    }
    let state = match function.body() {
        Ok(AnyJsFunctionBody::AnyJsExpression(expression)) => {
            expressions_state(vec![expression], model)
        }
        Ok(AnyJsFunctionBody::JsFunctionBody(body)) => returns_state(&body, model),
        Err(_) => BooleanState::Unknown,
    };
    // Without an annotation, an `async` function that returns booleans
    // returns a promise, which isn't a boolean on its own.
    if function.is_async() && state == BooleanState::Boolean {
        return BooleanState::Unknown;
    }
    state
}

/// Returns the combined state of the values returned by the `return`
/// statements of `body`. A body without a `return` statement, or with a
/// `return` statement without a value, returns `undefined`.
fn returns_state(body: &JsFunctionBody, model: &SemanticModel) -> BooleanState {
    let mut arguments = Vec::new();
    let mut returns_undefined = false;
    let mut preorder = body.syntax().preorder_of_kinds(RETURN_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        preorder.skip_subtree();
        if let Some(statement) = JsReturnStatement::cast(node) {
            match statement.argument() {
                Some(argument) => arguments.push(argument),
                None => returns_undefined = true,
            }
        }
    }
    if arguments.is_empty() {
        return BooleanState::NonBoolean;
    }
    let state = expressions_state(arguments, model);
    if returns_undefined {
        state.either(BooleanState::NonBoolean)
    } else {
        state
    }
}

/// Returns the state of a value that can be any of `expressions`.
///
/// Expressions that choose between values, such as `a && b` or `a ? b : c`,
/// and expressions that pass a value through, such as `(a)` or `await a`, are
/// replaced by the expressions they choose from. A reference to a variable
/// or parameter that is never reassigned is replaced by its value, up to
/// [MAX_RESOLUTION_DEPTH] references deep.
fn expressions_state(expressions: Vec<AnyJsExpression>, model: &SemanticModel) -> BooleanState {
    // Each pending expression comes with the number of references followed
    // to reach it.
    let mut pending: Vec<_> = expressions
        .into_iter()
        .map(|expression| (expression, 0))
        .collect();
    let mut result: Option<BooleanState> = None;
    while let Some((expression, depth)) = pending.pop() {
        let state = match expression {
            AnyJsExpression::AnyJsLiteralExpression(literal) => match literal {
                AnyJsLiteralExpression::JsBooleanLiteralExpression(_) => BooleanState::Boolean,
                AnyJsLiteralExpression::JsNullLiteralExpression(_) => BooleanState::Unknown,
                _ => BooleanState::NonBoolean,
            },
            AnyJsExpression::JsUnaryExpression(unary) => match unary.operator() {
                Ok(JsUnaryOperator::LogicalNot | JsUnaryOperator::Delete) => BooleanState::Boolean,
                Ok(JsUnaryOperator::Void) | Err(_) => BooleanState::Unknown,
                Ok(_) => BooleanState::NonBoolean,
            },
            AnyJsExpression::JsBinaryExpression(binary) => {
                if binary.is_comparison_operator() {
                    BooleanState::Boolean
                } else {
                    BooleanState::NonBoolean
                }
            }
            AnyJsExpression::JsInExpression(_) | AnyJsExpression::JsInstanceofExpression(_) => {
                BooleanState::Boolean
            }
            // `a && b`, `a || b`, and `a ?? b` are either `a` or `b`.
            AnyJsExpression::JsLogicalExpression(logical) => {
                let (Ok(left), Ok(right)) = (logical.left(), logical.right()) else {
                    return BooleanState::Unknown;
                };
                pending.push((left, depth));
                pending.push((right, depth));
                continue;
            }
            AnyJsExpression::JsConditionalExpression(conditional) => {
                let (Ok(consequent), Ok(alternate)) =
                    (conditional.consequent(), conditional.alternate())
                else {
                    return BooleanState::Unknown;
                };
                pending.push((consequent, depth));
                pending.push((alternate, depth));
                continue;
            }
            AnyJsExpression::JsAssignmentExpression(assignment) => {
                match (assignment.operator(), assignment.right()) {
                    (Ok(JsAssignmentOperator::Assign), Ok(right)) => {
                        pending.push((right, depth));
                        continue;
                    }
                    _ => BooleanState::Unknown,
                }
            }
            AnyJsExpression::JsParenthesizedExpression(expression) => {
                let Ok(expression) = expression.expression() else {
                    return BooleanState::Unknown;
                };
                pending.push((expression, depth));
                continue;
            }
            AnyJsExpression::JsSequenceExpression(sequence) => {
                let Ok(right) = sequence.right() else {
                    return BooleanState::Unknown;
                };
                pending.push((right, depth));
                continue;
            }
            AnyJsExpression::JsAwaitExpression(expression) => {
                let Ok(argument) = expression.argument() else {
                    return BooleanState::Unknown;
                };
                pending.push((argument, depth));
                continue;
            }
            AnyJsExpression::TsNonNullAssertionExpression(expression) => {
                let Ok(expression) = expression.expression() else {
                    return BooleanState::Unknown;
                };
                pending.push((expression, depth));
                continue;
            }
            // `value as T` and `<T>value` have the type `T`, when `T` decides
            // the state.
            AnyJsExpression::TsAsExpression(expression) => {
                let (Ok(ty), Ok(expression)) = (expression.ty(), expression.expression()) else {
                    return BooleanState::Unknown;
                };
                match type_state(ty, false) {
                    BooleanState::Unknown => {
                        pending.push((expression, depth));
                        continue;
                    }
                    state => state,
                }
            }
            AnyJsExpression::TsTypeAssertionExpression(expression) => {
                let (Ok(ty), Ok(expression)) = (expression.ty(), expression.expression()) else {
                    return BooleanState::Unknown;
                };
                match type_state(ty, false) {
                    BooleanState::Unknown => {
                        pending.push((expression, depth));
                        continue;
                    }
                    state => state,
                }
            }
            // `value satisfies T` doesn't change the type of `value`, but
            // proves that `value` is a boolean when every value of `T` is one.
            AnyJsExpression::TsSatisfiesExpression(expression) => {
                let (Ok(ty), Ok(expression)) = (expression.ty(), expression.expression()) else {
                    return BooleanState::Unknown;
                };
                if type_state(ty, false) == BooleanState::Boolean {
                    BooleanState::Boolean
                } else {
                    pending.push((expression, depth));
                    continue;
                }
            }
            AnyJsExpression::JsCallExpression(call) => call_state(&call, model),
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                if depth >= MAX_RESOLUTION_DEPTH {
                    return BooleanState::Unknown;
                }
                match referenced_value(&identifier, model) {
                    ReferencedValue::Expression(expression) => {
                        pending.push((expression, depth + 1));
                        continue;
                    }
                    ReferencedValue::Type(ty) => type_state(ty, false),
                    ReferencedValue::Unknown => BooleanState::Unknown,
                }
            }
            AnyJsExpression::JsArrayExpression(_)
            | AnyJsExpression::JsArrowFunctionExpression(_)
            | AnyJsExpression::JsClassExpression(_)
            | AnyJsExpression::JsFunctionExpression(_)
            | AnyJsExpression::JsImportCallExpression(_)
            | AnyJsExpression::JsNewExpression(_)
            | AnyJsExpression::JsObjectExpression(_)
            | AnyJsExpression::JsPostUpdateExpression(_)
            | AnyJsExpression::JsPreUpdateExpression(_)
            | AnyJsExpression::JsxTagExpression(_) => BooleanState::NonBoolean,
            // A tagged template, such as ``tag`text` ``, calls a function that
            // can return any value.
            AnyJsExpression::JsTemplateExpression(template) => {
                if template.tag().is_some() {
                    BooleanState::Unknown
                } else {
                    BooleanState::NonBoolean
                }
            }
            AnyJsExpression::JsBogusExpression(_)
            | AnyJsExpression::JsComputedMemberExpression(_)
            | AnyJsExpression::JsImportMetaExpression(_)
            | AnyJsExpression::JsMetavariable(_)
            | AnyJsExpression::JsNewTargetExpression(_)
            | AnyJsExpression::JsStaticMemberExpression(_)
            | AnyJsExpression::JsSuperExpression(_)
            | AnyJsExpression::JsThisExpression(_)
            | AnyJsExpression::JsYieldExpression(_)
            | AnyJsExpression::TsInstantiationExpression(_) => BooleanState::Unknown,
        };
        let combined = result.map_or(state, |result| result.either(state));
        if combined == BooleanState::Unknown {
            return combined;
        }
        result = Some(combined);
    }
    result.unwrap_or(BooleanState::Unknown)
}

/// Returns the state of the value of `call`, for calls to functions and
/// methods that are known to return booleans.
fn call_state(call: &JsCallExpression, model: &SemanticModel) -> BooleanState {
    let Ok(callee) = call.callee() else {
        return BooleanState::Unknown;
    };
    let callee = callee.omit_parentheses();
    if let Some((reference, name)) = callee
        .as_any_global_identifier_expression()
        .and_then(|callee| global_identifier(&callee))
        && matches!(name.text(), "Boolean" | "isFinite" | "isNaN")
        && model.binding(&reference).is_none()
    {
        return BooleanState::Boolean;
    }
    let Some(member) = AnyJsMemberExpression::cast(callee.into_syntax()) else {
        return BooleanState::Unknown;
    };
    let (Some(method), Ok(object)) = (member.member_name(), member.object()) else {
        return BooleanState::Unknown;
    };
    let method = method.text();
    let object = object.omit_parentheses();
    if BOOLEAN_INSTANCE_METHODS.contains(&method)
        || (method == "test"
            && matches!(
                object,
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsRegexLiteralExpression(_)
                )
            ))
    {
        return BooleanState::Boolean;
    }
    if let Some((reference, name)) = object
        .as_any_global_identifier_expression()
        .and_then(|object| global_identifier(&object))
        && BOOLEAN_STATIC_METHODS
            .binary_search_by(|(object, static_method)| {
                (*object, *static_method).cmp(&(name.text(), method))
            })
            .is_ok()
        && model.binding(&reference).is_none()
    {
        return BooleanState::Boolean;
    }
    BooleanState::Unknown
}

/// What a reference to a variable or parameter is known to hold.
enum ReferencedValue {
    /// The value of this expression.
    Expression(AnyJsExpression),
    /// A value of this type.
    Type(AnyTsType),
    /// Nothing known, or a function, whose value is the function itself
    /// rather than what it returns.
    Unknown,
}

/// Returns what `identifier` holds, when it refers to a variable or a
/// parameter that is never reassigned.
fn referenced_value(identifier: &JsIdentifierExpression, model: &SemanticModel) -> ReferencedValue {
    let Some(binding) = identifier
        .name()
        .ok()
        .and_then(|reference| model.binding(&reference))
    else {
        return ReferencedValue::Unknown;
    };
    if binding.all_writes().next().is_some() {
        return ReferencedValue::Unknown;
    }
    let AnyJsIdentifierBinding::JsIdentifierBinding(declared) = binding.tree() else {
        return ReferencedValue::Unknown;
    };
    let Some(parent) = declared.syntax().parent() else {
        return ReferencedValue::Unknown;
    };
    let (annotation, initializer) =
        if let Some(declarator) = JsVariableDeclarator::cast_ref(&parent) {
            (
                declarator
                    .variable_annotation()
                    .and_then(|annotation| annotation.type_annotation().ok().flatten()),
                declarator.initializer(),
            )
        } else if let Some(parameter) = JsFormalParameter::cast_ref(&parent) {
            (parameter.type_annotation(), parameter.initializer())
        } else {
            return ReferencedValue::Unknown;
        };
    if let Some(annotation) = annotation {
        return match annotation.ty() {
            Ok(ty) if function_type(&ty).is_none() => ReferencedValue::Type(ty),
            _ => ReferencedValue::Unknown,
        };
    }
    match initializer.and_then(|initializer| initializer.expression().ok()) {
        Some(expression) if !is_function_expression(&expression) => {
            ReferencedValue::Expression(expression)
        }
        _ => ReferencedValue::Unknown,
    }
}

/// Returns whether `expression` is an arrow function or a function
/// expression, possibly in parentheses.
fn is_function_expression(expression: &AnyJsExpression) -> bool {
    matches!(
        expression.clone().omit_parentheses(),
        AnyJsExpression::JsArrowFunctionExpression(_) | AnyJsExpression::JsFunctionExpression(_)
    )
}

/// Returns whether `name` is a React hook name: `use` followed by an
/// uppercase letter or a digit, such as `useIsReady` or `use2FA`.
fn is_hook_name(name: &str) -> bool {
    name.strip_prefix(HOOK_PREFIX)
        .and_then(|rest| rest.bytes().next())
        .is_some_and(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
}

/// Returns whether `name` starts with `prefix` as a separate word: `isReady`,
/// `is_ready`, `is2FA`, and `IS_READY` start with `is`, but `island` and
/// `ISREADY` don't. When `capitalized` is `true`, the first letter of the
/// prefix may also be uppercase, as in the `IsReady` of `useIsReady`.
fn starts_with_prefix(name: &str, prefix: &str, capitalized: bool) -> bool {
    let (name, prefix) = (name.as_bytes(), prefix.as_bytes());
    let (Some(&first), Some(&next)) = (prefix.first(), name.get(prefix.len())) else {
        return false;
    };
    if next == b'_'
        && name
            .iter()
            .zip(prefix)
            .all(|(name, prefix)| *name == prefix.to_ascii_uppercase())
    {
        return true;
    }
    let first_matches = name[0] == first || (capitalized && name[0] == first.to_ascii_uppercase());
    first_matches
        && name[1..prefix.len()] == prefix[1..]
        && matches!(next, b'0'..=b'9' | b'A'..=b'Z' | b'_')
}

#[cfg(test)]
mod tests {
    use super::BOOLEAN_STATIC_METHODS;

    #[test]
    fn boolean_static_methods_are_sorted() {
        for window in BOOLEAN_STATIC_METHODS.windows(2) {
            assert!(
                window[0] < window[1],
                "{:?} must come before {:?}",
                window[1],
                window[0]
            );
        }
    }
}
