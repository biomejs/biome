use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_markdown_syntax::MdFencedCodeBlock;
use biome_rowan::{AstNodeList, TextRange, TextSize};
use biome_rule_options::use_fenced_code_language::UseFencedCodeLanguageOptions;

declare_lint_rule! {
    /// Enforce that fenced code blocks specify a code tag (language).
    ///
    /// Renderers can use a code tag to select syntax highlighting when they support the language.
    /// Use `text` for snippets when syntax highlighting is unwanted or unsupported.
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
    /// The following options are available.
    ///
    /// ### `allowedLanguages`
    ///
    /// A list of language tags fenced code blocks are allowed to declare. When empty (the
    /// default), any language tag is accepted. Language tags are matched case-sensitively.
    ///
    /// Default: `[]`
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
    /// ### `languageOnly`
    ///
    /// When `true`, requires the info string to contain exactly the language tag, without
    /// metadata or other non-whitespace content.
    ///
    /// Default: `false`
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
    pub UseFencedCodeLanguage {
        version: "next",
        name: "useFencedCodeLanguage",
        language: "md",
        sources: &[RuleSource::MarkdownLint("md040", "fenced-code-language").same()],
        recommended: true,
    }
}

/// Describes a `useFencedCodeLanguage` violation for a fenced code block.
pub enum FencedCodeLanguageIssue {
    /// The info string has no language tag.
    Missing,
    /// The language tag isn't part of the `allowedLanguages` option.
    NotAllowed(TextRange),
    /// The info string has content beyond the language tag while `languageOnly` is enabled.
    ExtraInfo(TextRange),
}

impl Rule for UseFencedCodeLanguage {
    type Query = Ast<MdFencedCodeBlock>;
    type State = FencedCodeLanguageIssue;
    type Signals = Box<[Self::State]>;
    type Options = UseFencedCodeLanguageOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let block = ctx.query();
        let options = ctx.options();

        // Code-info-string lexing preserves the non-boundary info string as one literal, so
        // `code_list` contains one item that must be split into the language and metadata.
        let Some(item) = block.code_list().iter().next() else {
            return vec![FencedCodeLanguageIssue::Missing].into_boxed_slice();
        };
        let Ok(token) = item.value_token() else {
            return Box::default();
        };
        let full_text = token.text_trimmed();
        let full_range = token.text_trimmed_range();

        let trimmed = full_text.trim();
        if trimmed.is_empty() {
            return vec![FencedCodeLanguageIssue::Missing].into_boxed_slice();
        }

        let leading_ws = full_text.len() - full_text.trim_start().len();
        let language = trimmed
            .split_once(char::is_whitespace)
            .map_or(trimmed, |(language, _)| language);
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

        if options.language_only == Some(true) && trimmed.len() > language.len() {
            let extra_start = language_end;
            let extra_end = leading_ws + trimmed.len();
            if let Some(range) = to_range(extra_start, extra_end) {
                issues.push(FencedCodeLanguageIssue::ExtraInfo(range));
            }
        }

        issues.into_boxed_slice()
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
                        "This fenced code block is missing a language tag."
                    },
                )
                .note(markup! {
                    "Renderers use language tags to select syntax highlighting for supported languages."
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
                        "Add a language tag after the opening fence, such as "<Emphasis>"js"</Emphasis>", or use "<Emphasis>"text"</Emphasis>" when highlighting is unwanted or unsupported."
                    }))
                }
            }
            FencedCodeLanguageIssue::NotAllowed(range) => {
                let allowed_languages = options
                    .allowed_languages
                    .as_deref()
                    .unwrap_or_default();
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        range,
                        markup! {
                            "This language tag isn't in the list of allowed languages."
                        },
                    )
                    .note(markup! {
                        "The "<Emphasis>"allowedLanguages"</Emphasis>" option restricts fenced code blocks to a specific set of language tags."
                    })
                    .footer_list("Use one of the allowed languages:", allowed_languages),
                )
            }
            FencedCodeLanguageIssue::ExtraInfo(range) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "This info string contains content beyond the language tag."
                    },
                )
                .note(markup! {
                    "The "<Emphasis>"languageOnly"</Emphasis>" option requires the info string to contain only a language tag."
                })
                .note(markup! {
                    "Remove metadata after the language tag."
                }),
            ),
        }
    }
}
