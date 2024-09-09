use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;

use biome_js_syntax::JsStringLiteralExpression;

use biome_rowan::AstNode;
use regex::Regex;

use std::sync::{Arc, LazyLock, Mutex, Once};
use std::thread;

enum Pattern {
    Regex(&'static LazyLock<Regex>),
    Contains(&'static str),
}

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
        let text = Arc::new(token.text().to_string());

        // Skip processing if the string is less than the shortest possible length
        let min_pattern_len = get_min_pattern_len();
        if text.len() < min_pattern_len {
            return None;
        }

        let result = Arc::new(Mutex::new(None));
        let found = Arc::new(Mutex::new(false));
        let num_threads = 4;
        let patterns_per_thread = (SENSITIVE_PATTERNS.len() + num_threads - 1) / num_threads;

        let handles: Vec<_> = SENSITIVE_PATTERNS
            .chunks(patterns_per_thread)
            .filter(|chunk| {
                // Only spawn a thread if the string is long enough for any pattern in this chunk
                chunk.iter().any(|(_, _, min_len)| text.len() >= *min_len)
            })
            .map(|chunk| {
                let text = Arc::clone(&text);
                let result = Arc::clone(&result);
                let found = Arc::clone(&found);
                thread::spawn(move || {
                    for (pattern, comment, min_len) in chunk {
                        if *found.lock().unwrap() {
                            return;
                        }

                        // Skip this pattern if the string length is shorter than required
                        if text.len() < *min_len {
                            continue;
                        }

                        let matched = match pattern {
                            Pattern::Regex(re) => re.is_match(&text),
                            Pattern::Contains(substring) => text.contains(substring),
                        };

                        if matched {
                            let mut guard = result.lock().unwrap();
                            if guard.is_none() {
                                *guard = Some(*comment);
                                *found.lock().unwrap() = true;
                            }
                            return;
                        }
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let guard = result.lock().unwrap();
        if let Some(comment) = *guard {
            return Some(comment);
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
                "\n1. Remove the secret from your code."
                "\n2. If needed, use environment variables or a secure secret management system to store sensitive data."
                "\n3. If this is a false positive, consider adjusting the rule configuration or adding an inline disable comment."
            })
        )
    }
}

// Workaround: Since I couldn't figure out how to declare them inline,
// declare the LazyLock patterns separately
static SLACK_TOKEN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"xox[p|b|o|a]-[0-9]{12}-[0-9]{12}-[0-9]{12}-[a-z0-9]{32}").unwrap()
});

static GENERIC_SECRET_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"[sS][eE][cC][rR][eE][tT].*['\"][0-9a-zA-Z]{32,45}['\"]"#).unwrap()
});

static GENERIC_API_KEY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"[aA][pP][iI][_]?[kK][eE][yY].*['\"][0-9a-zA-Z]{32,45}['\"]"#).unwrap()
});

static SLACK_WEBHOOK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"https://hooks\.slack\.com/services/T[a-zA-Z0-9_]{8}/B[a-zA-Z0-9_]{8}/[a-zA-Z0-9_]{24}",
    )
    .unwrap()
});

static GITHUB_TOKEN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"[gG][iI][tT][hH][uU][bB].*['\"][0-9a-zA-Z]{35,40}['\"]"#).unwrap()
});

static TWITTER_OAUTH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"[tT][wW][iI][tT][tT][eE][rR].*['\"][0-9a-zA-Z]{35,44}['\"]"#).unwrap()
});

static FACEBOOK_OAUTH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"[fF][aA][cC][eE][bB][oO][oO][kK].*['\"][0-9a-f]{32}['\"]"#).unwrap()
});

static HEROKU_API_KEY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"[hH][eE][rR][oO][kK][uU].*[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}",
    )
    .unwrap()
});

static PASSWORD_IN_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"[a-zA-Z]{3,10}://[^/\s:@]{3,20}:[^/\s:@]{3,20}@.{1,100}['"\s]"#).unwrap()
});

static GOOGLE_SERVICE_ACCOUNT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""type": "service_account""#).unwrap());

static TWILIO_API_KEY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"SK[a-z0-9]{32}"#).unwrap());

static GOOGLE_OAUTH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""client_secret":"[a-zA-Z0-9-_]{24}""#).unwrap());

static AWS_API_KEY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"AKIA[0-9A-Z]{16}").unwrap());

// List of sensitive patterns, with comments
static SENSITIVE_PATTERNS: &[(Pattern, &str, usize)] = &[
    (Pattern::Regex(&SLACK_TOKEN_REGEX), "Slack Token", 32),
    (Pattern::Regex(&GENERIC_SECRET_REGEX), "Generic Secret", 32),
    (
        Pattern::Regex(&GENERIC_API_KEY_REGEX),
        "Generic API Key",
        32,
    ),
    (Pattern::Regex(&SLACK_WEBHOOK_REGEX), "Slack Webhook", 24),
    (Pattern::Regex(&GITHUB_TOKEN_REGEX), "GitHub", 35),
    (Pattern::Regex(&TWITTER_OAUTH_REGEX), "Twitter OAuth", 35),
    (Pattern::Regex(&FACEBOOK_OAUTH_REGEX), "Facebook OAuth", 32),
    (Pattern::Regex(&GOOGLE_OAUTH_REGEX), "Google OAuth", 24),
    (Pattern::Regex(&AWS_API_KEY_REGEX), "AWS API Key", 16),
    (Pattern::Regex(&HEROKU_API_KEY_REGEX), "Heroku API Key", 12),
    (
        Pattern::Regex(&PASSWORD_IN_URL_REGEX),
        "Password in URL",
        14,
    ),
    (
        Pattern::Regex(&GOOGLE_SERVICE_ACCOUNT_REGEX),
        "Google (GCP) Service-account",
        14,
    ),
    (Pattern::Regex(&TWILIO_API_KEY_REGEX), "Twilio API Key", 32),
    (
        Pattern::Contains("-----BEGIN RSA PRIVATE KEY-----"),
        "RSA Private Key",
        64,
    ),
    (
        Pattern::Contains("-----BEGIN OPENSSH PRIVATE KEY-----"),
        "SSH (OPENSSH) Private Key",
        64,
    ),
    (
        Pattern::Contains("-----BEGIN DSA PRIVATE KEY-----"),
        "SSH (DSA) Private Key",
        64,
    ),
    (
        Pattern::Contains("-----BEGIN EC PRIVATE KEY-----"),
        "SSH (EC) Private Key",
        64,
    ),
    (
        Pattern::Contains("-----BEGIN PGP PRIVATE KEY BLOCK-----"),
        "PGP Private Key Block",
        64,
    ),
];

static mut MIN_PATTERN_LEN: Option<usize> = None;
static INIT: Once = Once::new();

// TODO: Consider u8 instead of usize for a smaller footprint
fn get_min_pattern_len() -> usize {
    INIT.call_once(|| unsafe {
        MIN_PATTERN_LEN = Some(
            SENSITIVE_PATTERNS
                .iter()
                .map(|(_, _, len)| *len)
                .min()
                .unwrap_or(0),
        );
    });
    unsafe { MIN_PATTERN_LEN.unwrap_or(0) }
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
/// @param {*} str
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
