use biome_css_syntax::{ScssExpression, single_expression_item};

/// Describes who owns wrapping for an `@if` or `@while` condition.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ScssControlConditionLayout {
    /// The control header wraps plain chains such as `@if $a == 0 and $b == 0`.
    HeaderOwned,
    /// The call wraps its arguments, as in `@if foo($a, $b)`.
    FunctionCall,
    /// The delimiters wrap their contents, as in `@if ($a, $b)`.
    DelimitedExpression,
}

impl ScssControlConditionLayout {
    pub(crate) fn from_condition(condition: &ScssExpression) -> Self {
        let Some(item) = single_expression_item(condition) else {
            return Self::HeaderOwned;
        };

        if item.as_scss_parenthesized_expression().is_some()
            || item.as_scss_list_expression().is_some()
            || item.as_scss_map_expression().is_some()
        {
            return Self::DelimitedExpression;
        }

        if item
            .as_any_css_value()
            .and_then(|value| value.as_any_css_function())
            .is_some()
        {
            return Self::FunctionCall;
        }

        Self::HeaderOwned
    }

    pub(crate) const fn should_indent_condition(self) -> bool {
        matches!(self, Self::HeaderOwned)
    }

    pub(crate) const fn should_keep_block_on_same_line(self) -> bool {
        matches!(self, Self::DelimitedExpression)
    }
}
