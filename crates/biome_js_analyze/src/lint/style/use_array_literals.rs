use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{self};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyTsType, AnyTsVariableAnnotation, JsInitializerClause,
    JsNewOrCallExpression, JsSyntaxKind, JsVariableDeclarator, T, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_array_literals::UseArrayLiteralsOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Disallow Array constructors.
    ///
    /// Use of the Array constructor to construct a new array is generally discouraged in favor of array literal notation because of the single-argument pitfall and because the Array global may be redefined.
    /// The exception is when the Array constructor intentionally creates sparse arrays of a specified size by giving the constructor a single numeric argument.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const xs = Array();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const xs = Array(0, 1, 2);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const xs = new Array(0, 1, 2);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const xs = Array(...args);
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// const xs = new Array<number>()
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const xs = Array(65000);
    /// ```
    ///
    /// ```js
    /// const xs = [0, 1, 2];
    /// ```
    ///
    pub UseArrayLiterals {
        version: "1.7.2",
        name: "useArrayLiterals",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-array-constructor").same(),
            RuleSource::EslintTypeScript("no-array-constructor").same()
        ],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseArrayLiterals {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseArrayLiteralsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?.omit_parentheses();
        let (reference, name) = global_identifier(&callee)?;
        if name.text() != "Array" || ctx.model().binding(&reference).is_some() {
            return None;
        }
        if callee.syntax() != reference.syntax()
            && !reference
                .value_token()
                .is_ok_and(|name| matches!(name.text_trimmed(), "globalThis" | "window" | "Array"))
        {
            return None;
        }
        let Some(arguments) = node.arguments() else {
            return if matches!(node, JsNewOrCallExpression::JsNewExpression(_)) {
                // Report `new Array`
                Some(())
            } else {
                // ignore `Array`
                None
            };
        };
        let [arg1, arg2] = arguments.get_arguments_by_index([0, 1]);
        if arg1.is_some() && arg2.is_none() && !matches!(arg1?, AnyJsCallArgument::JsSpread(_)) {
            // Ignore `Array(length)`
            return None;
        }
        // Report `Array()`, `Array(x, y)`, and `Array(...xs)`
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use an array literal instead of the "<Emphasis>"Array"</Emphasis>" constructor."
                },
            )
            .note(markup! {
                "The "<Emphasis>"Array"</Emphasis>" constructor is misleading because it can be used to preallocate an array of a given length or to create an array with a given list of elements."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if node
            .syntax()
            .parent()
            .is_some_and(|parent| parent.kind() == JsSyntaxKind::JS_EXPRESSION_STATEMENT)
        {
            // Ignore useless expression statements.
            // This avoids issues with missing semicolons.
            return None;
        }
        let mut mutation = ctx.root().begin();
        let new_node = if let Some(args) = node.arguments() {
            let l_paren_trailing_trivia = args.l_paren_token().ok()?.trailing_trivia().pieces();
            let r_paren_leading_trivia = args.r_paren_token().ok()?.leading_trivia().pieces();
            let args = args.args();
            let items = args
                .elements()
                .flat_map(|item| item.into_node())
                .map(|item| item.into())
                .collect::<Vec<_>>();
            let separators = args.separators().flatten().collect::<Vec<_>>();
            make::js_array_expression(
                make::token(T!['[']).append_trivia_pieces(l_paren_trailing_trivia),
                make::js_array_element_list(items, separators),
                make::token(T![']']).prepend_trivia_pieces(r_paren_leading_trivia),
            )
        } else {
            // `new Array` -> `[]`
            make::js_array_expression(
                make::token(T!['[']),
                make::js_array_element_list([], []),
                make::token(T![']']),
            )
        };

        let array_is_empty = new_node.elements().len() == 0;
        let type_arg = get_type_arg_if_safe(node);
        let whitespace = make::token_decorated_with_space(T![.])
            .trailing_trivia()
            .pieces();

        if let Some(type_arg) = type_arg {
            // type param to be preserved
            let original_type_arg = type_arg.clone();
            let type_arg = type_arg
                .with_leading_trivia_pieces([])?
                .with_trailing_trivia_pieces([])?;

            // make the type into an array (with parens if required) i.e. (T)[]
            let type_arg: AnyTsType = if !type_arg.is_literal_type()
                && !type_arg.is_primitive_type()
                && !matches!(type_arg, AnyTsType::TsReferenceType(_))
            {
                // only wrap the type in parens if its not a literal, primitive, or reference
                make::parenthesized_ts(type_arg).into()
            } else {
                type_arg
            };
            let type_arg = AnyTsType::TsArrayType(make::ts_array_type(
                type_arg,
                make::token(T!['[']),
                make::token(T![']']),
            ));

            // copy trivia around type arg
            let type_arg = type_arg
                .with_leading_trivia_pieces(
                    original_type_arg
                        .syntax()
                        .first_token()?
                        .leading_trivia()
                        .pieces(),
                )?
                .with_trailing_trivia_pieces(
                    original_type_arg
                        .syntax()
                        .last_token()?
                        .trailing_trivia()
                        .pieces(),
                )?;

            let parent_declarator = get_untyped_parent_declarator(node);
            if let Some(parent_declarator) = parent_declarator {
                // move the array type arg to the declarator type annotation and replace the initializer with the new array literal
                // e.g. `const a = Array<T>()` -> `const a: T[] = []`
                let new_parent_declarator = parent_declarator
                    .clone()
                    .with_id(parent_declarator.id().ok()?.trim_trailing_trivia()?)
                    .with_variable_annotation(Some(AnyTsVariableAnnotation::TsTypeAnnotation(
                        make::ts_type_annotation(
                            make::token(T![:]).with_trailing_trivia_pieces(whitespace.clone()),
                            type_arg.append_trivia_pieces(whitespace)?,
                        ),
                    )))
                    .with_initializer(Some(
                        parent_declarator
                            .initializer()?
                            .with_expression(new_node.into()),
                    ));

                // replace the entire declarator, having new type and new initializer
                mutation.replace_node(parent_declarator, new_parent_declarator.clone());
            } else {
                // the parent node is not a declarator - we need to wrap the array in an "as" type (and a "satisfies" type if the array is not empty)
                // "satisfies" preserves the type checking behavior and "as" preserves the resulting expression type
                let mut new_node_with_cast: AnyJsExpression = new_node.clone().into();
                if !array_is_empty {
                    new_node_with_cast = make::ts_satisfies_expression(
                        new_node_with_cast,
                        make::token_decorated_with_space(T![satisfies]),
                        type_arg.clone(),
                    )
                    .into();
                }
                new_node_with_cast = make::ts_as_expression(
                    new_node_with_cast,
                    make::token_decorated_with_space(T![as]),
                    type_arg.clone(),
                )
                .into();

                // replace array with as+satisfies expression
                mutation.replace_node::<AnyJsExpression>(node.clone().into(), new_node_with_cast);
            }
        } else {
            // type param does not need to be preserved, only replace array
            mutation.replace_node::<AnyJsExpression>(node.clone().into(), new_node.into());
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use an array literal." }.to_owned(),
            mutation,
        ))
    }
}

/// If the node's parent is a [JsVariableDeclarator] with no type annotation, return it, otherwise [None]
fn get_untyped_parent_declarator(node: &JsNewOrCallExpression) -> Option<JsVariableDeclarator> {
    let parent_declarator = node
        .parent::<JsInitializerClause>()
        .and_then(|initializer| initializer.parent::<JsVariableDeclarator>())?;
    if parent_declarator.variable_annotation().is_some() {
        return None;
    }
    Some(parent_declarator)
}

/// Return the type param from the constructor call only if there's exactly one type param
fn get_type_arg_if_safe(node: &JsNewOrCallExpression) -> Option<AnyTsType> {
    let type_arguments = match node {
        JsNewOrCallExpression::JsNewExpression(expr) => expr.type_arguments(),
        JsNewOrCallExpression::JsCallExpression(expr) => expr.type_arguments(),
    };
    // check there's exactly one type arg
    if let Some(type_arguments) = type_arguments
        && type_arguments.ts_type_argument_list().len() == 1
    {
        let list = type_arguments.ts_type_argument_list();
        return list.first()?.ok();
    }
    None
}
