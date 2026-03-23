use super::generated::LICENSE_LIST;

/// Configuration for license trust checking.
#[derive(Debug, Default)]
pub struct TrustConfig<'a> {
    /// Additional license identifiers to trust beyond valid SPDX identifiers.
    pub allow: &'a [Box<str>],
    /// License identifiers to explicitly deny.
    pub deny: &'a [Box<str>],
    /// Require licenses to be OSI-approved.
    pub require_osi_approved: bool,
    /// Require licenses to be FSF libre.
    pub require_fsf_libre: bool,
    /// When `true`, deprecated SPDX licenses are accepted. When `false`
    /// (default), deprecated licenses are rejected.
    pub ignore_deprecated: bool,
}

/// Reason why a license expression was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectReason {
    /// The license is explicitly denied.
    Denied,
    /// The license is not a valid SPDX identifier and not in the allow list.
    Untrusted,
    /// The license is not OSI-approved.
    NotOsiApproved,
    /// The license is not FSF libre.
    NotFsfLibre,
    /// The license is a deprecated SPDX identifier.
    Deprecated,
}

/// A parsed SPDX license expression.
///
/// Supports simple identifiers (`MIT`), `OR` (disjunction), `AND` (conjunction),
/// parenthesized sub-expressions, and `WITH` exceptions (the exception is ignored
/// for trust checking — only the base license ID matters).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpdxExpression<'a> {
    /// A single SPDX license identifier, e.g. `MIT`.
    Id(&'a str),
    /// A disjunction: at least one branch must be trusted.
    Or(Box<Self>, Box<Self>),
    /// A conjunction: all branches must be trusted.
    And(Box<Self>, Box<Self>),
}

impl<'a> SpdxExpression<'a> {
    /// Parse an SPDX license expression string.
    ///
    /// Returns `None` if the input is empty or cannot be parsed.
    pub fn parse(input: &'a str) -> Option<Self> {
        let input = input.trim();
        if input.is_empty() {
            return None;
        }
        let tokens = tokenize(input);
        if tokens.is_empty() {
            return None;
        }
        let (expr, rest) = parse_or(&tokens)?;
        if rest.is_empty() { Some(expr) } else { None }
    }

    /// Check whether this expression is trusted given a full [`TrustConfig`].
    ///
    /// Returns `Ok(())` when trusted, or `Err(RejectReason)` with the first
    /// reason for rejection.
    ///
    /// For `OR(a, b)`: trusted if either branch is trusted.
    /// For `AND(a, b)`: trusted if both branches are trusted.
    pub fn check_trust(&self, config: &TrustConfig<'_>) -> Result<(), RejectReason> {
        match self {
            Self::Id(id) => {
                // Deny always wins
                let denied = config.deny.iter().any(|d| d.eq_ignore_ascii_case(id));
                if denied {
                    return Err(RejectReason::Denied);
                }

                // Check if explicitly allowed (bypasses SPDX, OSI, and FSF checks)
                let allowed = config.allow.iter().any(|a| a.eq_ignore_ascii_case(id));
                if allowed {
                    return Ok(());
                }

                // Must be a valid SPDX identifier
                let valid_spdx = LICENSE_LIST.is_valid(id);
                if !valid_spdx {
                    return Err(RejectReason::Untrusted);
                }

                // Deprecated check (reject when ignore_deprecated is false)
                if !config.ignore_deprecated && LICENSE_LIST.is_deprecated(id) {
                    return Err(RejectReason::Deprecated);
                }

                // OSI check
                if config.require_osi_approved && !LICENSE_LIST.is_osi_approved(id) {
                    return Err(RejectReason::NotOsiApproved);
                }

                // FSF check
                if config.require_fsf_libre && !LICENSE_LIST.is_fsf_libre(id) {
                    return Err(RejectReason::NotFsfLibre);
                }

                Ok(())
            }
            Self::Or(left, right) => {
                let left_result = left.check_trust(config);
                let right_result = right.check_trust(config);
                match (&left_result, &right_result) {
                    (Ok(()), _) | (_, Ok(())) => Ok(()),
                    // Both failed — return the left reason
                    (Err(_), Err(_)) => left_result,
                }
            }
            Self::And(left, right) => {
                left.check_trust(config)?;
                right.check_trust(config)
            }
        }
    }

    /// Collect all individual license IDs in the expression.
    pub fn license_ids(&self) -> Vec<&'a str> {
        let mut ids = Vec::new();
        self.collect_ids(&mut ids);
        ids
    }

    fn collect_ids(&self, ids: &mut Vec<&'a str>) {
        match self {
            Self::Id(id) => ids.push(id),
            Self::Or(left, right) | Self::And(left, right) => {
                left.collect_ids(ids);
                right.collect_ids(ids);
            }
        }
    }
}

/// Tokens produced by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token<'a> {
    Id(&'a str),
    Or,
    And,
    With,
    LParen,
    RParen,
}

/// Tokenize an SPDX expression string.
fn tokenize(input: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some(&(i, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        if c == '(' {
            tokens.push(Token::LParen);
            chars.next();
            continue;
        }
        if c == ')' {
            tokens.push(Token::RParen);
            chars.next();
            continue;
        }

        // Read an identifier (license ID or keyword)
        let start = i;
        while let Some(&(_, ch)) = chars.peek() {
            if ch.is_whitespace() || ch == '(' || ch == ')' {
                break;
            }
            chars.next();
        }
        let end = chars.peek().map_or(input.len(), |&(i, _)| i);
        let word = &input[start..end];

        match word {
            "OR" => tokens.push(Token::Or),
            "AND" => tokens.push(Token::And),
            "WITH" => tokens.push(Token::With),
            _ => tokens.push(Token::Id(word)),
        }
    }

    tokens
}

/// Parse an OR expression (lowest precedence).
fn parse_or<'a, 'b>(tokens: &'b [Token<'a>]) -> Option<(SpdxExpression<'a>, &'b [Token<'a>])> {
    let (mut left, mut rest) = parse_and(tokens)?;
    while let Some((Token::Or, remaining)) = rest.split_first() {
        let (right, new_rest) = parse_and(remaining)?;
        left = SpdxExpression::Or(Box::new(left), Box::new(right));
        rest = new_rest;
    }
    Some((left, rest))
}

/// Parse an AND expression (higher precedence than OR).
fn parse_and<'a, 'b>(tokens: &'b [Token<'a>]) -> Option<(SpdxExpression<'a>, &'b [Token<'a>])> {
    let (mut left, mut rest) = parse_primary(tokens)?;
    while let Some((Token::And, remaining)) = rest.split_first() {
        let (right, new_rest) = parse_primary(remaining)?;
        left = SpdxExpression::And(Box::new(left), Box::new(right));
        rest = new_rest;
    }
    Some((left, rest))
}

/// Parse a primary expression: a parenthesized expression, or a license ID
/// (optionally followed by `WITH <exception>`).
fn parse_primary<'a, 'b>(tokens: &'b [Token<'a>]) -> Option<(SpdxExpression<'a>, &'b [Token<'a>])> {
    let (first, rest) = tokens.split_first()?;
    match first {
        Token::LParen => {
            let (expr, rest) = parse_or(rest)?;
            let (closing, rest) = rest.split_first()?;
            if *closing != Token::RParen {
                return None;
            }
            Some((expr, rest))
        }
        Token::Id(id) => {
            // Handle `<id> WITH <exception>` — skip the exception identifier
            let rest = if let Some((Token::With, after_with)) = rest.split_first() {
                if let Some((Token::Id(_), after_exception)) = after_with.split_first() {
                    after_exception
                } else {
                    // WITH without an exception identifier — treat as end
                    rest
                }
            } else {
                rest
            };
            Some((SpdxExpression::Id(id), rest))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_id() {
        let expr = SpdxExpression::parse("MIT").unwrap();
        assert_eq!(expr, SpdxExpression::Id("MIT"));
    }

    #[test]
    fn parse_or_expression() {
        let expr = SpdxExpression::parse("MIT OR Apache-2.0").unwrap();
        assert_eq!(
            expr,
            SpdxExpression::Or(
                Box::new(SpdxExpression::Id("MIT")),
                Box::new(SpdxExpression::Id("Apache-2.0")),
            )
        );
    }

    #[test]
    fn parse_and_expression() {
        let expr = SpdxExpression::parse("MIT AND ISC").unwrap();
        assert_eq!(
            expr,
            SpdxExpression::And(
                Box::new(SpdxExpression::Id("MIT")),
                Box::new(SpdxExpression::Id("ISC")),
            )
        );
    }

    #[test]
    fn parse_with_exception() {
        let expr = SpdxExpression::parse("Apache-2.0 WITH LLVM-exception").unwrap();
        assert_eq!(expr, SpdxExpression::Id("Apache-2.0"));
    }

    #[test]
    fn parse_parenthesized() {
        let expr = SpdxExpression::parse("(MIT OR Apache-2.0) AND ISC").unwrap();
        assert_eq!(
            expr,
            SpdxExpression::And(
                Box::new(SpdxExpression::Or(
                    Box::new(SpdxExpression::Id("MIT")),
                    Box::new(SpdxExpression::Id("Apache-2.0")),
                )),
                Box::new(SpdxExpression::Id("ISC")),
            )
        );
    }

    #[test]
    fn parse_precedence_and_binds_tighter() {
        // "MIT OR ISC AND Apache-2.0" should parse as "MIT OR (ISC AND Apache-2.0)"
        let expr = SpdxExpression::parse("MIT OR ISC AND Apache-2.0").unwrap();
        assert_eq!(
            expr,
            SpdxExpression::Or(
                Box::new(SpdxExpression::Id("MIT")),
                Box::new(SpdxExpression::And(
                    Box::new(SpdxExpression::Id("ISC")),
                    Box::new(SpdxExpression::Id("Apache-2.0")),
                )),
            )
        );
    }

    #[test]
    fn parse_empty_returns_none() {
        assert!(SpdxExpression::parse("").is_none());
        assert!(SpdxExpression::parse("   ").is_none());
    }

    fn boxed(strings: &[&str]) -> Vec<Box<str>> {
        strings.iter().map(|s| Box::from(*s)).collect()
    }

    fn config<'a>(allow: &'a [Box<str>], deny: &'a [Box<str>]) -> TrustConfig<'a> {
        TrustConfig {
            allow,
            deny,
            ignore_deprecated: true,
            ..TrustConfig::default()
        }
    }

    #[test]
    fn trust_valid_spdx() {
        let expr = SpdxExpression::parse("MIT").unwrap();
        assert!(expr.check_trust(&config(&[], &[])).is_ok());
    }

    #[test]
    fn trust_invalid_spdx() {
        let expr = SpdxExpression::parse("foo").unwrap();
        assert_eq!(
            expr.check_trust(&config(&[], &[])),
            Err(RejectReason::Untrusted)
        );
    }

    #[test]
    fn trust_allowed_non_spdx() {
        let expr = SpdxExpression::parse("foo").unwrap();
        let allow = boxed(&["foo"]);
        assert!(expr.check_trust(&config(&allow, &[])).is_ok());
    }

    #[test]
    fn trust_denied_spdx() {
        let expr = SpdxExpression::parse("MIT").unwrap();
        let deny = boxed(&["MIT"]);
        assert_eq!(
            expr.check_trust(&config(&[], &deny)),
            Err(RejectReason::Denied)
        );
    }

    #[test]
    fn trust_deny_overrides_allow() {
        let expr = SpdxExpression::parse("foo").unwrap();
        let allow = boxed(&["foo"]);
        let deny = boxed(&["foo"]);
        assert_eq!(
            expr.check_trust(&config(&allow, &deny)),
            Err(RejectReason::Denied)
        );
    }

    #[test]
    fn trust_case_insensitive() {
        let expr = SpdxExpression::parse("foo").unwrap();
        let allow = boxed(&["FOO"]);
        assert!(expr.check_trust(&config(&allow, &[])).is_ok());

        let expr = SpdxExpression::parse("MIT").unwrap();
        let deny = boxed(&["mit"]);
        assert_eq!(
            expr.check_trust(&config(&[], &deny)),
            Err(RejectReason::Denied)
        );
    }

    #[test]
    fn trust_or_one_trusted() {
        let expr = SpdxExpression::parse("MIT OR foo").unwrap();
        // MIT is valid SPDX, so the OR is trusted
        assert!(expr.check_trust(&config(&[], &[])).is_ok());
    }

    #[test]
    fn trust_or_none_trusted() {
        let expr = SpdxExpression::parse("foo OR bar").unwrap();
        assert!(expr.check_trust(&config(&[], &[])).is_err());
    }

    #[test]
    fn trust_and_all_trusted() {
        let expr = SpdxExpression::parse("MIT AND ISC").unwrap();
        assert!(expr.check_trust(&config(&[], &[])).is_ok());
    }

    #[test]
    fn trust_and_one_untrusted() {
        let expr = SpdxExpression::parse("MIT AND foo").unwrap();
        assert_eq!(
            expr.check_trust(&config(&[], &[])),
            Err(RejectReason::Untrusted)
        );
    }

    #[test]
    fn trust_or_with_deny() {
        // "MIT OR GPL-3.0-only" with deny: ["GPL-3.0-only"]
        // MIT is trusted, so the OR passes
        let expr = SpdxExpression::parse("MIT OR GPL-3.0-only").unwrap();
        let deny = boxed(&["GPL-3.0-only"]);
        assert!(expr.check_trust(&config(&[], &deny)).is_ok());
    }

    #[test]
    fn trust_or_both_denied() {
        let expr = SpdxExpression::parse("MIT OR Apache-2.0").unwrap();
        let deny = boxed(&["MIT", "Apache-2.0"]);
        assert_eq!(
            expr.check_trust(&config(&[], &deny)),
            Err(RejectReason::Denied)
        );
    }

    #[test]
    fn collect_license_ids() {
        let expr = SpdxExpression::parse("(MIT OR Apache-2.0) AND ISC").unwrap();
        let mut ids = expr.license_ids();
        ids.sort_unstable();
        assert_eq!(ids, vec!["Apache-2.0", "ISC", "MIT"]);
    }

    // --- check_trust tests ---

    #[test]
    fn check_trust_require_osi_approved() {
        // MIT is OSI-approved
        let expr = SpdxExpression::parse("MIT").unwrap();
        let config = TrustConfig {
            require_osi_approved: true,
            ignore_deprecated: true,
            ..TrustConfig::default()
        };
        assert!(expr.check_trust(&config).is_ok());

        // Abstyles is valid SPDX but not OSI-approved
        let expr = SpdxExpression::parse("Abstyles").unwrap();
        assert_eq!(expr.check_trust(&config), Err(RejectReason::NotOsiApproved));
    }

    #[test]
    fn check_trust_require_fsf_libre() {
        // AFL-1.1 is FSF libre
        let expr = SpdxExpression::parse("AFL-1.1").unwrap();
        let config = TrustConfig {
            require_fsf_libre: true,
            ignore_deprecated: true,
            ..TrustConfig::default()
        };
        assert!(expr.check_trust(&config).is_ok());

        // 0BSD is valid SPDX but not FSF libre
        let expr = SpdxExpression::parse("0BSD").unwrap();
        assert_eq!(expr.check_trust(&config), Err(RejectReason::NotFsfLibre));
    }

    #[test]
    fn check_trust_deprecated_rejected_by_default() {
        // AGPL-1.0 is deprecated
        let expr = SpdxExpression::parse("AGPL-1.0").unwrap();
        let config = TrustConfig::default(); // ignore_deprecated = false
        assert_eq!(expr.check_trust(&config), Err(RejectReason::Deprecated));
    }

    #[test]
    fn check_trust_deprecated_accepted_when_ignored() {
        let expr = SpdxExpression::parse("AGPL-1.0").unwrap();
        let config = TrustConfig {
            ignore_deprecated: true,
            ..TrustConfig::default()
        };
        assert!(expr.check_trust(&config).is_ok());
    }

    #[test]
    fn check_trust_allow_bypasses_osi_and_fsf() {
        // Abstyles is not OSI or FSF, but explicitly allowed
        let expr = SpdxExpression::parse("Abstyles").unwrap();
        let allow = boxed(&["Abstyles"]);
        let config = TrustConfig {
            allow: &allow,
            require_osi_approved: true,
            require_fsf_libre: true,
            ..TrustConfig::default()
        };
        assert!(expr.check_trust(&config).is_ok());
    }

    #[test]
    fn check_trust_deny_overrides_everything() {
        let expr = SpdxExpression::parse("MIT").unwrap();
        let deny = boxed(&["MIT"]);
        let config = TrustConfig {
            deny: &deny,
            ..TrustConfig::default()
        };
        assert_eq!(expr.check_trust(&config), Err(RejectReason::Denied));
    }

    #[test]
    fn check_trust_or_one_branch_osi() {
        // MIT is OSI, Abstyles is not — OR should pass
        let expr = SpdxExpression::parse("MIT OR Abstyles").unwrap();
        let config = TrustConfig {
            require_osi_approved: true,
            ignore_deprecated: true,
            ..TrustConfig::default()
        };
        assert!(expr.check_trust(&config).is_ok());
    }

    #[test]
    fn check_trust_and_one_branch_not_osi() {
        // MIT is OSI, Abstyles is not — AND should fail
        let expr = SpdxExpression::parse("MIT AND Abstyles").unwrap();
        let config = TrustConfig {
            require_osi_approved: true,
            ignore_deprecated: true,
            ..TrustConfig::default()
        };
        assert_eq!(expr.check_trust(&config), Err(RejectReason::NotOsiApproved));
    }
}
