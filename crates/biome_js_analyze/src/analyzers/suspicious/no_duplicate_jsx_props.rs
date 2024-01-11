use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{AnyJsxAttribute, JsxAttribute};
use biome_rowan::AstNode;
use rustc_hash::FxHashMap;

declare_rule! {
    /// Prevents JSX properties to be assigned multiple times.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <Hello name="John" name="John" />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <label xml:lang="en-US" xml:lang="en-US"></label>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <Hello firstname="John" lastname="Doe" />
    /// ```
    ///
    /// ```js
    /// <label xml:lang="en-US" lang="en-US"></label>
    /// ```
 pub(crate) NoDuplicateJsxProps {
        version: "1.0.0",
        name: "noDuplicateJsxProps",
        source: RuleSource::EslintReact("jsx-no-duplicate-props"),
        recommended: true,
    }
}

impl Rule for NoDuplicateJsxProps {
    type Query = Ast<AnyJsxElement>;
    type State = (String, Vec<JsxAttribute>);
    type Signals = FxHashMap<String, Vec<JsxAttribute>>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut defined_attributes: FxHashMap<String, Vec<JsxAttribute>> = FxHashMap::default();
        for attribute in node.attributes() {
            if let AnyJsxAttribute::JsxAttribute(attr) = attribute {
                if let Ok(name) = attr.name() {
                    defined_attributes
                        .entry(name.text())
                        .or_default()
                        .push(attr);
                }
            }
        }

        defined_attributes.retain(|_, val| val.len() > 1);
        defined_attributes
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut attributes = state.1.iter();

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            attributes.next()?.syntax().text_trimmed_range(),
            markup!("This JSX property is assigned multiple times."),
        );

        for attr in attributes {
            diagnostic = diagnostic.detail(
                attr.syntax().text_trimmed_range(),
                "This attribute is assigned again here.",
            )
        }

        Some(diagnostic)
    }
}
