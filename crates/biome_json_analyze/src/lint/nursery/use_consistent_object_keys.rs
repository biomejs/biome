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
    /// See [Unicode Standard Annex #15](https://www.unicode.org/reports/tr15/) for the normalization
    /// standard.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// In JSON, `\u` followed by four hexadecimal digits represents a Unicode code point. The
    /// following key uses `\u0065` for `e` followed by `\u0301` for a separate combining accent:
    ///
    /// ```json,expect_diagnostic
    /// {
    ///     "caf\u0065\u0301": "espresso"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// The same visible key can use the single precomposed character `é`:
    ///
    /// ```json
    /// {
    ///     "caf\u00e9": "espresso"
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `form`
    ///
    /// Selects the Unicode normalization form. Defaults to `NFC`.
    ///
    /// - `NFC` combines equivalent character sequences where possible;
    /// - `NFD` separates characters into their base character and combining marks;
    /// - `NFKC` replaces compatibility characters with ordinary equivalents, then combines
    ///   sequences;
    /// - `NFKD` makes the same compatibility replacements but keeps sequences separated.
    ///
    /// For example, the compatibility forms rewrite the single character `½` as the three-character
    /// sequence `1⁄2`.
    ///
    /// The following configuration selects `NFC`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "form": "NFC"
    ///     }
    /// }
    /// ```
    ///
    /// ```json,expect_diagnostic,use_options
    /// {
    ///     "caf\u0065\u0301": "espresso"
    /// }
    /// ```
    ///
    /// The following configuration selects `NFD`:
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
    /// The following configuration selects `NFKC`:
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
    ///     "\u00BD": "vulgar fraction one half"
    /// }
    /// ```
    ///
    /// The following configuration selects `NFKD`:
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
        version: "2.5.14",
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
                    "This object key mixes Unicode characters that can be encoded in more than one way."
                },
            )
            .note(markup! {
                "Characters that look identical can have different byte representations, so such keys may fail to compare as equal."
            })
            .note(markup! {
                "Rewrite the key so equivalent characters use a single, consistent Unicode encoding."
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
                "Rewrite the key using a consistent Unicode encoding."
            },
            mutation,
        ))
    }
}
