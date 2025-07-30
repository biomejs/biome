use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsDeclarationClause, AnyTsType, JsSyntaxKind, TsInterfaceDeclaration, TsTypeAliasDeclaration,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_consistent_type_definitions::{
    ConsistentTypeDefinition, UseConsistentTypeDefinitionsOptions,
};

declare_lint_rule! {
    /// Enforce type definitions to consistently use either `interface` or `type`.
    ///
    /// _TypeScript_ provides two different ways to define an object type: `interface` and `type`.
    ///
    /// This rule enforces consistent usage of either `interface` or `type` for object type definitions.
    /// Consistent type definition styles, aside from improving code readability, help minimize cognitive load when developers
    /// switch between different codebases or within a large codebase.
    ///
    /// ## Example
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// type Point = { x: number; y: number; };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// interface Point {
    ///   x: number;
    ///   y: number;
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// The following options are available
    ///
    /// ### `style`
    ///
    /// This option will determine which style to use for type definitions.
    ///
    /// Default: `interface`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "style": "interface"
    ///     }
    /// }
    /// ```
    ///
    pub UseConsistentTypeDefinitions {
        version: "next",
        name: "useConsistentTypeDefinitions",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("consistent-type-definitions").same()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Debug, Clone)]
pub enum DeclarationType {
    Interface(TsInterfaceDeclaration),
    TypeAlias(TsTypeAliasDeclaration),
}

impl Rule for UseConsistentTypeDefinitions {
    type Query = Ast<AnyJsDeclarationClause>;
    type State = DeclarationType;
    type Signals = Option<Self::State>;
    type Options = UseConsistentTypeDefinitionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let query = ctx.query();
        let options = ctx.options();

        match query {
            AnyJsDeclarationClause::TsInterfaceDeclaration(interface_decl) => {
                if options.style == ConsistentTypeDefinition::Type {
                    // Check if interface can be converted to type alias
                    if can_convert_interface_to_type(interface_decl) {
                        return Some(DeclarationType::Interface(interface_decl.clone()));
                    }
                }
                None
            }
            AnyJsDeclarationClause::TsTypeAliasDeclaration(type_alias) => {
                if options.style == ConsistentTypeDefinition::Interface {
                    // Check if type alias can be converted to interface
                    if can_convert_type_to_interface(type_alias) {
                        return Some(DeclarationType::TypeAlias(type_alias.clone()));
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let _options = ctx.options();

        let (range, current_style, _preferred_style) = match state {
            DeclarationType::Interface(interface_decl) => {
                (interface_decl.range(), "interface", "type")
            }
            DeclarationType::TypeAlias(type_alias) => (type_alias.range(), "type", "interface"),
        };

        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Use of the "<Emphasis>{current_style}</Emphasis>" detected."
            },
        )
        .note("The codebase should use a consistent coding style for the definition of types. This improves the readability and consistency.")
    )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        match state {
            DeclarationType::Interface(interface_decl) => {
                let type_alias = convert_interface_to_type_alias(interface_decl)?;
                mutation.replace_node(
                    AnyJsDeclarationClause::TsInterfaceDeclaration(interface_decl.clone()),
                    AnyJsDeclarationClause::TsTypeAliasDeclaration(type_alias),
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Use "<Emphasis>"type"</Emphasis>" alias." }.to_owned(),
                    mutation,
                ))
            }
            DeclarationType::TypeAlias(type_alias) => {
                let interface_decl = convert_type_alias_to_interface(type_alias)?;
                mutation.replace_node(
                    AnyJsDeclarationClause::TsTypeAliasDeclaration(type_alias.clone()),
                    AnyJsDeclarationClause::TsInterfaceDeclaration(interface_decl),
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Use "<Emphasis>"interface"</Emphasis>"." }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}

fn can_convert_interface_to_type(interface_decl: &TsInterfaceDeclaration) -> bool {
    // Interfaces with extends clause cannot be easily converted
    if interface_decl.extends_clause().is_some() {
        return false;
    }

    true
}

fn can_convert_type_to_interface(type_alias: &TsTypeAliasDeclaration) -> bool {
    // Check if the type alias has type parameters
    // Type aliases with complex types cannot be converted to interfaces

    matches!(type_alias.ty(), Ok(AnyTsType::TsObjectType(_)))
}

fn convert_interface_to_type_alias(
    interface_decl: &TsInterfaceDeclaration,
) -> Option<TsTypeAliasDeclaration> {
    let id = interface_decl.id().ok()?;
    let type_params = interface_decl.type_parameters();
    let members = interface_decl.members();

    let object_type = make::ts_object_type(
        make::token(JsSyntaxKind::L_CURLY),
        members,
        make::token(JsSyntaxKind::R_CURLY),
    );

    let mut type_alias_builder = make::ts_type_alias_declaration(
        make::token(JsSyntaxKind::TYPE_KW),
        id,
        make::token(JsSyntaxKind::EQ),
        AnyTsType::TsObjectType(object_type),
    );

    if let Some(type_params) = type_params {
        type_alias_builder = type_alias_builder.with_type_parameters(type_params);
    }

    Some(type_alias_builder.build())
}

fn convert_type_alias_to_interface(
    type_alias: &TsTypeAliasDeclaration,
) -> Option<TsInterfaceDeclaration> {
    let id = type_alias.binding_identifier().ok()?;
    let type_params = type_alias.type_parameters();
    let ty = type_alias.ty().ok()?;

    // Only convert if it's an object type
    if let AnyTsType::TsObjectType(object_type) = ty {
        let members = object_type.members();

        let mut interface_builder = make::ts_interface_declaration(
            make::token(JsSyntaxKind::INTERFACE_KW),
            id,
            make::token(JsSyntaxKind::L_CURLY),
            members,
            make::token(JsSyntaxKind::R_CURLY),
        );

        if let Some(type_params) = type_params {
            interface_builder = interface_builder.with_type_parameters(type_params);
        }

        Some(interface_builder.build())
    } else {
        None
    }
}
