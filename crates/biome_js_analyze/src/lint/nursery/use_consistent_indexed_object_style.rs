use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyTsTypeMember, TsInterfaceDeclaration, TsObjectType, TsTypeMemberList};
use biome_rowan::{AstNode, AstNodeList, declare_node_union};
use biome_rule_options::use_consistent_indexed_object_style::UseConsistentIndexedObjectStyleOptions;

declare_lint_rule! {
    /// Require using the `Record` type instead of an index signature.
    ///
    /// A type whose only member is an index signature can be written with the
    /// built-in `Record` utility type, which is more concise and reads
    /// consistently across a codebase.
    ///
    /// This rule flags a type literal or an interface whose sole member is an
    /// index signature. Types that also declare other members, and `readonly`
    /// index signatures, are left alone.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// type Foo = { [key: string]: unknown };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// interface Bar {
    ///     [key: string]: unknown;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// type Foo = Record<string, unknown>;
    /// ```
    ///
    /// ```ts
    /// interface Bar {
    ///     [key: string]: unknown;
    ///     length: number;
    /// }
    /// ```
    ///
    pub UseConsistentIndexedObjectStyle {
        version: "next",
        name: "useConsistentIndexedObjectStyle",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("consistent-indexed-object-style").same()],
    }
}

declare_node_union! {
    pub AnyIndexSignatureContainer = TsObjectType | TsInterfaceDeclaration
}

impl Rule for UseConsistentIndexedObjectStyle {
    type Query = Ast<AnyIndexSignatureContainer>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseConsistentIndexedObjectStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let members: TsTypeMemberList = match ctx.query() {
            AnyIndexSignatureContainer::TsObjectType(object) => object.members(),
            AnyIndexSignatureContainer::TsInterfaceDeclaration(interface) => {
                // An interface with a heritage clause can't be rewritten as a
                // `Record` without dropping its `extends`, so leave it alone.
                if interface.extends_clause().is_some() {
                    return None;
                }
                interface.members()
            }
        };

        // The type must consist of exactly one member, an index signature.
        if members.len() != 1 {
            return None;
        }
        let AnyTsTypeMember::TsIndexSignatureTypeMember(signature) = members.iter().next()? else {
            return None;
        };

        // `readonly` index signatures map to `Readonly<Record<…>>`; leave them
        // for a later iteration rather than suggesting a lossy conversion.
        if signature.readonly_token().is_some() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Prefer the "<Emphasis>"Record"</Emphasis>" type over an index signature."
                },
            )
            .note(markup! {
                "A type whose only member is an index signature is equivalent to a "<Emphasis>"Record"</Emphasis>", which is more concise and consistent."
            })
            .note(markup! {
                "Replace it with "<Emphasis>"Record<Key, Value>"</Emphasis>"."
            }),
        )
    }
}
