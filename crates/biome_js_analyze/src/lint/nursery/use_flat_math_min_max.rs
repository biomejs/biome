use std::iter::FusedIterator;

use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, JsCallArgumentList, JsCallArguments, JsCallExpression, JsLanguage,
    JsParenthesizedExpression, JsSyntaxElement, T, global_identifier,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, Direction, TriviaPieceKind, WalkEvent,
    syntax::Preorder,
};
use biome_rule_options::use_flat_math_min_max::UseFlatMathMinMaxOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Prefer flat `Math.min()` and `Math.max()` calls over nested calls of the same method.
    ///
    /// `Math.min()` and `Math.max()` accept any number of arguments, so nesting the same call is unnecessary.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const biggest = Math.max(Math.max(a, b), c);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const smallest = Math.min(a, Math.min(b, c));
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const biggest = Math.max(a, b, c);
    /// const clamped = Math.max(Math.min(value, upper), lower);
    /// ```
    ///
    pub UseFlatMathMinMax {
        version: "2.5.12",
        name: "useFlatMathMinMax",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-flat-math-min-max").same()],
        recommended: true,
        severity: Severity::Warning,
        // unsafe because the fix is complex.
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseFlatMathMinMax {
    type Query = Semantic<JsCallExpression>;
    type State = MathMethod;
    type Signals = Option<Self::State>;
    type Options = UseFlatMathMinMaxOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        let method = math_method(call, model)?;

        if is_nested_in_same_call(call, method, model) || !has_nested_same_call(call, method, model)
        {
            return None;
        }

        Some(method)
    }

    fn diagnostic(ctx: &RuleContext<Self>, method: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"Math."{method}"()"</Emphasis>" call contains another call to the same method."
                },
            )
            .note(markup! {
                <Emphasis>"Math."{method}"()"</Emphasis>" accepts any number of arguments, so the nested call is unnecessary."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, method: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        if has_comments_inside(call) {
            return None;
        }

        let old_arguments = call.arguments().ok()?;
        let mut failed = false;
        let new_argument_list = {
            let flattened_arguments = FlattenedArguments::new(call, *method, ctx.model())?;
            let flattened_arguments =
                flattened_arguments.map_while(|argument| match argument {
                    Ok(argument) => Some(argument),
                    Err(()) => {
                        failed = true;
                        None
                    }
                });
            let mut first = true;
            let slots = flattened_arguments.flat_map(move |argument| {
                let separator: Option<JsSyntaxElement> = if first {
                    first = false;
                    None
                } else {
                    Some(
                        make::token(T![,])
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])
                            .into(),
                    )
                };
                separator
                    .into_iter()
                    .chain(std::iter::once(argument.into_syntax().into()))
                    .map(Some)
            });
            JsCallArgumentList::unwrap_cast(
                old_arguments
                    .args()
                    .into_syntax()
                    .splice_slots(.., slots),
            )
        };
        if failed {
            return None;
        }
        let new_arguments = make::js_call_arguments(
            make::token(T!['(']),
            new_argument_list,
            make::token(T![')']),
        );
        let mut mutation = ctx.root().begin();
        mutation.replace_node(old_arguments, new_arguments);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Flatten the nested "<Emphasis>"Math."{method}"()"</Emphasis>" calls."
            }
            .to_owned(),
            mutation,
        ))
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MathMethod {
    Min,
    Max,
}

impl biome_console::fmt::Display for MathMethod {
    fn fmt(&self, formatter: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        formatter.write_str(match self {
            Self::Min => "min",
            Self::Max => "max",
        })
    }
}

fn math_method(call: &JsCallExpression, model: &SemanticModel) -> Option<MathMethod> {
    if call.optional_chain_token().is_some() {
        return None;
    }

    let callee = call
        .callee()
        .ok()?
        .omit_parentheses()
        .as_js_static_member_expression()?
        .clone();
    if callee.operator_token().ok()?.kind() != T![.] {
        return None;
    }

    let method = match callee
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?
        .text_trimmed()
    {
        "min" => MathMethod::Min,
        "max" => MathMethod::Max,
        _ => return None,
    };

    let object = callee.object().ok()?.omit_parentheses();
    let identifier = object.as_js_identifier_expression()?;
    let (reference, name) = global_identifier(&identifier.clone().into())?;
    (name.text() == "Math" && model.binding(&reference).is_none()).then_some(method)
}

fn has_nested_same_call(
    call: &JsCallExpression,
    method: MathMethod,
    model: &SemanticModel,
) -> bool {
    call.arguments().is_ok_and(|arguments| {
        arguments.args().iter().any(|argument| {
            argument.is_ok_and(|argument| same_method_call(&argument, method, model).is_some())
        })
    })
}

fn same_method_call(
    argument: &AnyJsCallArgument,
    method: MathMethod,
    model: &SemanticModel,
) -> Option<JsCallExpression> {
    argument
        .as_any_js_expression()
        .cloned()
        .map(|expression| expression.omit_parentheses())
        .and_then(|expression| expression.as_js_call_expression().cloned())
        .filter(|call| math_method(call, model) == Some(method))
}

fn is_nested_in_same_call(
    call: &JsCallExpression,
    method: MathMethod,
    model: &SemanticModel,
) -> bool {
    call
        .syntax()
        .ancestors()
        .skip(1)
        .find(|parent| !JsParenthesizedExpression::can_cast(parent.kind()))
        .and_then(JsCallArgumentList::cast)
        .and_then(|arguments| arguments.parent::<JsCallArguments>())
        .and_then(|arguments| arguments.parent::<JsCallExpression>())
        .and_then(|parent_call| math_method(&parent_call, model))
        == Some(method)
}

struct FlattenedArguments<'a> {
    preorder: Preorder<JsLanguage>,
    method: MathMethod,
    model: &'a SemanticModel,
}

impl<'a> FlattenedArguments<'a> {
    fn new(
        call: &JsCallExpression,
        method: MathMethod,
        model: &'a SemanticModel,
    ) -> Option<Self> {
        let arguments = call.arguments().ok()?.args();
        Some(Self {
            preorder: arguments.syntax().preorder(),
            method,
            model,
        })
    }
}

impl Iterator for FlattenedArguments<'_> {
    type Item = Result<AnyJsCallArgument, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(event) = self.preorder.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };

            if let Some(arguments) = JsCallArgumentList::cast_ref(&node) {
                // A malformed argument list cannot be flattened without risking a
                // partial rewrite.
                if arguments.iter().any(|argument| argument.is_err()) {
                    return Some(Err(()));
                }
                continue;
            }

            // Only direct children of an argument list can contribute arguments.
            // Other nodes are syntax inside a candidate argument.
            if !node
                .parent()
                .is_some_and(|parent| JsCallArgumentList::can_cast(parent.kind()))
            {
                continue;
            }

            let Some(argument) = AnyJsCallArgument::cast(node) else {
                return Some(Err(()));
            };
            if let Some(call) = same_method_call(&argument, self.method, self.model) {
                if call.arguments().is_err() {
                    return Some(Err(()));
                }
                // A matching call contributes its arguments, not the call itself, so
                // continue into its argument list.
                continue;
            }

            // A nonmatching argument contributes as a whole. Skip its descendants so
            // `foo(Math.max(a, b))` remains one argument.
            self.preorder.skip_subtree();
            return Some(argument.trim_trivia().ok_or(()));
        }
        None
    }
}

impl FusedIterator for FlattenedArguments<'_> {}

fn has_comments_inside(call: &JsCallExpression) -> bool {
    let Some(first_token) = call.syntax().first_token() else {
        return false;
    };
    let Some(last_token) = call.syntax().last_token() else {
        return false;
    };

    call.syntax()
        .descendants_tokens(Direction::Next)
        .any(|token| {
            (token != first_token && token.has_leading_comments())
                || (token != last_token && token.has_trailing_comments())
        })
}
