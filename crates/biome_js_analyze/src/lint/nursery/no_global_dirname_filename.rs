use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    global_identifier, AnyJsExpression, AnyJsName, AnyJsObjectMember, JsFileSource,
    JsObjectExpression, JsPropertyObjectMember, JsStaticMemberExpression, JsSyntaxKind,
    JsSyntaxToken, JsVariableDeclarator,
};
use biome_rowan::{declare_node_union, AstSeparatedList, BatchMutationExt, TriviaPieceKind};

declare_lint_rule! {
    /// Disallow the use of `__dirname` and `__filename` in the global scope.
    ///
    /// They are [not available in ES modules](https://nodejs.org/api/esm.html#esm_no_filename_or_dirname).
    /// Starting with Node.js 20.11, `import.meta.dirname` and `import.meta.filename` have been introduced in ES modules, providing identical functionality to `__dirname` and `__filename` in CommonJS (CJS).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const dirname = __dirname;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const filename = __filename;
    /// ```
    ///
    /// ``` js,expect_diagnostic
    /// const foo = { __filename }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (__dirname.startsWith("/project/src/")) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const dirname = import.meta.dirname
    /// const filename = import.meta.filename
    /// const foo = {__filename: import.meta.filename };
    /// if (import.meta.dirname.startsWith("/project/src/")) {}
    /// ```
    ///
    pub NoGlobalDirnameFilename {
        version: "next",
        name: "noGlobalDirnameFilename",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-module")],
        source_kind: RuleSourceKind::Inspired,
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub AnyGlobalDirnameFileName =
        JsVariableDeclarator
        | JsObjectExpression
        | JsStaticMemberExpression
}

impl Rule for NoGlobalDirnameFilename {
    type Query = Semantic<AnyGlobalDirnameFileName>;
    type State = (JsSyntaxToken, String);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let file_source = ctx.source_type::<JsFileSource>();
        if file_source.is_script() {
            return None;
        };

        match node {
            // const dirname = __dirname;
            AnyGlobalDirnameFileName::JsVariableDeclarator(declarator) => {
                let init = declarator.initializer()?;
                let expr = init.expression().ok()?;
                validate_dirname_filename(&expr, model)
            }
            // `if (__dirname.startsWith("/project/src"))`
            AnyGlobalDirnameFileName::JsStaticMemberExpression(member_expr) => {
                let expr = member_expr.object().ok()?;
                let expr = expr.as_js_identifier_expression()?;
                let expr = AnyJsExpression::JsIdentifierExpression(expr.clone());
                validate_dirname_filename(&expr, model)
            }
            // const dirname = { __dirname };
            AnyGlobalDirnameFileName::JsObjectExpression(object_expr) => {
                for member in object_expr.members().iter().flatten() {
                    match member {
                        AnyJsObjectMember::JsPropertyObjectMember(member) => {
                            let expr = member.value().ok()?;
                            return validate_dirname_filename(&expr, model);
                        }
                        AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => {
                            let token = member.name().and_then(|name| name.value_token()).ok()?;
                            let text = maybe_text(&token)?;
                            return Some((token, text));
                        }
                        _ => continue,
                    }
                }
                None
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let syntax_token = &state.0;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                syntax_token.text_range(),
                markup! {
                    "Don't use "<Emphasis>""{syntax_token.text_trimmed()}""</Emphasis>"."
                },
            )
            .note(markup! {
                {syntax_token.text_trimmed()}" is not available in ES modules."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query();
        let syntax_token = &state.0;
        let dirname_or_filename = state.1.as_str();

        match node {
            AnyGlobalDirnameFileName::JsVariableDeclarator(declarator) => {
                mutation.replace_node(
                    declarator.initializer()?.expression().ok()?,
                    AnyJsExpression::JsStaticMemberExpression(make_import_meta(
                        dirname_or_filename,
                    )),
                );
            }
            AnyGlobalDirnameFileName::JsObjectExpression(object_expr) => {
                for member in object_expr.members().iter().flatten() {
                    match member {
                        AnyJsObjectMember::JsPropertyObjectMember(member) => {
                            let expr = member.value().ok()?;
                            let expr = expr.as_js_identifier_expression()?;
                            let id = expr.name().ok()?.value_token().ok()?;
                            if &id == syntax_token {
                                let key = member.name().ok()?;
                                let key = key.as_js_literal_member_name()?;
                                let property_member = make_property_object_member(
                                    &key.value().ok()?,
                                    dirname_or_filename,
                                );
                                mutation.replace_node(member.clone(), property_member);
                                break;
                            };
                        }
                        AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => {
                            let key = member.name().ok()?.value_token().ok()?;
                            if &key == syntax_token {
                                let property_member =
                                    make_property_object_member(&key, dirname_or_filename);
                                mutation.replace_node(
                                    AnyJsObjectMember::JsShorthandPropertyObjectMember(
                                        member.clone(),
                                    ),
                                    AnyJsObjectMember::JsPropertyObjectMember(property_member),
                                );
                                break;
                            };
                        }
                        _ => continue,
                    }
                }
            }
            AnyGlobalDirnameFileName::JsStaticMemberExpression(member_expr) => {
                mutation.replace_node(
                    member_expr.object().ok()?,
                    AnyJsExpression::JsStaticMemberExpression(make_import_meta(
                        dirname_or_filename,
                    )),
                );
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Replace "{syntax_token.text_trimmed()}" with "<Emphasis>{format!("import.meta.{}", dirname_or_filename)}</Emphasis>"."
            },
            mutation,
        ))
    }
}

fn validate_dirname_filename(
    expr: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<(JsSyntaxToken, String)> {
    let (reference, _name) = global_identifier(expr)?;
    let token = reference.value_token().ok()?;
    maybe_text(&token)
        .filter(|_| model.binding(&reference).is_none())
        .map(|name| (token, name))
}

fn maybe_text(token: &JsSyntaxToken) -> Option<String> {
    match token.text_trimmed() {
        "__dirname" => Some("dirname".to_string()),
        "__filename" => Some("filename".to_string()),
        _ => None,
    }
}

fn make_import_meta(dirname_or_filename: &str) -> JsStaticMemberExpression {
    make::js_static_member_expression(
        AnyJsExpression::from(make::js_import_meta_expression(
            make::token(JsSyntaxKind::IMPORT_KW),
            make::token(JsSyntaxKind::DOT),
            make::token(JsSyntaxKind::META_KW),
        )),
        make::token(JsSyntaxKind::DOT),
        AnyJsName::JsName(make::js_name(make::ident(dirname_or_filename))),
    )
}

fn make_property_object_member(
    key: &JsSyntaxToken,
    import_meta_property: &str,
) -> JsPropertyObjectMember {
    make::js_property_object_member(
        biome_js_syntax::AnyJsObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(
            make::ident(key.text_trimmed()),
        )),
        make::token(JsSyntaxKind::COLON).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        AnyJsExpression::JsStaticMemberExpression(make_import_meta(import_meta_property)),
    )
}
