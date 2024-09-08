use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind};
use biome_console::markup;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::AstNode;
use regex::Regex;

// List of sensitive patterns
const SENSITIVE_PATTERNS: &[&str] = &[
    r"xox[p|b|o|a]-[0-9]{12}-[0-9]{12}-[0-9]{12}-[a-z0-9]{32}",  // Slack Token
    r"-----BEGIN RSA PRIVATE KEY-----",  // RSA Private Key
    r"-----BEGIN OPENSSH PRIVATE KEY-----",  // SSH (OPENSSH) Private Key
    r"-----BEGIN DSA PRIVATE KEY-----",  // SSH (DSA) Private Key
    r"-----BEGIN EC PRIVATE KEY-----",  // SSH (EC) Private Key
    r"-----BEGIN PGP PRIVATE KEY BLOCK-----",  // PGP Private Key Block
    r#"[fF][aA][cC][eE][bB][oO][oO][kK].*['\"][0-9a-f]{32}['\"]"#,  // Facebook OAuth
    r#"[tT][wW][iI][tT][tT][eE][rR].*['\"][0-9a-zA-Z]{35,44}['\"]"#,  // Twitter OAuth
    r#"[gG][iI][tT][hH][uU][bB].*['\"][0-9a-zA-Z]{35,40}['\"]"#,  // GitHub
    r#""client_secret":"[a-zA-Z0-9-_]{24}""#,  // Google OAuth
    r"AKIA[0-9A-Z]{16}",  // AWS API Key
    r"[hH][eE][rR][oO][kK][uU].*[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}",  // Heroku API Key
    r#"[sS][eE][cC][rR][eE][tT].*['\"][0-9a-zA-Z]{32,45}['\"]"#,  // Generic Secret
    r#"[aA][pP][iI][_]?[kK][eE][yY].*['\"][0-9a-zA-Z]{32,45}['\"]"#,  // Generic API Key
    r"https://hooks\.slack\.com/services/T[a-zA-Z0-9_]{8}/B[a-zA-Z0-9_]{8}/[a-zA-Z0-9_]{24}",  // Slack Webhook
    r#""type": "service_account""#,  // Google (GCP) Service-account
    r"SK[a-z0-9]{32}",  // Twilio API Key
    r#"[a-zA-Z]{3,10}://[^/\s:@]{3,20}:[^/\s:@]{3,20}@.{1,100}['"\s]"#,  // Password in URL
];

// TODO: Try to get this to work in JavaScript comments as well
declare_lint_rule! {
    /// Disallow usage of sensitive data such as API keys and tokens.
    ///
    /// This rule checks for high-entropy strings and matches common patterns
    /// for secrets, such as AWS keys, Slack tokens, and private keys.
    ///
    /// Inspired by the ESLint [no-secrets/no-secrets](https://github.com/nickdeis/eslint-plugin-no-secrets) rule.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var secret = "AKIA1234567890EXAMPLE";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var nonSecret = "hello world";
    /// ```
    pub NoSecrets {
        version: "next",
        name: "noSecrets",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-secrets/no-secrets")],
        source_kind: RuleSourceKind::Inspired,
    }
}

impl Rule for NoSecrets {
    type Query = Ast<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if let Some(string_literal) = node.syntax().first_token() {
            let value = string_literal.text().to_string();

            for pattern in SENSITIVE_PATTERNS {
                let re = Regex::new(pattern).unwrap();
                if re.is_match(&value) {
                    return Some(());
                }
            }

            if is_high_entropy(&value) {
                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "Potential secret found." }
            ).note(markup! { "This looks like a sensitive value such as an API key or token." }) // TODO: Give a more detailed response on the *type* of API Key/token (based on the SENSITIVE PATTERNS)
        )
    }
}

fn is_high_entropy(text: &str) -> bool {
    let entropy = calculate_shannon_entropy(text);
    entropy > 4.5  // TODO: Make this optional, or controllable
}

// TODO: See if we can use an external crate to do this
fn calculate_shannon_entropy(data: &str) -> f64 {
    let mut freq = [0usize; 256];
    let mut len = 0usize;
    for &byte in data.as_bytes() {
        freq[byte as usize] += 1;
        len += 1;
    }

    let mut entropy = 0.0;
    for count in freq.iter() {
        if *count > 0 {
            let p = *count as f64 / len as f64;
            entropy -= p * p.log2();
        }
    }

    entropy
}
