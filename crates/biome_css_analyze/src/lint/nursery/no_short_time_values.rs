use crate::utils::{AnyPropertyStatement, nested_property_name, vender_prefix};
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::CssRegularDimension;
use biome_rowan::AstNode;
use biome_rule_options::no_short_time_values::NoShortTimeValuesOptions;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Disallow animation and transition times that are shorter than a configured minimum.
    ///
    /// An animation or transition that is too short is hard to see, and can look
    /// like flickering instead of smooth motion. A delay that is too short is hard
    /// to notice at all.
    ///
    /// This rule checks the times in the `transition-duration`, `transition-delay`,
    /// `animation-duration`, and `animation-delay` properties, and in the
    /// `transition` and `animation` properties that combine them. It also checks
    /// these properties when they have a browser prefix, such as
    /// `-webkit-transition`.
    ///
    /// Times of zero or less are allowed.
    ///
    /// Each time is checked as it is written. The rule doesn't work out the result
    /// of a calculation, so `calc(10ms * 20)` is reported when `minimum` is `100`.
    ///
    /// In SCSS files, the rule also checks nested properties, such as
    /// `transition: { duration: 80ms; }`.
    ///
    /// This rule does nothing until the `minimum` option is set.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// The following examples report a diagnostic when `minimum` is `100`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "minimum": 100
    ///     }
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic,use_options
    /// a { transition-duration: 80ms; }
    /// ```
    ///
    /// ```css,expect_diagnostic,use_options
    /// a { animation-delay: 0.01s; }
    /// ```
    ///
    /// ```css,expect_diagnostic,use_options
    /// a { transition: background-color 6ms linear; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css,use_options
    /// a { transition-duration: 0.8s; }
    /// a { animation-delay: 100ms; }
    /// a { transition: background-color 600ms linear; }
    /// a { animation: slide 8s, fade 0s; }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `minimum`
    ///
    /// Type: `number`
    ///
    /// Default: none. The rule does nothing until this option is set.
    ///
    /// The shortest time allowed, as a whole number of milliseconds. Times written in seconds are
    /// converted before they are compared, so `0.1s` is the same as `100ms`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "minimum": 200
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// a { animation: slide 150ms ease-in; }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// a { animation: slide 0.2s ease-in; }
    /// ```
    ///
    pub NoShortTimeValues {
        version: "2.6.3",
        name: "noShortTimeValues",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("time-min-milliseconds").same()],
    }
}

impl Rule for NoShortTimeValues {
    type Query = Ast<CssRegularDimension>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoShortTimeValuesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let minimum = ctx.options().minimum()?;
        let dimension = ctx.query();

        let unit = dimension.unit_token().ok()?;
        let unit = unit.text_trimmed();
        let milliseconds_per_unit = if unit.eq_ignore_ascii_case("ms") {
            1.0
        } else if unit.eq_ignore_ascii_case("s") {
            1000.0
        } else {
            return None;
        };

        let value: f64 = dimension.value_token().ok()?.text_trimmed().parse().ok()?;
        // The minimum is divided instead of multiplying the value, because the
        // division rounds to the same float that the source text parses to.
        // Multiplying doesn't: `1.001 * 1000.0` is less than `1001.0`.
        if value <= 0.0 || value >= f64::from(minimum) / milliseconds_per_unit {
            return None;
        }

        is_time_property_value(dimension).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let minimum = ctx.options().minimum()?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This time is shorter than the configured minimum of "<Emphasis>{minimum}"ms"</Emphasis>"."
                },
            )
            .note(markup! {
                "Times this short are hard to notice, and an animation or transition that is too short can look like flickering."
            })
            .note(markup! {
                "Use a time of at least "<Emphasis>{minimum}"ms"</Emphasis>"."
            }),
        )
    }
}

/// Returns `true` if `dimension` is part of the value of one of the
/// [`TIME_PROPERTIES`].
fn is_time_property_value(dimension: &CssRegularDimension) -> bool {
    let Some(statement) = dimension
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyPropertyStatement::cast)
    else {
        return false;
    };

    // The block of a group can hold times that don't belong to any property,
    // such as the value of an SCSS variable.
    if let AnyPropertyStatement::ScssNestingDeclaration(group) = &statement
        && !group
            .value()
            .is_ok_and(|value| value.range().contains_range(dimension.range()))
    {
        return false;
    }

    let Some(name) = statement.name() else {
        return false;
    };
    let Some(group) = statement.group() else {
        return is_time_property_name(name.text_trimmed());
    };
    nested_property_name(name.text_trimmed(), group)
        .is_some_and(|full_name| is_time_property_name(&full_name))
}

fn is_time_property_name(name: &str) -> bool {
    let name = name.to_ascii_lowercase_cow();
    let name = name.strip_prefix(vender_prefix(&name)).unwrap_or(&name);
    TIME_PROPERTIES.binary_search(&name).is_ok()
}

/// The properties whose times are checked, without browser prefix.
///
/// Must stay sorted for the binary search in [`is_time_property_name`].
const TIME_PROPERTIES: [&str; 6] = [
    "animation",
    "animation-delay",
    "animation-duration",
    "transition",
    "transition-delay",
    "transition-duration",
];

#[cfg(test)]
mod tests {
    use super::TIME_PROPERTIES;

    #[test]
    fn time_properties_are_sorted() {
        for pair in TIME_PROPERTIES.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} should be before {}",
                pair[1],
                pair[0]
            );
        }
    }
}
