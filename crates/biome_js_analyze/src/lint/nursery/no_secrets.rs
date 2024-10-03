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
    /// ## Recommendations
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

        let has_spaces = text.contains(' ');

        for sensitive_pattern in SENSITIVE_PATTERNS.iter() {
            if text.len() < sensitive_pattern.min_len {
                continue;
            }

            if has_spaces && !sensitive_pattern.allows_spaces {
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

        if is_path(text) {
            return None;
        }

        let entropy_threshold = ctx
            .options()
            .entropy_threshold
            .unwrap_or(DEFAULT_HIGH_ENTROPY_THRESHOLD);
        detect_secret(text, &entropy_threshold)
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
                "\n3. If this is a false positive, consider adding an inline disable comment, or tweak the entropy threshold. Learn more: https://biomejs.dev/linter/rules/no-secrets/#options"
                "\nThis rule only catches very basic vulnerabilities. For more robust, proper solutions, we recommend heading over to https://biomejs.dev/linter/rules/no-secrets/#recommendations"
            })
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoSecretsOptions {
    /// Set entropy threshold (default is 41).
    entropy_threshold: Option<u16>,
}

fn is_path(text: &str) -> bool {
    // Check for common path indicators
    text.starts_with("./") || text.starts_with("../")
}

const DEFAULT_HIGH_ENTROPY_THRESHOLD: u16 = 41;

// Known sensitive patterns start here
static JWT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(ey[a-zA-Z0-9]{17,}\.ey[a-zA-Z0-9\/\\_-]{17,}\.(?:[a-zA-Z0-9\/\\_-]{10,}={0,2})?)(?:['|\"|\n|\r|\s|\x60|;]|$)"#).unwrap()
});

static JWT_BASE64_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\bZXlK(?:(?P<alg>aGJHY2lPaU)|(?P<apu>aGNIVWlPaU)|(?P<apv>aGNIWWlPaU)|(?P<aud>aGRXUWlPaU)|(?P<b64>aU5qUWlP)|(?P<crit>amNtbDBJanBi)|(?P<cty>amRIa2lPaU)|(?P<epk>bGNHc2lPbn)|(?P<enc>bGJtTWlPaU)|(?P<jku>cWEzVWlPaU)|(?P<jwk>cWQyc2lPb)|(?P<iss>cGMzTWlPaU)|(?P<iv>cGRpSTZJ)|(?P<kid>cmFXUWlP)|(?P<key_ops>clpYbGZiM0J6SWpwY)|(?P<kty>cmRIa2lPaUp)|(?P<nonce>dWIyNWpaU0k2)|(?P<p2c>d01tTWlP)|(?P<p2s>d01uTWlPaU)|(?P<ppt>d2NIUWlPaU)|(?P<sub>emRXSWlPaU)|(?P<svt>emRuUWlP)|(?P<tag>MFlXY2lPaU)|(?P<typ>MGVYQWlPaUp)|(?P<url>MWNtd2l)|(?P<use>MWMyVWlPaUp)|(?P<ver>MlpYSWlPaU)|(?P<version>MlpYSnphVzl1SWpv)|(?P<x>NElqb2)|(?P<x5c>NE5XTWlP)|(?P<x5t>NE5YUWlPaU)|(?P<x5ts256>NE5YUWpVekkxTmlJNkl)|(?P<x5u>NE5YVWlPaU)|(?P<zip>NmFYQWlPaU))[a-zA-Z0-9\/\\_+\-\r\n]{40,}={0,2}"#).unwrap()
});

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
        pattern: Pattern::Regex(&JWT_REGEX),
        comment: "JSON Web Token (JWT)",
        min_len: 100,
        allows_spaces: false,
    },
    SensitivePattern {
        pattern: Pattern::Regex(&JWT_BASE64_REGEX),
        comment: "Base64-encoded JWT",
        min_len: 100,
        allows_spaces: false,
    },
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
const MIN_PATTERN_LEN: usize = 14;

// Known safe patterns start here
static BASE64_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$").unwrap()
});
static URL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^https?://[a-zA-Z0-9.-]+(/[a-zA-Z0-9./_-]*)?$").unwrap());
static RELATIVE_PATH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:\.\./|\./|[a-zA-Z0-9_-]+)/?$").unwrap());
static UNIX_PATH_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(/[^/]+)+/?$").unwrap());
static WINDOWS_PATH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z]:\\(?:[^\\]+\\?)*$").unwrap());
static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());
static PHONE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap()); // E.164 format

// Combine all known safe patterns into a single list
static KNOWN_SAFE_PATTERNS: &[&LazyLock<Regex>] = &[
    &BASE64_REGEX,
    &URL_REGEX,
    &RELATIVE_PATH_REGEX,
    &UNIX_PATH_REGEX,
    &WINDOWS_PATH_REGEX,
    &EMAIL_REGEX,
    &PHONE_REGEX,
];

fn is_known_safe_pattern(data: &str) -> bool {
    for pattern in KNOWN_SAFE_PATTERNS {
        if pattern.is_match(data) {
            return true;
        }
    }
    false
}

fn detect_secret(data: &str, entropy_threshold: &u16) -> Option<&'static str> {
    let tokens: Vec<&str> = split_into_tokens(data);

    for token in tokens {
        println!("Token: {}, length: {}", token, token.len());

        if token.len() >= MIN_PATTERN_LEN {

            if is_known_safe_pattern(token) {
                continue;
            }

            let entropy =
                calculate_entropy_with_case_and_classes(token, *entropy_threshold as f64, 15.0);
            println!("Token: {}, entropy: {}", token, entropy);
            if (entropy as u16) > *entropy_threshold {
                return Some("Detected high entropy string");
            }
        }
    }
    None
}

fn calculate_entropy_with_case_and_classes(
    data: &str,
    base_threshold: f64,
    scaling_factor: f64,
) -> f64 {
    let mut freq = [0usize; 256];
    let len = data.len();

    for &byte in data.as_bytes() {
        freq[byte as usize] += 1;
    }

    let mut shannon_entropy = 0.0;
    let mut letter_count = 0;
    let mut uppercase_count = 0;
    let mut lowercase_count = 0;
    let mut digit_count = 0;
    let mut symbol_count = 0;
    let mut case_switches = 0;
    let mut previous_char_was_upper = false;

    for count in freq.iter() {
        if *count > 0 {
            let p = *count as f64 / len as f64;
            shannon_entropy -= p * p.log2();
        }
    }

    // Letter classification and case switching
    for (i, c) in data.chars().enumerate() {
        if c.is_ascii_alphabetic() {
            letter_count += 1;
            if c.is_uppercase() {
                uppercase_count += 1;
                if i > 0 && !previous_char_was_upper {
                    case_switches += 1;
                }
                previous_char_was_upper = true;
            } else {
                lowercase_count += 1;
                if i > 0 && previous_char_was_upper {
                    case_switches += 1;
                }
                previous_char_was_upper = false;
            }
        } else if c.is_ascii_digit() {
            digit_count += 1;
        } else if !c.is_whitespace() {
            symbol_count += 1;
        }
    }

    // Adjust entropy: case switches and symbol boosts
    let case_entropy_boost = if uppercase_count > 0 && lowercase_count > 0 {
        (case_switches as f64 / letter_count as f64) * 2.0
    } else {
        0.0
    };

    let symbol_entropy_boost = if symbol_count > 0 {
        symbol_count as f64 / len as f64
    } else {
        0.0
    };

    let digit_entropy_boost = if digit_count > 0 {
        digit_count as f64 / len as f64
    } else {
        0.0
    };

    let adjusted_entropy = shannon_entropy
        + (case_entropy_boost * 2.5)
        + (symbol_entropy_boost * 1.5)
        + digit_entropy_boost;

    // Apply exponential scaling to avoid excessive boosting for long, structured tokens
    apply_exponential_entropy_scaling(adjusted_entropy, len, base_threshold, scaling_factor)
}

fn apply_exponential_entropy_scaling(
    entropy: f64,
    token_length: usize,
    base_threshold: f64,
    scaling_factor: f64,
) -> f64 {
    // We will apply a logarithmic dampening to prevent excessive scaling for long tokens
    let scaling_adjustment = (token_length as f64 / scaling_factor).ln();
    base_threshold + entropy * scaling_adjustment
}

fn split_into_tokens(value: &str) -> Vec<&str> {
    let delimiters = [' ', '\t', '\n', '.', ',', ';', ':', '/', '-', '_', '@'];
    let mut tokens = vec![value];

    for &delimiter in delimiters.iter() {
        tokens = tokens
            .into_iter()
            .flat_map(|token| token.split(delimiter).filter(|&s| !s.is_empty()))
            .collect();
    }

    tokens
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
