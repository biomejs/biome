use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::keywords::{
    COLOR_PROFILE_DESCRIPTORS, COUNTER_STYLE_DESCRIPTORS, FONT_FACE_DESCRIPTORS,
    FONT_PALETTE_VALUES_DESCRIPTORS, POSITION_TRY_PROPERTIES, PROPERTY_DESCRIPTORS,
    VIEW_TRANSITION_DESCRIPTORS,
};
use biome_css_syntax::{
    AnyCssDashedIdentifier, AnyCssDeclarationName, CssColorProfileAtRule, CssCounterStyleAtRule,
    CssDeclaration, CssDeclarationWithSemicolon, CssFontFaceAtRule, CssFontPaletteValuesAtRule,
    CssGenericProperty, CssPositionTryAtRule, CssPropertyAtRule, CssSyntaxToken,
    CssViewTransitionAtRule,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_unknown_at_rule_descriptor::NoUnknownAtRuleDescriptorOptions;
use biome_string_case::StrLikeExtension;

use crate::utils::vendor_prefixed;

declare_lint_rule! {
    /// Disallow unknown descriptors in at-rules.
    ///
    /// Some at-rules, such as `@font-face` and `@property`, contain descriptors instead of
    /// properties. A descriptor looks like a property, but each at-rule accepts only the
    /// descriptors defined for it. Browsers ignore a descriptor they do not know, so it has no
    /// effect. An unknown descriptor is often a typo, or a descriptor that belongs to another
    /// at-rule.
    ///
    /// This rule checks the descriptors of `@color-profile`, `@counter-style`, `@font-face`,
    /// `@font-palette-values`, `@position-try`, `@property`, and `@view-transition`.
    /// In `@position-try`, it checks the properties that the at-rule accepts: the inset, margin,
    /// sizing, and self-alignment properties, `position-anchor`, and `position-area`.
    ///
    /// Vendor-prefixed names, such as `-webkit-font-smoothing`, are ignored.
    ///
    /// The known descriptors come from [webref](https://github.com/w3c/webref), which collects
    /// them from the CSS specifications, including Editor's Drafts.
    ///
    /// ## SCSS limitations
    ///
    /// Descriptor names containing SCSS interpolation are ignored because the emitted name cannot
    /// be determined statically. Declarations inside SCSS blocks, such as `@if`, `@include`, or
    /// nested properties, are also ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @font-face {
    ///   font-famliy: "Example";
    /// }
    /// ```
    ///
    /// `system` is a descriptor of `@counter-style`, not of `@property`.
    ///
    /// ```css,expect_diagnostic
    /// @property --size {
    ///   syntax: "<length>";
    ///   inherits: false;
    ///   initial-value: 0px;
    ///   system: cyclic;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @font-face {
    ///   font-family: "Example";
    ///   src: url("example.woff2") format("woff2");
    /// }
    ///
    /// @counter-style stars {
    ///   system: cyclic;
    ///   symbols: "*";
    ///   suffix: " ";
    /// }
    /// ```
    ///
    pub NoUnknownAtRuleDescriptor {
        version: "next",
        name: "noUnknownAtRuleDescriptor",
        language: "css",
        recommended: false,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("at-rule-descriptor-no-unknown").inspired()],
    }
}

declare_node_union! {
    pub AnyDescriptorAtRule = CssColorProfileAtRule
        | CssCounterStyleAtRule
        | CssFontFaceAtRule
        | CssFontPaletteValuesAtRule
        | CssPositionTryAtRule
        | CssPropertyAtRule
        | CssViewTransitionAtRule
}

impl AnyDescriptorAtRule {
    fn name(&self) -> &'static str {
        match self {
            Self::CssColorProfileAtRule(_) => "@color-profile",
            Self::CssCounterStyleAtRule(_) => "@counter-style",
            Self::CssFontFaceAtRule(_) => "@font-face",
            Self::CssFontPaletteValuesAtRule(_) => "@font-palette-values",
            Self::CssPositionTryAtRule(_) => "@position-try",
            Self::CssPropertyAtRule(_) => "@property",
            Self::CssViewTransitionAtRule(_) => "@view-transition",
        }
    }

    /// Returns the sorted, lowercase names this at-rule accepts in its block.
    fn known_descriptors(&self) -> &'static [&'static str] {
        match self {
            Self::CssColorProfileAtRule(_) => COLOR_PROFILE_DESCRIPTORS,
            Self::CssCounterStyleAtRule(_) => COUNTER_STYLE_DESCRIPTORS,
            Self::CssFontFaceAtRule(_) => FONT_FACE_DESCRIPTORS,
            Self::CssFontPaletteValuesAtRule(_) => FONT_PALETTE_VALUES_DESCRIPTORS,
            Self::CssPositionTryAtRule(_) => POSITION_TRY_PROPERTIES,
            Self::CssPropertyAtRule(_) => PROPERTY_DESCRIPTORS,
            Self::CssViewTransitionAtRule(_) => VIEW_TRANSITION_DESCRIPTORS,
        }
    }
}

pub struct NoUnknownAtRuleDescriptorState {
    descriptor: CssSyntaxToken,
    at_rule: &'static str,
}

impl Rule for NoUnknownAtRuleDescriptor {
    type Query = Ast<CssGenericProperty>;
    type State = NoUnknownAtRuleDescriptorState;
    type Signals = Option<Self::State>;
    type Options = NoUnknownAtRuleDescriptorOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        // Only declarations placed directly in the at-rule's block are descriptors. A declaration
        // in a nested block, such as an SCSS `@include` or a nested property, belongs to that
        // block instead.
        let block = node
            .parent::<CssDeclaration>()?
            .parent::<CssDeclarationWithSemicolon>()?
            .syntax()
            .grand_parent()?;
        let at_rule = block.parent().and_then(AnyDescriptorAtRule::cast)?;

        let descriptor = match node.name().ok()? {
            AnyCssDeclarationName::CssIdentifier(name) => name.value_token().ok()?,
            AnyCssDeclarationName::AnyCssDashedIdentifier(
                AnyCssDashedIdentifier::CssDashedIdentifier(name),
            ) => name.value_token().ok()?,
            AnyCssDeclarationName::AnyCssDashedIdentifier(
                AnyCssDashedIdentifier::ScssInterpolatedDashedIdentifier(_),
            )
            | AnyCssDeclarationName::ScssInterpolatedIdentifier(_)
            | AnyCssDeclarationName::TwValueThemeReference(_)
            | AnyCssDeclarationName::CssMetavariable(_) => return None,
        };

        let name = descriptor.text_trimmed().to_ascii_lowercase_cow();
        if vendor_prefixed(&name)
            || at_rule
                .known_descriptors()
                .binary_search(&name.as_ref())
                .is_ok()
        {
            return None;
        }

        Some(NoUnknownAtRuleDescriptorState {
            descriptor,
            at_rule: at_rule.name(),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let descriptor = state.descriptor.text_trimmed();
        let at_rule = state.at_rule;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.descriptor.text_trimmed_range(),
                markup! {
                    "Unknown descriptor "<Emphasis>{descriptor}</Emphasis>" in the "<Emphasis>{at_rule}</Emphasis>" at-rule."
                },
            )
            .note(markup! {
                "Browsers ignore descriptors that the at-rule does not define, so this declaration has no effect."
            })
            .note(markup! {
                "Fix the descriptor name if it is a typo, or remove the declaration."
            }),
        )
    }
}
