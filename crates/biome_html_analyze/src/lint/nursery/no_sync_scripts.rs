use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{HtmlFileSource, HtmlOpeningElement};
use biome_rowan::AstNode;
use biome_rule_options::no_sync_scripts::NoSyncScriptsOptions;

declare_lint_rule! {
    /// Prevent the usage of synchronous scripts.
    ///
    /// A synchronous script can impact your webpage performance, read more on how to [Efficiently load third-party JavaScript](https://web.dev/articles/efficiently-load-third-party-javascript).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <script src=""></script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <script src="" async></script>
    /// <script src="" defer></script>
    /// <script src="" type="module"></script>
    /// ```
    ///
    pub NoSyncScripts {
        version: "2.3.6",
        name: "noSyncScripts",
        language: "html",
        recommended: false,
        sources: &[RuleSource::EslintNext("no-sync-scripts").same()],
    }
}

impl Rule for NoSyncScripts {
    type Query = Ast<HtmlOpeningElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoSyncScriptsOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let binding = ctx.query();
        let file_source = ctx.source_type::<HtmlFileSource>();

        let name = binding.name().ok()?;
        let name_text = name.token_text_trimmed()?;

        if file_source.is_html() && !name_text.eq_ignore_ascii_case("script") {
            return None;
        }
        if !file_source.is_html() && name_text != "script" {
            return None;
        }

        let attributes = binding.attributes();
        if attributes.find_by_name("src").is_none()
            || attributes.find_by_name("type").is_some_and(|attribute| {
                attribute.initializer().is_some_and(|initializer| {
                    initializer.value().ok().is_some_and(|value| {
                        value.as_html_string().is_some_and(|html_string| {
                            html_string
                                .inner_string_text()
                                .is_ok_and(|inner_string| inner_string.text() == "module")
                        })
                    })
                })
            })
            || attributes.find_by_name("async").is_some()
            || attributes.find_by_name("defer").is_some()
        {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected synchronous script."
                },
            )
            .note(markup! {
                "Synchronous scripts can impact your webpage performance. Add the \"async\" or \"defer\" attribute."
            }),
        )
    }
}
