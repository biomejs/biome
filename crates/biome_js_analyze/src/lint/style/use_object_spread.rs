use crate::JsRuleAction;
use crate::services::semantic::Semantic;
use biome_analyze::{
    FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsObjectMember, JsCallArgumentList, JsCallExpression,
    JsLanguage, JsSyntaxKind, T, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange};
use biome_rule_options::use_object_spread::UseObjectSpreadOptions;

declare_lint_rule! {
    /// Prefer object spread over `Object.assign()` when constructing new objects.
    ///
    /// Object spread syntax is more concise, more readable, and performs better
    /// than `Object.assign()` when creating a new object from existing objects.
    /// It also has better TypeScript integration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Object.assign({}, foo);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.assign({}, { foo: 'bar' });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.assign({ foo: 'bar' }, baz);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.assign({}, baz, { foo: 'bar' });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// ({ ...foo });
    /// ```
    ///
    /// ```js
    /// ({ ...baz, foo: 'bar' });
    /// ```
    ///
    /// Modifying an existing object is allowed:
    /// ```js
    /// Object.assign(foo, { bar: baz });
    /// ```
    ///
    pub UseObjectSpread {
        version: "2.0.0",
        name: "useObjectSpread",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-object-spread").same()],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseObjectSpread {
    type Query = Semantic<JsCallExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = UseObjectSpreadOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let callee = node.callee().ok()?;
        let member_expr = callee.as_js_static_member_expression()?;

        let obj = member_expr.object().ok()?;
        let (reference, obj_name) = global_identifier(&obj)?;
        if obj_name.text() != "Object" || ctx.model().binding(&reference).is_some() {
            return None;
        }

        let method = member_expr.member().ok()?;
        if method.value_token().ok()?.text_trimmed() != "assign" {
            return None;
        }

        let args = node.arguments().ok()?.args();

        let first_arg = args.first()?.ok()?;

        let expression = first_arg.as_any_js_expression()?;
        if !matches!(expression, AnyJsExpression::JsObjectExpression(_)) {
            return None;
        }

        if args
            .iter()
            .skip(1)
            .any(|arg| matches!(arg, Ok(AnyJsCallArgument::JsSpread(_))))
        {
            // If there are any spread arguments, we cannot convert to object spread
            return None;
        }

        Some(RuleState {
            member_expr_range: member_expr.range(),
            args,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.member_expr_range,
            markup! {
                "Object spread syntax is more concise, readable, and performs better"
                " than "<Emphasis>"Object.assign"</Emphasis>"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleAction<JsLanguage>> {
        let args = &state.args;
        let mut object_members = Vec::new();
        let mut object_member_separators = Vec::new();

        for arg in args.iter().flatten() {
            let AnyJsCallArgument::AnyJsExpression(expression) = arg else {
                return None;
            };
            match expression {
                // Flatten nested object expressions.
                AnyJsExpression::JsObjectExpression(object_expr) => {
                    let object_member_list = object_expr.members();
                    let mut separators = object_member_list
                        .separators()
                        .flatten()
                        .collect::<Vec<_>>();
                    let members = object_member_list.iter().flatten().collect::<Vec<_>>();
                    // Keep original separators to preserve comments and whitespace.
                    separators.resize_with(members.len(), || make::token(T![,]));
                    object_member_separators.extend(separators);
                    object_members.extend(members);
                }
                // All the other expressions will be spread.
                _ => {
                    object_members.push(AnyJsObjectMember::JsSpread(make::js_spread(
                        make::token(JsSyntaxKind::DOT3),
                        expression,
                    )));
                    object_member_separators.push(make::token(T![,]));
                }
            }
        }
        let mut mutation = ctx.root().begin();
        // Building the final object expression.
        // Formatter should be able to remove unnecessary trailing comma depending on configuration.
        let result_object = make::js_object_expression(
            make::token(T!['{']),
            make::js_object_member_list(object_members, object_member_separators),
            make::token(T!['}']),
        );

        mutation.replace_node(
            AnyJsExpression::JsCallExpression(ctx.query().clone()),
            // Wrap into parens in case we are in a statement expression or arrow function body.
            // Formatter should be able to remove unnecessary parens.
            AnyJsExpression::JsParenthesizedExpression(make::js_parenthesized_expression(
                make::token(T!['(']),
                AnyJsExpression::JsObjectExpression(result_object.clone()),
                make::token(T![')']),
            )),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Replace "<Emphasis>"Object.assign({...}, <object>)"</Emphasis>
                " with "<Emphasis>"{ ...<object> }"</Emphasis>"."
            }
            .to_owned(),
            mutation,
        ))
    }
}

pub struct RuleState {
    member_expr_range: TextRange,
    args: JsCallArgumentList,
}
