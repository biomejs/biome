use crate::{JsRuleAction, services::semantic::Semantic};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsName, AnyJsObjectMember, JsFileSource, JsIdentifierExpression,
    JsPropertyObjectMember, JsReferenceIdentifier, JsShorthandPropertyObjectMember,
    JsStaticMemberExpression, JsSyntaxKind, JsSyntaxToken,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::no_global_dirname_filename::NoGlobalDirnameFilenameOptions;

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
        version: "2.0.0",
        name: "noGlobalDirnameFilename",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-module").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoGlobalDirnameFilename {
    type Query = Semantic<JsReferenceIdentifier>;
    type State = (JsSyntaxToken, String);
    type Signals = Option<Self::State>;
    type Options = NoGlobalDirnameFilenameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let file_source = ctx.source_type::<JsFileSource>();
        if file_source.is_script() {
            return None;
        };

        validate_dirname_filename(node, model)
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

        if let Some(expr) = node.parent::<JsIdentifierExpression>() {
            mutation.replace_node::<AnyJsExpression>(
                expr.into(),
                make_import_meta(dirname_or_filename).into(),
            );
        } else if let Some(member) = node.parent::<JsShorthandPropertyObjectMember>() {
            mutation.replace_node::<AnyJsObjectMember>(
                member.into(),
                make_property_object_member(syntax_token, dirname_or_filename).into(),
            )
        } else {
            return None;
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
    expr: &JsReferenceIdentifier,
    model: &SemanticModel,
) -> Option<(JsSyntaxToken, String)> {
    match model.binding(expr) {
        // Some binding exists, not global one
        Some(_) => None,
        // No binding exists, global one
        None => {
            let token = expr.value_token().ok()?;
            let name = maybe_text(&token)?;
            Some((token, name))
        }
    }
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
