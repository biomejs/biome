use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_factory::make;
use biome_css_syntax::{
    AnyCssValue, CssGenericProperty, CssIdentifier, CssNumber,
    CssSyntaxKind::{CSS_NUMBER_LITERAL, IDENT},
    CssSyntaxToken,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxResult, declare_node_union};
use biome_rule_options::use_consistent_font_weight_notation::{
    FontWeightNotation, UseConsistentFontWeightNotationOptions,
};

use crate::CssRuleAction;
use crate::fonts::{CssFontComponent, font_components};

declare_lint_rule! {
    /// Enforce a consistent notation for font weights.
    ///
    /// A font weight can be written as a number, such as `700`, or as a keyword, such as `bold`.
    /// The keywords `normal` and `bold` are the same as the numbers `400` and `700`. Writing font
    /// weights the same way everywhere makes styles easier to read and compare.
    ///
    /// This rule checks the `font-weight` property, including the two weights allowed in
    /// `@font-face`, and the weight in the `font` shorthand property. Values that come from
    /// variables, such as `var(--weight)` or Sass variables, are ignored.
    ///
    /// The keywords `bolder` and `lighter` set a weight relative to the parent element, so no
    /// single number replaces them. With the `"numeric"` notation, they are reported without a
    /// fix unless the `ignoreRelative` option is enabled.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { font-weight: bold; }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { font: italic normal 20px sans-serif; }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { font-weight: lighter; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { font-weight: 700; }
    /// a { font: italic 400 20px sans-serif; }
    /// a { font-weight: var(--weight); }
    /// @font-face { font-weight: 400 700; }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `notation`
    ///
    /// Default: `"numeric"`
    ///
    /// Whether font weights are written as numbers (`"numeric"`), or as keywords wherever a
    /// keyword exists (`"namedWherePossible"`). Only `400` and `700` have keywords, so
    /// `"namedWherePossible"` reports only those two numbers.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "notation": "namedWherePossible"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// a { font-weight: 700; }
    /// ```
    ///
    /// ```css,expect_diagnostic,use_options
    /// a { font: italic 400 20px sans-serif; }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// a { font-weight: bold; }
    /// a { font-weight: 600; }
    /// a { font: italic normal 20px sans-serif; }
    /// ```
    ///
    /// ### `ignoreRelative`
    ///
    /// Default: `false`
    ///
    /// Whether to allow the keywords `bolder` and `lighter` when `notation` is `"numeric"`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreRelative": true
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// a { font-weight: bold; }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// a { font-weight: bolder; }
    /// a { font: italic lighter 16px/3 cursive; }
    /// ```
    ///
    pub UseConsistentFontWeightNotation {
        version: "2.6.7",
        name: "useConsistentFontWeightNotation",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("font-weight-notation").same()],
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub AnyCssFontWeightValue = CssIdentifier | CssNumber
}

/// A font weight written as a keyword, or the keyword for a number that has one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontWeightKeyword {
    Normal,
    Bold,
    Bolder,
    Lighter,
}

impl Rule for UseConsistentFontWeightNotation {
    type Query = Ast<CssGenericProperty>;
    type State = (AnyCssFontWeightValue, FontWeightKeyword);
    type Signals = Box<[Self::State]>;
    type Options = UseConsistentFontWeightNotationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let Some(name) = node.name().ok().and_then(|name| name.identifier_text()) else {
            return Box::default();
        };
        let is_shorthand = if name.eq_ignore_ascii_case("font") {
            true
        } else if name.eq_ignore_ascii_case("font-weight") {
            false
        } else {
            return Box::default();
        };
        let Some(components) = node.value().ok().and_then(font_components) else {
            return Box::default();
        };

        let options = ctx.options();
        if is_shorthand {
            find_shorthand_font_weight(&components)
                .and_then(|value| check_font_weight(value, options))
                .into_iter()
                .collect()
        } else {
            components
                .iter()
                .filter_map(|component| as_font_weight_value(component.as_value()?))
                .filter_map(|value| check_font_weight(value, options))
                .collect()
        }
    }

    fn diagnostic(_: &RuleContext<Self>, (value, keyword): &Self::State) -> Option<RuleDiagnostic> {
        let token = value.value_token().ok()?;
        let text = token.text_trimmed();
        let diagnostic = match value {
            AnyCssFontWeightValue::CssIdentifier(_) => RuleDiagnostic::new(
                rule_category!(),
                value.range(),
                markup! {
                    "This font weight is written as the keyword "<Emphasis>{text}</Emphasis>" instead of a number."
                },
            ),
            AnyCssFontWeightValue::CssNumber(_) => RuleDiagnostic::new(
                rule_category!(),
                value.range(),
                markup! {
                    "This font weight is written as the number "<Emphasis>{text}</Emphasis>" instead of the keyword "<Emphasis>{keyword}</Emphasis>"."
                },
            ),
        }
        .note(markup! {
            "Writing font weights the same way everywhere makes styles easier to read and compare."
        });

        Some(if keyword.numeric_value().is_none() {
            diagnostic.note(markup! {
                "The keyword "<Emphasis>{keyword}</Emphasis>" sets the weight relative to the parent element, so no single number can replace it. Use a number instead, or enable the "<Emphasis>"ignoreRelative"</Emphasis>" option to allow relative keywords."
            })
        } else {
            diagnostic
        })
    }

    fn action(ctx: &RuleContext<Self>, (value, keyword): &Self::State) -> Option<CssRuleAction> {
        let mut mutation = ctx.root().begin();
        let replacement = match value {
            AnyCssFontWeightValue::CssIdentifier(identifier) => {
                let number = keyword.numeric_value()?;
                mutation.replace_node(
                    AnyCssValue::CssIdentifier(identifier.clone()),
                    AnyCssValue::CssNumber(make::css_number(CssSyntaxToken::new_detached(
                        CSS_NUMBER_LITERAL,
                        number,
                        [],
                        [],
                    ))),
                );
                number
            }
            AnyCssFontWeightValue::CssNumber(number) => {
                let name = keyword.name();
                mutation.replace_node(
                    AnyCssValue::CssNumber(number.clone()),
                    AnyCssValue::CssIdentifier(make::css_identifier(
                        CssSyntaxToken::new_detached(IDENT, name, [], []),
                    )),
                );
                name
            }
        };
        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{replacement}</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

impl AnyCssFontWeightValue {
    fn value_token(&self) -> SyntaxResult<CssSyntaxToken> {
        match self {
            Self::CssIdentifier(identifier) => identifier.value_token(),
            Self::CssNumber(number) => number.value_token(),
        }
    }
}

impl FontWeightKeyword {
    fn from_identifier(identifier: &CssIdentifier) -> Option<Self> {
        let token = identifier.value_token().ok()?;
        let text = token.text_trimmed();
        [Self::Normal, Self::Bold, Self::Bolder, Self::Lighter]
            .into_iter()
            .find(|keyword| text.eq_ignore_ascii_case(keyword.name()))
    }

    /// Returns the keyword for numbers equal to `400` or `700`, including forms such as `400.0`.
    fn from_number(number: &CssNumber) -> Option<Self> {
        let value: f64 = number.value_token().ok()?.text_trimmed().parse().ok()?;
        if value == 400.0 {
            Some(Self::Normal)
        } else if value == 700.0 {
            Some(Self::Bold)
        } else {
            None
        }
    }

    const fn name(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Bold => "bold",
            Self::Bolder => "bolder",
            Self::Lighter => "lighter",
        }
    }

    /// Returns the number with the same weight, or `None` for the relative keywords.
    const fn numeric_value(self) -> Option<&'static str> {
        match self {
            Self::Normal => Some("400"),
            Self::Bold => Some("700"),
            Self::Bolder | Self::Lighter => None,
        }
    }
}

impl biome_console::fmt::Display for FontWeightKeyword {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(self.name())
    }
}

/// Returns the font weight value when it doesn't follow the selected notation.
fn check_font_weight(
    value: AnyCssFontWeightValue,
    options: &UseConsistentFontWeightNotationOptions,
) -> Option<(AnyCssFontWeightValue, FontWeightKeyword)> {
    let keyword = match (options.notation(), &value) {
        (FontWeightNotation::Numeric, AnyCssFontWeightValue::CssIdentifier(identifier)) => {
            let keyword = FontWeightKeyword::from_identifier(identifier)?;
            if keyword.numeric_value().is_none() && options.ignore_relative() {
                return None;
            }
            keyword
        }
        (FontWeightNotation::NamedWherePossible, AnyCssFontWeightValue::CssNumber(number)) => {
            FontWeightKeyword::from_number(number)?
        }
        _ => return None,
    };
    Some((value, keyword))
}

fn as_font_weight_value(value: &AnyCssValue) -> Option<AnyCssFontWeightValue> {
    match value {
        AnyCssValue::CssIdentifier(identifier) => Some(identifier.clone().into()),
        AnyCssValue::CssNumber(number) => Some(number.clone().into()),
        _ => None,
    }
}

/// Finds the font weight in the components of a `font` shorthand value.
///
/// The weight comes before the font size, so the search stops at the first size, string, `/`,
/// or `,`. The keyword `normal` can also set the style, variant, or stretch, so it is the weight
/// only when no number, other weight keyword, or function appears. For example, in
/// `font: normal bold 16px serif` the weight is `bold`, and in
/// `font: normal var(--weight) 16px serif` the weight is unknown.
fn find_shorthand_font_weight(components: &[CssFontComponent]) -> Option<AnyCssFontWeightValue> {
    let mut first_normal = None;
    let mut has_function = false;
    for component in components {
        let value = match component {
            CssFontComponent::Value(
                AnyCssValue::AnyCssDimension(_) | AnyCssValue::CssString(_),
            )
            | CssFontComponent::Slash
            | CssFontComponent::Comma
            | CssFontComponent::OtherDelimiter => break,
            CssFontComponent::Value(value) => value,
        };
        let Some(value) = as_font_weight_value(value) else {
            has_function |= matches!(value, AnyCssValue::AnyCssFunction(_));
            continue;
        };
        match &value {
            AnyCssFontWeightValue::CssNumber(_) => return Some(value),
            AnyCssFontWeightValue::CssIdentifier(identifier) => {
                match FontWeightKeyword::from_identifier(identifier) {
                    Some(FontWeightKeyword::Normal) => {
                        first_normal.get_or_insert(value);
                    }
                    Some(_) => return Some(value),
                    None => {}
                }
            }
        }
    }
    if has_function { None } else { first_normal }
}
