use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{AnyJsxAttribute, JsxAttribute};
use biome_rowan::AstNode;
use rustc_hash::FxHashMap;

declare_rule! {
    /// Prevents React-specific JSX properties from being used.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <Hello className="John" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <Hello class="Doe" />
    /// ```
 pub NoReactSpecificProps {
        version: "1.0.0",
        name: "noReactSpecificProps",
        sources: &[RuleSource::EslintSolid("no-react-specific-props")],
        recommended: false,
    }
}

const REACT_SPECIFIC_JSX_PROPS: &[&str] = &["className", "htmlFor"];

impl Rule for NoReactSpecificProps {
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

        defined_attributes.retain(|key, _| REACT_SPECIFIC_JSX_PROPS.contains(&key.as_str()));
        defined_attributes
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut attributes = state.1.iter();

        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            attributes.next()?.syntax().text_trimmed_range(),
            markup!("This JSX property is specific to React."),
        );

        Some(diagnostic)
    }

    // TODO: auto fix by converting "className" to "class" and "htmlFor" to "for"
}
