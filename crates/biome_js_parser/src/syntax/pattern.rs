//! Provides traits for parsing pattern like nodes
use crate::prelude::*;
use crate::syntax::expr::ExpressionContext;
use crate::ParsedSyntax::{Absent, Present};
use crate::{JsParser, ParseRecoveryTokenSet, ParsedSyntax};
use biome_js_syntax::JsSyntaxKind::{EOF, JS_ARRAY_HOLE};
use biome_js_syntax::{JsSyntaxKind, TextRange, T};
use biome_parser::ParserProgress;

use super::class::parse_initializer_clause;

/// Trait for parsing a pattern with an optional default of the form `pattern = default`
pub(crate) trait ParseWithDefaultPattern {
    /// The syntax kind of the node for a pattern with a default value
    fn pattern_with_default_kind() -> JsSyntaxKind;

    /// Creates a diagnostic for the case where the pattern is missing. For example, if the
    /// code only contains ` = default`
    fn expected_pattern_error(p: &JsParser, range: TextRange) -> ParseDiagnostic;

    /// Parses a pattern (without its default value)
    fn parse_pattern(&self, p: &mut JsParser) -> ParsedSyntax;

    /// Parses a pattern and wraps it in a pattern with default if a `=` token follows the pattern
    fn parse_pattern_with_optional_default(&self, p: &mut JsParser) -> ParsedSyntax {
        self.parse_pattern(p).and_then(|pattern| {
            let m = pattern.precede(p);
            // test_err js js_invalid_assignment
            // ([=[(p[=[(p%]>([=[(p[=[(

            // test js pattern_with_default_in_keyword
            // for ([a = "a" in {}] in []) {}
            parse_initializer_clause(p, ExpressionContext::default()).ok();
            Present(m.complete(p, Self::pattern_with_default_kind()))
        })
    }
}

/// Trait for parsing an array like pattern of the form `[a, b = "c", { }]`
pub(crate) trait ParseArrayPattern<P: ParseWithDefaultPattern> {
    /// The kind of a bogus pattern. Used in case the pattern contains elements that aren't valid patterns
    fn bogus_pattern_kind() -> JsSyntaxKind;
    /// The kind of the array like pattern (array assignment or array binding)
    fn array_pattern_kind() -> JsSyntaxKind;
    /// The kind of the rest pattern
    fn rest_pattern_kind() -> JsSyntaxKind;
    /// The kind of the list
    fn list_kind() -> JsSyntaxKind;
    ///  Creates a diagnostic saying that the parser expected an element at the position passed as an argument.
    fn expected_element_error(p: &JsParser, range: TextRange) -> ParseDiagnostic;
    /// Creates a pattern with default instance. Used to parse the array elements.
    fn pattern_with_default(&self) -> P;

    /// Tries to parse an array like pattern
    fn parse_array_pattern(&self, p: &mut JsParser) -> ParsedSyntax {
        if !p.at(T!['[']) {
            return Absent;
        }

        let m = p.start();

        p.bump(T!['[']);
        let elements = p.start();
        let mut progress = ParserProgress::default();

        {
            while !p.at(EOF) && !p.at(T![']']) {
                progress.assert_progressing(p);

                let recovery = ParseRecoveryTokenSet::new(
                    Self::bogus_pattern_kind(),
                    token_set!(EOF, T![,], T![']'], T![=], T![;], T![...], T![')']),
                )
                .enable_recovery_on_line_break();

                let element = self.parse_any_array_element(p, &recovery);

                if element
                    .or_recover_with_token_set(p, &recovery, Self::expected_element_error)
                    .is_err()
                {
                    // Failed to recover
                    break;
                }

                if !p.at(T![']']) {
                    p.expect(T![,]);
                }
            }
        }

        elements.complete(p, Self::list_kind());
        p.expect(T![']']);

        Present(m.complete(p, Self::array_pattern_kind()))
    }

    /// Parses a single array element
    fn parse_any_array_element(
        &self,
        p: &mut JsParser,
        recovery: &ParseRecoveryTokenSet<JsSyntaxKind>,
    ) -> ParsedSyntax {
        match p.cur() {
            T![,] => Present(p.start().complete(p, JS_ARRAY_HOLE)),
            T![...] => self
                .parse_rest_pattern(p)
                .map(|rest_pattern| validate_rest_pattern(p, rest_pattern, T![']'], recovery)),
            _ => self
                .pattern_with_default()
                .parse_pattern_with_optional_default(p),
        }
    }

    /// Parses a rest element
    fn parse_rest_pattern(&self, p: &mut JsParser) -> ParsedSyntax {
        if !p.at(T![...]) {
            return Absent;
        }

        let m = p.start();
        let rest_end = p.cur_range().end();
        p.bump(T![...]);

        let with_default = self.pattern_with_default();

        with_default.parse_pattern(p).or_add_diagnostic(p, |p, _| {
            P::expected_pattern_error(p, TextRange::new(rest_end, rest_end))
        });

        Present(m.complete(p, Self::rest_pattern_kind()))
    }
}

/// Trait for parsing an object pattern like node of the form `{ a, b: c}`
pub(crate) trait ParseObjectPattern {
    /// Kind used when recovering from invalid properties.
    fn bogus_pattern_kind() -> JsSyntaxKind;
    /// The kind of the pattern like node this trait parses
    fn object_pattern_kind() -> JsSyntaxKind;
    /// The kind of the property list
    fn list_kind() -> JsSyntaxKind;
    /// Creates a diagnostic saying that a property is expected at the passed in range that isn't present.
    fn expected_property_pattern_error(p: &JsParser, range: TextRange) -> ParseDiagnostic;

    /// Parses the object pattern like node
    fn parse_object_pattern(&self, p: &mut JsParser) -> ParsedSyntax {
        if !p.at(T!['{']) {
            return Absent;
        }

        let m = p.start();

        p.bump(T!['{']);
        let elements = p.start();
        let mut progress = ParserProgress::default();

        while !p.at(T!['}']) {
            progress.assert_progressing(p);

            if p.at(T![,]) {
                // missing element
                p.error(Self::expected_property_pattern_error(p, p.cur_range()));
                p.bump_any(); // bump ,
                continue;
            }
            let recovery_set = ParseRecoveryTokenSet::new(
                Self::bogus_pattern_kind(),
                token_set!(EOF, T![,], T!['}'], T![...], T![;], T![')'], T![=]),
            )
            .enable_recovery_on_line_break();

            let pattern = self.parse_any_property_pattern(p, &recovery_set);

            if pattern
                .or_recover_with_token_set(p, &recovery_set, Self::expected_property_pattern_error)
                .is_err()
            {
                break;
            }

            if !p.at(T!['}']) {
                p.expect(T![,]);
            }
        }

        elements.complete(p, Self::list_kind());
        p.expect(T!['}']);

        Present(m.complete(p, Self::object_pattern_kind()))
    }

    /// Parses a single property
    fn parse_any_property_pattern(
        &self,
        p: &mut JsParser,
        recovery: &ParseRecoveryTokenSet<JsSyntaxKind>,
    ) -> ParsedSyntax {
        if p.at(T![...]) {
            self.parse_rest_property_pattern(p)
                .map(|rest_pattern| validate_rest_pattern(p, rest_pattern, T!['}'], recovery))
        } else {
            self.parse_property_pattern(p)
        }
    }

    /// Parses a shorthand `{ a }` or a "named" `{ a: b }` property
    fn parse_property_pattern(&self, p: &mut JsParser) -> ParsedSyntax;

    /// Parses a rest property `{ ...a }`
    fn parse_rest_property_pattern(&self, p: &mut JsParser) -> ParsedSyntax;
}

/// Validates if the parsed completed rest marker is a valid rest element inside of a
/// array or object assignment target and converts it to an bogus assignment target if not.
/// A rest element must be:
///
/// * the last element
/// * not followed by a trailing comma
/// * not have a default value
fn validate_rest_pattern(
    p: &mut JsParser,
    mut rest: CompletedMarker,
    end_token: JsSyntaxKind,
    recovery: &ParseRecoveryTokenSet<JsSyntaxKind>,
) -> CompletedMarker {
    if p.at(end_token) {
        return rest;
    }

    if p.at(T![=]) {
        let kind = rest.kind(p);
        let rest_range = rest.range(p);
        let rest_marker = rest.undo_completion(p);
        let default_start = p.cur_range().start();
        p.bump(T![=]);

        if let Ok(recovered) = recovery.recover(p) {
            recovered.undo_completion(p).abandon(p); // append recovered content to parent
        }
        p.error(
            p.err_builder(
                "rest element cannot have a default",
                default_start..p.cur_range().start(),
            )
            .with_detail(
                default_start..p.cur_range().start(),
                "Remove the default value here",
            )
            .with_detail(rest_range, "Rest element"),
        );

        let mut invalid = rest_marker.complete(p, kind);
        invalid.change_to_bogus(p);
        invalid
    } else if p.at(T![,]) && p.nth_at(1, end_token) {
        p.error(
            p.err_builder("rest element may not have a trailing comma", p.cur_range())
                .with_detail(p.cur_range(), "Remove the trailing comma here")
                .with_detail(rest.range(p), "Rest element"),
        );
        rest.change_to_bogus(p);
        rest
    } else {
        p.error(
            p.err_builder("rest element must be the last element", rest.range(p),)
                .with_hint(
                    format!(
                    "Move the rest element to the end of the pattern, right before the closing '{}'",
                    end_token.to_string().unwrap(),
                    ),
                ),
        );
        rest.change_to_bogus(p);
        rest
    }
}
