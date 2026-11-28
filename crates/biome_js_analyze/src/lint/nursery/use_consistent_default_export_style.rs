use crate::JsRuleAction;
use crate::services::semantic::Semantic;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsExportClause, AnyJsExportDefaultDeclaration,
    AnyJsExpression, AnyJsModuleItem, AnyJsStatement, JsClassDeclaration,
    JsClassExportDefaultDeclaration, JsEmptyStatement, JsExport, JsExportDefaultDeclarationClause,
    JsExportDefaultExpressionClause, JsFunctionDeclaration, JsIdentifierBinding, JsModule,
    JsModuleItemList, JsSyntaxToken, JsSyntaxTrivia, JsVariableStatement, T, TriviaPieceKind,
    TsDeclareFunctionDeclaration,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, declare_node_union};
use biome_rule_options::use_consistent_default_export_style::{
    DefaultExportStyle, UseConsistentDefaultExportStyleOptions,
};

declare_lint_rule! {
    /// Enforce a consistent style for default-exported functions and classes.
    ///
    /// A function or class can be declared inside the `export default` statement,
    /// or declared on its own and then exported by name with `export default name;`.
    /// Using one style across a project makes default exports easier to find and read.
    ///
    /// By default, the rule requires functions and classes to be declared inside the `export default` statement.
    /// Use the `functions` and `classes` options to require the separate style instead, or to turn off the check.
    ///
    /// The rule only checks a declaration that comes directly before `export default` with its name.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo() {}
    /// export default foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = () => {};
    /// export default foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {}
    /// export default Foo;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export default function foo() {}
    ///
    /// const bar = () => {};
    /// export { bar };
    /// ```
    ///
    /// ```js
    /// export default class Foo {}
    /// ```
    ///
    /// ## Options
    ///
    /// ### `functions`
    ///
    /// Type: `"inline" | "separate" | "ignore"`
    ///
    /// Default: `"inline"`
    ///
    /// - `"inline"` requires functions to be declared inside the `export default` statement,
    ///   like `export default function foo() {}`.
    /// - `"separate"` requires functions to be declared as `const` arrow functions, then exported by name,
    ///   like `const foo = () => {};` followed by `export default foo;`.
    ///   Generator functions (`function*`) are ignored, because an arrow function can't be a generator.
    /// - `"ignore"` turns off the check for functions.
    ///
    /// The following examples use `"separate"`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "functions": "separate"
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// export default function foo() {}
    /// ```
    ///
    /// ```js,use_options,expect_diagnostic
    /// function foo() {}
    /// export default foo;
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// const foo = () => {};
    /// export default foo;
    /// ```
    ///
    /// ### `classes`
    ///
    /// Type: `"inline" | "separate" | "ignore"`
    ///
    /// Default: `"inline"`
    ///
    /// - `"inline"` requires classes to be declared inside the `export default` statement,
    ///   like `export default class Foo {}`.
    /// - `"separate"` requires classes to be declared on their own, then exported by name,
    ///   like `class Foo {}` followed by `export default Foo;`.
    /// - `"ignore"` turns off the check for classes.
    ///
    /// The following examples use `"separate"`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "classes": "separate"
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// export default class Foo {}
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// class Foo {}
    /// export default Foo;
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow default exports entirely, see [`noDefaultExport`](https://biomejs.dev/linter/rules/no-default-export/).
    /// - If you want to choose between function declarations and functions assigned to variables, see [`useConsistentFunctionStyle`](https://biomejs.dev/linter/rules/use-consistent-function-style/).
    ///
    pub UseConsistentDefaultExportStyle {
        version: "2.6.10",
        name: "useConsistentDefaultExportStyle",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("default-export-style").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyJsExportDefaultClause = JsExportDefaultDeclarationClause | JsExportDefaultExpressionClause
}

/// A default export written in a style that the options don't allow.
pub enum DefaultExportStyleViolation {
    /// A function or class declared inside `export default`, when the separate style is required.
    InlineDeclaration(DeclarationKind),
    /// A declaration followed by `export default name;`, when the inline style is required.
    SeparateDeclaration(AnyJsSeparateDeclaration),
    /// A function declaration followed by `export default name;`, when functions must be separate
    /// `const` arrow functions.
    SeparateFunctionDeclaration(JsFunctionDeclaration),
}

impl Rule for UseConsistentDefaultExportStyle {
    type Query = Semantic<AnyJsExportDefaultClause>;
    type State = DefaultExportStyleViolation;
    type Signals = Option<Self::State>;
    type Options = UseConsistentDefaultExportStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let export = JsExport::cast(ctx.query().syntax().parent()?)?;
        // Only exports at the top level of a module are checked, not those inside TypeScript
        // module declarations.
        export.syntax().grand_parent().and_then(JsModule::cast)?;
        let options = ctx.options();
        let model = ctx.model();

        match ctx.query() {
            AnyJsExportDefaultClause::JsExportDefaultDeclarationClause(clause) => {
                match clause.declaration().ok()? {
                    AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(function) => {
                        if options.functions() != DefaultExportStyle::Separate
                            || function.star_token().is_some()
                            || follows_default_export_overload(&export)
                            || is_reassigned(model, &function.id()?)
                        {
                            return None;
                        }
                        Some(DefaultExportStyleViolation::InlineDeclaration(
                            DeclarationKind::Function,
                        ))
                    }
                    AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(class) => {
                        if options.classes() != DefaultExportStyle::Separate
                            || is_reassigned(model, &class.id()?)
                        {
                            return None;
                        }
                        Some(DefaultExportStyleViolation::InlineDeclaration(
                            DeclarationKind::Class,
                        ))
                    }
                    _ => None,
                }
            }
            AnyJsExportDefaultClause::JsExportDefaultExpressionClause(clause) => {
                let expression = clause.expression().ok()?.omit_parentheses();
                let reference = expression.as_js_identifier_expression()?.name().ok()?;
                let reference = reference.value_token().ok()?;
                let previous = AnyJsModuleItem::cast(export.syntax().prev_sibling()?)?;
                let AnyJsModuleItem::AnyJsStatement(statement) = previous else {
                    return None;
                };
                let declaration = AnyJsSeparateDeclaration::cast(statement.into_syntax())?;
                let id = declaration.identifier()?;
                if id.name_token().ok()?.text_trimmed() != reference.text_trimmed()
                    || model.as_binding(&id).all_writes().next().is_some()
                {
                    return None;
                }

                match &declaration {
                    AnyJsSeparateDeclaration::JsFunctionDeclaration(function) => {
                        if function
                            .syntax()
                            .prev_sibling()
                            .is_some_and(|node| TsDeclareFunctionDeclaration::can_cast(node.kind()))
                        {
                            return None;
                        }
                        match options.functions() {
                            DefaultExportStyle::Inline => Some(
                                DefaultExportStyleViolation::SeparateDeclaration(declaration),
                            ),
                            DefaultExportStyle::Separate if function.star_token().is_none() => {
                                Some(DefaultExportStyleViolation::SeparateFunctionDeclaration(
                                    function.clone(),
                                ))
                            }
                            _ => None,
                        }
                    }
                    AnyJsSeparateDeclaration::JsClassDeclaration(_) => {
                        (options.classes() == DefaultExportStyle::Inline).then_some(
                            DefaultExportStyleViolation::SeparateDeclaration(declaration),
                        )
                    }
                    AnyJsSeparateDeclaration::JsVariableStatement(_) => {
                        (options.functions() == DefaultExportStyle::Inline).then_some(
                            DefaultExportStyleViolation::SeparateDeclaration(declaration),
                        )
                    }
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let export = JsExport::cast(ctx.query().syntax().parent()?)?;
        let diagnostic = match state {
            DefaultExportStyleViolation::InlineDeclaration(kind) => {
                let AnyJsExportDefaultClause::JsExportDefaultDeclarationClause(clause) =
                    ctx.query()
                else {
                    return None;
                };
                let id = match clause.declaration().ok()? {
                    AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(function) => {
                        function.id()?
                    }
                    AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(class) => {
                        class.id()?
                    }
                    _ => return None,
                };
                let range = export.export_token().ok()?.text_trimmed_range().cover(id.range());
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "This "{kind}" is declared inside its "<Emphasis>"export default"</Emphasis>" statement."
                    },
                )
                .note(markup! {
                    "This project declares default-exported "{kind.plural()}" separately, so that all default exports look the same."
                })
                .note(match kind {
                    DeclarationKind::Function => markup! {
                        "Declare the function as a "<Emphasis>"const"</Emphasis>" arrow function, then export it with "<Emphasis>"export default"</Emphasis>" and its name."
                    },
                    DeclarationKind::Class => markup! {
                        "Declare the class first, then export it with "<Emphasis>"export default"</Emphasis>" and its name."
                    },
                })
            }
            DefaultExportStyleViolation::SeparateDeclaration(declaration) => {
                let kind = declaration.kind();
                RuleDiagnostic::new(
                    rule_category!(),
                    export.range(),
                    markup! {
                        "This default-exported "{kind}" is declared separately from its "<Emphasis>"export default"</Emphasis>" statement."
                    },
                )
                .detail(
                    declaration.identifier()?.range(),
                    markup! { "The "{kind}" is declared here." },
                )
                .note(markup! {
                    "This project declares default-exported "{kind.plural()}" inside the "<Emphasis>"export default"</Emphasis>" statement, so that all default exports look the same."
                })
                .note(match declaration {
                    AnyJsSeparateDeclaration::JsVariableStatement(_) => markup! {
                        "Rewrite the arrow function as a function declaration inside the "<Emphasis>"export default"</Emphasis>" statement, like "<Emphasis>"export default function foo() {}"</Emphasis>"."
                    },
                    _ => markup! {
                        "Move the declaration into the "<Emphasis>"export default"</Emphasis>" statement."
                    },
                })
            }
            DefaultExportStyleViolation::SeparateFunctionDeclaration(function) => {
                RuleDiagnostic::new(
                    rule_category!(),
                    export.range(),
                    markup! {
                        "This default-exported function is declared with the "<Emphasis>"function"</Emphasis>" keyword."
                    },
                )
                .detail(function.id().ok()?.range(), markup! { "The function is declared here." })
                .note(markup! {
                    "This project declares default-exported functions as "<Emphasis>"const"</Emphasis>" arrow functions, so that all default exports look the same."
                })
                .note(markup! {
                    "Rewrite the function as a "<Emphasis>"const"</Emphasis>" arrow function, like "<Emphasis>"const foo = () => {};"</Emphasis>"."
                })
            }
        };
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let export = JsExport::cast(ctx.query().syntax().parent()?)?;
        let mut mutation = ctx.root().begin();
        let category = ctx.metadata().action_category(ctx.category(), ctx.group());
        let applicability = ctx.metadata().applicability();
        match state {
            DefaultExportStyleViolation::InlineDeclaration(DeclarationKind::Class) => {
                let AnyJsExportDefaultClause::JsExportDefaultDeclarationClause(clause) =
                    ctx.query()
                else {
                    return None;
                };
                let declaration = clause.declaration().ok()?;
                let class = declaration.as_js_class_export_default_declaration()?;
                let list = JsModuleItemList::cast(export.syntax().parent()?)?;
                let (class, new_export) = split_inline_class(&export, clause, class)?;
                let mut items = Vec::with_capacity(list.len() + 1);
                for item in list.iter() {
                    if item.syntax() == export.syntax() {
                        items.push(AnyJsModuleItem::AnyJsStatement(
                            AnyJsStatement::JsClassDeclaration(class.clone()),
                        ));
                        items.push(AnyJsModuleItem::JsExport(new_export.clone()));
                    } else {
                        items.push(item);
                    }
                }
                mutation.replace_node_discard_trivia(list, make::js_module_item_list(items));
                Some(JsRuleAction::new(
                    category,
                    applicability,
                    markup! { "Declare the class separately." },
                    mutation,
                ))
            }
            DefaultExportStyleViolation::SeparateDeclaration(declaration) => {
                // A comment on the removed `export default name;` statement would be lost.
                if export.syntax().has_comments_descendants() {
                    return None;
                }
                let new_export = match declaration {
                    AnyJsSeparateDeclaration::JsFunctionDeclaration(function) => {
                        inline_function(function)?
                    }
                    AnyJsSeparateDeclaration::JsClassDeclaration(class) => inline_class(class)?,
                    AnyJsSeparateDeclaration::JsVariableStatement(_) => return None,
                };
                mutation.replace_element_discard_trivia(
                    declaration.syntax().clone().into(),
                    new_export.into_syntax().into(),
                );
                mutation.remove_node(export);
                Some(JsRuleAction::new(
                    category,
                    applicability,
                    markup! { "Declare the "{declaration.kind()}" inside the "<Emphasis>"export default"</Emphasis>" statement." },
                    mutation,
                ))
            }
            DefaultExportStyleViolation::InlineDeclaration(DeclarationKind::Function)
            | DefaultExportStyleViolation::SeparateFunctionDeclaration(_) => None,
        }
    }
}

declare_node_union! {
    /// A declaration that can be default-exported by name in the statement after it.
    pub AnyJsSeparateDeclaration = JsFunctionDeclaration | JsClassDeclaration | JsVariableStatement
}

impl AnyJsSeparateDeclaration {
    /// Returns the declared name.
    ///
    /// For a variable statement, returns the name only if the statement is
    /// `const name = () => {}` with a single variable and no type annotation, because only
    /// that form can be rewritten as a function declaration.
    fn identifier(&self) -> Option<JsIdentifierBinding> {
        let id = match self {
            Self::JsFunctionDeclaration(function) => function.id().ok()?,
            Self::JsClassDeclaration(class) => class.id().ok()?,
            Self::JsVariableStatement(statement) => {
                let declaration = statement.declaration().ok()?;
                if !declaration.is_const() {
                    return None;
                }
                let mut declarators = declaration.declarators().iter();
                let declarator = declarators.next()?.ok()?;
                if declarators.next().is_some() || declarator.variable_annotation().is_some() {
                    return None;
                }
                let initializer = declarator.initializer()?.expression().ok()?;
                if !matches!(
                    initializer.omit_parentheses(),
                    AnyJsExpression::JsArrowFunctionExpression(_)
                ) {
                    return None;
                }
                let AnyJsBindingPattern::AnyJsBinding(id) = declarator.id().ok()? else {
                    return None;
                };
                id
            }
        };
        id.as_js_identifier_binding().cloned()
    }

    fn kind(&self) -> DeclarationKind {
        match self {
            Self::JsClassDeclaration(_) => DeclarationKind::Class,
            Self::JsFunctionDeclaration(_) | Self::JsVariableStatement(_) => {
                DeclarationKind::Function
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum DeclarationKind {
    Function,
    Class,
}

impl DeclarationKind {
    fn plural(self) -> &'static str {
        match self {
            Self::Function => "functions",
            Self::Class => "classes",
        }
    }
}

impl biome_console::fmt::Display for DeclarationKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Function => "function",
            Self::Class => "class",
        })
    }
}

/// Returns `true` if `id` names a variable that is assigned a new value somewhere.
fn is_reassigned(model: &SemanticModel, id: &AnyJsBinding) -> bool {
    id.as_js_identifier_binding()
        .is_some_and(|id| model.as_binding(id).all_writes().next().is_some())
}

/// Returns `true` if the module item before `export` is a TypeScript overload signature
/// of the default-exported function, such as `export default function foo(value: string): void;`.
fn follows_default_export_overload(export: &JsExport) -> bool {
    export
        .syntax()
        .prev_sibling()
        .and_then(JsExport::cast)
        .and_then(|previous| previous.export_clause().ok())
        .and_then(|clause| match clause {
            AnyJsExportClause::JsExportDefaultDeclarationClause(clause) => {
                clause.declaration().ok()
            }
            _ => None,
        })
        .is_some_and(|declaration| {
            matches!(
                declaration,
                AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
            )
        })
}

/// Builds `export default` followed by a space, with `leading` as the leading trivia of `export`.
fn export_default_tokens(leading: Option<JsSyntaxTrivia>) -> (JsSyntaxToken, JsSyntaxToken) {
    let export_token =
        make::token(T![export]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
    let export_token = match leading {
        Some(leading) => export_token.with_leading_trivia_pieces(leading.pieces()),
        None => export_token,
    };
    let default_token =
        make::token(T![default]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
    (export_token, default_token)
}

/// Converts `function foo() {}` to `export default function foo() {}`.
fn inline_function(function: &JsFunctionDeclaration) -> Option<JsExport> {
    let mut builder = make::js_function_export_default_declaration(
        function.function_token().ok()?,
        function.parameters().ok()?,
        function.body().ok()?,
    )
    .with_id(function.id().ok()?);
    if let Some(token) = function.async_token() {
        builder = builder.with_async_token(token);
    }
    if let Some(token) = function.star_token() {
        builder = builder.with_star_token(token);
    }
    if let Some(type_parameters) = function.type_parameters() {
        builder = builder.with_type_parameters(type_parameters);
    }
    if let Some(return_type) = function.return_type_annotation() {
        builder = builder.with_return_type_annotation(return_type);
    }
    let declaration = builder.build();
    // The comments and line breaks before the function move before `export`.
    let leading = declaration.syntax().first_leading_trivia();
    let declaration = declaration.with_leading_trivia_pieces([])?;
    let (export_token, default_token) = export_default_tokens(leading);
    Some(make::js_export(
        make::js_decorator_list([]),
        export_token,
        make::js_export_default_declaration_clause(default_token, declaration.into())
            .build()
            .into(),
    ))
}

/// Converts `class Foo {}` to `export default class Foo {}`.
/// Decorators of the class are placed before `export`.
fn inline_class(class: &JsClassDeclaration) -> Option<JsExport> {
    let mut builder = make::js_class_export_default_declaration(
        make::js_decorator_list([]),
        class.class_token().ok()?,
        class.l_curly_token().ok()?,
        class.members(),
        class.r_curly_token().ok()?,
    )
    .with_id(class.id().ok()?);
    if let Some(token) = class.abstract_token() {
        builder = builder.with_abstract_token(token);
    }
    if let Some(type_parameters) = class.type_parameters() {
        builder = builder.with_type_parameters(type_parameters);
    }
    if let Some(extends_clause) = class.extends_clause() {
        builder = builder.with_extends_clause(extends_clause);
    }
    if let Some(implements_clause) = class.implements_clause() {
        builder = builder.with_implements_clause(implements_clause);
    }
    let declaration = builder.build();
    // The trivia before `class` (or `abstract`) moves before `export`. When the class has
    // decorators, this is the line break between the decorators and the class.
    let leading = declaration.syntax().first_leading_trivia();
    let declaration = declaration.with_leading_trivia_pieces([])?;
    let (export_token, default_token) = export_default_tokens(leading);
    Some(make::js_export(
        class.decorators(),
        export_token,
        make::js_export_default_declaration_clause(default_token, declaration.into())
            .build()
            .into(),
    ))
}

/// Converts `export default class Foo {}` to `class Foo {}` followed by `export default Foo;`.
///
/// Returns `None` if a comment would be lost, or if both `export` and `class` have decorators.
fn split_inline_class(
    export: &JsExport,
    clause: &JsExportDefaultDeclarationClause,
    class: &JsClassExportDefaultDeclaration,
) -> Option<(JsClassDeclaration, JsExport)> {
    let export_token = export.export_token().ok()?;
    let default_token = clause.default_token().ok()?;
    if export_token.has_trailing_comments()
        || default_token.has_leading_comments()
        || default_token.has_trailing_comments()
        || class.syntax().has_leading_comments()
    {
        return None;
    }
    let export_decorators = export.decorators();
    let has_export_decorators = !export_decorators.is_empty();
    if has_export_decorators && !class.decorators().is_empty() {
        return None;
    }
    let id = class.id()?;
    let name = id.as_js_identifier_binding()?.name_token().ok()?;

    let mut builder = make::js_class_declaration(
        make::js_decorator_list([]),
        class.class_token().ok()?,
        id,
        class.l_curly_token().ok()?,
        class.members(),
        class.r_curly_token().ok()?,
    );
    if let Some(token) = class.abstract_token() {
        builder = builder.with_abstract_token(token);
    }
    if let Some(type_parameters) = class.type_parameters() {
        builder = builder.with_type_parameters(type_parameters);
    }
    if let Some(extends_clause) = class.extends_clause() {
        builder = builder.with_extends_clause(extends_clause);
    }
    if let Some(implements_clause) = class.implements_clause() {
        builder = builder.with_implements_clause(implements_clause);
    }
    let declaration = builder.build();
    let leading = export_token.leading_trivia();
    let declaration = if has_export_decorators {
        // The line break between the decorators and `export` moves before the class keyword.
        declaration
            .with_leading_trivia_pieces(leading.pieces())?
            .with_decorators(export_decorators)
    } else {
        // The comments and line breaks before `export` move before the class.
        declaration
            .with_decorators(class.decorators())
            .with_leading_trivia_pieces(leading.pieces())?
    };

    let (_, new_default_token) = export_default_tokens(None);
    let new_export_token = make::token(T![export])
        .with_leading_trivia([(TriviaPieceKind::Newline, "\n")])
        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
    let reference = make::js_identifier_expression(make::js_reference_identifier(make::ident(
        name.text_trimmed(),
    )));
    let mut new_clause =
        make::js_export_default_expression_clause(new_default_token, reference.into());
    if let Some(semicolon) = clause.semicolon_token() {
        new_clause = new_clause.with_semicolon_token(semicolon);
    } else if !export
        .syntax()
        .next_sibling()
        .is_some_and(|next| JsEmptyStatement::can_cast(next.kind()))
    {
        // In `export default class Foo {};`, the `;` is an empty statement that now ends
        // `export default Foo`.
        new_clause = new_clause.with_semicolon_token(make::token(T![;]));
    }
    let new_clause = new_clause.build();
    let new_export = make::js_export(
        make::js_decorator_list([]),
        new_export_token,
        new_clause.into(),
    );
    Some((declaration, new_export))
}
