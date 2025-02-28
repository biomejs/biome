use crate::utils::{get_longhand_sub_properties, get_reset_to_initial_properties, vender_prefix};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, RuleSource, ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssDeclarationName, CssGenericProperty, CssLanguage, CssSyntaxKind};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, Language, SyntaxNode, TextRange, WalkEvent};

fn remove_vendor_prefix<'a>(prop: &'a str, prefix: &'a str) -> &'a str {
    if let Some(prop) = prop.strip_prefix(prefix) {
        return prop;
    }

    prop
}

fn get_override_props(property: &str) -> Vec<&str> {
    let longhand_sub_props = get_longhand_sub_properties(property);
    let reset_to_initial_props = get_reset_to_initial_properties(property);

    let mut merged = Vec::with_capacity(longhand_sub_props.len() + reset_to_initial_props.len());

    let (mut i, mut j) = (0, 0);

    while i < longhand_sub_props.len() && j < reset_to_initial_props.len() {
        if longhand_sub_props[i] < reset_to_initial_props[j] {
            merged.push(longhand_sub_props[i]);
            i += 1;
        } else {
            merged.push(reset_to_initial_props[j]);
            j += 1;
        }
    }

    if i < longhand_sub_props.len() {
        merged.extend_from_slice(&longhand_sub_props[i..]);
    }

    if j < reset_to_initial_props.len() {
        merged.extend_from_slice(&reset_to_initial_props[j..]);
    }

    merged
}

declare_lint_rule! {
    /// Disallow shorthand properties that override related longhand properties.
    ///
    /// For details on shorthand properties, see the [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/CSS/Shorthand_properties).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { padding-left: 10px; padding: 20px; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { padding: 10px; padding-left: 20px; }
    /// ```
    ///
    /// ```css
    /// a { transition-property: opacity; } a { transition: opacity 1s linear; }
    /// ```
    ///
    pub NoShorthandPropertyOverrides {
        version: "1.8.2",
        name: "noShorthandPropertyOverrides",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("declaration-block-no-shorthand-property-overrides")],
    }
}

#[derive(Default)]
struct PriorProperty {
    original: String,
    lowercase: String,
}

#[derive(Default)]
struct NoDeclarationBlockShorthandPropertyOverridesVisitor {
    prior_props_in_block: Vec<PriorProperty>,
}

impl Visitor for NoDeclarationBlockShorthandPropertyOverridesVisitor {
    type Language = CssLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        if let WalkEvent::Enter(node) = event {
            match node.kind() {
                CssSyntaxKind::CSS_DECLARATION_OR_RULE_BLOCK => {
                    self.prior_props_in_block.clear();
                }
                CssSyntaxKind::CSS_GENERIC_PROPERTY => {
                    if let Some(prop_node) = CssGenericProperty::cast_ref(node)
                        .and_then(|property_node| property_node.name().ok())
                    {
                        let prop = prop_node.to_trimmed_string();
                        #[expect(clippy::disallowed_methods)]
                        let prop_lowercase = prop.to_lowercase();

                        let prop_prefix = vender_prefix(&prop_lowercase);
                        let unprefixed_prop = remove_vendor_prefix(&prop_lowercase, prop_prefix);
                        let override_props = get_override_props(unprefixed_prop);

                        self.prior_props_in_block.iter().for_each(|prior_prop| {
                            let prior_prop_prefix = vender_prefix(&prior_prop.lowercase);
                            let unprefixed_prior_prop =
                                remove_vendor_prefix(&prior_prop.lowercase, prior_prop_prefix);

                            if prop_prefix == prior_prop_prefix
                                && override_props.binary_search(&unprefixed_prior_prop).is_ok()
                            {
                                ctx.match_query(
                                    NoDeclarationBlockShorthandPropertyOverridesQuery {
                                        property_node: prop_node.clone(),
                                        override_property: prior_prop.original.clone(),
                                    },
                                );
                            }
                        });

                        self.prior_props_in_block.push(PriorProperty {
                            original: prop,
                            lowercase: prop_lowercase,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone)]
pub struct NoDeclarationBlockShorthandPropertyOverridesQuery {
    property_node: AnyCssDeclarationName,
    override_property: String,
}

impl QueryMatch for NoDeclarationBlockShorthandPropertyOverridesQuery {
    fn text_range(&self) -> TextRange {
        self.property_node.range()
    }
}

impl Queryable for NoDeclarationBlockShorthandPropertyOverridesQuery {
    type Input = Self;
    type Language = CssLanguage;
    type Output = NoDeclarationBlockShorthandPropertyOverridesQuery;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(
            Phases::Syntax,
            NoDeclarationBlockShorthandPropertyOverridesVisitor::default,
        );
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.clone()
    }
}

pub struct NoDeclarationBlockShorthandPropertyOverridesState {
    target_property: String,
    override_property: String,
    span: TextRange,
}

impl Rule for NoShorthandPropertyOverrides {
    type Query = NoDeclarationBlockShorthandPropertyOverridesQuery;
    type State = NoDeclarationBlockShorthandPropertyOverridesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let query = ctx.query();

        Some(NoDeclarationBlockShorthandPropertyOverridesState {
            target_property: query.property_node.to_trimmed_string(),
            override_property: query.override_property.clone(),
            span: query.text_range(),
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.span,
            markup! {
                "Unexpected shorthand property "<Emphasis>{state.target_property}</Emphasis>" after "<Emphasis>{state.override_property}</Emphasis>
            },
        ))
    }
}
