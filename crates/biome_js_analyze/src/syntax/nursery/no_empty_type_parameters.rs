use biome_analyze::Ast;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::TextRange;
use biome_js_syntax::{TsInterfaceDeclaration, TsTypeAliasDeclaration};
use biome_rowan::declare_node_union;
use biome_rowan::AstNode;
use biome_rowan::AstSeparatedList;

declare_rule! {
    /// Disallow empty type parameters in type aliases and interfaces.
    ///
    /// TypeScript permits the use of empty type parameter lists in type alias and interface declarations; however, this practice is generally discouraged.
    /// Allowing empty type parameter lists can lead to unclear or ambiguous code, where the intention of the generic type is not self-evident.
    /// This rule disallows empty type parameter lists in type alias and interface declarations.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface Foo<> {}
    /// ```
    /// ```ts,expect_diagnostic
    /// type Foo<> = {}
    /// ```
    ///
    /// ## Valid
    ///
    /// ```ts
    /// interface Foo{}
    /// ```
    ///
    /// ```ts
    /// type Foo<T> = {
    ///  bar: T;
    /// }
    /// ```
    ///
    pub(crate) NoEmptyTypeParameters {
        version: "next",
        name: "noEmptyTypeParameters",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) NoEmptyTypeParametersQuery = TsInterfaceDeclaration | TsTypeAliasDeclaration
}

impl Rule for NoEmptyTypeParameters {
    type Query = Ast<NoEmptyTypeParametersQuery>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        match binding {
            NoEmptyTypeParametersQuery::TsInterfaceDeclaration(decl) => {
                let type_parameters = decl.type_parameters()?;
                if type_parameters.items().is_empty() {
                    return Some(type_parameters.items().range());
                }
                None
            }
            NoEmptyTypeParametersQuery::TsTypeAliasDeclaration(decl) => {
                let type_parameters = decl.type_parameters()?;
                if type_parameters.items().is_empty() {
                    return Some(type_parameters.items().range());
                }
                None
            }
        }
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            reference,
            markup! {"Empty type parameter list "<Emphasis>"<>"</Emphasis>" is not recommended"},
        ))
    }
}
