use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;

use biome_js_syntax::JsStringLiteralExpression;

use biome_rowan::AstNode;
use regex::Regex;

use std::sync::LazyLock;

use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

// TODO: Try to get this to work in JavaScript comments as well
declare_lint_rule! {
    /// Disallow usage of sensitive data such as API keys and tokens.
    ///
    /// This rule checks for high-entropy strings and matches common patterns
    /// for secrets, including AWS keys, Slack tokens, and private keys.
    /// It aims to help users identify immediate potential secret leaks in their codebase,
    /// especially for those who may not be aware of the risks associated with
    /// sensitive data exposure.
    ///
    /// While this rule is beneficial for catching the most egregious cases,
    /// it is not infallible and may yield false positives. Therefore, always
    /// review your code carefully and consider implementing additional security
    /// measures, such as automated secret scanning in your CI/CD and git pipeline.
    /// Some recommended tools for more comprehensive secret detection include:
    /// - [Gitleaks](https://github.com/gitleaks/gitleaks/): A mature secret scanning tool.
    /// - [Trufflehog](https://github.com/trufflesecurity/trufflehog): A tool for finding secrets in git history.
    /// - [Sensleak](https://github.com/crates-pro/sensleak-rs): A Rust-based solution for secret detection.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const secret = "AKIA1234567890EXAMPLE";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const nonSecret = "hello world";
    /// ```
    ///
    /// ## Disclaimer
    /// This rule is intended to catch obvious secret leaks, but for more robust detection
    /// across different languages and scenarios, we encourage users to explore the dedicated
    /// tools mentioned above.
    pub NoSecrets {
        version: "1.9.0",
        name: "noSecrets",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintNoSecrets("no-secrets")],
        source_kind: RuleSourceKind::Inspired,
    }
}

impl Rule for NoSecrets {
    type Query = Ast<JsStringLiteralExpression>;
    type State = &'static str;
    type Signals = Option<Self::State>;
    type Options = NoSecretsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let token = node.value_token().ok()?;
        let text = token.text();

        if text.len() < MIN_PATTERN_LEN {
            return None;
        }

        let hasSpaces = text.contains(' ');

        for sensitive_pattern in SENSITIVE_PATTERNS.iter() {
            if text.len() < sensitive_pattern.min_len {
                continue;
            }

            if hasSpaces && !sensitive_pattern.allows_spaces {
                continue;
            }

            let matched = match &sensitive_pattern.pattern {
                Pattern::Regex(re) => re.is_match(text),
                Pattern::Contains(substring) => text.contains(substring),
            };

            if matched {
                return Some(sensitive_pattern.comment);
            }
        }

        if !hasSpaces {
            return detect_secret(ctx, text)
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

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoSecretsOptions {
    /// Set entropy threshold (default is 4.5).
    entropy_threshold: f64, // @TODO: Doesn't work currently.
}

const DEFAULT_HIGH_ENTROPY_THRESHOLD: f64 = 4.5;

// Known sensitive patterns start here
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
    allows_spaces: bool,
}

static SENSITIVE_PATTERNS: &[SensitivePattern] = &[
    SensitivePattern {
        pattern: Pattern::Regex(&SLACK_TOKEN_REGEX),
        comment: "Slack Token",
        min_len: 32,
        allows_spaces: false,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&SLACK_WEBHOOK_REGEX),
        comment: "Slack Webhook",
        min_len: 24,
        allows_spaces: false,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&GITHUB_TOKEN_REGEX),
        comment: "GitHub",
        min_len: 35,
        allows_spaces: false,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&TWITTER_OAUTH_REGEX),
        comment: "Twitter OAuth",
        min_len: 35,
        allows_spaces: false,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&FACEBOOK_OAUTH_REGEX),
        comment: "Facebook OAuth",
        min_len: 32,
        allows_spaces: false,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&GOOGLE_OAUTH_REGEX),
        comment: "Google OAuth",
        min_len: 24,
        allows_spaces: false,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&AWS_API_KEY_REGEX),
        comment: "AWS API Key",
        min_len: 16,
        allows_spaces: false,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&HEROKU_API_KEY_REGEX),
        comment: "Heroku API Key",
        min_len: 12,
        allows_spaces: false,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&PASSWORD_IN_URL_REGEX),
        comment: "Password in URL",
        min_len: 14,
        allows_spaces: false,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&GOOGLE_SERVICE_ACCOUNT_REGEX),
        comment: "Google (GCP) Service-account",
        min_len: 14,
        allows_spaces: true,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&TWILIO_API_KEY_REGEX),
        comment: "Twilio API Key",
        min_len: 32,
        allows_spaces: false,
    },
    SensitivePattern {
        pattern: Pattern::Contains("-----BEGIN RSA PRIVATE KEY-----"),
        comment: "RSA Private Key",
        min_len: 64,
        allows_spaces: true,
    },
    SensitivePattern {
        pattern: Pattern::Contains("-----BEGIN OPENSSH PRIVATE KEY-----"),
        comment: "SSH (OPENSSH) Private Key",
        min_len: 64,
        allows_spaces: true,
    },
    SensitivePattern {
        pattern: Pattern::Contains("-----BEGIN DSA PRIVATE KEY-----"),
        comment: "SSH (DSA) Private Key",
        min_len: 64,
        allows_spaces: true,
    },
    SensitivePattern {
        pattern: Pattern::Contains("-----BEGIN EC PRIVATE KEY-----"),
        comment: "SSH (EC) Private Key",
        min_len: 64,
        allows_spaces: true,
    },
    SensitivePattern {
        pattern: Pattern::Contains("-----BEGIN PGP PRIVATE KEY BLOCK-----"),
        comment: "PGP Private Key Block",
        min_len: 64,
        allows_spaces: true,
    },
];

const MIN_PATTERN_LEN: usize = 12;

// Known safe patterns start here
static BASE64_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[A-Za-z0-9+/]{40,}={0,2}$").unwrap());
static URL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^https?://[a-zA-Z0-9.-]+(/[a-zA-Z0-9./_-]*)?$").unwrap());
static UNIX_PATH_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(/[^/\0]+)+/?$").unwrap());
static WINDOWS_PATH_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z]:\\(?:[^\\\0]+\\?)*$").unwrap());

// Since list is smaller, heuristics may not be needed as were in sensitive patterns.
static KNOWN_SAFE_PATTERNS: &[&LazyLock<Regex>] = &[
    &BASE64_REGEX,
    &URL_REGEX,
    &UNIX_PATH_REGEX,
    &WINDOWS_PATH_REGEX,
];


fn detect_secret(ctx: &RuleContext, data: &str) -> std::option::Option<&str> {
    if is_known_safe_pattern(data) {
        return None;
    }

    let entropy_threshold = ctx.options().entropy_threshold.unwrap_or(DEFAULT_HIGH_ENTROPY_THRESHOLD);
    let entropy = calculate_shannon_entropy(data);
    
    if entropy > entropy_threshold {
        Some(format!(
            "Detected high entropy string: {:.2} (Threshold: {:.2})",
            entropy, entropy_threshold
        ))
    } else {
        None
    }
}

fn is_known_safe_pattern(data: &str) -> bool {
    for pattern in KNOWN_SAFE_PATTERNS {
        if pattern.is_match(data) {
            return true;
        }
    }
    false
}

/// Inspired by https://github.com/nickdeis/eslint-plugin-no-secrets/blob/master/utils.js#L93
/// Adapted from https://docs.rs/entropy/latest/src/entropy/lib.rs.html#14-33
/// Calculates Shannon entropy to measure the randomness of data. High entropy values indicate potentially
/// secret or sensitive information, as such data is typically more random and less predictable than regular text.
/// Useful for detecting API keys, passwords, and other secrets within code or configuration files.
fn calculate_shannon_entropy(data: &str) -> f64 {
    let mut freq = [0usize; 256];
    let len = data.len();
    for &byte in data.as_bytes() {
        freq[byte as usize] += 1;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_pattern_len() {
        let actual_min_pattern_len = SENSITIVE_PATTERNS
            .iter()
            .map(|pattern| pattern.min_len)
            .min()
            .unwrap_or(0);

        let initialized_min_pattern_len = MIN_PATTERN_LEN;
        assert_eq!(initialized_min_pattern_len, actual_min_pattern_len, "The initialized MIN_PATTERN_LEN value is not correct. Please ensure it's the smallest possible number from the SENSITIVE_PATTERNS.");
    }
}
