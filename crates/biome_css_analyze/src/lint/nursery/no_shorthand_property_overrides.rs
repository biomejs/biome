use biome_analyze::{
    context::RuleContext, declare_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssDeclarationName, CssGenericProperty, CssLanguage, CssSyntaxKind};
use biome_rowan::{AstNode, Language, SyntaxNode, SyntaxNodeCast, TextRange, WalkEvent};

use crate::utils::{get_longhand_sub_properties, get_reset_to_initial_properties, vender_prefix};

declare_rule! {
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
        version: "next",
        name: "noShorthandPropertyOverrides",
        language: "css",
        recommended: true,
    }
}

#[derive(Default)]
struct PriorProperty {
    original: String,
    lowercase: String,
}

#[derive(Default)]
struct NoDeclarationBlockShorthandPropertyOverridesVisitor {
    prior_properties_in_block: Vec<PriorProperty>,
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
                    self.prior_properties_in_block.clear();
                }
                CssSyntaxKind::CSS_GENERIC_PROPERTY => {
                    if let Some(property_node) = node
                        .clone()
                        .cast::<CssGenericProperty>()
                        .and_then(|property_node| property_node.name().ok())
                    {
                        let property_original = property_node.text();
                        let property_lowercase = property_original.to_lowercase();

                        let vendor_prefix = vender_prefix(&property_lowercase);
                        let unprefixed_property =
                            property_lowercase[vendor_prefix.len()..].to_string();

                        let longhand_sub_properties =
                            get_longhand_sub_properties(&unprefixed_property);
                        let reset_to_initial_properties =
                            get_reset_to_initial_properties(&unprefixed_property);

                        longhand_sub_properties
                            .iter()
                            .chain(reset_to_initial_properties.iter())
                            .for_each(|override_property| {
                                self.prior_properties_in_block
                                    .iter()
                                    .for_each(|prior_property| {
                                        if prior_property.lowercase
                                            == (vendor_prefix.clone() + override_property)
                                        {
                                            ctx.match_query(
                                                NoDeclarationBlockShorthandPropertyOverridesQuery {
                                                    property_node: property_node.clone(),
                                                    override_property: prior_property
                                                        .original
                                                        .clone(),
                                                },
                                            );
                                        }
                                    });
                            });

                        self.prior_properties_in_block.push(PriorProperty {
                            original: property_original,
                            lowercase: property_lowercase,
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
            target_property: query.property_node.text(),
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
