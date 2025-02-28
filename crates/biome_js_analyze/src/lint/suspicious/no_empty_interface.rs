use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{
    declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::{
    make,
    syntax::{AnyTsType, T},
};
use biome_js_syntax::{
    AnyJsDeclarationClause, JsSyntaxKind, TriviaPieceKind, TsInterfaceDeclaration,
    TsTypeAliasDeclaration,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, SyntaxResult};

declare_lint_rule! {
    /// Disallow the declaration of empty interfaces.
    ///
    /// An empty interface in TypeScript does very little: any non-nullable value is assignable to `{}`.
    /// Using an empty interface is often a sign of programmer error, such as misunderstanding the concept of `{}` or forgetting to fill in fields.
    ///
    /// The rule ignores empty interfaces that `extends` one or multiple types.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface A {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// interface A {
    ///   prop: string;
    /// }
    ///
    /// // Allow empty interfaces that extend a type.
    /// interface B extends A {}
    ///
    /// // Allow empty interfaces in ambient modules
    /// declare module "mod" {
    ///   interface C {}
    /// }
    /// ```
    pub NoEmptyInterface {
        version: "1.0.0",
        name: "noEmptyInterface",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-empty-interface")],
        source_kind: RuleSourceKind::Inspired,
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoEmptyInterface {
    type Query = Ast<TsInterfaceDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let is_in_ambient_module = node.syntax().ancestors().skip(1).any(|ancestor| {
            matches!(
                ancestor.kind(),
                JsSyntaxKind::TS_GLOBAL_DECLARATION | JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION
            )
        });
        if is_in_ambient_module {
            return None;
        }
        (node.members().is_empty() && node.extends_clause().is_none()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! { "An "<Emphasis>"empty interface"</Emphasis>" is equivalent to "<Emphasis>"{}"</Emphasis>"." },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query();
        let new_node = make_type_alias_from_interface(
            node,
            AnyTsType::from(make::ts_object_type(
                make::token(T!['{']),
                make::ts_type_member_list([]),
                make::token(T!['}']),
            )),
        )
        .ok()?;
        mutation.replace_node(
            AnyJsDeclarationClause::from(node.clone()),
            AnyJsDeclarationClause::from(new_node),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use a type alias instead." }.to_owned(),
            mutation,
        ))
    }
}

/// Builds a [TsTypeAliasDeclaration] from an [TsInterfaceDeclaration].
fn make_type_alias_from_interface(
    node: &TsInterfaceDeclaration,
    ts_type: AnyTsType,
) -> SyntaxResult<TsTypeAliasDeclaration> {
    let new_node = make::ts_type_alias_declaration(
        make::token(T![type]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        node.id()?,
        make::token(T![=]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        ts_type,
    );
    let new_node = if let Some(type_params) = node.type_parameters() {
        new_node.with_type_parameters(type_params)
    } else {
        new_node
    };
    Ok(new_node.build())
}
