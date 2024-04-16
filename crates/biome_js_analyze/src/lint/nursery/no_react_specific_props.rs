use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::JsxAttribute;
use biome_rowan::{AstNode, TextRange};

declare_rule! {
    /// Prevents React-specific JSX properties from being used.
    ///
    /// This rule is intended for use in JSX-based frameworks (mainly Solid.js)
    /// that do not use React-style prop names.
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

fn get_replacement_for_react_prop(str: &str) -> &'static str {
    match str {
        "className" => "class",
        "htmlFor" => "for",
        _ => panic!("should be a React prop: {str:?}"),
    }
}

impl Rule for NoReactSpecificProps {
    type Query = Ast<JsxAttribute>;
    type State = (TextRange, &'static str);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        let name = attribute.name().ok()?;
        let range = name.range();
        let name = name.text();

        if REACT_SPECIFIC_JSX_PROPS.contains(&name.as_str()) {
            Some((range, get_replacement_for_react_prop(&name)))
        } else {
            None
        }
    }

    fn diagnostic(
        _: &RuleContext<Self>,
        (range, replacement): &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup!("This JSX property is specific to React."),
            )
            .detail(
                range,
                format!("Replace this attribute name with {replacement:?}"),
            ),
        )
    }

    // TODO: auto fix by converting "className" to "class" and "htmlFor" to "for"
}
