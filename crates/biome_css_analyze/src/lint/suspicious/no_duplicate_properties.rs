use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{CssDeclarationOrRuleList, CssKeyframesAtRule};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_duplicate_properties::NoDuplicatePropertiesOptions;

use crate::services::semantic::Semantic;
use crate::utils::{DuplicateGroup, collect_duplicates};

declare_lint_rule! {
    /// Disallow duplicate properties within declaration blocks.
    ///
    /// This rule checks the declaration blocks for duplicate properties. It ignores custom properties and declarations inside `@keyframes` blocks.
    ///
    /// ## Fallback declarations
    ///
    /// Declaring a property twice is a common way to provide a fallback for browsers that do not
    /// support a newer value, for example `100vh` followed by `100dvh`. The rule can't tell
    /// intentional fallbacks from mistakes, so it reports them too.
    ///
    /// To keep a fallback, either guard the newer value with `@supports`, or suppress the
    /// diagnostic on the first declaration with a suppression comment that explains why.
    ///
    /// ## SCSS limitations
    ///
    /// Declarations are compared within each authored declaration block. The rule does not expand
    /// mixins or includes, so duplicates introduced only after SCSS expansion are not reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   color: pink;
    ///   color: orange;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a {
    ///   color: pink;
    ///   background: orange;
    /// }
    /// ```
    ///
    pub NoDuplicateProperties {
        version: "1.9.4",
        name: "noDuplicateProperties",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("declaration-block-no-duplicate-properties").same()],
    }
}

impl Rule for NoDuplicateProperties {
    type Query = Semantic<CssDeclarationOrRuleList>;
    type State = DuplicateGroup<TokenText>;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicatePropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        let Some(rule) = model.get_rule_by_range(node.range()) else {
            return Box::default();
        };

        collect_duplicates(
            rule.declarations()
                .iter()
                .filter(|declaration| {
                    declaration
                        .declaration()
                        .syntax()
                        .parent()
                        .and_then(|parent| parent.parent())
                        .as_ref()
                        == Some(node.syntax())
                })
                .filter_map(|declaration| {
                    let prop = declaration.property();
                    let prop_name = prop.value().ok()?;

                    if prop_name.text().starts_with("--") {
                        return None;
                    }

                    // We skip this declaration if it's inside a keyframes block.
                    if prop
                        .syntax()
                        .ancestors()
                        .skip(1)
                        .any(|node| CssKeyframesAtRule::can_cast(node.kind()))
                    {
                        return None;
                    }

                    Some((prop_name.clone(), prop_name, prop.range()))
                }),
        )
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let DuplicateGroup {
            name,
            first,
            duplicates,
        } = state;
        let name = name.text();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first,
            markup! {
                "The property "<Emphasis>{name}</Emphasis>" is declared more than once in the same block."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    <Emphasis>{name}</Emphasis>" is declared again here."
                },
            );
        }

        Some(
            diagnostic
                .note(markup! {
                    "Duplicate properties can lead to unexpected behavior and may override previous declarations unintentionally."
                })
                .note(markup! {
                    "Remove or rename the duplicate property to ensure consistent styling."
                }),
        )
    }
}
