use biome_analyze::{
    context::RuleContext, declare_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_css_syntax::{CssGenericProperty, CssLanguage, CssSyntaxKind};
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
struct NoDeclarationBlockShorthandPropertyOverridesVisitor {
    prior_property_names_in_declaration_block: Vec<String>,
}

impl Visitor for NoDeclarationBlockShorthandPropertyOverridesVisitor {
    type Language = CssLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => match node.kind() {
                CssSyntaxKind::CSS_DECLARATION_OR_RULE_BLOCK => {
                    self.prior_property_names_in_declaration_block.clear();
                }
                CssSyntaxKind::CSS_GENERIC_PROPERTY => {
                    if let Some(property_node) = node.clone().cast::<CssGenericProperty>() {
                        if let Ok(target_property_name_node) = property_node.name() {
                            let target_property_name =
                                target_property_name_node.text().to_lowercase();

                            let vendor_prefix = vender_prefix(&target_property_name);
                            let unprefixed =
                                target_property_name[vendor_prefix.len()..].to_string();

                            let longhand_sub_properties = get_longhand_sub_properties(&unprefixed);
                            let reset_to_initial_properties =
                                get_reset_to_initial_properties(&unprefixed);

                            longhand_sub_properties
                                .iter()
                                .chain(reset_to_initial_properties.iter())
                                .for_each(|property| {
                                    if self
                                        .prior_property_names_in_declaration_block
                                        .contains(&(vendor_prefix.clone() + property))
                                    {
                                        ctx.match_query(
                                            NoDeclarationBlockShorthandPropertyOverridesQuery {
                                                target_property_node: property_node.clone(),
                                                override_property_name: property.to_string(),
                                            },
                                        );
                                    }
                                });

                            self.prior_property_names_in_declaration_block
                                .push(target_property_name);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[derive(Clone)]
pub struct NoDeclarationBlockShorthandPropertyOverridesQuery {
    target_property_node: CssGenericProperty,
    override_property_name: String,
}

impl QueryMatch for NoDeclarationBlockShorthandPropertyOverridesQuery {
    fn text_range(&self) -> TextRange {
        self.target_property_node.range()
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
    target_property_name: String,
    override_property_name: String,
    span: TextRange,
}

impl Rule for NoShorthandPropertyOverrides {
    type Query = NoDeclarationBlockShorthandPropertyOverridesQuery;
    type State = NoDeclarationBlockShorthandPropertyOverridesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let query = ctx.query();

        let (target_property_name, span) = {
            let name = query.target_property_node.name().ok()?;
            (name.text().to_lowercase(), name.range())
        };

        Some(NoDeclarationBlockShorthandPropertyOverridesState {
            target_property_name,
            override_property_name: query.override_property_name.to_string(),
            span,
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.span,
            markup! {
                "Unexpected shorthand property "<Emphasis>{state.target_property_name}</Emphasis>" after "<Emphasis>{state.override_property_name}</Emphasis>
            },
        ))
    }
}
