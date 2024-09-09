use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;

use biome_js_syntax::JsStringLiteralExpression;

use biome_rowan::AstNode;
use regex::Regex;

use std::sync::{LazyLock, Once};

// TODO: Try to get this to work in JavaScript comments as well
declare_lint_rule! {
    /// Disallow usage of sensitive data such as API keys and tokens.
    ///
    /// This rule checks for high-entropy strings and matches common patterns
    /// for secrets, such as AWS keys, Slack tokens, and private keys.
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
    type Query = Ast<JsStringLiteralExpression>;
    type State = &'static str;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let token = node.value_token().ok()?;
        let text = token.text().to_string();

        let min_pattern_len = get_min_pattern_len();
        if text.len() < min_pattern_len {
            return None;
        }

        for sensitive_pattern in SENSITIVE_PATTERNS.iter() {
            if text.len() < sensitive_pattern.min_len {
                continue;
            }

            let matched = match &sensitive_pattern.pattern {
                Pattern::Regex(re) => re.is_match(&text),
                Pattern::Contains(substring) => text.contains(substring),
            };

            if matched {
                return Some(sensitive_pattern.comment);
            }
        }

        if is_high_entropy(&text) {
            Some("The string has a high entropy value")
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "Potential secret found." },
            )
            .note(markup! { "Type of secret detected: " {state} })
            .note(markup! {
                "Storing secrets in source code is a security risk. Consider the following steps:"
                "\n1. Remove the secret from your code. If you've already committed it, consider removing the commit entirely from your git tree."
                "\n2. If needed, use environment variables or a secure secret management system to store sensitive data."
                "\n3. If this is a false positive, consider adding an inline disable comment."
            })
        )
    }
}

// Workaround: Since I couldn't figure out how to declare them inline,
// declare the LazyLock patterns separately
static SLACK_TOKEN_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"xox[baprs]-([0-9a-zA-Z]{10,48})?").unwrap());

static SLACK_WEBHOOK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"https://hooks\.slack\.com/services/T[a-zA-Z0-9_]{8}/B[a-zA-Z0-9_]{8}/[a-zA-Z0-9_]{24}",
    )
    .unwrap()
});

static GITHUB_TOKEN_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[gG][iI][tT][hH][uU][bB].*[0-9a-zA-Z]{35,40}"#).unwrap());

static TWITTER_OAUTH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[tT][wW][iI][tT][tT][eE][rR].*[0-9a-zA-Z]{35,44}"#).unwrap());

static FACEBOOK_OAUTH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[fF][aA][cC][eE][bB][oO][oO][kK].*(?:.{0,42})"#).unwrap());

static HEROKU_API_KEY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"[hH][eE][rR][oO][kK][uU].*[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}",
    )
    .unwrap()
});

static PASSWORD_IN_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"[a-zA-Z]{3,10}://[^/\s:@]{3,20}:[^/\s:@]{3,20}@.{1,100}['"\s]"#).unwrap()
});

static GOOGLE_SERVICE_ACCOUNT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:^|[,\s])"type"\s*:\s*(?:['"]service_account['"']|service_account)(?:[,\s]|$)"#)
        .unwrap()
});

static TWILIO_API_KEY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"SK[a-z0-9]{32}"#).unwrap());

static GOOGLE_OAUTH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"ya29\\.[0-9A-Za-z\\-_]+"#).unwrap());

static AWS_API_KEY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"AKIA[0-9A-Z]{16}").unwrap());

enum Pattern {
    Regex(&'static LazyLock<Regex>),
    Contains(&'static str),
}

struct SensitivePattern {
    pattern: Pattern,
    comment: &'static str,
    min_len: usize,
}

static SENSITIVE_PATTERNS: &[SensitivePattern] = &[
    SensitivePattern {
        pattern: Pattern::Regex(&SLACK_TOKEN_REGEX),
        comment: "Slack Token",
        min_len: 32,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&SLACK_WEBHOOK_REGEX),
        comment: "Slack Webhook",
        min_len: 24,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&GITHUB_TOKEN_REGEX),
        comment: "GitHub",
        min_len: 35,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&TWITTER_OAUTH_REGEX),
        comment: "Twitter OAuth",
        min_len: 35,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&FACEBOOK_OAUTH_REGEX),
        comment: "Facebook OAuth",
        min_len: 32,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&GOOGLE_OAUTH_REGEX),
        comment: "Google OAuth",
        min_len: 24,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&AWS_API_KEY_REGEX),
        comment: "AWS API Key",
        min_len: 16,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&HEROKU_API_KEY_REGEX),
        comment: "Heroku API Key",
        min_len: 12,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&PASSWORD_IN_URL_REGEX),
        comment: "Password in URL",
        min_len: 14,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&GOOGLE_SERVICE_ACCOUNT_REGEX),
        comment: "Google (GCP) Service-account",
        min_len: 14,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&TWILIO_API_KEY_REGEX),
        comment: "Twilio API Key",
        min_len: 32,
    },
    SensitivePattern {
        pattern: Pattern::Contains("-----BEGIN RSA PRIVATE KEY-----"),
        comment: "RSA Private Key",
        min_len: 64,
    },
    SensitivePattern {
        pattern: Pattern::Contains("-----BEGIN OPENSSH PRIVATE KEY-----"),
        comment: "SSH (OPENSSH) Private Key",
        min_len: 64,
    },
    SensitivePattern {
        pattern: Pattern::Contains("-----BEGIN DSA PRIVATE KEY-----"),
        comment: "SSH (DSA) Private Key",
        min_len: 64,
    },
    SensitivePattern {
        pattern: Pattern::Contains("-----BEGIN EC PRIVATE KEY-----"),
        comment: "SSH (EC) Private Key",
        min_len: 64,
    },
    SensitivePattern {
        pattern: Pattern::Contains("-----BEGIN PGP PRIVATE KEY BLOCK-----"),
        comment: "PGP Private Key Block",
        min_len: 64,
    },
];

static mut MIN_PATTERN_LEN: Option<usize> = None;
static INIT: Once = Once::new();

fn get_min_pattern_len() -> usize {
    unsafe {
        INIT.call_once(|| {
            MIN_PATTERN_LEN = Some(
                SENSITIVE_PATTERNS
                    .iter()
                    .map(|pattern| pattern.min_len)
                    .min()
                    .unwrap_or(0),
            );
        });
        MIN_PATTERN_LEN.unwrap_or(0)
    }
}

fn is_high_entropy(text: &str) -> bool {
    let entropy = calculate_shannon_entropy(text);
    entropy > 4.5 // TODO: Make this optional, or controllable
}

/// Inspired by https://github.com/nickdeis/eslint-plugin-no-secrets/blob/master/utils.js#L93
/// Adapted from https://docs.rs/entropy/latest/src/entropy/lib.rs.html#14-33
/// Calculates Shannon entropy to measure the randomness of data. High entropy values indicate potentially
/// secret or sensitive information, as such data is typically more random and less predictable than regular text.
/// Useful for detecting API keys, passwords, and other secrets within code or configuration files.
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