use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{AnyJsxAttribute, AnyJsxAttributeName, JsxAttribute};
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_duplicate_jsx_props::NoDuplicateJsxPropsOptions;

declare_lint_rule! {
    /// Prevents JSX properties to be assigned multiple times.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <Hello name="John" name="John" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <label xml:lang="en-US" xml:lang="en-US"></label>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <Hello firstname="John" lastname="Doe" />
    /// ```
    ///
    /// ```jsx
    /// <label xml:lang="en-US" lang="en-US"></label>
    /// ```
 pub NoDuplicateJsxProps {
        version: "1.0.0",
        name: "noDuplicateJsxProps",
        language: "jsx",
        sources: &[RuleSource::EslintReact("jsx-no-duplicate-props").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum AttributeNameKey {
    Name(TokenText),
    Namespace(TokenText, TokenText),
}

impl Rule for NoDuplicateJsxProps {
    type Query = Ast<AnyJsxElement>;
    type State = Vec<JsxAttribute>;
    type Signals = Vec<Self::State>;
    type Options = NoDuplicateJsxPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let attributes = node
            .attributes()
            .into_iter()
            .filter_map(|attribute| match attribute {
                AnyJsxAttribute::JsxAttribute(attr) => Some(attr),
                _ => None,
            })
            .collect::<Vec<_>>();

        let mut duplicated_attributes = Vec::new();
        let mut processed_names = Vec::new();

        for (index, attr) in attributes.iter().enumerate() {
            let Some(name) = attribute_name_key(attr) else {
                continue;
            };

            if processed_names.contains(&name) {
                continue;
            }

            let mut duplicates = vec![attr.clone()];

            for other in attributes.iter().skip(index + 1) {
                let Some(other_name) = attribute_name_key(other) else {
                    continue;
                };

                if name == other_name {
                    duplicates.push(other.clone());
                }
            }

            if duplicates.len() > 1 {
                processed_names.push(name);
                duplicated_attributes.push(duplicates);
            }
        }

        duplicated_attributes
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut attributes = state.iter();

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

fn attribute_name_key(attribute: &JsxAttribute) -> Option<AttributeNameKey> {
    match attribute.name().ok()? {
        AnyJsxAttributeName::JsxName(name) => {
            Some(AttributeNameKey::Name(name.value_token().ok()?.token_text_trimmed()))
        }
        AnyJsxAttributeName::JsxNamespaceName(name) => Some(AttributeNameKey::Namespace(
            name.namespace().ok()?.value_token().ok()?.token_text_trimmed(),
            name.name().ok()?.value_token().ok()?.token_text_trimmed(),
        )),
    }
}
