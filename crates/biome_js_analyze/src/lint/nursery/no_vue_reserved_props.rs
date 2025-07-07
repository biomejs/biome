use crate::frameworks::vue::vue_component::{
    VueComponent, VueComponentDeclarations, VueComponentQuery, VueDeclaration,
    VueDeclarationCollectionFilter, VueDeclarationName,
};
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsFileSource;
use biome_rowan::AstNode;
use enumflags2::make_bitflags;

declare_lint_rule! {
    /// Succinct description of the rule.
    pub NoVueReservedProps {
        version: "next",
        name: "noVueReservedProps",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
    }
}

impl Rule for NoVueReservedProps {
    type Query = VueComponentQuery;
    type State = VueDeclaration;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type::<JsFileSource>(),
        ) else {
            return Box::new([]);
        };

        println!("Has component: {}", ctx.query().to_trimmed_text().text());

        component
            .declarations(make_bitflags!(VueDeclarationCollectionFilter::Prop))
            .into_iter()
            .filter_map(|declaration| {
                let name = declaration.declaration_name()?;
                println!("Decl name: {}", name.text());
                if RESERVED_PROPS.binary_search(&name.text()).is_ok() {
                    Some(declaration)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.declaration_name_range()?,
            markup! {
                <Emphasis>{state.declaration_name()?.text()}</Emphasis>" is a reserved attribute and cannot be used as props."
            },
        ).note(
            markup! {
                "Rename the prop to avoid possible conflicts."
            },
        ))
    }
}

const RESERVED_PROPS: &[&str] = &[
    "class",
    "is",
    "key",
    "ref",
    "slot",
    "slot-scope",
    "slotScope",
    "style",
];
