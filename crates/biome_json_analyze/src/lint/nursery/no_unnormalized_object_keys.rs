use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize::json::unescape_json_string;
use biome_diagnostics::Severity;
use biome_json_factory::make::{json_member_name, json_string_literal};
use biome_json_syntax::JsonMemberName;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_unnormalized_object_keys::{
    NoUnnormalizedObjectKeysOptions, NormalizationForm,
};
use unicode_normalization::{UnicodeNormalization, is_nfc, is_nfd, is_nfkc, is_nfkd};

use crate::JsonRuleAction;

declare_lint_rule! {
    /// Disallow JSON keys with inconsistent Unicode representation.
    ///
    /// Unicode characters can sometimes have multiple representations that look identical but are technically different character sequences.
    /// For example, the character "é" can be represented as a single code point (U+00E9) or as an "e" followed by a combining accent (U+0065 + U+0301).
    /// This can lead to confusion, comparison issues, and unexpected behavior when working with JSON data.
    /// Using normalized Unicode ensures consistent representation of text, which is important for key comparison, sorting, and searching operations.
    /// When keys are properly normalized, operations like key lookups and equality checks will work as expected across different systems and platforms.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic
    /// {
    ///   "caf\u0065\u0301": "espresso"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// {
    ///   "café": "espresso"
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `form`
    ///
    /// This option specifies which Unicode normalization form to use when checking keys
    ///
    /// Each normalization form has specific characteristics:
    /// - `NFC`: Canonical Decomposition followed by Canonical Composition (default)
    /// - `NFD`: Canonical Decomposition
    /// - `NFKC`: Compatibility Decomposition followed by Canonical Composition
    /// - `NFKD`: Compatibility Decomposition
    ///
    /// Default: `NFC`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "form": "NFKC"
    ///     }
    /// }
    /// ```
    ///
    pub NoUnnormalizedObjectKeys {
        version: "next",
        name: "noUnnormalizedObjectKeys",
        language: "json",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintJson("no-unnormalized-keys").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUnnormalizedObjectKeys {
    type Query = Ast<JsonMemberName>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUnnormalizedObjectKeysOptions;

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
