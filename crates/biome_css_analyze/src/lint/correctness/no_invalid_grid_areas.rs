use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssGenericPropertyValueOrExpression, AnyCssValue, AnyScssExpressionItem,
    CssDeclarationOrRuleList, CssSyntaxToken, css_grid_template_property, inner_string_text,
};
use biome_diagnostics::Severity;
use biome_rowan::{TextRange, TextSize, TokenText};
use biome_rule_options::no_invalid_grid_areas::NoInvalidGridAreasOptions;

use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

declare_lint_rule! {
    /// Disallow invalid named areas in CSS grid templates.
    ///
    /// Each quoted row in `grid-template-areas` contains space-separated cells. Every row must
    /// contain the same number of cells and must not be empty. When an area name appears in more
    /// than one cell, those cells must form one filled rectangle.
    ///
    /// ## SCSS limitations
    ///
    /// A `grid-template-areas` value that requires SCSS evaluation is ignored. This includes values
    /// containing variables, interpolation, functions, or arithmetic expressions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { grid-template-areas: "a a"
    ///                          "b b b"; }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { grid-template-areas: "b b b"
    ///                          ""; }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { grid-template-areas: "a a a"
    ///                          "b b a"; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { grid-template-areas: "a a a"
    ///                          "b b b"; }
    /// ```
    ///
    /// ```css
    /// a { grid-template-areas: "a a a"
    ///                          "a a a"; }
    /// ```
    ///
    pub NoInvalidGridAreas {
        version: "1.9.0",
        name: "noInvalidGridAreas",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("named-grid-areas-no-invalid").same(), RuleSource::EslintCss("no-invalid-named-grid-areas").same()],
    }
}

#[derive(Debug)]
enum GridAreaValidationError {
    EmptyGridArea,
    InconsistentCellCount,
    /// The name of an area whose cells do not form one filled rectangle.
    DuplicateGridToken(TokenText),
}

pub struct UseConsistentGridAreasState {
    span: TextRange,
    reason: GridAreaValidationError,
}

impl Rule for NoInvalidGridAreas {
    type Query = Ast<CssDeclarationOrRuleList>;
    type State = UseConsistentGridAreasState;
    type Signals = Option<Self::State>;
    type Options = NoInvalidGridAreasOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        // Extract named grid-area strings from `grid`, `grid-template`, and
        // `grid-template-areas` declarations.
        let rows = node
            .into_iter()
            .filter_map(|item| {
                let binding = item
                    .as_css_declaration_with_semicolon()?
                    .declaration()
                    .ok()?
                    .property()
                    .ok()?;

                let decl = binding.as_css_generic_property()?;
                let name = decl.name().ok()?.as_css_identifier()?.value_token().ok()?;

                if css_grid_template_property(name.text_trimmed())
                    .is_some_and(|property| property.is_grid_area_property())
                {
                    let grid_props = decl.value();
                    return Some(grid_props);
                }
                None
            })
            .flat_map(|grid_props| {
                grid_props
                    .ok()
                    .map_or_else(Vec::new, grid_area_string_tokens)
            })
            .map(|token| GridRow {
                content: inner_string_text(&token),
                range: token.text_trimmed_range(),
            })
            .collect::<Vec<_>>();

        check_grid_rows(&rows)
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match &state.reason {
            GridAreaValidationError::EmptyGridArea => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    state.span,
                    markup! {
                        "Empty grid areas are not allowed."
                    },
                )
                .note(markup! {
                    "Consider adding the cell token within string."
                }),
            ),
            GridAreaValidationError::InconsistentCellCount => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    state.span,
                    markup! {
                        "Inconsistent cell count in grid areas are not allowed."
                    },
                )
                .note(markup! {
                    "Consider adding the same number of cell tokens in each string."
                }),
            ),
            GridAreaValidationError::DuplicateGridToken(name) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    state.span,
                    markup! {
                        "Duplicate filled in rectangle are not allowed."
                    },
                )
                .note(markup! {
                    "Consider removing the duplicated filled-in rectangle: " <Emphasis>{name.text()}</Emphasis>
                }),
            ),
        }
    }
}

fn grid_area_string_tokens(value: AnyCssGenericPropertyValueOrExpression) -> Vec<CssSyntaxToken> {
    match value {
        AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(list) => list
            .into_iter()
            .filter_map(|value| {
                value
                    .as_any_css_value()?
                    .as_css_string()?
                    .value_token()
                    .ok()
            })
            .collect(),
        AnyCssGenericPropertyValueOrExpression::ScssExpression(expression) => {
            let mut strings = Vec::new();
            for item in expression.items() {
                match item {
                    AnyScssExpressionItem::AnyCssValue(AnyCssValue::CssString(string)) => {
                        if let Ok(token) = string.value_token() {
                            strings.push(token);
                        }
                    }
                    AnyScssExpressionItem::AnyCssValue(
                        AnyCssValue::AnyCssFunction(_)
                        | AnyCssValue::ScssInterpolatedIdentifier(_)
                        | AnyCssValue::ScssInterpolatedString(_)
                        | AnyCssValue::ScssInterpolatedValue(_)
                        | AnyCssValue::ScssModuleMemberAccess(_)
                        | AnyCssValue::ScssVariable(_),
                    ) => return Vec::new(),
                    AnyScssExpressionItem::AnyCssValue(_)
                    | AnyScssExpressionItem::CssGenericDelimiter(_) => {}
                    _ => return Vec::new(),
                }
            }
            strings
        }
        AnyCssGenericPropertyValueOrExpression::CssCustomPropertyValue(_)
        | AnyCssGenericPropertyValueOrExpression::CssLegacyFilterValue(_) => Vec::new(),
    }
}

/// A quoted row of a grid template.
struct GridRow {
    /// The text between the quotes.
    content: TokenText,
    range: TextRange,
}

/// The cells covered so far by a named area: columns `start..end` of every row up to `last_row`.
struct AreaBounds {
    last_row: usize,
    start: usize,
    end: usize,
}

/// Checks that every row contains at least one cell, that all rows contain the same number of
/// cells, and that the cells of each named area form one filled rectangle.
fn check_grid_rows(rows: &[GridRow]) -> Option<UseConsistentGridAreasState> {
    let first_row = rows.first()?;
    let first_count = GridCells::new(first_row.content.text()).count();

    for row in rows {
        let count = GridCells::new(row.content.text()).count();
        if count == 0 {
            return Some(UseConsistentGridAreasState {
                span: row.range,
                reason: GridAreaValidationError::EmptyGridArea,
            });
        }
        if count != first_count {
            // Point at the row with fewer cells, because that is the row missing cells.
            let span = if count < first_count {
                row.range
            } else {
                first_row.range
            };
            return Some(UseConsistentGridAreasState {
                span,
                reason: GridAreaValidationError::InconsistentCellCount,
            });
        }
    }

    // An area forms a rectangle when it covers the same consecutive columns in consecutive rows.
    let mut areas = FxHashMap::<&str, AreaBounds>::default();
    for (row_index, row) in rows.iter().enumerate() {
        let text = row.content.text();
        let mut cells = GridCells::new(text).enumerate().peekable();
        while let Some((start, range)) = cells.next() {
            let name = &text[range];
            let mut end = start + 1;
            while cells.next_if(|(_, next)| &text[*next] == name).is_some() {
                end += 1;
            }
            if is_null_cell(name) {
                continue;
            }
            match areas.entry(name) {
                Entry::Vacant(entry) => {
                    entry.insert(AreaBounds {
                        last_row: row_index,
                        start,
                        end,
                    });
                }
                Entry::Occupied(mut entry) => {
                    let bounds = entry.get_mut();
                    if bounds.last_row + 1 != row_index
                        || bounds.start != start
                        || bounds.end != end
                    {
                        return Some(UseConsistentGridAreasState {
                            span: row.range,
                            reason: GridAreaValidationError::DuplicateGridToken(
                                row.content.clone().slice(range),
                            ),
                        });
                    }
                    bounds.last_row = row_index;
                }
            }
        }
    }

    None
}

/// Iterator over the ranges of the cells in a grid row, relative to the row. Cells are separated
/// by whitespace, and a sequence of `.` is a cell of its own even when it touches a name: `a.b`
/// contains three cells.
struct GridCells<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> GridCells<'a> {
    fn new(row: &'a str) -> Self {
        Self {
            bytes: row.as_bytes(),
            index: 0,
        }
    }
}

impl Iterator for GridCells<'_> {
    type Item = TextRange;

    fn next(&mut self) -> Option<Self::Item> {
        while self.bytes.get(self.index)?.is_ascii_whitespace() {
            self.index += 1;
        }
        let start = self.index;
        let is_dot = self.bytes[start] == b'.';
        while self
            .bytes
            .get(self.index)
            .is_some_and(|byte| !byte.is_ascii_whitespace() && (*byte == b'.') == is_dot)
        {
            self.index += 1;
        }
        Some(TextRange::new(
            TextSize::from(start as u32),
            TextSize::from(self.index as u32),
        ))
    }
}

/// A cell made only of `.` characters leaves its grid cell unnamed.
fn is_null_cell(cell: &str) -> bool {
    cell.bytes().all(|byte| byte == b'.')
}
