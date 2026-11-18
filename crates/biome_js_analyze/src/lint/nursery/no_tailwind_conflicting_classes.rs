use crate::tailwind::{AnyTailwindClassString, host_range};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    JsBinaryExpression, JsConditionalExpression, JsLanguage, JsLogicalExpression,
    JsParenthesizedExpression, JsTemplateElement, JsTemplateElementList, JsTemplateExpression,
    TsAsExpression, TsNonNullAssertionExpression, TsSatisfiesExpression,
};
use biome_rowan::{AstNode, SyntaxKindSet, TextSize};
use biome_rule_options::no_tailwind_conflicting_classes::NoTailwindConflictingClassesOptions;
use biome_tailwind_logic::no_tailwind_conflicting_classes::{
    ConflictingClass, find_conflicting_classes,
};
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use biome_tailwind_logic::tailwind_v4::TailwindDesignSystem;
use std::sync::Arc;

declare_lint_rule! {
    /// Disallow Tailwind CSS classes that set the same CSS properties on an element.
    ///
    /// When two classes in a class list set the same CSS properties, only one of them takes effect.
    /// The other class does nothing, and it hides which style you meant to apply.
    /// In a `class` attribute, Tailwind CSS decides which class wins from the order of the styles
    /// it generates, so changing the order of the classes doesn't change the result.
    ///
    /// Two classes only conflict when they have the same variants.
    /// For example, `flex` and `md:grid` don't conflict, because `md:grid` only applies on wider screens.
    /// A class that sets some of the properties of another class doesn't conflict with it either,
    /// so you can write `p-2 px-4` to change the horizontal padding.
    ///
    /// To make the rule aware of your own theme values and variants, point the
    /// [`tailwind.stylesheet`](https://biomejs.dev/reference/configuration/#tailwindstylesheet)
    /// option at the CSS file that holds your Tailwind CSS configuration. Without it, the rule
    /// only knows the default configuration, and skips classes that use your own theme values
    /// or variants.
    ///
    /// Biome scans your project when this rule is enabled, so that it can read the stylesheet
    /// and the files it imports. The scan happens even if `tailwind.stylesheet` isn't set,
    /// although the rule then only uses the default configuration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="flex grid" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="p-2 hover:text-red-500 hover:text-blue-500" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className="flex w-full" />;
    /// <div className="flex md:grid" />;
    /// <div className="p-2 px-4" />;
    /// <div className="text-lg text-red-500" />;
    /// ```
    ///
    /// ## Recognized class strings
    ///
    /// This rule checks the attributes and functions recognized by the top-level
    /// [`tailwind` configuration](https://biomejs.dev/reference/configuration/#tailwind).
    ///
    /// ## Known limitations
    ///
    /// This rule compares the classes of one string at a time.
    /// It doesn't report classes that conflict across strings:
    ///
    /// ```jsx
    /// <div className={clsx("flex", isGrid && "grid")} />;
    /// ```
    ///
    /// It only checks a class that your stylesheet declares with `@utility` when the
    /// `@utility` rule holds nothing but declarations, and the declarations that apply to
    /// the class each set a different property. It skips a class whose rule nests other
    /// rules or uses `@apply`, and a class that more than one utility defines, such as a
    /// name that two `@utility` rules declare.
    ///
    /// It doesn't compare classes whose variants are written in a different order,
    /// such as `sm:hover:flex` and `hover:sm:grid`.
    ///
    pub NoTailwindConflictingClasses {
        version: "next",
        name: "noTailwindConflictingClasses",
        language: "jsx",
        domains: &[RuleDomain::Tailwind, RuleDomain::Project],
        sources: &[RuleSource::EslintBetterTailwindcss("no-conflicting-classes").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoTailwindConflictingClasses {
    type Query = TailwindSyntax<AnyTailwindClassString>;
    type State = ConflictingClass;
    type Signals = Box<[Self::State]>;
    type Options = NoTailwindConflictingClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx.query().tailwind_has_errors() {
            return Box::default();
        }
        let design: &TailwindDesignSystem = match ctx.get_service::<Arc<TailwindDesignSystem>>() {
            Some(design) => design,
            None => TailwindDesignSystem::default_ref(),
        };
        let query = ctx.query();
        let root = query.tailwind_root();
        // A class at the edge of a string joined with `+` can continue in the
        // other operand, as `border` does in `"border" + suffix`.
        let (leading, trailing) = concatenated_edges(query.node());
        let end = root.syntax().text_range_with_trivia().end();
        find_conflicting_classes(&root.candidates(), design, |range| {
            query.touches_partial_class(range)
                || leading && range.start() == TextSize::from(0)
                || trailing && range.end() == end
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, conflict: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query().node();
        let class = conflict.class.syntax().text_trimmed();
        let earlier = conflict.earlier.syntax().text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                host_range(node, conflict.class.range())?,
                markup! {
                    "The Tailwind CSS class "<Emphasis>{format_args!("{class}")}</Emphasis>" sets the same CSS properties as "<Emphasis>{format_args!("{earlier}")}</Emphasis>"."
                },
            )
            .detail(
                host_range(node, conflict.earlier.range())?,
                markup! {
                    "The other class is here."
                },
            )
            .note(markup! {
                "Only one of these classes takes effect, so the other one does nothing."
            })
            .note(markup! {
                "Remove the class that you don't need."
            }),
        )
    }
}

/// Returns whether the start and the end of `node` touch other text of the
/// same class string: another operand of a `+` expression, or the chunks around
/// a template substitution. Both do for `"b"` in `a + "b" + c`.
fn concatenated_edges(node: &AnyTailwindClassString) -> (bool, bool) {
    let mut leading = false;
    let mut trailing = false;
    let mut child = node.syntax().clone();
    for ancestor in node.syntax().ancestors().skip(1) {
        if let Some(binary) = JsBinaryExpression::cast_ref(&ancestor) {
            if binary.left().is_ok_and(|left| left.syntax() == &child) {
                trailing = true;
            } else {
                leading = true;
            }
        } else if JsTemplateElement::can_cast(ancestor.kind()) {
            return (true, true);
        } else if !CONCATENATION_WRAPPER_KINDS.matches(ancestor.kind()) {
            break;
        }
        child = ancestor;
    }
    (leading, trailing)
}

/// The nodes a class string can sit in while still being an operand of an outer
/// `+` expression.
const CONCATENATION_WRAPPER_KINDS: SyntaxKindSet<JsLanguage> = JsParenthesizedExpression::KIND_SET
    .union(JsConditionalExpression::KIND_SET)
    .union(JsLogicalExpression::KIND_SET)
    .union(JsTemplateExpression::KIND_SET)
    .union(JsTemplateElementList::KIND_SET)
    .union(TsAsExpression::KIND_SET)
    .union(TsSatisfiesExpression::KIND_SET)
    .union(TsNonNullAssertionExpression::KIND_SET);
