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

use crate::{
    frameworks::unit_tests::{is_describe_call, is_unit_test},
    JsRuleAction,
};

declare_lint_rule! {
    /// Enforce valid titles for unit test cases and test suites.
    ///
    /// Checks that the titles of test blocks (`describe`, `test`, `it`, `suite`) are valid:
    /// - Titles must not be empty.
    /// - Titles must not have accidental leading or trailing whitespace.
    /// - Titles must be string or template literals (unless configured otherwise).
    /// - Titles must not contain disallowed words (if configured).
    /// - Titles must match required patterns and must not match forbidden patterns (if configured).
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
    /// ### `ignoreSpaces`
    ///
    /// When `true`, leading and trailing whitespace will not be checked.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreSpaces": true
    ///     }
    /// }
    /// ```
    ///
    /// ### `ignoreTypeOfDescribeName`
    ///
    /// When `true`, non-string titles in `describe` and `suite` blocks will be allowed.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreTypeOfDescribeName": true
    ///     }
    /// }
    /// ```
    ///
    /// ### `ignoreTypeOfTestName`
    ///
    /// When `true`, non-string titles in `test` and `it` blocks will be allowed.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreTypeOfTestName": true
    ///     }
    /// }
    /// ```
    ///
    /// ### `disallowedWords`
    ///
    /// A list of words that are not allowed in test titles. Matching is whole-word and case-insensitive.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "disallowedWords": ["skip", "only"]
    ///     }
    /// }
    /// ```
    ///
    /// ### `mustNotMatch`
    ///
    /// An array of regular expressions that titles must not match.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "mustNotMatch": ["[0-9]+"]
    ///     }
    /// }
    /// ```
    ///
    /// ### `mustMatch`
    ///
    /// An array of regular expressions that titles must match.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "mustMatch": ["should .*"]
    ///     }
    /// }
    /// ```
    ///
    pub UseValidTestTitle {
        version: "next",
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestBlockKind {
    Test,
    Describe,
    Suite,
}

impl TestBlockKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Test => "test",
            Self::Describe => "describe",
            Self::Suite => "suite",
        }
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
        word: Box<str>,
    },
    MustNotMatch {
        range: TextRange,
        kind: TestBlockKind,
        pattern: String,
    },
    MustMatch {
        range: TextRange,
        kind: TestBlockKind,
        pattern: String,
    },
}

impl Rule for UseValidTestTitle {
    type Query = Ast<JsCallExpression>;
    type State = TitleError;
    type Signals = Option<Self::State>;
    type Options = UseValidTestTitleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let kind = get_test_block_kind(call)?;
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

                if !options.ignore_spaces() {
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
                }

                if let Some(word) =
                    find_disallowed_word(text_str, options.disallowed_words.as_deref())
                {
                    return Some(TitleError::DisallowedWord {
                        range: s.range(),
                        kind,
                        word,
                    });
                }

                if let Some(pattern) =
                    find_must_not_match(text_str, options.must_not_match.as_deref())
                {
                    return Some(TitleError::MustNotMatch {
                        range: s.range(),
                        kind,
                        pattern,
                    });
                }

                if let Some(pattern) =
                    find_must_match_failure(text_str, options.must_match.as_deref())
                {
                    return Some(TitleError::MustMatch {
                        range: s.range(),
                        kind,
                        pattern,
                    });
                }

                None
            }
            AnyJsExpression::AnyJsLiteralExpression(lit) => {
                let is_ignored = match kind {
                    TestBlockKind::Describe | TestBlockKind::Suite => {
                        options.ignore_type_of_describe_name()
                    }
                    TestBlockKind::Test => options.ignore_type_of_test_name(),
                };
                if !is_ignored {
                    Some(TitleError::TitleMustBeString {
                        range: lit.range(),
                        kind,
                    })
                } else {
                    None
                }
            }
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

                    if !options.ignore_spaces() {
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
                    }

                    if let Some(word) =
                        find_disallowed_word(&text_str, options.disallowed_words.as_deref())
                    {
                        return Some(TitleError::DisallowedWord {
                            range: template.range(),
                            kind,
                            word,
                        });
                    }

                    if let Some(pattern) =
                        find_must_not_match(&text_str, options.must_not_match.as_deref())
                    {
                        return Some(TitleError::MustNotMatch {
                            range: template.range(),
                            kind,
                            pattern,
                        });
                    }

                    if let Some(pattern) =
                        find_must_match_failure(&text_str, options.must_match.as_deref())
                    {
                        return Some(TitleError::MustMatch {
                            range: template.range(),
                            kind,
                            pattern,
                        });
                    }

                    None
                } else {
                    if !options.ignore_spaces() {
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
                    }

                    None
                }
            }
            _ => {
                let is_ignored = match kind {
                    TestBlockKind::Describe | TestBlockKind::Suite => {
                        options.ignore_type_of_describe_name()
                    }
                    TestBlockKind::Test => options.ignore_type_of_test_name(),
                };
                if !is_ignored {
                    Some(TitleError::TitleMustBeString {
                        range: expr.range(),
                        kind,
                    })
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            TitleError::EmptyTitle { range, kind } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *range,
                    markup! {
                        "The " {kind.as_str()} " title should not be empty."
                    },
                )
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
                            "The " {kind.as_str()} " title should not have leading or trailing whitespace."
                        },
                    )
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
                        "The " {kind.as_str()} " title must be a string."
                    },
                )
                .note(markup! {
                    "Provide a string literal or template literal as the title."
                }),
            ),
            TitleError::DisallowedWord { range, kind, word } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *range,
                    markup! {
                        "The " {kind.as_str()} " title contains the disallowed word "<Emphasis>{word.as_ref()}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Remove or replace the disallowed word."
                }),
            ),
            TitleError::MustNotMatch {
                range,
                kind,
                pattern,
            } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *range,
                    markup! {
                        "The " {kind.as_str()} " title matches the forbidden pattern "<Emphasis>{pattern.as_str()}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Update the title so it does not match the pattern."
                }),
            ),
            TitleError::MustMatch {
                range,
                kind,
                pattern,
            } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *range,
                    markup! {
                        "The " {kind.as_str()} " title does not match the required pattern "<Emphasis>{pattern.as_str()}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Update the title to match the required pattern."
                }),
            ),
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

fn is_suite_call(call: &JsCallExpression) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };
    let callee = callee.omit_parentheses();
    match callee {
        AnyJsExpression::JsIdentifierExpression(ident) => ident
            .name()
            .and_then(|r| r.value_token())
            .is_ok_and(|tok| tok.text_trimmed() == "suite"),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let Ok(object) = member.object() else {
                return false;
            };
            let object = object.omit_parentheses();
            if let AnyJsExpression::JsIdentifierExpression(ident) = object {
                ident
                    .name()
                    .and_then(|r| r.value_token())
                    .is_ok_and(|tok| tok.text_trimmed() == "suite")
            } else {
                false
            }
        }
        _ => false,
    }
}

fn is_each_call(call: &JsCallExpression) -> Option<TestBlockKind> {
    let callee = call.callee().ok()?.omit_parentheses();
    let AnyJsExpression::JsCallExpression(inner_call) = callee else {
        return None;
    };
    let inner_callee = inner_call.callee().ok()?.omit_parentheses();

    let is_suite_each = match &inner_callee {
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let object_is_suite = member.object().is_ok_and(|object| {
                let mut current = object.omit_parentheses();
                loop {
                    match current {
                        AnyJsExpression::JsIdentifierExpression(ident) => {
                            return ident
                                .name()
                                .and_then(|r| r.value_token())
                                .is_ok_and(|tok| tok.text_trimmed() == "suite");
                        }
                        AnyJsExpression::JsStaticMemberExpression(member) => {
                            let Ok(object) = member.object() else {
                                return false;
                            };
                            current = object.omit_parentheses();
                        }
                        _ => return false,
                    }
                }
            });

            let member_is_each = matches!(
                member.member().ok(),
                Some(biome_js_syntax::AnyJsName::JsName(name))
                    if name
                        .value_token()
                        .is_ok_and(|tok| matches!(tok.text_trimmed(), "each" | "for" | "prop"))
            );

            object_is_suite && member_is_each
        }
        _ => false,
    };

    if is_suite_each {
        Some(TestBlockKind::Suite)
    } else if inner_callee.contains_a_test_each_pattern() {
        if is_describe_call(&inner_call) || inner_callee.contains_describe_call() {
            Some(TestBlockKind::Describe)
        } else {
            Some(TestBlockKind::Test)
        }
    } else {
        None
    }
}

fn get_test_block_kind(call: &JsCallExpression) -> Option<TestBlockKind> {
    if is_describe_call(call) {
        Some(TestBlockKind::Describe)
    } else if is_unit_test(call) {
        Some(TestBlockKind::Test)
    } else if is_suite_call(call) {
        Some(TestBlockKind::Suite)
    } else {
        is_each_call(call)
    }
}

fn find_disallowed_word(text: &str, words: Option<&[Box<str>]>) -> Option<Box<str>> {
    let words = words?;
    let lower_text = text.to_lowercase();
    for word in words {
        let lower_word = word.to_lowercase();
        if lower_word.is_empty() {
            continue;
        }
        for (idx, _) in lower_text.match_indices(&lower_word) {
            let before_is_word_char = lower_text[..idx]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_');
            let after_idx = idx + lower_word.len();
            let after_is_word_char = lower_text[after_idx..]
                .chars()
                .next()
                .is_some_and(|c| c.is_alphanumeric() || c == '_');

            if !before_is_word_char && !after_is_word_char {
                return Some(word.clone());
            }
        }
    }
    None
}

fn find_must_not_match(
    text: &str,
    patterns: Option<&[biome_rule_options::restricted_regex::RestrictedRegex]>,
) -> Option<String> {
    let patterns = patterns?;
    for pattern in patterns {
        if pattern.is_match(text) {
            return Some(pattern.to_string());
        }
    }
    None
}

fn find_must_match_failure(
    text: &str,
    patterns: Option<&[biome_rule_options::restricted_regex::RestrictedRegex]>,
) -> Option<String> {
    let patterns = patterns?;
    for pattern in patterns {
        if !pattern.is_match(text) {
            return Some(pattern.to_string());
        }
    }
    None
}
