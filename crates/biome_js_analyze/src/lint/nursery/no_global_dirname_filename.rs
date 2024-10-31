use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    global_identifier, AnyJsExpression, AnyJsName, AnyJsObjectMember, JsIdentifierExpression,
    JsObjectExpression, JsStaticMemberExpression, JsSyntaxKind, JsSyntaxToken,
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
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub AnyGlobalDirnameFileNameExpression =
        JsIdentifierExpression
        | JsObjectExpression
        | JsStaticMemberExpression
}

impl Rule for NoGlobalDirnameFilename {
    type Query = Semantic<AnyGlobalDirnameFileNameExpression>;
    type State = (JsSyntaxToken, String);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let mut signals = vec![];

        match node {
            // const dirname = __dirname;
            AnyGlobalDirnameFileNameExpression::JsIdentifierExpression(
                js_identifier_expression,
            ) => {
                let expr =
                    AnyJsExpression::JsIdentifierExpression(js_identifier_expression.clone());
                if let Some(state) = validate_dirname_filename(&expr, model) {
                    signals.push(state);
                }
            }
            // `if (__dirname.startsWith("/project/src"))`
            AnyGlobalDirnameFileNameExpression::JsStaticMemberExpression(
                js_static_member_expression,
            ) => {
                if let Ok(expr) = js_static_member_expression.object() {
                    if let Some(expr) = expr.as_js_identifier_expression() {
                        let expr = AnyJsExpression::JsIdentifierExpression(expr.clone());
                        if let Some(state) = validate_dirname_filename(&expr, model) {
                            signals.push(state);
                        }
                    }
                }
            }
            // const dirname = { __dirname };
            AnyGlobalDirnameFileNameExpression::JsObjectExpression(js_object_expression) => {
                for member in js_object_expression.members().iter().flatten() {
                    if let Some(member) = member.as_js_shorthand_property_object_member() {
                        if let Ok(token) = member.name().and_then(|name| name.value_token()) {
                            if let Some(text) = maybe_text(&token) {
                                signals.push((token, text));
                            }
                        }
                    };
                }
            }
        };
        signals
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
            AnyGlobalDirnameFileNameExpression::JsIdentifierExpression(
                js_identifier_expression,
            ) => {
                mutation.replace_node(
                    AnyJsExpression::JsIdentifierExpression(js_identifier_expression.clone()),
                    AnyJsExpression::JsStaticMemberExpression(make_import_meta(
                        dirname_or_filename,
                    )),
                );
            }
            AnyGlobalDirnameFileNameExpression::JsObjectExpression(js_object_expression) => {
                for member in js_object_expression.members().iter().flatten() {
                    if let Some(shorthand_property_member) =
                        member.as_js_shorthand_property_object_member()
                    {
                        let id = shorthand_property_member.name().ok()?.value_token().ok()?;
                        if &id == syntax_token {
                            let property_member = make::js_property_object_member(
                                biome_js_syntax::AnyJsObjectMemberName::JsLiteralMemberName(
                                    make::js_literal_member_name(make::ident(id.text_trimmed())),
                                ),
                                make::token(JsSyntaxKind::COLON)
                                    .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                                AnyJsExpression::JsStaticMemberExpression(make_import_meta(
                                    dirname_or_filename,
                                )),
                            );
                            mutation.replace_node(
                                AnyJsObjectMember::JsShorthandPropertyObjectMember(
                                    shorthand_property_member.clone(),
                                ),
                                AnyJsObjectMember::JsPropertyObjectMember(property_member),
                            );
                            break;
                        };
                    }
                }
            }
            AnyGlobalDirnameFileNameExpression::JsStaticMemberExpression(
                js_static_member_expression,
            ) => {
                mutation.replace_node(
                    js_static_member_expression.object().ok()?,
                    AnyJsExpression::JsStaticMemberExpression(make_import_meta(
                        dirname_or_filename,
                    )),
                );
            }
        }

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
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
            make::token(JsSyntaxKind::META),
        )),
        make::token(JsSyntaxKind::DOT),
        AnyJsName::JsName(make::js_name(make::ident(dirname_or_filename))),
    )
}
