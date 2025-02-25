use std::ops::Range;

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsRegexLiteralExpression;
use biome_rowan::{TextRange, TextSize};

declare_lint_rule! {
    /// Disallow empty character classes in regular expression literals.
    ///
    /// Empty character classes don't match anything.
    /// In contrast, negated empty classes match any character.
    /// They are often the result of a typing mistake.
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
    /// ### Valid
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
    pub NoEmptyCharacterClassInRegex {
        version: "1.3.0",
        name: "noEmptyCharacterClassInRegex",
        language: "js",
        sources: &[RuleSource::Eslint("no-empty-character-class")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoEmptyCharacterClassInRegex {
    type Query = Ast<JsRegexLiteralExpression>;
    type State = Range<usize>;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut empty_classes = vec![];
        let regex = ctx.query();
        let Ok((pattern, flags)) = regex.decompose() else {
            return empty_classes.into_boxed_slice();
        };
        let has_v_flag = flags.text().contains('v');
        let trimmed_text = pattern.text();
        let mut class_start_index = None;
        let mut is_negated_class = false;
        let mut enumerated_char_iter = trimmed_text.bytes().enumerate();
        while let Some((i, ch)) = enumerated_char_iter.next() {
            match ch {
                b'\\' => {
                    // We eat the next character because it is escaped with `\`
                    enumerated_char_iter.next();
                }
                b'[' => {
                    // The `v` flag allows to embed a class in another class.
                    if class_start_index.is_none() || has_v_flag {
                        class_start_index = Some(i);
                        is_negated_class = false;
                    }
                }
                b'^' => {
                    if let Some(class_start_index) = class_start_index {
                        is_negated_class = (i - class_start_index) == 1;
                    }
                }
                b']' => {
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
        empty_classes.into_boxed_slice()
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
                    regex_token_range.start() + TextSize::from(empty_class_range.start as u32 + 1),
                    regex_token_range.start() + TextSize::from((empty_class_range.end + 2) as u32),
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
