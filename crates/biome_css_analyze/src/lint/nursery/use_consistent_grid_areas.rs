use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleList;
use biome_rowan::{TextRange, TokenText};

use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Disallows invalid named grid areas in CSS Grid Layouts.
    ///
    /// For a named grid area to be valid, all strings must define:
    ///
    /// - the same number of cell tokens
    /// - at least one cell token
    ///
    /// And all named grid areas that spans multiple grid cells must form a single filled-in rectangle.
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
    pub UseConsistentGridAreas {
        version: "next",
        name: "useConsistentGridAreas",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("named-grid-areas-no-invalid")],
    }
}

type GridAreasProp = (String, TextRange);
type GridAreasProps = Vec<(TokenText, TextRange)>;

#[derive(Debug)]
enum GridAreaValidationError {
    EmptyGridArea,
    InconsistentCellCount,
    DuplicateGridToken,
}

pub struct UseConsistentGridAreasState {
    text: Option<String>,
    span: TextRange,
    reason: GridAreaValidationError,
}

impl Rule for UseConsistentGridAreas {
    type Query = Ast<CssDeclarationOrRuleList>;
    type State = UseConsistentGridAreasState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        // Extracting the property values of grid-template-areas
        let plain_grid_areas_props = node
            .into_iter()
            .filter_map(|item| {
                let grid_props = item
                    .as_css_declaration_with_semicolon()?
                    .declaration()
                    .ok()?
                    .property()
                    .ok()?
                    .as_css_generic_property()?
                    .value();
                Some(grid_props)
            })
            .flat_map(|grid_props| {
                grid_props
                    .into_iter()
                    .filter_map(|x| x.as_any_css_value()?.as_css_string()?.value_token().ok())
            })
            // Need to remove `"` with escaping slash from the grid area
            // Ex: "\"a a a\""
            .map(|x| {
                let trimmed_text = x.token_text();
                let text_range = x.text_range();
                (trimmed_text, text_range)
            })
            .collect::<GridAreasProps>();

        if !plain_grid_areas_props.is_empty() {
            is_consistent_grids(plain_grid_areas_props)
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state.reason {
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
            GridAreaValidationError::DuplicateGridToken => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    state.span,
                    markup! {
                        "Duplicate filled in rectangle are not allowed."
                    },
                )
                .note(markup! {
                    "Consider removing the duplicated filled-in rectangle: " <Emphasis>{state.text.as_ref().unwrap()}</Emphasis>
                }),
            ),
        }
    }
}

// Check if the grid areas are consistent
fn is_consistent_grids(grid_areas_props: GridAreasProps) -> Option<UseConsistentGridAreasState> {
    let first_prop = clean_text(&grid_areas_props[0].0);
    let first_len = first_prop.len();
    let mut shortest = &grid_areas_props[0];

    for grid_areas_prop in &grid_areas_props {
        let cleaned_text = clean_text(&grid_areas_prop.0);
        // Check if the grid areas are empty
        if cleaned_text.is_empty() {
            return Some(UseConsistentGridAreasState {
                text: None,
                span: grid_areas_prop.1,
                reason: GridAreaValidationError::EmptyGridArea,
            });
        }
        // Check if all elements have the same length
        if cleaned_text.len() != first_len {
            if cleaned_text.len() < clean_text(&shortest.0).len() {
                shortest = grid_areas_prop;
            }
            return Some(UseConsistentGridAreasState {
                text: None,
                span: shortest.1,
                reason: GridAreaValidationError::InconsistentCellCount,
            });
        }
    }

    // Check if there are no duplicate grid tokens
    // It should be partial match because for example, in the following grid areas:
    // {"a a a"
    //  "b b b"; }
    //  are the consistent grid properties because it forms a single filled-in rectangle.
    if grid_areas_props
        .iter()
        .all(|prop| is_all_same(prop.0.clone()))
    {
        return None;
    }
    //  But in the following grid areas:
    //  {"a a a"
    //   "b b a"; }
    //   are not consistent because `a` breaks a single filled-in rectangle.
    if let Some(result) = has_partial_match(&grid_areas_props) {
        return Some(UseConsistentGridAreasState {
            text: Some(result.0),
            span: result.1,
            reason: GridAreaValidationError::DuplicateGridToken,
        });
    }

    None
}

// Check if all characters in a string are the same
fn is_all_same(token_text: TokenText) -> bool {
    let prop = clean_text(&token_text);
    let chars: Vec<char> = prop.chars().filter(|c| !c.is_whitespace()).collect();
    let head = chars[0];
    chars.iter().all(|&c| c == head)
}

fn has_partial_match(grid_areas_props: &GridAreasProps) -> Option<GridAreasProp> {
    let mut seen_parts = FxHashSet::default();

    for (text, range) in grid_areas_props {
        let prop = clean_text(text);
        let parts: FxHashSet<String> = prop
            .split_whitespace()
            .map(|part| part.to_string())
            .collect();
        for part in parts {
            if !seen_parts.insert(part.clone()) {
                return Some((part, *range));
            }
        }
    }

    None
}

fn clean_text(text: &TokenText) -> String {
    text.replace('"', "").trim().to_string()
}
