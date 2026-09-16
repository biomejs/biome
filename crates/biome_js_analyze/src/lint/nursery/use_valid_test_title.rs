use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleDomain,
    RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, JsCallExpression,
    JsStringLiteralExpression,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};
use biome_rule_options::use_valid_test_title::UseValidTestTitleOptions;

use crate::{frameworks::unit_tests::TestBlockKind, JsRuleAction};

declare_lint_rule! {
    /// Enforce valid titles for unit test cases and test suites.
    ///
    /// Checks that the titles of test blocks (`describe`, `test`, `it`) are valid:
    /// - Titles must not be empty.
    /// - Titles must not have accidental leading or trailing whitespace.
    /// - Titles must be string or template literals.
    /// - Titles must not contain disallowed words (if configured).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// it("", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// describe(" foo", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test(123, () => {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// it("should work", () => {});
    /// describe("my suite", () => {});
    /// ```
    ///
    /// ## Options
    ///
    /// ### `disallowedWords`
    ///
    /// A list of words that are not allowed in test titles. Matching is whole-word and case-insensitive.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "disallowedWords": ["skip", "only"]
    ///     }
    /// }
    /// ```
    ///
    pub UseValidTestTitle {
        version: "2.5.14",
        name: "useValidTestTitle",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintJest("valid-title").same(),
            RuleSource::EslintVitest("valid-title").same(),
        ],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseValidTestTitle {
    type Query = Ast<JsCallExpression>;
    type State = TitleError;
    type Signals = Option<Self::State>;
    type Options = UseValidTestTitleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let kind = TestBlockKind::from_call_expression(call)?;
        let options = ctx.options();

        let arguments = call.arguments().ok()?;
        let mut args = arguments.args().into_iter();
        let first_arg = match args.next() {
            Some(Ok(arg)) => arg,
            _ => {
                return Some(TitleError::EmptyTitle {
                    range: call.range(),
                    kind,
                });
            }
        };

        if first_arg.as_js_spread().is_some() {
            return None;
        }

        let expr = first_arg.as_any_js_expression()?;

        match expr {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(s),
            ) => {
                let text = s.inner_string_text().ok()?;
                let text_str = text.text();

                if text_str.is_empty() {
                    return Some(TitleError::EmptyTitle {
                        range: s.range(),
                        kind,
                    });
                }

                let leading = text_str.starts_with(char::is_whitespace);
                let trailing = text_str.ends_with(char::is_whitespace);
                if leading || trailing {
                    return Some(TitleError::AccidentalSpace {
                        range: s.range(),
                        kind,
                        string_literal: Some(s.clone()),
                        leading,
                        trailing,
                    });
                }

                if let Some(word_index) =
                    find_disallowed_word_index(text_str, options.disallowed_words.as_deref())
                {
                    return Some(TitleError::DisallowedWord {
                        range: s.range(),
                        kind,
                        word_index,
                    });
                }

                None
            }
            AnyJsExpression::AnyJsLiteralExpression(lit) => Some(TitleError::TitleMustBeString {
                range: lit.range(),
                kind,
            }),
            AnyJsExpression::JsTemplateExpression(template) => {
                if template.tag().is_some() {
                    return None;
                }

                let elements: Vec<_> = template.elements().into_iter().collect();
                if elements.is_empty() {
                    return Some(TitleError::EmptyTitle {
                        range: template.range(),
                        kind,
                    });
                }

                let has_substitutions = elements
                    .iter()
                    .any(|elem| matches!(elem, AnyJsTemplateElement::JsTemplateElement(_)));

                if !has_substitutions {
                    let text_str = elements
                        .iter()
                        .filter_map(|elem| match elem {
                            AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                                chunk.template_chunk_token().ok().map(|t| t.token_text())
                            }
                            _ => None,
                        })
                        .map(|t| t.to_string())
                        .collect::<String>();

                    if text_str.is_empty() {
                        return Some(TitleError::EmptyTitle {
                            range: template.range(),
                            kind,
                        });
                    }

                    let leading = text_str.starts_with(char::is_whitespace);
                    let trailing = text_str.ends_with(char::is_whitespace);
                    if leading || trailing {
                        return Some(TitleError::AccidentalSpace {
                            range: template.range(),
                            kind,
                            string_literal: None,
                            leading,
                            trailing,
                        });
                    }

                    if let Some(word_index) =
                        find_disallowed_word_index(&text_str, options.disallowed_words.as_deref())
                    {
                        return Some(TitleError::DisallowedWord {
                            range: template.range(),
                            kind,
                            word_index,
                        });
                    }

                    None
                } else {
                    let first_leading = match elements.first() {
                        Some(AnyJsTemplateElement::JsTemplateChunkElement(chunk)) => {
                            chunk
                                .template_chunk_token()
                                .is_ok_and(|t| t.text().starts_with(char::is_whitespace))
                        }
                        _ => false,
                    };
                    let last_trailing = match elements.last() {
                        Some(AnyJsTemplateElement::JsTemplateChunkElement(chunk)) => {
                            chunk
                                .template_chunk_token()
                                .is_ok_and(|t| t.text().ends_with(char::is_whitespace))
                        }
                        _ => false,
                    };
                    if first_leading || last_trailing {
                        return Some(TitleError::AccidentalSpace {
                            range: template.range(),
                            kind,
                            string_literal: None,
                            leading: first_leading,
                            trailing: last_trailing,
                        });
                    }

                    None
                }
            }
            _ => Some(TitleError::TitleMustBeString {
                range: expr.range(),
                kind,
            }),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            TitleError::EmptyTitle { range, kind } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *range,
                    markup! {
                        "The " {kind.as_str()} " title is empty."
                    },
                )
                .note(markup! {
                    "A title is required to identify the " {kind.as_str()} " in test reports."
                })
                .note(markup! {
                    "Provide a descriptive title for this " {kind.as_str()} "."
                }),
            ),
            TitleError::AccidentalSpace {
                range,
                kind,
                leading,
                trailing,
                ..
            } => {
                let position = if *leading && *trailing {
                    "start and end"
                } else if *leading {
                    "start"
                } else {
                    "end"
                };
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        *range,
                        markup! {
                            "The " {kind.as_str()} " title has leading or trailing whitespace."
                        },
                    )
                    .note(markup! {
                        "Accidental whitespace can cause inconsistent test output and formatting issues in reports."
                    })
                    .note(markup! {
                        "Remove the accidental whitespace at the " {position} " of the title."
                    }),
                )
            }
            TitleError::TitleMustBeString { range, kind } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *range,
                    markup! {
                        "The " {kind.as_str()} " title is not a string."
                    },
                )
                .note(markup! {
                    "Test titles must be strings to ensure test runners and reporters can properly display them."
                })
                .note(markup! {
                    "Provide a string literal or template literal as the title."
                }),
            ),
            TitleError::DisallowedWord {
                range,
                kind,
                word_index,
            } => {
                let word = ctx
                    .options()
                    .disallowed_words
                    .as_deref()
                    .and_then(|words| words.get(*word_index))
                    .map_or("", |w| w.as_ref());
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        *range,
                        markup! {
                            "The " {kind.as_str()} " title contains the disallowed word "<Emphasis>{word}</Emphasis>"."
                        },
                    )
                    .note(markup! {
                        "This word is disallowed by configuration to maintain consistent test titles."
                    })
                    .note(markup! {
                        "Remove or replace the disallowed word."
                    }),
                )
            }
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let TitleError::AccidentalSpace {
            string_literal: Some(literal),
            ..
        } = state
        else {
            return None;
        };

        let token = literal.value_token().ok()?;
        let text = literal.inner_string_text().ok()?;
        let trimmed = text.text().trim();
        if trimmed.is_empty() {
            return None;
        }

        let is_single = token.text_trimmed().starts_with('\'');
        let replacement = if is_single {
            make::js_string_literal_expression(make::js_string_literal_single_quotes(trimmed))
        } else {
            make::js_string_literal_expression(make::js_string_literal(trimmed))
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_node(literal.clone(), replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Trim leading and trailing whitespace from the title." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug, Clone)]
pub enum TitleError {
    EmptyTitle {
        range: TextRange,
        kind: TestBlockKind,
    },
    AccidentalSpace {
        range: TextRange,
        kind: TestBlockKind,
        string_literal: Option<JsStringLiteralExpression>,
        leading: bool,
        trailing: bool,
    },
    TitleMustBeString {
        range: TextRange,
        kind: TestBlockKind,
    },
    DisallowedWord {
        range: TextRange,
        kind: TestBlockKind,
        word_index: usize,
    },
}

/// Matches whole words (equivalent to `\bword\b` in ESLint `valid-title`),
/// preventing false positives like "benefit" when "fit" is disallowed.
fn find_disallowed_word_index(text: &str, words: Option<&[Box<str>]>) -> Option<usize> {
    let words = words?;
    for (index, word) in words.iter().enumerate() {
        if word.is_empty() {
            continue;
        }
        for token in text.split(|c: char| !c.is_alphanumeric() && c != '_') {
            if token.eq_ignore_ascii_case(word) {
                return Some(index);
            }
        }
    }
    None
}
