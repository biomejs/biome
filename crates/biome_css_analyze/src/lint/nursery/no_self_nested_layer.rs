use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{AnyCssLayer, AnyCssLayerName, CssLayerAtRule, CssLayerNameList};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstSeparatedList};
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
        version: "next",
        name: "noSelfNestedLayer",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
    }
}

pub struct RuleState {
    name: CssLayerNameList,
    enclosing: CssLayerNameList,
}

impl Rule for NoSelfNestedLayer {
    type Query = Ast<CssLayerAtRule>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoSelfNestedLayerOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let names = match node.layer() {
            Ok(AnyCssLayer::CssLayerDeclaration(declaration)) => declaration.references(),
            Ok(AnyCssLayer::CssLayerReference(reference)) => reference.references(),
            _ => return Box::default(),
        };
        let enclosing: Vec<_> = enclosing_layer_names(node).collect();
        if enclosing.is_empty() {
            return Box::default();
        }

        names
            .iter()
            .flatten()
            .filter_map(|name| {
                let enclosing = enclosing
                    .iter()
                    .find(|enclosing| starts_with_layer_name(&name, enclosing))?;
                Some(RuleState {
                    name,
                    enclosing: enclosing.clone(),
                })
            })
            .collect()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.name.to_trimmed_string();
        let enclosing = state.enclosing.to_trimmed_string();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.name.range(),
            markup! {
                "The layer "<Emphasis>{name}</Emphasis>" repeats the name of the enclosing layer "<Emphasis>{enclosing}</Emphasis>"."
            },
        );
        let diagnostic = match full_layer_name(ctx.query(), &name) {
            Some(full_name) => diagnostic.note(markup! {
                "Layer names inside a layer block are relative to it, so this creates the sublayer "<Emphasis>{full_name}</Emphasis>". A sublayer has its own place in the cascade order."
            }),
            None => diagnostic.note(markup! {
                "Layer names inside a layer block are relative to it, so this creates a new sublayer. A sublayer has its own place in the cascade order."
            }),
        };
        Some(diagnostic.note(markup! {
            "Rename the layer so it doesn't repeat "<Emphasis>{enclosing}</Emphasis>", or remove the nested "<Emphasis>"@layer"</Emphasis>" if you didn't mean to create a sublayer."
        }))
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
                (Ok(enclosing), Ok(name)) => enclosing.text_trimmed() == name.text_trimmed(),
                _ => false,
            },
            (
                Ok(AnyCssLayerName::ScssInterpolatedIdentifier(enclosing)),
                Ok(AnyCssLayerName::ScssInterpolatedIdentifier(name)),
            ) => enclosing.syntax().text_trimmed() == name.syntax().text_trimmed(),
            _ => false,
        })
}

/// Joins the names of every enclosing `@layer` block with `name`, outermost first.
///
/// Returns `None` when an enclosing layer is anonymous, because the resulting sublayer can't be
/// written as a name.
fn full_layer_name(node: &CssLayerAtRule, name: &str) -> Option<String> {
    let mut segments = vec![name.to_string()];
    for layer in node.syntax().ancestors().skip(1).filter_map(CssLayerAtRule::cast) {
        let declaration = layer.layer().ok()?;
        let references = declaration.as_css_layer_declaration()?.references();
        if references.len() != 1 {
            return None;
        }
        segments.push(references.first()?.ok()?.to_trimmed_string());
    }
    segments.reverse();
    Some(segments.join("."))
}
