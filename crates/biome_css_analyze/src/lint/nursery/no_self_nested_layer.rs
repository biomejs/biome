use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssLayer, AnyCssLayerName, CssLayerAtRule, CssLayerNameList, decode_css_identifier,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_self_nested_layer::NoSelfNestedLayerOptions;

declare_lint_rule! {
    /// Disallow nesting a cascade layer inside a layer with the same name.
    ///
    /// A layer name written inside an `@layer` block is relative to that block. Writing the
    /// enclosing layer's name again doesn't add styles to that layer: it creates a sublayer. For
    /// example, `@layer base { @layer base { ... } }` creates `base.base`, which has its own place in
    /// the cascade order. The styles still apply, so the mistake is easy to miss.
    ///
    /// The rule also reports names that start with an enclosing layer's name, such as `@layer base.reset`
    /// inside `@layer base`, which creates `base.base.reset` instead of `base.reset`.
    ///
    /// In SCSS, a name that uses interpolation only matches the same interpolation, such as
    /// `@layer #{$name}` inside `@layer #{$name}`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @layer components {
    ///   @layer components {
    ///     .button {
    ///       color: red;
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @layer base {
    ///   @layer base.reset {
    ///     * {
    ///       margin: 0;
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @layer base {
    ///   @layer reset {
    ///     * {
    ///       margin: 0;
    ///     }
    ///   }
    /// }
    /// ```
    ///
    pub NoSelfNestedLayer {
        version: "2.6.3",
        name: "noSelfNestedLayer",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
    }
}

pub struct RuleState {
    name: TextRange,
    enclosing: TextRange,
}

impl Rule for NoSelfNestedLayer {
    type Query = Ast<CssLayerAtRule>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoSelfNestedLayerOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let names = match node.layer() {
            Ok(AnyCssLayer::CssLayerDeclaration(declaration)) => declaration.references(),
            Ok(AnyCssLayer::CssLayerReference(reference)) => reference.references(),
            _ => return None,
        };
        names.iter().flatten().find_map(|name| {
            let enclosing = enclosing_layer_names(node)
                .find(|enclosing| starts_with_layer_name(&name, enclosing))?;
            Some(RuleState {
                name: name.range(),
                enclosing: enclosing.range(),
            })
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.name,
                markup! {
                    "This layer repeats the name of an enclosing layer."
                },
            )
            .detail(state.enclosing, markup! {
                "The enclosing layer is declared here."
            })
            .note(markup! {
                "Layer names inside a layer block are relative to it, so this creates a sublayer instead of adding to the enclosing layer. A sublayer has its own place in the cascade order."
            })
            .note(markup! {
                "Rename the layer, or remove the nested "<Emphasis>"@layer"</Emphasis>" if you didn't mean to create a sublayer."
            }),
        )
    }
}

/// Returns the names of the `@layer` blocks that contain `node`, innermost first.
///
/// Anonymous layers and blocks that declare more than one name have no single name to compare,
/// so they are skipped.
fn enclosing_layer_names(node: &CssLayerAtRule) -> impl Iterator<Item = CssLayerNameList> {
    node.syntax()
        .ancestors()
        .skip(1)
        .filter_map(CssLayerAtRule::cast)
        .filter_map(|layer| {
            let declaration = layer.layer().ok()?;
            let references = declaration.as_css_layer_declaration()?.references();
            match (references.first(), references.len()) {
                (Some(Ok(name)), 1) => Some(name),
                _ => None,
            }
        })
}

/// Whether the dot-separated segments of `enclosing` are a prefix of those of `name`.
///
/// A segment that uses Sass interpolation only matches the same interpolation text, such as
/// `#{$name}` in both names, because its value is only known after compilation. That assumes the
/// variable keeps its value between the two layers. Segments after the compared prefix don't
/// affect the result, so `base.#{$x}` still starts with `base`.
fn starts_with_layer_name(name: &CssLayerNameList, enclosing: &CssLayerNameList) -> bool {
    if enclosing.len() > name.len() {
        return false;
    }
    enclosing
        .iter()
        .zip(name.iter())
        .all(|(enclosing, name)| match (enclosing, name) {
            (
                Ok(AnyCssLayerName::CssIdentifier(enclosing)),
                Ok(AnyCssLayerName::CssIdentifier(name)),
            ) => match (enclosing.value_token(), name.value_token()) {
                (Ok(enclosing), Ok(name)) => {
                    decode_css_identifier(enclosing.text_trimmed())
                        == decode_css_identifier(name.text_trimmed())
                }
                _ => false,
            },
            (
                Ok(AnyCssLayerName::ScssInterpolatedIdentifier(enclosing)),
                Ok(AnyCssLayerName::ScssInterpolatedIdentifier(name)),
            ) => enclosing.syntax().text_trimmed() == name.syntax().text_trimmed(),
            _ => false,
        })
}
