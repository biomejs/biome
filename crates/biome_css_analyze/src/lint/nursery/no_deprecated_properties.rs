use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_factory::make;
use biome_css_syntax::{AnyCssDeclarationName, CssGenericProperty, CssSyntaxKind, CssSyntaxToken};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_deprecated_properties::NoDeprecatedPropertiesOptions;

use crate::CssRuleAction;

declare_lint_rule! {
    /// Disallow deprecated properties.
    ///
    /// This rule flags properties that were removed or deprecated after being in the CSS
    /// specifications, including editor drafts, and were either:
    ///
    /// - shipped in a stable version of a browser
    /// - shipped by a developer channel/edition browser
    /// - shipped but behind experimental flags
    /// - polyfilled with some adoption before any browser actually shipped
    /// - had an MDN page at one point in time
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { clip: rect(0, 0, 0, 0); }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { word-wrap: break-word; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { clip-path: rect(0 0 0 0); }
    /// ```
    ///
    /// ```css
    /// a { overflow-wrap: break-word; }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignoreProperties`
    ///
    /// Ignores the specified properties.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreProperties": ["clip", "grid-row-gap"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// a { clip: rect(0, 0, 0, 0); }
    /// ```
    ///
    /// ```css,use_options
    /// a { grid-row-gap: 4px; }
    /// ```
    pub NoDeprecatedProperties {
        version: "next",
        name: "noDeprecatedProperties",
        language: "css",
        recommended: false,
        fix_kind: FixKind::Unsafe,
        sources: &[RuleSource::Stylelint("property-no-deprecated").same()],
    }
}

impl Rule for NoDeprecatedProperties {
    type Query = Ast<CssGenericProperty>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoDeprecatedPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        is_deprecated_property(ctx.query(), ctx.options())?.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        let name = ctx.query().name().ok()?.to_trimmed_text();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! { "Deprecated property "<Emphasis>{name.text()}</Emphasis>" is not allowed." },
            )
            .note(markup! {
                "The property has been removed or deprecated in the CSS specification that were previously existed."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<CssRuleAction> {
        let node = ctx.query();
        let replacement = get_replacement(node)?;
        let mut mutation = ctx.root().begin();

        mutation.replace_node(
            node.clone(),
            node.clone()
                .with_name(AnyCssDeclarationName::CssIdentifier(make::css_identifier(
                    CssSyntaxToken::new_detached(CssSyntaxKind::IDENT, replacement, [], []),
                ))),
        );

        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace it with a valid CSS property." }.to_owned(),
            mutation,
        ))
    }
}

/// Check if the property is deprecated.
fn is_deprecated_property(
    property: &CssGenericProperty,
    options: &NoDeprecatedPropertiesOptions,
) -> Option<bool> {
    let name = property.name().ok()?.to_trimmed_text();
    let name = name.text();

    if options
        .ignore_properties
        .iter()
        .any(|ignored_property| ignored_property.as_ref() == name)
    {
        return Some(false);
    }

    Some(match name {
        // Allowed when the value is `vertical`
        "-webkit-box-orient" if property.value().to_trimmed_text() == "vertical" => false,

        "-khtml-box-align"
        | "-khtml-box-direction"
        | "-khtml-box-flex"
        | "-khtml-box-lines"
        | "-khtml-box-ordinal-group"
        | "-khtml-box-orient"
        | "-khtml-box-pack"
        | "-khtml-user-modify"
        | "-moz-box-align"
        | "-moz-box-direction"
        | "-moz-box-flex"
        | "-moz-box-lines"
        | "-moz-box-ordinal-group"
        | "-moz-box-orient"
        | "-moz-box-pack"
        | "-moz-user-modify"
        | "-ms-box-align"
        | "-ms-box-direction"
        | "-ms-box-flex"
        | "-ms-box-lines"
        | "-ms-box-ordinal-group"
        | "-ms-box-orient"
        | "-ms-box-pack"
        | "-webkit-box-align"
        | "-webkit-box-direction"
        | "-webkit-box-flex"
        | "-webkit-box-lines"
        | "-webkit-box-ordinal-group"
        | "-webkit-box-orient"
        | "-webkit-box-pack"
        | "-webkit-user-modify"
        | "grid-column-gap"
        | "grid-gap"
        | "grid-row-gap"
        | "ime-mode"
        | "page-break-after"
        | "page-break-before"
        | "page-break-inside"
        | "position-try-options"
        | "scroll-snap-coordinate"
        | "scroll-snap-destination"
        | "scroll-snap-margin-bottom"
        | "scroll-snap-margin-left"
        | "scroll-snap-margin-right"
        | "scroll-snap-margin-top"
        | "scroll-snap-margin"
        | "scroll-snap-points-x"
        | "scroll-snap-points-y"
        | "scroll-snap-type-x"
        | "scroll-snap-type-y"
        | "word-wrap"
        | "clip" => true,

        _ => false,
    })
}

/// Return a replacement of the deprecated property if applicable.
fn get_replacement(property: &CssGenericProperty) -> Option<&'static str> {
    Some(match property.name().ok()?.to_trimmed_text().text() {
        "-khtml-box-align" | "-moz-box-align" | "-ms-box-align" | "-webkit-box-align" => {
            "align-items"
        }
        "-khtml-box-flex" | "-moz-box-flex" | "-ms-box-flex" | "-webkit-box-flex" => "flex-grow",
        "-khtml-box-ordinal-group"
        | "-moz-box-ordinal-group"
        | "-ms-box-ordinal-group"
        | "-webkit-box-ordinal-group" => "order",
        "grid-column-gap" => "column-gap",
        "grid-gap" => "gap",
        "grid-row-gap" => "row-gap",
        "page-break-after" => "break-after",
        "page-break-before" => "break-before",
        "page-break-inside" => "break-inside",
        "position-try-options" => "position-try-fallbacks",
        "scroll-snap-margin-bottom" => "scroll-margin-bottom",
        "scroll-snap-margin-left" => "scroll-margin-left",
        "scroll-snap-margin-right" => "scroll-margin-right",
        "scroll-snap-margin-top" => "scroll-margin-top",
        "scroll-snap-margin" => "scroll-margin",
        "word-wrap" => "overflow-wrap",
        _ => return None,
    })
}
