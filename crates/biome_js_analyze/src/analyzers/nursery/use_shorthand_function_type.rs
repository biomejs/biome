use crate::JsRuleAction;
use biome_analyze::RuleSource;
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_factory::make::ts_type_alias_declaration;
use biome_js_syntax::AnyTsType::TsThisType;
use biome_js_syntax::{
    AnyJsDeclarationClause, AnyTsReturnType, AnyTsType, TsCallSignatureTypeMember, TsFunctionType,
    TsInterfaceDeclaration, TsObjectType, TsTypeMemberList, T,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPieceKind};

declare_rule! {
    /// Enforce using function types instead of object type with call signatures.
    ///
    /// TypeScript allows for two common ways to declare a type for a function:
    ///
    /// - Function type: `() => string`
    /// - Object type with a signature: `{ (): string }`
    ///
    /// The function type form is generally preferred when possible for being more succinct.
    ///
    /// This rule suggests using a function type instead of an interface or object type literal with a single call signature.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface Example {
    ///   (): string;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function foo(example: { (): number }): number {
    ///   return example();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// type Example = () => string;
    /// ```
    ///
    /// ```ts
    /// function foo(example: () => number): number {
    ///   return bar();
    /// }
    /// ```
    ///
    /// ```ts
    /// // returns the function itself, not the `this` argument.
    /// type ReturnsSelf2 = (arg: string) => ReturnsSelf;
    /// ```
    ///
    /// ```ts
    /// interface Foo {
    ///   bar: string;
    /// }
    /// interface Bar extends Foo {
    ///   (): void;
    /// }
    /// ```
    ///
    /// ```ts
    /// // multiple call signatures (overloads) is allowed:
    /// interface Overloaded {
    ///   (data: string): number;
    ///   (id: number): string;
    /// }
    /// // this is equivalent to Overloaded interface.
    /// type Intersection = ((data: string) => number) & ((id: number) => string);
    ///```
    ///
    pub(crate) UseShorthandFunctionType {
        version: "1.5.0",
        name: "useShorthandFunctionType",
        source: RuleSource::EslintTypeScript("prefer-function-type"),
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseShorthandFunctionType {
    type Query = Ast<TsCallSignatureTypeMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();

        if let Some(ts_type_member_list) = query.parent::<TsTypeMemberList>() {
            // If there is more than one member, it's not a single call signature.
            if ts_type_member_list.len() > 1 {
                return None;
            }
            // If the parent is an interface with an extends clause, it's not a single call signature.
            if let Some(interface_decl) = ts_type_member_list.parent::<TsInterfaceDeclaration>() {
                if interface_decl.extends_clause().is_some() {
                    return None;
                }

                if let AnyTsReturnType::AnyTsType(TsThisType(_)) =
                    query.return_type_annotation()?.ty().ok()?
                {
                    return None;
                }
            }
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(rule_category!(), ctx.query().range(), markup! {
            "Use a function type instead of a call signature."
        }).note(markup! { "Types containing only a call signature can be shortened to a function type." }))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        // If there are comments, it's not a single call signature.
        if node.syntax().has_comments_direct() {
            return None;
        }

        let ts_type_member_list = node.parent::<TsTypeMemberList>()?;

        if let Some(interface_decl) = ts_type_member_list.parent::<TsInterfaceDeclaration>() {
            let type_alias_declaration = ts_type_alias_declaration(
                make::token(T![type]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                interface_decl.id().ok()?,
                make::token(T![=]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                AnyTsType::from(convert_ts_call_signature_type_member_to_function_type(
                    node,
                )?),
            )
            .build();

            mutation.replace_node(
                AnyJsDeclarationClause::from(interface_decl),
                AnyJsDeclarationClause::from(type_alias_declaration),
            );

            return Some(JsRuleAction {
                category: ActionCategory::QuickFix,
                applicability: Applicability::Always,
                message: markup! { "Alias a function type instead of using an interface with a call signature." }.to_owned(),
                mutation,
            });
        }

        if let Some(ts_object_type) = ts_type_member_list.parent::<TsObjectType>() {
            let new_function_type = convert_ts_call_signature_type_member_to_function_type(node)?;

            mutation.replace_node(
                AnyTsType::from(ts_object_type),
                AnyTsType::from(new_function_type),
            );

            return Some(JsRuleAction {
                category: ActionCategory::QuickFix,
                applicability: Applicability::Always,
                message: markup! { "Use a function type instead of an object type with a call signature." }.to_owned(),
                mutation,
            });
        }

        None
    }
}

fn convert_ts_call_signature_type_member_to_function_type(
    node: &TsCallSignatureTypeMember,
) -> Option<TsFunctionType> {
    let new_node = make::ts_function_type(
        make::js_parameters(
            make::token(T!['(']),
            node.parameters().ok()?.items(),
            make::token(T![')']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        ),
        make::token(T![=>]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        node.return_type_annotation()?.ty().ok()?,
    )
    .build();

    Some(new_node.with_type_parameters(node.type_parameters()))
}
