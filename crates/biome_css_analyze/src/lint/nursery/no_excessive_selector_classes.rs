use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssSelector, CssClassSelector, CssNestedQualifiedRule, CssQualifiedRule,
    CssRelativeSelector, CssRelativeSelectorList, CssSelectorList, ScssInterpolatedIdentifier,
    ScssInterpolation,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_excessive_selector_classes::NoExcessiveSelectorClassesOptions;

declare_lint_rule! {
    /// Limit the number of classes in a selector.
    ///
    /// Selectors with too many chained classes are harder to read, harder to override,
    /// and often signal overly specific styling.
    /// This rule enforces an upper bound on how many class selectors can appear in one selector.
    ///
    /// Each selector in a selector list is evaluated separately.
    /// For example, `.foo, .bar.baz` is treated as two selectors, and only `.bar.baz`
    /// contributes two class selectors.
    ///
    /// Nested selectors are checked as written instead of being resolved against their parent selector.
    /// For example, in `.foo { &.bar {} }`, the nested selector `&.bar` contributes one class selector.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// The following example will show a diagnostic when `maxClasses` is set to `1`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxClasses": 1
    ///     }
    /// }
    /// ```
    /// ```css,expect_diagnostic,use_options
    /// .foo .bar {}
    /// ```
    ///
    /// ```css,expect_diagnostic,use_options
    /// :is(.foo, .bar.baz) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// The following examples are valid when `maxClasses` is set to `1`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxClasses": 1
    ///     }
    /// }
    /// ```
    /// ```css,use_options
    /// .foo {}
    /// ```
    ///
    /// ```css,use_options
    /// .foo, div {}
    /// ```
    ///
    /// ## Options
    ///
    /// ### `maxClasses`
    ///
    /// The maximum number of class selectors allowed in a single selector.
    ///
    /// This option has no default value. Configure it explicitly to enable the rule.
    /// A value of `0` disallows class selectors entirely.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxClasses": 2
    ///     }
    /// }
    /// ```
    ///
    /// The following selector exceeds the configured limit because it contains three
    /// class selectors:
    ///
    /// ```css,expect_diagnostic,use_options
    /// .foo .bar.baz {}
    /// ```
    ///
    pub NoExcessiveSelectorClasses {
        version: "next",
        name: "noExcessiveSelectorClasses",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("selector-max-class").same()],
    }
}

#[derive(Debug)]
pub struct ExcessiveSelectorClasses {
    range: TextRange,
    count: usize,
}

impl Rule for NoExcessiveSelectorClasses {
    type Query = Ast<AnyCssSelector>;
    type State = ExcessiveSelectorClasses;
    type Signals = Option<Self::State>;
    type Options = NoExcessiveSelectorClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let selector = ctx.query();
        let max_classes = ctx.options().max_classes()?;

        if !is_reportable_selector(selector) {
            return None;
        }

        let max_classes = usize::from(max_classes);
        let class_count = selector
            .syntax()
            .descendants()
            .filter(|descendant| CssClassSelector::can_cast(descendant.kind()))
            .count();

        if class_count > max_classes {
            return Some(ExcessiveSelectorClasses {
                range: selector.range(),
                count: class_count,
            });
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let max = ctx.options().max_classes()?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Expected this selector to have no more than "{max}" class "{class_selector_label(usize::from(max))}", but found "{state.count}"."
                },
            )
            .note(markup! {
                "Selectors with too many chained classes are harder to read, override, and reuse."
            })
            .note(markup! {
                "Reduce the number of class selectors in this selector, or split it into simpler selectors."
            }),
        )
    }
}

fn class_selector_label(count: usize) -> &'static str {
    if count == 1 { "selector" } else { "selectors" }
}

fn is_reportable_selector(selector: &AnyCssSelector) -> bool {
    // Count class selectors anywhere in the selector subtree so pseudo-class
    // arguments such as :is(.foo, .bar) and :nth-child(... of .foo) contribute
    // to the same top-level selector total.
    let (AnyCssSelector::CssCompoundSelector(_) | AnyCssSelector::CssComplexSelector(_)) = selector
    else {
        return false;
    };

    // Match stylelint and ignore selectors with SCSS interpolation because
    // the final selector can't be determined statically.
    if selector.syntax().descendants().any(|descendant| {
        ScssInterpolation::can_cast(descendant.kind())
            || ScssInterpolatedIdentifier::can_cast(descendant.kind())
    }) {
        return false;
    }

    let Some(parent) = selector.syntax().parent() else {
        return false;
    };

    if CssSelectorList::cast(parent.clone())
        .and_then(|_| parent.parent())
        .and_then(CssQualifiedRule::cast)
        .is_some()
    {
        return true;
    }

    CssRelativeSelector::cast(parent.clone())
        .and_then(|_| parent.parent())
        .and_then(CssRelativeSelectorList::cast)
        .and_then(|list| list.syntax().parent())
        .and_then(CssNestedQualifiedRule::cast)
        .is_some()
}
