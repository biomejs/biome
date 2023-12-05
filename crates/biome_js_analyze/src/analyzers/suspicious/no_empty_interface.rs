use crate::control_flow::AnyJsControlFlowRoot;
use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::{
    make,
    syntax::{AnyTsType, T},
};
use biome_js_syntax::{
    AnyJsDeclarationClause, TriviaPieceKind, TsExternalModuleDeclaration, TsInterfaceDeclaration,
    TsTypeAliasDeclaration,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt};

declare_rule! {
    /// Disallow the declaration of empty interfaces.
    ///
    /// An empty interface in TypeScript does very little: any non-nullable value is assignable to `{}`.
    /// Using an empty interface is often a sign of programmer error, such as misunderstanding the concept of `{}` or forgetting to fill in fields.
    ///
    /// Source: https://typescript-eslint.io/rules/no-empty-interface
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface A {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// interface A extends B {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// interface A {
    ///   prop: string;
    /// }
    ///
    /// // Allow empty interfaces that extend at least two types.
    /// interface A extends B, C {}
    ///
    /// declare module "@external/module" {
    ///   // Allow empty interfaces that extend at least one type in external module.
    ///   interface Existing extends A {}
    /// }
    /// ```
    pub(crate) NoEmptyInterface {
        version: "1.0.0",
        name: "noEmptyInterface",
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

pub enum DiagnosticMessage {
    NoEmptyInterface,
    NoEmptyInterfaceWithSuper,
}

impl DiagnosticMessage {
    /// Convert a [DiagnosticMessage] to a string
    fn as_str(&self) -> &'static str {
        match self {
            Self::NoEmptyInterface => "An empty interface is equivalent to '{}'.",
            Self::NoEmptyInterfaceWithSuper => {
                "An interface declaring no members is equivalent to its supertype."
            }
        }
    }

    /// Retrieves a [TsTypeAliasDeclaration] from a [DiagnosticMessage] that will be used to
    /// replace it on the rule action
    fn fix_with(&self, node: &TsInterfaceDeclaration) -> Option<TsTypeAliasDeclaration> {
        match self {
            Self::NoEmptyInterface => make_type_alias_from_interface(
                node,
                AnyTsType::from(make::ts_object_type(
                    make::token(T!['{']),
                    make::ts_type_member_list([]),
                    make::token(T!['}']),
                )),
            ),
            Self::NoEmptyInterfaceWithSuper => {
                let super_interface = node.extends_clause()?.types().into_iter().next()?.ok()?;
                let type_arguments = super_interface.type_arguments();
                let ts_reference_type = make::ts_reference_type(super_interface.name().ok()?);

                let ts_reference_type = if type_arguments.is_some() {
                    ts_reference_type
                        .with_type_arguments(type_arguments?)
                        .build()
                } else {
                    ts_reference_type.build()
                };

                make_type_alias_from_interface(node, AnyTsType::from(ts_reference_type))
            }
        }
    }
}

impl Rule for NoEmptyInterface {
    type Query = Ast<TsInterfaceDeclaration>;
    type State = DiagnosticMessage;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !node.members().is_empty() {
            return None;
        }
        let Some(extends_clause) = node.extends_clause() else {
            return Some(DiagnosticMessage::NoEmptyInterface);
        };
        if node
            .syntax()
            .ancestors()
            .skip(1)
            .find(|x| AnyJsControlFlowRoot::can_cast(x.kind()))
            .is_some_and(|x| TsExternalModuleDeclaration::can_cast(x.kind()))
        {
            // Ignore interfaces that extend a type in an external module declaration.
            // The interface can be merged with an existing interface.
            return None;
        }
        if extends_clause.types().len() == 1 {
            return Some(DiagnosticMessage::NoEmptyInterfaceWithSuper);
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            state.as_str(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query();

        mutation.replace_node(
            AnyJsDeclarationClause::from(node.clone()),
            AnyJsDeclarationClause::from(state.fix_with(node)?),
        );

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Convert empty interface to type alias." }.to_owned(),
            mutation,
        })
    }
}

/// Builds a [TsTypeAliasDeclaration] from an [TsInterfaceDeclaration].
fn make_type_alias_from_interface(
    node: &TsInterfaceDeclaration,
    ts_type: AnyTsType,
) -> Option<TsTypeAliasDeclaration> {
    let type_params = node.type_parameters();
    let new_node = make::ts_type_alias_declaration(
        make::token(T![type]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        node.id().ok()?,
        make::token(T![=]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        ts_type,
    );

    let new_node = if type_params.is_some() {
        new_node.with_type_parameters(type_params?).build()
    } else {
        new_node.build()
    };

    Some(new_node)
}
