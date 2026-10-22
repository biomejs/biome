use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::{Applicability, Severity};
use biome_js_syntax::{JsModule, JsScript, JsSyntaxKind, JsSyntaxToken, TsDeclarationModule};
use biome_languages::{DocumentFileSource, JsFileSource};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, declare_node_union};
use biome_rule_options::use_consistent_unicode_bom::{UnicodeBom, UseConsistentUnicodeBomOptions};

declare_lint_rule! {
    /// Require or disallow a Unicode byte order mark at the beginning of a file.
    ///
    /// A byte order mark (BOM) is the invisible U+FEFF character at the start of a file.
    /// UTF-8 does not need a BOM, so this rule disallows it by default.
    /// Set `bom` to `"always"` if your tools require one.
    /// U+FEFF characters elsewhere in the file are not checked by this rule.
    /// Embedded JavaScript in HTML, Vue, Svelte, and Astro files is checked through the containing file.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// The following example starts with an invisible U+FEFF character.
    ///
    /// ```js,expect_diagnostic
    /// ﻿const value = 1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const value = 1;
    /// ```
    ///
    /// ## Options
    ///
    /// ### `bom`
    ///
    /// Type: `"always" | "never"`
    ///
    /// Default: `"never"`
    ///
    /// Whether files must start with a BOM.
    /// Use `"always"` to require a BOM, including in empty files.
    ///
    /// ```json,options
    /// { "options": { "bom": "always" } }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// const value = 1;
    /// ```
    ///
    /// #### Valid
    ///
    /// The following example starts with an invisible U+FEFF character.
    ///
    /// ```js,use_options
    /// ﻿const value = 1;
    /// ```
    ///
    /// ## See Also
    ///
    /// - [`noIrregularWhitespace`](https://biomejs.dev/linter/rules/no-irregular-whitespace/) checks irregular whitespace elsewhere in code.
    ///
    pub UseConsistentUnicodeBom {
        version: "next",
        name: "useConsistentUnicodeBom",
        language: "js",
        sources: &[RuleSource::Eslint("unicode-bom").same()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub AnyUnicodeBomRoot = JsModule | JsScript | TsDeclarationModule
}

impl Rule for UseConsistentUnicodeBom {
    type Query = Ast<AnyUnicodeBomRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseConsistentUnicodeBomOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let embedding = ctx.source_type::<JsFileSource>().as_embedding_kind();
        if matches!(
            DocumentFileSource::from_path(ctx.file_path(), true),
            DocumentFileSource::Html(_)
        ) || embedding.is_astro()
            || embedding.is_vue()
            || embedding.is_svelte_component()
        {
            return None;
        }

        let bom = ctx.query().bom_token();
        match (ctx.options().bom(), bom.is_some()) {
            (UnicodeBom::Always, false) | (UnicodeBom::Never, true) => Some(()),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(match ctx.query().bom_token() {
            Some(token) => RuleDiagnostic::new(
                rule_category!(),
                token.text_trimmed_range(),
                markup! { "This file starts with a Unicode byte order mark (BOM)." },
            )
            .note(markup! { "UTF-8 does not require a byte order mark." }),
            None => RuleDiagnostic::new(
                rule_category!(),
                TextRange::empty(ctx.query().syntax().text_range_with_trivia().start()),
                markup! { "This file is missing a Unicode byte order mark (BOM)." },
            )
            .note(markup! { "The configured BOM policy requires files to start with U+FEFF." }),
        })
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let message = match ctx.query().bom_token() {
            Some(token) => {
                let next_token = token.next_token()?;
                let new_next_token = next_token
                    .prepend_trivia_pieces(token.trailing_trivia().pieces())
                    .prepend_trivia_pieces(token.leading_trivia().pieces());
                mutation.remove_token(token.clone());
                mutation.replace_token_discard_trivia(next_token, new_next_token);
                markup! { "Remove the Unicode byte order mark." }.to_owned()
            }
            None => {
                let root = ctx.query().syntax();
                let token =
                    JsSyntaxToken::new_detached(JsSyntaxKind::UNICODE_BOM, "\u{feff}", [], []);
                // All queried roots reserve their first slot for the BOM token.
                let new_root = root
                    .clone()
                    .splice_slots(0..=0, std::iter::once(Some(token.into())));
                mutation.replace_element_discard_trivia(root.clone().into(), new_root.into());
                markup! { "Insert a Unicode byte order mark." }.to_owned()
            }
        };
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            message,
            mutation,
        ))
    }
}

impl AnyUnicodeBomRoot {
    fn bom_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsModule(root) => root.bom_token(),
            Self::JsScript(root) => root.bom_token(),
            Self::TsDeclarationModule(root) => root.bom_token(),
        }
    }
}
