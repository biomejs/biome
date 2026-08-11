use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{
    CssGenericComponentValueList, CssPropertyAtRule, decode_css_identifier,
    property_syntax::PropertySyntaxResult,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_invalid_property_init_value::NoInvalidPropertyInitValueOptions;

declare_lint_rule! {
    /// Checks that the `initial-value` of an `@property` rule follows the value format declared by its `syntax`.
    ///
    /// Browsers do not register a custom property when its `initial-value` does not follow this
    /// format.
    ///
    /// For function values, this rule checks the function name but does not check its arguments.
    /// It leaves the browser to validate:
    ///
    /// - indexed or unknown `env()` values, whose result may depend on an index or fallback;
    /// - math functions whose result depends on their arguments, such as `calc()`, `min()`, and
    ///   `max()`, used with `<angle>`, `<integer>`, `<length>`, `<length-percentage>`, `<number>`,
    ///   `<percentage>`, `<resolution>`, or `<time>`;
    /// - color functions such as `rgb()` and `color-mix()` used with `<color>`;
    /// - image functions such as `linear-gradient()` and `image-set()` used with `<image>`;
    /// - transform functions such as `rotate()` and `translateX()` used with
    ///   `<transform-function>` or `<transform-list>`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// `red` is a color, not a length, so the browser does not register `--size`.
    ///
    /// ```css,expect_diagnostic
    /// @property --size {
    ///   syntax: "<length>";
    ///   inherits: false;
    ///   initial-value: red;
    /// }
    /// ```
    ///
    /// `#fff` is a color, not an image, so the browser does not register `--background-image`.
    ///
    /// ```css,expect_diagnostic
    /// @property --background-image {
    ///   syntax: "<image>";
    ///   inherits: false;
    ///   initial-value: #fff;
    /// }
    /// ```
    ///
    /// `<color>#` requires one or more colors separated by commas. The browser does not register
    /// `--palette` because `red blue` has no comma.
    ///
    /// ```css,expect_diagnostic
    /// @property --palette {
    ///   syntax: "<color>#";
    ///   inherits: false;
    ///   initial-value: red blue;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// Both `1rem` and `calc(1px + 2px)` use length values, so they follow their declared formats.
    ///
    /// ```css
    /// @property --size {
    ///   syntax: "<length>";
    ///   inherits: false;
    ///   initial-value: 1rem;
    /// }
    ///
    /// @property --calculated-size {
    ///   syntax: "<length>";
    ///   inherits: false;
    ///   initial-value: calc(1px + 2px);
    /// }
    /// ```
    ///
    pub NoInvalidPropertyInitValue {
        version: "2.5.8",
        name: "noInvalidPropertyInitValue",
        language: "css",
        recommended: true,
    }
}

impl Rule for NoInvalidPropertyInitValue {
    type Query = Semantic<CssPropertyAtRule>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoInvalidPropertyInitValueOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let property = model
            .global_custom_variables()
            .at_property_by_range(node.range())?;

        let PropertySyntaxResult::Value(syntax) = property.syntax() else {
            return None;
        };
        if syntax.is_universal() {
            return None;
        }

        let initial_value = find_initial_value(node)?;
        if syntax.matches_value(&initial_value) {
            None
        } else {
            Some(initial_value.range())
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! { "The "<Emphasis>"initial-value"</Emphasis>" does not match the registered property syntax." },
            )
            .note(markup! {
                "A mismatched initial value prevents the custom property from being registered."
            })
            .note(markup! {
                "Use an initial value accepted by the "<Emphasis>"syntax"</Emphasis>" descriptor."
            }),
        )
    }
}

fn find_initial_value(node: &CssPropertyAtRule) -> Option<CssGenericComponentValueList> {
    node.block()
        .ok()?
        .as_css_declaration_block()?
        .declarations()
        .into_iter()
        .filter_map(|declaration| {
            declaration
                .as_css_declaration_with_semicolon()?
                .declaration()
                .ok()?
                .property()
                .ok()?
                .as_css_generic_property()
                .cloned()
        })
        .filter(|property| {
            property
                .name()
                .ok()
                .and_then(|name| name.identifier_text())
                .is_some_and(|name| {
                    decode_css_identifier(name.text()).eq_ignore_ascii_case("initial-value")
                })
        })
        .filter_map(|property| {
            property
                .value()
                .ok()?
                .as_css_generic_component_value_list()
                .cloned()
        })
        .last()
}
