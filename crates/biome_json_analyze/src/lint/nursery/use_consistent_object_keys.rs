use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize::json::unescape_json_string;
use biome_diagnostics::Severity;
use biome_json_factory::make::{json_member_name, json_string_literal};
use biome_json_syntax::JsonMemberName;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_consistent_object_keys::{
    NormalizationForm, UseConsistentObjectKeysOptions,
};
use unicode_normalization::{UnicodeNormalization, is_nfc, is_nfd, is_nfkc, is_nfkd};

use crate::JsonRuleAction;

declare_lint_rule! {
    /// Enforce JSON keys with consistent Unicode representation.
    ///
    /// Unicode characters can have different internal representations that look identical.
    /// For example, "é" can be stored as one code point (U+00E9) or as "e" plus a combining accent (U+0065 + U+0301).
    /// Unicode normalization converts text to a standard form (such as NFC) so visually identical keys share the same representation.
    /// This avoids confusing behavior in JSON objects where equality checks and key lookups should treat matching text consistently.
    ///
    /// More on Unicode normalization can be found [here](https://www.unicode.org/reports/tr15/).
    ///
    /// ## Examples
    ///
    /// `NFC`: Canonical Decomposition followed by Canonical Composition
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "form": "NFD"
    ///     }
    /// }
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// {
    ///     "caf\u0065\u0301": "espresso"
    /// }
    /// ```
    ///
    /// `NFD`: Canonical Decomposition
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "form": "NFD"
    ///     }
    /// }
    /// ```
    ///
    /// ```json,expect_diagnostic,use_options
    /// {
    ///     "\u00C5": "precomposed A-ring"
    /// }
    /// ```
    ///
    /// `NFKC`: Compatibility Decomposition followed by Canonical Composition
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "form": "NFKC"
    ///     }
    /// }
    /// ```
    ///
    /// ```json,expect_diagnostic,use_options
    /// {
    ///     "\u00BD": "circled digit one"
    /// }
    /// ```
    ///
    /// `NFKD`: Compatibility Decomposition
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "form": "NFKD"
    ///     }
    /// }
    /// ```
    ///
    /// ```json,expect_diagnostic,use_options
    /// {
    ///     "\u00BD": "vulgar fraction one half"
    /// }
    /// ```
    ///
    pub UseConsistentObjectKeys {
        version: "next",
        name: "useConsistentObjectKeys",
        language: "json",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintJson("no-unnormalized-keys").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseConsistentObjectKeys {
    type Query = Ast<JsonMemberName>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseConsistentObjectKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let form = ctx.options().form();

        let token_text = node.inner_string_text().ok()?;
        let unescaped = unescape_json_string(token_text);
        let text = unescaped.text();

        let is_normalized = match form {
            NormalizationForm::NFC => is_nfc(text),
            NormalizationForm::NFD => is_nfd(text),
            NormalizationForm::NFKC => is_nfkc(text),
            NormalizationForm::NFKD => is_nfkd(text),
        };

        if is_normalized { None } else { Some(()) }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected unnormalized key found."
                },
            )
            .note(markup! {
                "Unicode keys should be normalized to ensure consistent representation and reliable key comparison. Replace the object key with normalized Unicode."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsonRuleAction> {
        let node = ctx.query();
        let form = ctx.options().form();

        let token_text = node.inner_string_text().ok()?;
        let unescaped = unescape_json_string(token_text);
        let text = unescaped.text();

        let normalized = match form {
            NormalizationForm::NFC => text.nfc().collect::<String>(),
            NormalizationForm::NFD => text.nfd().collect::<String>(),
            NormalizationForm::NFKC => text.nfkc().collect::<String>(),
            NormalizationForm::NFKD => text.nfkd().collect::<String>(),
        };

        let mut mutation = ctx.root().begin();
        let new_node = json_member_name(json_string_literal(&normalized));

        mutation.replace_node(node.clone(), new_node);

        Some(JsonRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Replace with normalized value."
            },
            mutation,
        ))
    }
}
