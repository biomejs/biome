mod class_info;
mod class_like_visitor;
mod class_parser;
mod options;
mod presets;
mod sort;
mod sort_config;

use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make::{js_string_literal, js_string_literal_expression, jsx_string};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

pub use self::options::UseSortedClassesOptions;
use self::{
    class_like_visitor::AnyClassStringLike,
    presets::{get_utilities_preset, UseSortedClassesPreset},
    sort::sort_class_name,
    sort_config::SortConfig,
};

// rule metadata
// -------------

declare_rule! {
    /// Enforce the sorting of CSS classes.
    ///
    /// TODO: description
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="px-2 foo px-4 bar" />;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// // TODO: examples
    /// ```
    ///
    pub(crate) UseSortedClasses {
        version: "next",
        name: "useSortedClasses",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

// rule
// ----

impl Rule for UseSortedClasses {
    type Query = Ast<AnyClassStringLike>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = UseSortedClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let value = ctx.query().value()?;
        // TODO: unsure if options are needed here. The sort config should ideally be created once
        // from the options and then reused for all queries.
        // let options = &ctx.options();
        // TODO: the sort config should already exist at this point, and be generated from the options,
        // including the preset and extended options as well.
        let sort_config = SortConfig::new(
            get_utilities_preset(&UseSortedClassesPreset::default()),
            Vec::new(),
        );
        let sorted_value = sort_class_name(value.as_str(), &sort_config);
        if value != sorted_value {
            Some(sorted_value)
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(rule_category!(), ctx.query().range(), "TODO: title").note(
                markup! {
                    "TODO: description."
                },
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        match ctx.query() {
            AnyClassStringLike::JsStringLiteralExpression(string_literal) => {
                let replacement = js_string_literal_expression(js_string_literal(state));
                mutation.replace_node(string_literal.clone(), replacement);
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! {
                        "TODO: message."
                    }
                    .to_owned(),
                    mutation,
                })
            }
            AnyClassStringLike::JsxString(jsx_string_node) => {
                let replacement = jsx_string(js_string_literal(state));
                mutation.replace_node(jsx_string_node.clone(), replacement);
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! {
                        "TODO: message."
                    }
                    .to_owned(),
                    mutation,
                })
            }
            _ => None,
        }
    }
}
