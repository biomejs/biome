use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::{
    HtmlAttribute, HtmlString, HtmlSyntaxKind, HtmlSyntaxToken, inner_string_text,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_sorted_classes::UseSortedClassesOptions;
use biome_tailwind_logic::sorted_classes::{EMPTY_REGISTRY, sort_class_string};

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce the sorting of CSS utility classes.
    ///
    /// This rule sorts classes the way [Tailwind CSS v4](https://tailwindcss.com/blog/automatic-class-sorting-with-prettier#how-classes-are-sorted) and its Prettier plugin do: utilities in the order Tailwind emits them, variants after plain utilities and grouped by variant, and classes the rule doesn't recognize kept at the front in their original order.
    ///
    /// It is analogous to [`prettier-plugin-tailwindcss`](https://github.com/tailwindlabs/prettier-plugin-tailwindcss).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div class="px-2 foo p-4 bar"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div class="hover:focus:m-2 foo hover:px-2 p-4"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="bar foo p-4 px-2"></div>
    /// ```
    ///
    pub UseSortedClasses {
        version: "2.5.0",
        name: "useSortedClasses",
        language: "html",
        recommended: false,
        fix_kind: FixKind::Unsafe,
        issue_number: Some("9181"),
    }
}

impl Rule for UseSortedClasses {
    type Query = Ast<HtmlAttribute>;
    type State = HtmlSortedClassesState;
    type Signals = Option<Self::State>;
    type Options = UseSortedClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let attribute = ctx.query();
        let file_source = ctx.source_type::<HtmlFileSource>();
        let options = ctx.options();

        let name = attribute.name().ok()?;
        let name_token = name.value_token().ok()?;
        let name_text = name_token.text_trimmed();
        let matches = if file_source.is_html() {
            name_text.eq_ignore_ascii_case("class")
                || options
                    .attributes
                    .iter()
                    .flatten()
                    .any(|attr| attr.eq_ignore_ascii_case(name_text))
        } else {
            name_text == "class"
                || options
                    .attributes
                    .iter()
                    .flatten()
                    .any(|attr| attr.as_ref() == name_text)
        };
        if !matches {
            return None;
        }

        let initializer = attribute.initializer()?;
        let value = initializer.value().ok()?;
        let html_string = value.as_html_string()?.clone();
        let value_token = html_string.value_token().ok()?;
        let inner_text = inner_string_text(&value_token);
        let value_str = inner_text.text();

        let sorted_value = sort_class_string(value_str, &EMPTY_REGISTRY);
        if sorted_value.is_empty() {
            return None;
        }
        if value_str != sorted_value {
            let is_single_quote = value_token.text_trimmed().starts_with('\'');
            return Some(HtmlSortedClassesState {
                html_string,
                sorted: sorted_value.into(),
                is_single_quote,
            });
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.html_string.range(),
            "These CSS classes should be sorted.",
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();

        let new_token = if state.is_single_quote {
            HtmlSyntaxToken::new_detached(
                HtmlSyntaxKind::HTML_STRING_LITERAL,
                &format!("'{}'", state.sorted),
                [],
                [],
            )
        } else {
            make::html_string_literal(&state.sorted)
        };

        let new_html_string = make::html_string(new_token);
        mutation.replace_node(state.html_string.clone(), new_html_string);

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Sort the classes."
            }
            .to_owned(),
            mutation,
        ))
    }
}

pub struct HtmlSortedClassesState {
    html_string: HtmlString,
    sorted: Box<str>,
    is_single_quote: bool,
}
