use crate::{CssRuleAction, utils::vender_prefix};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_factory::make;
use biome_css_syntax::{
    CssCustomPropertyFunction, CssFunction, CssGenericProperty, CssRegularDimension, CssSyntaxKind,
    T, decode_css_identifier, keywords::LENGTH_UNITS,
};
use biome_diagnostics::Applicability;
use biome_languages::CssFileSource;
use biome_rowan::{AstNode, BatchMutationExt, declare_node_union};
use biome_rule_options::no_zero_length_unit::NoZeroLengthUnitOptions;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Disallow units on zero lengths.
    ///
    /// A length of zero is the same distance in every unit, so `0px`, `0em`, and `0rem` all mean the same as `0`.
    /// CSS allows a zero length to be written without a unit, which is shorter and keeps zero values consistent.
    ///
    /// This rule checks length units, such as `px`, `em`, `rem`, `vw`, and `cm`.
    /// It doesn't report zeros with other kinds of units, such as `0s`, `0deg`, `0%`, and `0fr`, because only lengths can be written without a unit.
    ///
    /// The rule also doesn't report a zero length in the following places, where removing the unit changes the meaning of the value or makes it invalid:
    ///
    /// - Inside math functions, such as `calc()`, `min()`, `max()`, and `clamp()`.
    /// - In the `line-height` and `flex` properties, and in the line height of the `font` property, which is the value after the `/`.
    ///   In these places, a `0` without a unit is read as a plain number instead of a length.
    /// - In the value of a Sass variable, such as `$gap: 0px`.
    ///
    /// ## Custom properties
    ///
    /// The rule reports zero lengths in custom properties, such as `--gap: 0px`, but the fix is unsafe there.
    /// Removing the unit can break a `calc()` that uses the custom property:
    /// `calc(var(--gap) + 1px)` is valid when `--gap` is `0px`, and invalid when `--gap` is `0`.
    /// For the same reason, the fix is unsafe for values passed to or returned from custom functions, which are functions defined with `@function`.
    ///
    /// The fix is also unsafe in SCSS files, because Sass calculations treat `0px` and `0` differently.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   top: 0px;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   margin: 0.0em auto;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @media (min-width: 0px) {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   --gap: 0px;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a {
    ///   top: 0;
    ///   left: 2px;
    ///   transition-delay: 0s;
    ///   width: calc(0px + 100%);
    ///   line-height: 0px;
    ///   flex: 1 1 0px;
    ///   font: 16px/0px sans-serif;
    /// }
    /// ```
    ///
    pub NoZeroLengthUnit {
        version: "2.6.3",
        name: "noZeroLengthUnit",
        language: "css",
        recommended: true,
        sources: &[RuleSource::Stylelint("length-zero-no-unit").same()],
        fix_kind: FixKind::Unsafe,
    }
}

/// How CSS uses the value that contains a reported zero length.
#[derive(Clone, Copy)]
pub enum ZeroLengthUse {
    /// CSS copies the value into other places: the value of a custom
    /// property, or an argument, default parameter value, or result of a
    /// custom function. One of those places can be a calculation, where a
    /// length needs its unit.
    Substituted,
    /// The value is used where it is written.
    Direct,
}

impl Rule for NoZeroLengthUnit {
    type Query = Ast<CssRegularDimension>;
    type State = ZeroLengthUse;
    type Signals = Option<Self::State>;
    type Options = NoZeroLengthUnitOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let dimension = ctx.query();
        if !is_zero(dimension) || !dimension.matches_unit(LENGTH_UNITS) {
            return None;
        }

        let mut is_substituted = false;
        for ancestor in dimension.syntax().ancestors().skip(1) {
            match ancestor.kind() {
                CssSyntaxKind::SCSS_VARIABLE_DECLARATION => return None,
                CssSyntaxKind::CSS_FUNCTION | CssSyntaxKind::CSS_CUSTOM_PROPERTY_FUNCTION => {
                    match function_kind(&AnyCssNamedFunction::cast(ancestor)?) {
                        Some(FunctionKind::Math) => return None,
                        Some(FunctionKind::Custom) => is_substituted = true,
                        None => {}
                    }
                }
                CssSyntaxKind::CSS_FUNCTION_PARAMETER_DEFAULT_VALUE => is_substituted = true,
                CssSyntaxKind::CSS_GENERIC_PROPERTY => {
                    let property_use =
                        property_use(dimension, &CssGenericProperty::cast(ancestor)?)?;
                    return Some(if is_substituted {
                        ZeroLengthUse::Substituted
                    } else {
                        property_use
                    });
                }
                _ => {}
            }
        }

        Some(if is_substituted {
            ZeroLengthUse::Substituted
        } else {
            ZeroLengthUse::Direct
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let unit = ctx.query().unit_token().ok()?;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            unit.text_trimmed_range(),
            markup! {
                "The unit "<Emphasis>{unit.text_trimmed()}</Emphasis>" is unnecessary on a zero length."
            },
        )
        .note(markup! {
            "A length of zero is the same in every unit, so "<Emphasis>"0"</Emphasis>" is enough."
        });

        Some(match state {
            ZeroLengthUse::Substituted => diagnostic.note(markup! {
                "Here, "<Emphasis>"0"</Emphasis>" and a zero length aren't always interchangeable. For example, CSS can copy this value into a "<Emphasis>"calc()"</Emphasis>", where a zero without a unit can't be added to or subtracted from a length. Check how the value is used before removing the unit."
            }),
            ZeroLengthUse::Direct => diagnostic,
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<CssRuleAction> {
        let dimension = ctx.query();
        let number = make::css_number(dimension.value_token().ok()?);
        let mut mutation = ctx.root().begin();
        mutation.replace_element(
            dimension.syntax().clone().into(),
            number.into_syntax().into(),
        );

        // The rule declares an unsafe fix because editors label a fix from the
        // rule metadata alone, before this function runs. That label must not
        // present the fixes below as safe.
        //
        // Sass evaluates the values in SCSS files, and its arithmetic treats
        // `0` and a zero length differently.
        let applicability = if matches!(state, ZeroLengthUse::Substituted)
            || ctx.source_type::<CssFileSource>().is_scss()
        {
            Applicability::MaybeIncorrect
        } else {
            Applicability::Always
        };

        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            applicability,
            markup! { "Remove the unit." }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    /// `CssCustomPropertyFunction` is a function call inside a value that the
    /// parser keeps as written, such as a custom property value in SCSS files.
    pub AnyCssNamedFunction = CssFunction | CssCustomPropertyFunction
}

enum FunctionKind {
    /// The arguments are a calculation. A zero without a unit is a plain
    /// number there, which can't be combined with a length.
    Math,
    /// A function defined with `@function`, whose arguments CSS copies into
    /// the function body.
    Custom,
}

/// Lowercase names of the math functions, without a vendor prefix.
const MATH_FUNCTIONS: &[&str] = &[
    "abs",
    "acos",
    "asin",
    "atan",
    "atan2",
    "calc",
    "calc-size",
    "clamp",
    "cos",
    "exp",
    "hypot",
    "log",
    "max",
    "min",
    "mod",
    "pow",
    "progress",
    "random",
    "rem",
    "round",
    "sign",
    "sin",
    "sqrt",
    "tan",
];

fn is_zero(dimension: &CssRegularDimension) -> bool {
    dimension.value_token().is_ok_and(|token| {
        token
            .text_trimmed()
            .parse::<f64>()
            .is_ok_and(|value| value == 0.0)
    })
}

/// Returns `None` for a function that is neither a math function nor a custom
/// function.
fn function_kind(function: &AnyCssNamedFunction) -> Option<FunctionKind> {
    let token = match function {
        AnyCssNamedFunction::CssFunction(function) => function
            .name()
            .ok()?
            .as_css_identifier()?
            .value_token()
            .ok()?,
        AnyCssNamedFunction::CssCustomPropertyFunction(function) => {
            function.name().ok()?.value_token().ok()?
        }
    };
    let name = decode_css_identifier(token.text_trimmed());
    if name.starts_with("--") {
        return Some(FunctionKind::Custom);
    }

    let name = name.to_ascii_lowercase_cow();
    MATH_FUNCTIONS
        .binary_search(&&name[vender_prefix(&name).len()..])
        .is_ok()
        .then_some(FunctionKind::Math)
}

/// Returns how CSS uses a zero length written in the value of `property`.
///
/// Returns `None` when the property reads a `0` without a unit as a number
/// instead of a length.
fn property_use(
    dimension: &CssRegularDimension,
    property: &CssGenericProperty,
) -> Option<ZeroLengthUse> {
    let name = property.name().ok()?;
    if name.as_any_css_dashed_identifier().is_some() {
        return Some(ZeroLengthUse::Substituted);
    }

    let Some(name) = name.identifier_text() else {
        return Some(ZeroLengthUse::Direct);
    };
    let name = decode_css_identifier(name.text());
    let name = name.to_ascii_lowercase_cow();

    match &name[vender_prefix(&name).len()..] {
        "line-height" | "flex" => None,
        "font" => {
            let is_line_height = dimension
                .value_token()
                .ok()?
                .prev_token()
                .is_some_and(|token| token.kind() == T![/]);
            (!is_line_height).then_some(ZeroLengthUse::Direct)
        }
        // These descriptors set the value of a custom property in `@property`
        // and the result of a custom function in `@function`.
        "initial-value" | "result" => Some(ZeroLengthUse::Substituted),
        _ => Some(ZeroLengthUse::Direct),
    }
}

#[cfg(test)]
mod tests {
    use super::MATH_FUNCTIONS;

    #[test]
    fn math_functions_are_sorted() {
        for pair in MATH_FUNCTIONS.windows(2) {
            assert!(pair[0] < pair[1], "{} < {}", pair[0], pair[1]);
        }
    }
}
