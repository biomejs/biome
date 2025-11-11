use crate::react::components::{AnyPotentialReactComponentDeclaration, ReactComponentInfo};
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_rule_options::no_nested_component_definitions::NoNestedComponentDefinitionsOptions;

declare_lint_rule! {
    /// Disallows defining React components inside other components.
    ///
    /// Component definitions inside other components cause them to be recreated on every render,
    /// which can lead to performance issues and unexpected behavior.
    ///
    /// When a component is defined inside another component:
    /// - It gets recreated on every render of the parent component
    /// - It loses its internal state when the parent rerenders
    /// - It defeats props memoization and optimization techniques
    /// - It creates new function references on every render
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// A new component is created every time ParentComponent renders:
    /// ```jsx,expect_diagnostic
    /// function ParentComponent() {
    ///   function ChildComponent() {
    ///     return <div>Hello</div>;
    ///   }
    ///
    ///   return <ChildComponent />;
    /// }
    /// ```
    ///
    /// Even with memo, a new component is still created on each render:
    /// ```jsx,expect_diagnostic
    /// function ParentComponent() {
    ///   const MemoizedChild = memo(() => {
    ///     return <div>Hello</div>;
    ///   });
    ///
    ///   return <MemoizedChild />;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// Component is defined outside other components:
    /// ```jsx
    /// function ChildComponent() {
    ///   return <div>Hello</div>;
    /// }
    ///
    /// function ParentComponent() {
    ///   return <ChildComponent />;
    /// }
    /// ```
    ///
    /// ## Correct approaches
    ///
    /// 1. Move the component definition outside:
    ///    ```jsx
    ///    function ChildComponent() {
    ///      return <div>Hello</div>;
    ///    }
    ///
    ///    function ParentComponent() {
    ///      return <ChildComponent />;
    ///    }
    ///    ```
    ///
    /// 2. Pass components as props:
    ///    ```jsx
    ///    function ParentComponent({ CustomComponent }) {
    ///      return <CustomComponent />;
    ///    }
    ///    ```
    ///
    /// 3. Use React's Children API:
    ///    ```jsx
    ///    function ParentComponent({ children }) {
    ///      return <div>{children}</div>;
    ///    }
    ///    ```
    pub NoNestedComponentDefinitions {
        version: "2.0.0",
        name: "noNestedComponentDefinitions",
        language: "jsx",
        sources: &[
            RuleSource::EslintReactX("no-nested-components").same(),
            RuleSource::EslintReactXyz("no-nested-components").same(),
            RuleSource::EslintReactXyz("no-nested-component-definitions").same(),
        ],
        recommended: false,
        domains: &[RuleDomain::React],
        severity: Severity::Error,
    }
}

impl Rule for NoNestedComponentDefinitions {
    type Query = Ast<AnyPotentialReactComponentDeclaration>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoNestedComponentDefinitionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if let Some(component) = ReactComponentInfo::from_declaration(node.syntax())
            && let Some(parent_component) = node
                .syntax()
                .ancestors()
                .skip_while(|ancestor| ancestor.eq(node.syntax()))
                .find_map(|syntax| ReactComponentInfo::from_declaration(&syntax))
        {
            return Some(RuleState {
                component_range: component.declaration_highlight_range(),
                parent_component_range: parent_component.declaration_highlight_range(),
            });
        };
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.component_range,
                markup! {
                    "Components should not be defined inside other components."
                },
            )
            .detail(
                state.parent_component_range,
                markup! {
                    "Move it outside of the parent component or pass it as a prop."
                },
            )
            .note(markup! {
                "Component definitions inside other components cause them to be recreated on every render, which can lead to performance issues and unexpected behavior."
            }),
        )
    }
}

pub struct RuleState {
    /// The component range.
    pub component_range: TextRange,
    /// The parent component range.
    pub parent_component_range: TextRange,
}
