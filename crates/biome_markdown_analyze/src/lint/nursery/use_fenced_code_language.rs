use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_markdown_syntax::MdFencedCodeBlock;
use biome_rowan::{AstNodeList, TextRange, TextSize};
use biome_rule_options::use_fenced_code_language::UseFencedCodeLanguageOptions;

declare_lint_rule! {
    /// Enforce that fenced code blocks specify a language.
    ///
    /// Fenced code blocks without a language aren't syntax-highlighted when rendered to HTML,
    /// which makes the code harder to read. Use `text` for snippets that are intentionally
    /// plain text.
    ///
    /// This rule is a port of markdownlint's [`MD040`](https://github.com/DavidAnson/markdownlint/blob/main/doc/md040.md).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ````md,expect_diagnostic
    /// ```
    /// console.log(1)
    /// ```
    /// ````
    ///
    /// ### Valid
    ///
    /// ````md
    /// ```js
    /// console.log(1)
    /// ```
    /// ````
    ///
    /// ## Options
    ///
    /// ### `allowedLanguages`
    ///
    /// A list of languages fenced code blocks are allowed to declare. When empty (the default),
    /// any language is accepted. Language names are matched case-sensitively.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowedLanguages": ["js", "ts"]
    ///     }
    /// }
    /// ```
    ///
    /// ````md,expect_diagnostic,use_options
    /// ```python
    /// print(1)
    /// ```
    /// ````
    ///
    /// Default: `[]`
    ///
    /// ### `languageOnly`
    ///
    /// When `true`, requires the info string to contain exactly the language, without
    /// surrounding whitespace or other content.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "languageOnly": true
    ///     }
    /// }
    /// ```
    ///
    /// ````md,expect_diagnostic,use_options
    /// ```js startLine=3
    /// console.log(1)
    /// ```
    /// ````
    ///
    /// Default: `false`
    ///
    pub UseFencedCodeLanguage {
        version: "next",
        name: "useFencedCodeLanguage",
        language: "md",
        sources: &[RuleSource::Markdownlint("MD040").same()],
        recommended: true,
    }
}

pub enum FencedCodeLanguageIssue {
    /// The fenced code block has no language in its info string.
    Missing,
    /// The declared language isn't part of the `allowedLanguages` option.
    NotAllowed(TextRange),
    /// The info string carries more than just the language, but `languageOnly` is enabled.
    ExtraInfo(TextRange),
}

impl Rule for UseFencedCodeLanguage {
    type Query = Ast<MdFencedCodeBlock>;
    type State = FencedCodeLanguageIssue;
    type Signals = Vec<Self::State>;
    type Options = UseFencedCodeLanguageOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let block = ctx.query();
        let options = ctx.options();

        // `code_list` is empty when there's no info string at all right after the fence.
        let Some(item) = block.code_list().iter().next() else {
            return vec![FencedCodeLanguageIssue::Missing];
        };
        let Ok(token) = item.value_token() else {
            return Vec::new();
        };
        let full_text = token.text_trimmed();
        let full_range = token.text_trimmed_range();

        let trimmed = full_text.trim();
        if trimmed.is_empty() {
            return vec![FencedCodeLanguageIssue::Missing];
        }

        // `code_list` holds a single token because the lexer consumes the whole info
        // string (up to the newline) as one literal; split it manually into the
        // language word and whatever trails it.
        let leading_ws = full_text.len() - full_text.trim_start().len();
        let Some(language) = trimmed.split_whitespace().next() else {
            return Vec::new();
        };
        let language_start = leading_ws;
        let language_end = language_start + language.len();
        let to_range = |start, end| {
            Some(TextRange::new(
                full_range.start() + TextSize::try_from(start).ok()?,
                full_range.start() + TextSize::try_from(end).ok()?,
            ))
        };
        let mut issues = Vec::new();

        if let Some(allowed_languages) = &options.allowed_languages
            && !allowed_languages.is_empty()
            && !allowed_languages
                .iter()
                .any(|allowed| &**allowed == language)
            && let Some(range) = to_range(language_start, language_end)
        {
            issues.push(FencedCodeLanguageIssue::NotAllowed(range));
        }

        if options.language_only == Some(true) {
            let has_leading_whitespace = language_start > 0;
            let has_trailing_content = full_text.len() > language_end;
            let extra_offsets = match (has_leading_whitespace, has_trailing_content) {
                (true, true) => Some((0, full_text.len())),
                (true, false) => Some((0, language_start)),
                (false, true) => Some((language_end, full_text.len())),
                (false, false) => None,
            };
            if let Some((extra_start, extra_end)) = extra_offsets
                && let Some(range) = to_range(extra_start, extra_end)
            {
                issues.push(FencedCodeLanguageIssue::ExtraInfo(range));
            }
        }

        issues
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let block = ctx.query();
        let options = ctx.options();

        match state {
            FencedCodeLanguageIssue::Missing => {
                let range = block.l_fence().ok()?.text_trimmed_range();
                let diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "This fenced code block has no language specifier."
                    },
                )
                .note(markup! {
                    "Without a language, the code isn't syntax-highlighted when the document is rendered."
                });

                if options
                    .allowed_languages
                    .as_deref()
                    .is_some_and(|languages| !languages.is_empty())
                {
                    Some(diagnostic.note(markup! {
                        "Add one of the languages configured in "<Emphasis>"allowedLanguages"</Emphasis>" after the opening fence."
                    }))
                } else {
                    Some(diagnostic.note(markup! {
                        "Add a language after the opening fence, such as "<Emphasis>"js"</Emphasis>", or use "<Emphasis>"text"</Emphasis>" for plain text."
                    }))
                }
            }
            FencedCodeLanguageIssue::NotAllowed(range) => {
                let allowed_languages = options
                    .allowed_languages
                    .as_deref()
                    .unwrap_or_default()
                    .iter()
                    .map(|language| format!("\"{language}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        range,
                        markup! {
                            "This language isn't in the list of allowed languages."
                        },
                    )
                    .note(markup! {
                        "The "<Emphasis>"allowedLanguages"</Emphasis>" option restricts fenced code blocks to a specific set of languages."
                    })
                    .note(markup! {
                        "Use one of the allowed languages: "{allowed_languages}"."
                    }),
                )
            }
            FencedCodeLanguageIssue::ExtraInfo(range) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "This info string contains more than just the language."
                    },
                )
                .note(markup! {
                    "The "<Emphasis>"languageOnly"</Emphasis>" option requires the info string to contain nothing but the language."
                })
                .note(markup! {
                    "Remove any whitespace or extra content surrounding the language."
                }),
            ),
        }
    }
}
