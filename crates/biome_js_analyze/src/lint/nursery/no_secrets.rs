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
    /// ## Detected Secrets
    ///
    /// The following list contains the patterns we detect:
    ///
    /// - **JSON Web Token (JWT)**: Tokens in the format of `ey...`
    /// - **Base64-encoded JWT**: Base64-encoded JWT tokens with various parameters (alg, aud, iss, etc.)
    /// - **Slack Token**: Tokens such as `xox[baprs]-...`
    /// - **Slack Webhook URL**: URLs like `https://hooks.slack.com/services/...`
    /// - **GitHub Token**: GitHub tokens with lengths between 35-40 characters
    /// - **Twitter OAuth Token**: Twitter OAuth tokens with lengths between 35-44 characters
    /// - **Facebook OAuth Token**: Facebook OAuth tokens with possible lengths up to 42 characters
    /// - **Google OAuth Token**: Google OAuth tokens in the format `ya29...`
    /// - **AWS API Key**: Keys that begin with `AKIA` followed by 16 alphanumeric characters
    /// - **Passwords in URLs**: Passwords included in URL credentials (`protocol://user:pass@...`)
    /// - **Google Service Account**: JSON structure with the service-account identifier
    /// - **Twilio API Key**: API keys starting with `SK...` followed by 32 characters
    /// - **RSA Private Key**: Key blocks that start with `-----BEGIN RSA PRIVATE KEY-----`
    /// - **OpenSSH Private Key**: Key blocks that start with `-----BEGIN OPENSSH PRIVATE KEY-----`
    /// - **DSA Private Key**: Key blocks that start with `-----BEGIN DSA PRIVATE KEY-----`
    /// - **EC Private Key**: Key blocks that start with `-----BEGIN EC PRIVATE KEY-----`
    /// - **PGP Private Key Block**: Key blocks that start with `-----BEGIN PGP PRIVATE KEY BLOCK-----`
    ///
    /// ## Entropy Check
    ///
    /// In addition to detecting the above patterns, we also employ a **string entropy checker** to catch potential secrets based on their entropy (randomness). The entropy checker is configurable through the `Options`, allowing customization of thresholds for string entropy to fine-tune detection and minimize false positives.
    ///
    /// ## Disclaimer
    ///
    /// While this rule helps with most common cases, it is not intended to handle all of them.
    /// Therefore, always review your code carefully and consider implementing additional security
    /// measures, such as automated secret scanning in your CI/CD and git pipeline.
    ///
    /// ## Recommendations
    ///
    /// Some recommended tools for more comprehensive secret detection include:
    /// - [SonarQube](https://www.sonarsource.com/products/sonarqube/downloads/): Clean Code scanning solution with a secret scanner (Community version).
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
                "\n3. If this is a false positive, consider adding an inline disable comment, or tweak the entropy threshold. See options "<Hyperlink href="https://biomejs.dev/linter/rules/no-secrets/#options">"in our docs."</Hyperlink>
                "\nThis rule only catches basic vulnerabilities. For more robust, proper solutions, check out our recommendations at: "<Hyperlink href="https://biomejs.dev/linter/rules/no-secrets/#recommendations">"https://biomejs.dev/linter/rules/no-secrets/#recommendations"</Hyperlink>
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
    if is_known_safe_pattern(data) {
        return None;
    }

    let tokens = data
        .split([' ', '\t', '\n', '.', ',', ';', ':', '/', '-', '_', '@'])
        .filter(|s| !s.is_empty());

    for token in tokens {
        if token.len() >= MIN_PATTERN_LEN {
            if is_known_safe_pattern(token) {
                continue;
            }

            let entropy =
                calculate_entropy_with_case_and_classes(token, *entropy_threshold as f64, 15.0);
            if (entropy as u16) > *entropy_threshold {
                return Some("Detected high entropy string");
            }
        }
    }
    None
}

/*
Uses Shannon Entropy as a base algorithm, then adds "boosts" for special patterns/occurrences.
For example, Continuous mixed cases (lIkE tHiS) are more likely to contribute to a higher score than single cases.
Symbols also contribute highly to secrets.

TODO: This needs work. False positives/negatives are highlighted in valid.js and invalid.js.

References:
- ChatGPT chat: https://chatgpt.com/share/670370bf-3e18-8011-8454-f3bd01be0319
- Original paper for Shannon Entropy: https://ieeexplore.ieee.org/abstract/document/6773024/
*/
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

/*
    A simple mechanism to scale entropy as the string length increases, the reason being that
    large length strings are likely to be secrets.
    TODO: However, at some point there should definitely be a cutoff i.e. 100 characters, because it's
    probably base64 data or something similar at that point.
    This was taken from GPT, and I sadly couldn't find references for it.
*/
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
