use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsDeclarationClause, AnyTsType, AnyTsTypeMember, JsSyntaxKind, TsInterfaceDeclaration,
    TsTypeAliasDeclaration,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPieceKind};
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
    ///         "style": "type"
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options,expect_diagnostic
    /// interface Point {
    ///   x: number;
    ///   y: number;
    /// }
    /// ```
    ///
    pub UseConsistentTypeDefinitions {
        version: "2.1.4",
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
    let l_curly = interface_decl.l_curly_token().ok()?;
    let r_curly = interface_decl.r_curly_token().ok()?;

    let object_type = make::ts_object_type(
        l_curly,
        members,
        make::token(JsSyntaxKind::R_CURLY)
            .with_leading_trivia_pieces(r_curly.leading_trivia().pieces()),
    );

    let mut type_alias_builder = make::ts_type_alias_declaration(
        make::token(JsSyntaxKind::TYPE_KW)
            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        id,
        make::token(JsSyntaxKind::EQ).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        AnyTsType::TsObjectType(object_type),
    )
    .with_semicolon_token(make::token(JsSyntaxKind::SEMICOLON));

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

        // Check if this is an inline type (all on one line)
        let object_text = object_type.syntax().to_string();
        let is_inline = !object_text.contains('\n');

        // For inline types, we need to add proper formatting
        let (members, l_curly_token, r_curly_token) = if is_inline && !members.is_empty() {
            // For inline types like { x: number; y: number; }, we need to:
            // 1. Add a newline after the opening brace
            // 2. Ensure each member is on its own line with indentation
            // 3. Add a newline before the closing brace

            let mut formatted_members = Vec::new();
            let mut is_first = true;

            for member in members {
                let mut updated_member = member.clone();

                // Get the first token of the member to add leading trivia
                if let Some(first_token) = member.syntax().first_token() {
                    let new_first_token = if is_first {
                        first_token.with_leading_trivia([
                            (TriviaPieceKind::Newline, "\n"),
                            (TriviaPieceKind::Whitespace, "    "),
                        ])
                    } else {
                        // For subsequent members, we need to ensure they start on a new line
                        let has_newline = first_token
                            .leading_trivia()
                            .pieces()
                            .any(|p| p.is_newline());
                        if has_newline {
                            first_token.clone()
                        } else {
                            first_token.with_leading_trivia([
                                (TriviaPieceKind::Newline, "\n"),
                                (TriviaPieceKind::Whitespace, "    "),
                            ])
                        }
                    };

                    // Replace the first token in the member
                    if let Some(new_syntax) = updated_member
                        .syntax()
                        .clone()
                        .replace_child(first_token.clone().into(), new_first_token.into())
                        && let Some(new_member) = AnyTsTypeMember::cast(new_syntax)
                    {
                        updated_member = new_member;
                    }
                }

                // For property signature members, clean up the separator token
                if let AnyTsTypeMember::TsPropertySignatureTypeMember(prop) = &updated_member
                    && let Some(sep_token) = prop.separator_token()
                {
                    // Remove any trailing whitespace from the separator (semicolon)
                    let clean_sep = sep_token.with_trailing_trivia([]);
                    if let Some(new_syntax) = updated_member
                        .syntax()
                        .clone()
                        .replace_child(sep_token.into(), clean_sep.into())
                        && let Some(new_member) = AnyTsTypeMember::cast(new_syntax)
                    {
                        updated_member = new_member;
                    }
                }

                formatted_members.push(updated_member);
                is_first = false;
            }

            let new_members = make::ts_type_member_list(formatted_members);

            (
                new_members,
                make::token(JsSyntaxKind::L_CURLY),
                make::token(JsSyntaxKind::R_CURLY)
                    .with_leading_trivia([(TriviaPieceKind::Newline, "\n")]),
            )
        } else {
            // For already multiline types, preserve the formatting
            let r_curly = object_type.r_curly_token().ok()?;
            let has_newline = r_curly
                .leading_trivia()
                .pieces()
                .any(|piece| piece.is_newline() || piece.text().contains('\n'));

            let r_curly_token = if has_newline {
                make::token(JsSyntaxKind::R_CURLY)
                    .with_leading_trivia_pieces(r_curly.leading_trivia().pieces())
            } else {
                make::token(JsSyntaxKind::R_CURLY)
                    .with_leading_trivia([(TriviaPieceKind::Newline, "\n")])
            };

            (members, make::token(JsSyntaxKind::L_CURLY), r_curly_token)
        };

        let mut interface_builder = make::ts_interface_declaration(
            make::token(JsSyntaxKind::INTERFACE_KW)
                .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            id,
            l_curly_token,
            members,
            r_curly_token,
        );

        if let Some(type_params) = type_params {
            interface_builder = interface_builder.with_type_parameters(type_params);
        }

        Some(interface_builder.build())
    } else {
        None
    }
}
