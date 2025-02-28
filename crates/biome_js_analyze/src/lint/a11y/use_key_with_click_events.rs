use std::borrow::Cow;

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{jsx_ext::AnyJsxElement, AnyJsxAttribute, AnyJsxElementName};
use biome_rowan::AstNode;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Enforce onClick is accompanied by at least one of the following: `onKeyUp`, `onKeyDown`, `onKeyPress`.
    ///
    /// Coding for the keyboard is important for users with physical disabilities who cannot use a mouse, AT compatibility, and screenreader users.
    /// This does not apply for interactive or hidden elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div onClick={() => {}} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div onClick={() => {}} onKeyDown={handleKeyDown} />
    ///```
    ///
    /// ```jsx
    /// <div onClick={() => {}} onKeyUp={handleKeyUp} />
    ///```
    ///
    /// ```jsx
    /// <div onClick={() => {}} onKeyPress={handleKeyPress} />
    ///```
    ///
    /// ```jsx
    /// // this rule doesn't apply to user created component
    /// <MyComponent onClick={() => {}} />
    ///```
    ///
    /// ```jsx,
    /// <div onClick={() => {}} {...spread}></div>
    /// ```
    ///
    /// ```jsx
    /// <div {...spread} onClick={() => {}} ></div>
    /// ```
    ///
    /// ```jsx
    /// <button onClick={() => console.log("test")}>Submit</button>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)
    ///
    pub UseKeyWithClickEvents {
        version: "1.0.0",
        name: "useKeyWithClickEvents",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("click-events-have-key-events")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseKeyWithClickEvents {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        match element.name() {
            Ok(AnyJsxElementName::JsxName(name)) => {
                let name_token = name.value_token().ok()?;
                let element_name = name_token.text_trimmed().to_ascii_lowercase_cow();

                // Don't handle interactive roles
                // TODO Support aria roles https://github.com/rome/tools/issues/3640
                if matches!(
                    element_name,
                    Cow::Borrowed("button" | "checkbox" | "combobox" | "a" | "input")
                ) {
                    return None;
                }
            }
            _ => {
                return None;
            }
        }

        let attributes = element.attributes();

        #[expect(clippy::question_mark)]
        if attributes.find_by_name("onClick").is_none() {
            return None;
        }

        for attribute in attributes {
            match attribute {
                AnyJsxAttribute::JsxAttribute(attribute) => {
                    let attribute_name = attribute.name().ok()?;
                    let name = attribute_name.as_jsx_name()?;
                    let name_token = name.value_token().ok()?;

                    if matches!(
                        name_token.text_trimmed(),
                        "onKeyDown" | "onKeyUp" | "onKeyPress"
                    ) {
                        return None;
                    }
                }
                AnyJsxAttribute::JsxSpreadAttribute(_) => {
                    return None;
                }
            }
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Enforce to have the "<Emphasis>"onClick"</Emphasis>" mouse event with the "<Emphasis>"onKeyUp"</Emphasis>", the "<Emphasis>"onKeyDown"</Emphasis>", or the "<Emphasis>"onKeyPress"</Emphasis>" keyboard event."
            },
        ).note(markup! {
            "Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation."
        }))
    }
}
