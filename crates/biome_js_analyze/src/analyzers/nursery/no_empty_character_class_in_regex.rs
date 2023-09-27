use std::ops::Range;

use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsRegexLiteralExpression;
use biome_rowan::{TextRange, TextSize};

declare_rule! {
    /// Disallow empty character classes in regular expression literals.
    ///
    /// Empty character classes don't match anything.
    /// In contrast, negated empty classes match any character.
    /// They are often the result of a typing mistake.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-empty-character-class/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// /^a[]/.test("a"); // false
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /^a[^]/.test("ax"); // true
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// /^a[xy]/.test("ay"); // true
    /// ```
    ///
    /// ```js
    /// /^a[^xy]/.test("ab"); // true
    /// ```
    ///
    /// ```js
    /// /^a\[]/.test("a[]"); // true
    /// ```
    ///
    pub(crate) NoEmptyCharacterClassInRegex {
        version: "next",
        name: "noEmptyCharacterClassInRegex",
        recommended: true,
    }
}

impl Rule for NoEmptyCharacterClassInRegex {
    type Query = Ast<JsRegexLiteralExpression>;
    type State = Range<usize>;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut empty_classes = vec![];
        let regex = ctx.query();
        let (Ok(regex_token), Ok(regex_flags)) = (regex.value_token(), regex.flags()) else {
            return empty_classes;
        };
        let has_v_flag = regex_flags.contains('v');
        let trimmed_text = regex_token.text_trimmed();
        let mut class_start_index = None;
        let mut is_negated_class = false;
        let mut enumerated_char_iter = trimmed_text.chars().enumerate();
        while let Some((i, ch)) = enumerated_char_iter.next() {
            match ch {
                '\\' => {
                    // We eat the next character because it is escaped with `\`
                    enumerated_char_iter.next();
                }
                '[' => {
                    // The `v` flag allows to embed a class in another class.
                    if class_start_index.is_none() || has_v_flag {
                        class_start_index = Some(i);
                        is_negated_class = false;
                    }
                }
                '^' => {
                    if let Some(class_start_index) = class_start_index {
                        is_negated_class = (i - class_start_index) == 1;
                    }
                }
                ']' => {
                    if let Some(class_start_index) = class_start_index.take() {
                        let empty_class_len = if is_negated_class { 2 } else { 1 };
                        if (i - class_start_index) == empty_class_len {
                            empty_classes.push(class_start_index..i)
                        }
                    }
                }
                _ => {}
            }
        }
        empty_classes
    }

    fn diagnostic(
        ctx: &RuleContext<Self>,
        empty_class_range: &Self::State,
    ) -> Option<RuleDiagnostic> {
        let regex = ctx.query();
        let regex_token = regex.value_token().ok()?;
        let regex_token_range = regex_token.text_trimmed_range();
        let is_negated = empty_class_range.len() > 1;
        let maybe_negated = if is_negated { "negated " } else { "" };
        let specific_note = if is_negated {
            "Negated empty character classes match anything."
        } else {
            "Empty character classes don't match anything."
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                TextRange::new(
                    regex_token_range.start() + TextSize::from(empty_class_range.start as u32),
                    regex_token_range.start() + TextSize::from((empty_class_range.end + 1) as u32),
                ),
                markup! {
                    "The regular expression includes this "<Emphasis>{maybe_negated}"empty character class"</Emphasis>"."
                },
            )
            .note(markup! {
                {specific_note}"\nIf you want to match against "<Emphasis>"["</Emphasis>", escape it "<Emphasis>"\\["</Emphasis>".\nOtherwise, remove the character class or fill it."
            }),
        )
    }
}
