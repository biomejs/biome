use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsxElementName, JsxAttributeList, JsxOpeningElement, JsxSelfClosingElement,
};
use biome_rowan::{AstNode, declare_node_union};
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
    /// ```jsx,expect_diagnostic
    /// const Invalid = () => <script src="https://third-party-script.js" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// const Valid = () => {
    ///   return (
    ///     <>
    ///       <script src="https://third-party-script.js" async />
    ///       <script src="https://third-party-script.js" defer />
    ///       <script src="https://third-party-script.js" type="module" />
    ///     </>
    ///   );
    /// }
    /// ```
    ///
    /// #### Next.js
    ///
    /// ```jsx
    /// import Script from 'next/script'
    ///
    /// const Valid = () => <Script src="https://third-party-script.js" />;
    /// ```
    ///
    pub NoSyncScripts {
        version: "2.3.6",
        name: "noSyncScripts",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintNext("no-sync-scripts").same()],
        domains: &[RuleDomain::React, RuleDomain::Next],
    }
}

declare_node_union! {
    pub NoSyncScriptsQuery =
        JsxOpeningElement
        | JsxSelfClosingElement
}

fn validate_name(node: &AnyJsxElementName) -> Option<()> {
    let name = node.as_jsx_name()?;
    let value_token = name.value_token().ok()?;

    if value_token.text_trimmed() == "script" {
        return Some(());
    }

    None
}

fn validate_attributes(list: &JsxAttributeList) -> Option<()> {
    if list.find_by_name("src").is_none()
        || list.find_by_name("type").is_some_and(|attribute| {
            attribute.initializer().is_some_and(|initializer| {
                initializer.value().ok().is_some_and(|value| {
                    value.as_jsx_string().is_some_and(|jsx_string| {
                        jsx_string
                            .inner_string_text()
                            .is_ok_and(|inner_string| inner_string.text() == "module")
                    })
                })
            })
        })
        || list.find_by_name("async").is_some()
        || list.find_by_name("defer").is_some()
    {
        return None;
    }

    Some(())
}

impl Rule for NoSyncScripts {
    type Query = Ast<NoSyncScriptsQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoSyncScriptsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();

        match binding {
            NoSyncScriptsQuery::JsxOpeningElement(node) => {
                let name = node.name().ok()?;
                validate_name(&name)?;

                let attributes = node.attributes();
                validate_attributes(&attributes)
            }
            NoSyncScriptsQuery::JsxSelfClosingElement(node) => {
                let name = node.name().ok()?;
                validate_name(&name)?;

                let attributes = node.attributes();
                validate_attributes(&attributes)
            }
        }
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
                "Synchronous scripts can impact your webpage performance. Add the \"async\" or \"defer\" attribute. If using Next.js, consider the Script component instead."
            }),
        )
    }
}
