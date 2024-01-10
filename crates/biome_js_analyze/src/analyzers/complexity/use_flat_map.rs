use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make::{ident, js_name};
use biome_js_syntax::{AnyJsExpression, AnyJsMemberExpression, AnyJsName, JsCallExpression};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

declare_rule! {
    /// Promotes the use of `.flatMap()` when `map().flat()` are used together.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const array = ["split", "the text", "into words"];
    /// array.map(sentence => sentence.split(' ')).flat();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const array = ["split", "the text", "into words"];
    /// array.map(sentence => sentence.split(' ')).flat(1);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const array = ["split", "the text", "into words"];
    /// array.map(sentence => sentence.split(' ')).flat(2);
    /// ```
    ///
    pub(crate) UseFlatMap {
        version: "1.0.0",
        name: "useFlatMap",
        source: RuleSource::EslintUnicorn("prefer-array-flat-map"),
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseFlatMap {
    type Query = Ast<JsCallExpression>;
    type State = JsCallExpression;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let flat_call = ctx.query();
        let arguments = flat_call.arguments().ok()?.args();
        // Probably not a `flat` call.
        if arguments.len() > 1 {
            return None;
        }
        if let Some(first_argument) = arguments.first() {
            let first_argument = first_argument.ok()?;
            let first_argument = first_argument
                .as_any_js_expression()?
                .as_any_js_literal_expression()?
                .as_js_number_literal_expression()?;

            if first_argument.value_token().ok()?.text_trimmed() != "1" {
                return None;
            }
        }
        let flat_member_expression =
            AnyJsMemberExpression::cast_ref(flat_call.callee().ok()?.syntax())?;
        if flat_member_expression.member_name()?.text() == "flat" {
            let Ok(AnyJsExpression::JsCallExpression(map_call)) = flat_member_expression.object()
            else {
                return None;
            };
            let map_call_arguments = map_call.arguments().ok()?.args();
            let map_member_expression =
                AnyJsMemberExpression::cast_ref(map_call.callee().ok()?.syntax())?;
            if map_member_expression.member_name()?.text() == "map" && map_call_arguments.len() == 1
            {
                return Some(map_call);
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "The call chain "<Emphasis>".map().flat()"</Emphasis>" can be replaced with a single "<Emphasis>".flatMap()"</Emphasis>" call."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, flat_call: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let Ok(AnyJsExpression::JsStaticMemberExpression(old_static_member_expression)) =
            flat_call.callee()
        else {
            return None;
        };
        let member = js_name(ident("flatMap"));

        let flat_map_member_expression =
            old_static_member_expression.with_member(AnyJsName::JsName(member));

        let flat_map_call =
            flat_call
                .clone()
                .with_callee(AnyJsExpression::JsStaticMemberExpression(
                    flat_map_member_expression,
                ));

        mutation.replace_node(node.clone(), flat_map_call);

        Some(JsRuleAction {
            mutation,
            message: markup! {"Replace the chain with "<Emphasis>".flatMap()"</Emphasis>"."}
                .to_owned(),
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
        })
    }
}
