use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsObjectMember, AnyTsTupleTypeElement, AnyTsType,
    JsArrowFunctionExpression, JsCallExpression, JsComputedMemberExpression,
    JsConstructorClassMember, JsFormalParameter, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsFunctionExpression, JsGetterClassMember,
    JsGetterObjectMember, JsIdentifierExpression, JsInstanceofExpression, JsLanguage,
    JsMethodClassMember, JsMethodObjectMember, JsNewExpression, JsObjectExpression,
    JsReferenceIdentifier, JsSetterClassMember, JsSetterObjectMember,
    JsShorthandPropertyObjectMember, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
    JsVariableDeclarator, T,
};
use biome_languages::JsFileSource;
use biome_languages::javascript::JsEmbeddingKind;
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, Direction, SyntaxKindSet, TextRange, TokenText,
    TriviaPieceKind, WalkEvent, declare_node_union,
};
use biome_rule_options::use_tanstack_query_exhaustive_dependencies::UseTanstackQueryExhaustiveDependenciesOptions;

use crate::JsRuleAction;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require query keys to include every value that the query function uses.
    ///
    /// TanStack Query saves the data returned by a query function under the query key, and
    /// fetches the data again when the key changes. When the query function uses a value that
    /// the key doesn't include, changing that value doesn't fetch new data. The query keeps
    /// returning the data that belongs to the old value, and queries that use different values
    /// share the same saved data.
    ///
    /// This rule checks objects that have both a `queryKey` and a `queryFn` property, such as
    /// the options passed to `useQuery` or `queryOptions`. It reports the variables that the
    /// query function reads but the query key doesn't include.
    ///
    /// The rule only checks variables declared inside the function that contains the query
    /// options, such as a component, a hook, or a function that creates query options. It
    /// doesn't report:
    ///
    /// - variables declared outside that function, such as imports and top-level constants;
    /// - functions that the query function calls, such as `fetchTodo` in `fetchTodo(id)`;
    /// - objects whose methods the query function calls, such as `api` in `api.getTodo(id)`;
    /// - classes used with `new` or `instanceof`.
    ///
    /// When a property of a variable is in the query key, such as `todo.id`, the query function
    /// can read that property. When the whole variable is in the query key, the query function
    /// can read any of its properties.
    ///
    /// In Vue files, the rule also checks variables declared at the top level of the script,
    /// except imports.
    ///
    /// When `queryFn` chooses between a function and `skipToken`, such as
    /// `enabled ? () => fetchTodo(id) : skipToken`, the rule only checks the function.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { useQuery } from "@tanstack/react-query";
    ///
    /// function Todo({ todoId }) {
    ///     const query = useQuery({
    ///         queryKey: ["todo"],
    ///         queryFn: () => fetchTodo(todoId),
    ///     });
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const todoQueries = {
    ///     detail: (id) => ({ queryKey: ["todo"], queryFn: () => fetchTodo(id) }),
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { useQuery } from "@tanstack/react-query";
    ///
    /// function Todo({ todoId }) {
    ///     const todos = useTodos();
    ///     const query = useQuery({
    ///         queryKey: ["todo", todoId],
    ///         queryFn: () => todos.getTodo(todoId),
    ///     });
    /// }
    ///
    /// const todoQueries = {
    ///     detail: (id) => ({ queryKey: ["todo", id], queryFn: () => fetchTodo(id) }),
    /// };
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignoredVariables`
    ///
    /// Names of variables that the query function can use without adding them to the query key.
    /// The query function can also read any property of these variables.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoredVariables": ["config"]
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// function useTodo(todoId, config) {
    ///     return useQuery({
    ///         queryKey: ["todo", todoId],
    ///         queryFn: () => fetchTodo(todoId, config.baseUrl),
    ///     });
    /// }
    /// ```
    ///
    /// ### `ignoredTypes`
    ///
    /// Names of TypeScript types. The query function can use a variable or parameter declared
    /// with one of these types without adding it to the query key. The rule reads the type
    /// written after the name of the variable or parameter, such as `config: Config`. It also
    /// finds the type inside unions, intersections, arrays, and tuples, such as
    /// `config: Config | undefined`.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoredTypes": ["Config"]
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options
    /// function useTodo(todoId: string, config: Config) {
    ///     return useQuery({
    ///         queryKey: ["todo", todoId],
    ///         queryFn: () => fetchTodo(todoId, config.baseUrl),
    ///     });
    /// }
    /// ```
    ///
    pub UseTanstackQueryExhaustiveDependencies {
        version: "next",
        name: "useTanstackQueryExhaustiveDependencies",
        language: "js",
        sources: &[RuleSource::EslintTanstackQuery("exhaustive-deps").same()],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::TanstackQuery],
        fix_kind: FixKind::Unsafe,
    }
}

/// The values that a query function reads but its query key doesn't include.
pub struct MissingDependencies {
    /// The `queryKey` member of the query options object.
    query_key_member: AnyJsObjectMember,
    /// The value of the query key after following variables and type assertions. For example,
    /// the value of `queryKey: key` is `["todo"]` when `key` is declared as
    /// `const key = ["todo"] as const`.
    query_key: AnyJsExpression,
    /// The missing values, in the order in which the query function first reads them.
    dependencies: Box<[Dependency]>,
}

/// A value that a query function reads.
pub struct Dependency {
    /// The expression that reads the value, such as `todo.id`.
    path: AnyDependencyPath,
    /// The name of the variable at the start of [`Self::path`], such as `todo`.
    root: TokenText,
    /// The variable at the start of [`Self::path`].
    binding: Binding,
}

declare_node_union! {
    /// An expression that reads a dependency: a variable, or a chain of property reads that
    /// starts with a variable, such as `a.b[c]`. A shorthand property such as `{ id }` is
    /// represented by its identifier.
    pub AnyDependencyPath = AnyJsExpression | JsReferenceIdentifier
}

impl Rule for UseTanstackQueryExhaustiveDependencies {
    type Query = Semantic<JsObjectExpression>;
    type State = MissingDependencies;
    type Signals = Option<Self::State>;
    type Options = UseTanstackQueryExhaustiveDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let object = ctx.query();
        let model = ctx.model();
        let options = ctx.options();

        let (query_key_member, query_fn_member) = find_query_members(object)?;
        let query_functions = query_functions(&query_fn_member);
        if query_functions.iter().all(Option::is_none) {
            return None;
        }

        let owner = match object
            .syntax()
            .ancestors()
            .skip(1)
            .filter_map(AnyOwnerFunction::cast)
            .last()
        {
            Some(function) => DependencyOwner::Function(function),
            None if matches!(
                ctx.source_type::<JsFileSource>().as_embedding_kind(),
                JsEmbeddingKind::Vue { .. }
            ) =>
            {
                DependencyOwner::VueScript
            }
            None => return None,
        };

        let mut required = Vec::new();
        for function in query_functions.iter().flatten() {
            collect_query_function_dependencies(function, &owner, model, &mut required);
        }
        if required.is_empty() {
            return None;
        }

        let query_key = match &query_key_member {
            AnyJsObjectMember::JsPropertyObjectMember(member) => member.value().ok()?,
            // A shorthand `{ queryKey }` can only be checked when the variable's value is known.
            AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => {
                variable_initializer(&member.name().ok()?, model)?
            }
            _ => return None,
        };
        let query_key = dereference_query_key(query_key, model);
        let key_dependencies = QueryKeyDependencies::from_query_key(&query_key, model);

        let mut missing: Vec<Dependency> = Vec::new();
        for dependency in required {
            let root = dependency.root.text();
            let is_covered = key_dependencies
                .roots
                .iter()
                .any(|key_root| key_root.text() == root)
                || options
                    .ignored_variables()
                    .iter()
                    .any(|name| name.as_ref() == root)
                || key_dependencies
                    .paths
                    .iter()
                    .any(|path| is_same_path(path.syntax(), dependency.path.syntax()))
                || missing
                    .iter()
                    .any(|other| is_same_path(other.path.syntax(), dependency.path.syntax()))
                || is_ignored_by_type(&dependency.binding, options.ignored_types());
            if !is_covered {
                missing.push(dependency);
            }
        }

        // When a whole variable is missing, reporting its properties as well is redundant.
        let missing_roots: Vec<_> = missing
            .iter()
            .filter(|dependency| dependency.path.is_root())
            .map(|dependency| dependency.root.clone())
            .collect();
        let dependencies: Box<[_]> = missing
            .into_iter()
            .filter(|dependency| {
                dependency.path.is_root()
                    || !missing_roots
                        .iter()
                        .any(|root| root.text() == dependency.root.text())
            })
            .collect();

        if dependencies.is_empty() {
            return None;
        }

        Some(MissingDependencies {
            query_key_member,
            query_key,
            dependencies,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.query_key_member.range(),
            markup! {
                "This query key doesn't include all the values that the query function uses."
            },
        );
        for dependency in &state.dependencies {
            diagnostic = diagnostic.detail(
                dependency.path.range(),
                markup! {
                    "The query function uses "<Emphasis>{PathText(&dependency.path)}</Emphasis>" here, but the query key doesn't include it."
                },
            );
        }

        Some(
            diagnostic
                .note(markup! {
                    "TanStack Query saves the result of a query under its key, and fetches the data again when the key changes. When one of these values changes, the query doesn't fetch new data and keeps returning the data that belongs to the old value."
                })
                .note(markup! {
                    "Add the missing values to the query key."
                }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let array = state.query_key.as_js_array_expression()?;

        // The query function runs later, but the key is evaluated where it's written. A variable
        // that the key would read must already be declared there, and be visible there. This
        // matters when the key is declared away from the query options, or when the variable is
        // declared after them.
        let model = ctx.model();
        let array_range = array.range();
        let is_readable_at_key = state.dependencies.iter().all(|dependency| {
            dependency
                .path
                .syntax()
                .preorder_of_kinds(JsReferenceIdentifier::KIND_SET)
                .filter_map(|event| match event {
                    WalkEvent::Enter(node) => JsReferenceIdentifier::cast(node),
                    WalkEvent::Leave(_) => None,
                })
                .all(|reference| {
                    model.binding(&reference).is_none_or(|binding| {
                        binding.range().start() < array_range.start()
                            && binding.scope().range().contains_range(array_range)
                    })
                })
        });
        if !is_readable_at_key {
            return None;
        }

        let elements = array.elements();
        let mut items = Vec::with_capacity(elements.len() + state.dependencies.len());
        let mut separators = Vec::with_capacity(elements.len() + state.dependencies.len());
        for element in elements.elements() {
            items.push(element.node().ok()?.clone());
            if let Some(separator) = element.trailing_separator().ok()? {
                separators.push(separator.clone());
            }
        }

        let trailing_separator = if !items.is_empty() && separators.len() == items.len() {
            separators.pop()
        } else {
            None
        };
        // Without a trailing comma, the new values follow the last element directly, and a
        // line comment after that element would comment them out.
        if trailing_separator.is_none()
            && items
                .last()
                .is_some_and(|item| item.syntax().has_trailing_comments())
        {
            return None;
        }

        for dependency in &state.dependencies {
            if !items.is_empty() {
                separators.push(
                    make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                );
            }
            let expression = match &dependency.path {
                AnyDependencyPath::AnyJsExpression(expression) => expression.clone().trim_trivia()?,
                AnyDependencyPath::JsReferenceIdentifier(reference) => {
                    make::js_identifier_expression(reference.clone().trim_trivia()?).into()
                }
            };
            items.push(AnyJsArrayElement::AnyJsExpression(expression));
        }
        separators.extend(trailing_separator);

        let mut mutation = ctx.root().begin();
        mutation.replace_node(elements, make::js_array_element_list(items, separators));

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add the missing values to the query key." }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    /// A query function written inline in the query options.
    pub AnyQueryFunction = JsArrowFunctionExpression | JsFunctionExpression | JsMethodObjectMember
}

declare_node_union! {
    /// A function that can contain query options, such as a component or a hook.
    pub AnyOwnerFunction =
        JsFunctionDeclaration
        | JsFunctionExportDefaultDeclaration
        | JsFunctionExpression
        | JsArrowFunctionExpression
        | JsMethodObjectMember
        | JsGetterObjectMember
        | JsSetterObjectMember
        | JsMethodClassMember
        | JsGetterClassMember
        | JsSetterClassMember
        | JsConstructorClassMember
}

/// Where a variable must be declared for the query key to need it.
enum DependencyOwner {
    /// The outermost function that contains the query options.
    Function(AnyOwnerFunction),
    /// The script of a Vue file, when no function contains the query options.
    VueScript,
}

impl DependencyOwner {
    /// Returns whether `binding` is declared by the owner of the query options.
    fn declares(&self, binding: &Binding) -> bool {
        match self {
            Self::Function(function) => {
                function.range().contains_range(binding.range())
                    // The name of a function declaration belongs to the enclosing scope.
                    && binding.syntax().parent().as_ref() != Some(function.syntax())
            }
            Self::VueScript => !binding.is_imported(),
        }
    }
}

/// The variables and property paths that a query key includes.
struct QueryKeyDependencies {
    /// Variables that the query key includes as a whole, including all of their properties.
    roots: Vec<TokenText>,
    /// Expressions that the query key includes, which don't include every property of their
    /// variable, such as `todo.id`.
    paths: Vec<AnyJsExpression>,
}

impl QueryKeyDependencies {
    /// Collects the variables that `query_key` reads.
    ///
    /// Inside a function in the query key, such as `ids.map((id) => id)`, only variables
    /// declared outside the function count.
    fn from_query_key(query_key: &AnyJsExpression, model: &SemanticModel) -> Self {
        let mut dependencies = Self {
            roots: Vec::new(),
            paths: Vec::new(),
        };
        let mut function: Option<JsSyntaxNode> = None;

        for event in query_key.syntax().preorder_of_kinds(QUERY_KEY_SEARCH_KINDS) {
            let node = match event {
                WalkEvent::Enter(node) => node,
                WalkEvent::Leave(node) => {
                    if function.as_ref() == Some(&node) {
                        function = None;
                    }
                    continue;
                }
            };
            let Some(reference) = JsReferenceIdentifier::cast_ref(&node) else {
                function.get_or_insert(node);
                continue;
            };
            if let Some(function) = &function
                && !model.binding(&reference).is_some_and(|binding| {
                    !function.text_trimmed_range().contains_range(binding.range())
                })
            {
                continue;
            }
            let Ok(name) = reference.value_token() else {
                continue;
            };
            let name = name.token_text_trimmed();
            let Some(parent) = node.parent() else {
                continue;
            };

            if JsShorthandPropertyObjectMember::can_cast(parent.kind()) {
                dependencies.roots.push(name);
                continue;
            }
            let Some(expression) = JsIdentifierExpression::cast(parent) else {
                continue;
            };
            let expression = AnyJsExpression::from(expression);
            let chain = member_chain(expression.clone(), |_| false);

            if !is_callee(&chain) {
                if chain == expression {
                    dependencies.roots.push(name);
                } else {
                    dependencies.paths.push(chain);
                }
                continue;
            }

            // A method call, such as `api.key()`, uses the object that the method is read from.
            let object = match &chain {
                AnyJsExpression::JsStaticMemberExpression(member) => member.object().ok(),
                AnyJsExpression::JsComputedMemberExpression(member) => member.object().ok(),
                _ => None,
            };
            match object {
                Some(object)
                    if object
                        .clone()
                        .omit_type_wrappers()
                        .is_some_and(|object| object == expression) =>
                {
                    dependencies.roots.push(name);
                }
                Some(object) => dependencies.paths.push(object),
                // A function call, such as `key()`, uses the function but not its properties.
                None => dependencies.paths.push(chain),
            }
        }

        dependencies
    }
}

impl AnyDependencyPath {
    /// Returns whether the path is a variable on its own, without property reads.
    fn is_root(&self) -> bool {
        match self {
            Self::AnyJsExpression(expression) => {
                matches!(expression, AnyJsExpression::JsIdentifierExpression(_))
            }
            Self::JsReferenceIdentifier(_) => true,
        }
    }
}

/// Writes the source text of a dependency path, without comments or whitespace.
struct PathText<'a>(&'a AnyDependencyPath);

impl biome_console::fmt::Display for PathText<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        for token in self.0.syntax().descendants_tokens(Direction::Next) {
            fmt.write_str(token.text_trimmed())?;
        }
        Ok(())
    }
}

/// Returns the `queryKey` and `queryFn` members of `object`, if it has both.
fn find_query_members(object: &JsObjectExpression) -> Option<(AnyJsObjectMember, AnyJsObjectMember)> {
    let mut query_key = None;
    let mut query_fn = None;
    for member in object.members().iter().flatten() {
        let Some(name) = member.name() else {
            continue;
        };
        match name.text() {
            "queryKey" if query_key.is_none() => query_key = Some(member),
            "queryFn" if query_fn.is_none() => query_fn = Some(member),
            _ => {}
        }
    }
    Some((query_key?, query_fn?))
}

/// Returns the functions that the `queryFn` member can run.
///
/// A conditional expression such as `enabled ? () => fetchTodo(id) : skipToken` can run the
/// function in either branch, except a branch that is `skipToken`.
fn query_functions(member: &AnyJsObjectMember) -> [Option<AnyQueryFunction>; 2] {
    let value = match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => {
            return [Some(method.clone().into()), None];
        }
        AnyJsObjectMember::JsPropertyObjectMember(property) => property.value(),
        _ => return [None, None],
    };
    let Ok(value) = value else {
        return [None, None];
    };
    let conditional = match value.omit_parentheses() {
        AnyJsExpression::JsConditionalExpression(conditional) => conditional,
        value => return [as_query_function(value), None],
    };

    let consequent = conditional.consequent().ok();
    let alternate = conditional.alternate().ok();
    if consequent.as_ref().is_some_and(is_skip_token) {
        return [alternate.and_then(as_query_function), None];
    }
    if alternate.as_ref().is_some_and(is_skip_token) {
        return [consequent.and_then(as_query_function), None];
    }
    [
        consequent.and_then(as_query_function),
        alternate.and_then(as_query_function),
    ]
}

fn as_query_function(expression: AnyJsExpression) -> Option<AnyQueryFunction> {
    match expression.omit_parentheses() {
        AnyJsExpression::JsArrowFunctionExpression(function) => Some(function.into()),
        AnyJsExpression::JsFunctionExpression(function) => Some(function.into()),
        _ => None,
    }
}

fn is_skip_token(expression: &AnyJsExpression) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_js_identifier_expression()
        .and_then(|identifier| identifier.name().ok())
        .is_some_and(|name| name.has_name("skipToken"))
}

/// Collects the variables that `function` reads and that the query key must include, in source
/// order.
fn collect_query_function_dependencies(
    function: &AnyQueryFunction,
    owner: &DependencyOwner,
    model: &SemanticModel,
    dependencies: &mut Vec<Dependency>,
) {
    let function_range = function.range();
    for event in function
        .syntax()
        .preorder_of_kinds(JsReferenceIdentifier::KIND_SET)
    {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        let reference = JsReferenceIdentifier::unwrap_cast(node);
        if reference.is_undefined() {
            continue;
        }
        let Some(binding) = model.binding(&reference) else {
            continue;
        };
        if function_range.contains_range(binding.range()) || !owner.declares(&binding) {
            continue;
        }
        let Ok(root) = reference.value_token() else {
            continue;
        };
        let Some(parent) = reference.syntax().parent() else {
            continue;
        };

        let path = if let Some(expression) = JsIdentifierExpression::cast_ref(&parent) {
            let expression = AnyJsExpression::from(expression);
            if is_instanceof_operand(&expression) {
                continue;
            }
            let chain = member_chain(expression.clone(), |_| false);
            if is_callee(&chain) || is_computed_member_property(&chain) {
                continue;
            }
            // The query key can't read a variable of the query function, such as `pageParam`
            // in `cursors[pageParam]`, so the path stops before that property read.
            AnyDependencyPath::from(member_chain(expression, |member| {
                member.member().is_ok_and(|property| {
                    reads_variable_declared_in(&property, function_range, model)
                })
            }))
        } else if JsShorthandPropertyObjectMember::can_cast(parent.kind()) {
            AnyDependencyPath::from(reference)
        } else {
            // Type references and other names that don't read a value.
            continue;
        };

        dependencies.push(Dependency {
            path,
            root: root.token_text_trimmed(),
            binding,
        });
    }
}

/// Returns the longest chain of property reads that starts with `expression`, such as `a.b[c]`
/// for `a`. Non-null assertions are part of the chain.
///
/// The chain ends before the first computed property read for which `stop_before` returns
/// `true`.
fn member_chain(
    expression: AnyJsExpression,
    stop_before: impl Fn(&JsComputedMemberExpression) -> bool,
) -> AnyJsExpression {
    let mut chain = expression;
    while let Some(parent) = chain.syntax().parent().and_then(AnyJsExpression::cast) {
        let extends_chain = match &parent {
            AnyJsExpression::JsStaticMemberExpression(member) => {
                member.object().is_ok_and(|object| object == chain)
            }
            AnyJsExpression::JsComputedMemberExpression(member) => {
                member.object().is_ok_and(|object| object == chain) && !stop_before(member)
            }
            AnyJsExpression::TsNonNullAssertionExpression(_) => true,
            _ => false,
        };
        if !extends_chain {
            break;
        }
        chain = parent;
    }
    chain
}

/// Returns whether `expression` reads a variable declared within `range`.
fn reads_variable_declared_in(
    expression: &AnyJsExpression,
    range: TextRange,
    model: &SemanticModel,
) -> bool {
    expression
        .syntax()
        .preorder_of_kinds(JsReferenceIdentifier::KIND_SET)
        .any(|event| match event {
            WalkEvent::Enter(node) => model
                .binding(&JsReferenceIdentifier::unwrap_cast(node))
                .is_some_and(|binding| range.contains_range(binding.range())),
            WalkEvent::Leave(_) => false,
        })
}

/// Returns whether `expression` is the function called by a call or `new` expression.
fn is_callee(expression: &AnyJsExpression) -> bool {
    let Some(parent) = expression.syntax().parent() else {
        return false;
    };
    if let Some(call) = JsCallExpression::cast_ref(&parent) {
        return call.callee().is_ok_and(|callee| &callee == expression);
    }
    JsNewExpression::cast(parent)
        .is_some_and(|new| new.callee().is_ok_and(|callee| &callee == expression))
}

/// Returns whether `expression` is the property in a computed property read, such as `key` in
/// `map[key]`. The value read from `map[key]` already depends on `key`.
///
/// A computed method call, such as `client[method]()`, is an exception: the object is not a
/// dependency because its method is called, so `method` is.
fn is_computed_member_property(expression: &AnyJsExpression) -> bool {
    expression
        .syntax()
        .parent()
        .and_then(JsComputedMemberExpression::cast)
        .is_some_and(|member| {
            member
                .member()
                .is_ok_and(|property| &property == expression)
                && !is_callee(&member.into())
        })
}

/// Returns whether `expression` is an operand of `instanceof`, such as `value` or `Date` in
/// `value instanceof Date`.
fn is_instanceof_operand(expression: &AnyJsExpression) -> bool {
    expression
        .outer_expression()
        .and_then(|outer| outer.syntax().parent())
        .is_some_and(|parent| JsInstanceofExpression::can_cast(parent.kind()))
}

/// Follows variables and type assertions from `expression` to the value of the query key.
fn dereference_query_key(expression: AnyJsExpression, model: &SemanticModel) -> AnyJsExpression {
    let mut expression = expression;
    // Bounds the search when variables refer to each other in a cycle.
    for _ in 0..MAX_DEREFERENCE_STEPS {
        let Some(inner) = expression.clone().omit_type_wrappers() else {
            break;
        };
        expression = inner;
        let Some(initializer) = expression
            .as_js_identifier_expression()
            .and_then(|identifier| identifier.name().ok())
            .and_then(|reference| variable_initializer(&reference, model))
        else {
            break;
        };
        expression = initializer;
    }
    expression
}

/// Returns the initial value of the variable that `reference` refers to, such as `["todo"]`
/// for `const key = ["todo"]`.
fn variable_initializer(
    reference: &JsReferenceIdentifier,
    model: &SemanticModel,
) -> Option<AnyJsExpression> {
    model
        .binding(reference)?
        .tree()
        .parent::<JsVariableDeclarator>()?
        .initializer()?
        .expression()
        .ok()
}

/// Returns whether `binding` is a parameter or variable whose type annotation mentions one of
/// `ignored_types`.
fn is_ignored_by_type(binding: &Binding, ignored_types: &[Box<str>]) -> bool {
    if ignored_types.is_empty() {
        return false;
    }
    let Some(parent) = binding.syntax().parent() else {
        return false;
    };
    let annotation = if let Some(parameter) = JsFormalParameter::cast_ref(&parent) {
        parameter.type_annotation()
    } else if let Some(declarator) = JsVariableDeclarator::cast(parent) {
        declarator
            .variable_annotation()
            .and_then(|annotation| annotation.type_annotation().ok().flatten())
    } else {
        None
    };

    annotation
        .and_then(|annotation| annotation.ty().ok())
        .is_some_and(|ty| mentions_ignored_type(ty, ignored_types))
}

/// Returns whether `ty` is one of `ignored_types`, or a union, intersection, array, or tuple
/// that contains one of them.
fn mentions_ignored_type(ty: AnyTsType, ignored_types: &[Box<str>]) -> bool {
    let mut pending = vec![ty];
    while let Some(ty) = pending.pop() {
        match ty {
            AnyTsType::TsReferenceType(reference) => {
                let is_ignored = reference
                    .name()
                    .ok()
                    .and_then(|name| name.as_js_reference_identifier()?.value_token().ok())
                    .is_some_and(|name| {
                        ignored_types
                            .iter()
                            .any(|ignored| ignored.as_ref() == name.text_trimmed())
                    });
                if is_ignored {
                    return true;
                }
            }
            AnyTsType::TsUnionType(union) => pending.extend(union.types().iter().flatten()),
            AnyTsType::TsIntersectionType(intersection) => {
                pending.extend(intersection.types().iter().flatten());
            }
            AnyTsType::TsArrayType(array) => pending.extend(array.element_type().ok()),
            AnyTsType::TsParenthesizedType(parenthesized) => {
                pending.extend(parenthesized.ty().ok());
            }
            AnyTsType::TsTupleType(tuple) => {
                pending.extend(tuple.elements().iter().flatten().filter_map(|element| {
                    match element {
                        AnyTsTupleTypeElement::AnyTsType(ty) => Some(ty),
                        AnyTsTupleTypeElement::TsNamedTupleTypeElement(element) => {
                            element.ty().ok()
                        }
                        AnyTsTupleTypeElement::TsOptionalTupleTypeElement(element) => {
                            element.ty().ok()
                        }
                        AnyTsTupleTypeElement::TsRestTupleTypeElement(element) => {
                            element.ty().ok()
                        }
                    }
                }));
            }
            _ => {}
        }
    }
    false
}

/// Returns whether `left` and `right` read the same value. Optional chaining and non-null
/// assertions are ignored, so `a?.b` and `a!.b` match `a.b`.
fn is_same_path(left: &JsSyntaxNode, right: &JsSyntaxNode) -> bool {
    let mut left = path_tokens(left);
    let mut right = path_tokens(right);
    loop {
        match (left.next(), right.next()) {
            (None, None) => return true,
            (Some(left), Some(right)) if path_token_text(&left) == path_token_text(&right) => {}
            _ => return false,
        }
    }
}

/// Returns the tokens of `node`, without the `!` of non-null assertions.
fn path_tokens(node: &JsSyntaxNode) -> impl Iterator<Item = JsSyntaxToken> {
    node.descendants_tokens(Direction::Next).filter(|token| {
        !(token.kind() == T![!]
            && token
                .parent()
                .is_some_and(|parent| parent.kind() == JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION))
    })
}

/// Returns the text of `token`, reading `?.` as `.`.
fn path_token_text(token: &JsSyntaxToken) -> &str {
    if token.kind() == T![?.] {
        "."
    } else {
        token.text_trimmed()
    }
}

/// The number of variables that [`dereference_query_key`] follows at most.
const MAX_DEREFERENCE_STEPS: usize = 8;

/// The nodes that [`QueryKeyDependencies::from_query_key`] visits: variable references, and the
/// functions that change which references count.
const QUERY_KEY_SEARCH_KINDS: SyntaxKindSet<JsLanguage> =
    JsReferenceIdentifier::KIND_SET.union(AnyOwnerFunction::KIND_SET);
