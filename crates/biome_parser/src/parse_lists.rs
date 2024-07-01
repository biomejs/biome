//! A set of traits useful to parse various types of lists

use crate::parse_recovery::RecoveryResult;
use crate::prelude::*;
use crate::ParserProgress;
use biome_rowan::SyntaxKind;

/// Use this trait to parse simple lists that don't have particular requirements.
///
/// ```rust,ignore
/// use biome_js_parser::{ParseSeparatedList};
///
/// struct MyList;
///
///
/// impl ParseNormalList for MyList {
///   // impl missing members
/// }
/// ```
pub trait ParseNodeList {
    type Kind: SyntaxKind;
    type Parser<'source>: Parser<Kind = Self::Kind>;

    /// The kind of the list node
    const LIST_KIND: Self::Kind;

    /// Parses a single element of the list
    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax;

    /// It creates a marker just before starting a list
    fn start_list(&mut self, p: &mut Self::Parser<'_>) -> Marker {
        p.start()
    }

    /// This method is used to check the current token inside the loop. When this method return [false],
    /// the trait will exit from the loop.
    ///
    /// Usually here you want to check the current token.
    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool;

    /// This method is used to recover the parser in case [Self::parse_element] returns [ParsedSyntax::Absent]
    fn recover(&mut self, p: &mut Self::Parser<'_>, parsed_element: ParsedSyntax)
        -> RecoveryResult;

    /// It creates a [ParsedSyntax] that will contain the list
    fn finish_list(&mut self, p: &mut Self::Parser<'_>, m: Marker) -> CompletedMarker {
        m.complete(p, Self::LIST_KIND)
    }

    /// Parses a simple list
    ///
    /// # Panics
    ///
    /// It panics if the parser doesn't advance at each cycle of the loop
    fn parse_list(&mut self, p: &mut Self::Parser<'_>) -> CompletedMarker {
        let elements = self.start_list(p);
        let mut progress = ParserProgress::default();

        while !p.at(<<Self::Parser<'_> as Parser>::Kind as SyntaxKind>::EOF)
            && !self.is_at_list_end(p)
        {
            progress.assert_progressing(p);

            let parsed_element = self.parse_element(p);

            if self.recover(p, parsed_element).is_err() {
                break;
            }
        }

        self.finish_list(p, elements)
    }
}

/// A trait to parse lists that will be separated by a recurring element
///
/// ```rust,ignore
/// use biome_js_parser::{ParseSeparatedList};
///
/// struct MyList;
///
/// impl ParseSeparatedList for MyList {
///   // impl missing members
/// }
/// ```
pub trait ParseSeparatedList {
    type Kind: SyntaxKind;
    type Parser<'source>: Parser<Kind = Self::Kind>;

    /// The kind of the list node
    const LIST_KIND: Self::Kind;

    /// Parses a single element of the list
    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax;

    /// It creates a marker just before starting a list
    fn start_list(&mut self, p: &mut Self::Parser<'_>) -> Marker {
        p.start()
    }

    /// This method is used to check the current token inside the loop. When this method return [false],
    /// the trait will exit from the loop.
    ///
    /// Usually here you want to check the current token.
    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool;

    /// This method is used to recover the parser in case [Self::parse_element] returns [ParsedSyntax::Absent]
    fn recover(&mut self, p: &mut Self::Parser<'_>, parsed_element: ParsedSyntax)
        -> RecoveryResult;

    /// It creates a [ParsedSyntax] that will contain the list
    /// Only called if the list isn't empty
    fn finish_list(&mut self, p: &mut Self::Parser<'_>, m: Marker) -> CompletedMarker {
        m.complete(p, Self::LIST_KIND)
    }

    /// `false` if the list must have at least one element
    fn allow_empty(&self) -> bool {
        true
    }

    /// The [SyntaxKind] of the element that separates the elements of the list
    fn separating_element_kind(&mut self) -> Self::Kind;

    /// `true` if the list allows for an optional trailing element
    fn allow_trailing_separating_element(&self) -> bool {
        false
    }

    /// Method called at each iteration of the loop and checks if the expected
    /// separator is present.
    ///
    /// If present, it [parses](Self::separating_element_kind) it and continues with loop.
    /// If not present, it adds a missing marker.
    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        p.expect(self.separating_element_kind())
    }

    /// Parses a list of elements separated by a recurring element
    ///
    /// # Panics
    ///
    /// It panics if the parser doesn't advance at each cycle of the loop
    fn parse_list(&mut self, p: &mut Self::Parser<'_>) -> CompletedMarker {
        let elements = self.start_list(p);
        let mut progress = ParserProgress::default();
        let mut first = true;
        while (!self.allow_empty() && first)
            || (!p.at(<Self::Parser<'_> as Parser>::Kind::EOF) && !self.is_at_list_end(p))
        {
            if first {
                first = false;
            } else {
                self.expect_separator(p);

                if self.allow_trailing_separating_element() && self.is_at_list_end(p) {
                    break;
                }
            }

            progress.assert_progressing(p);

            let parsed_element = self.parse_element(p);

            if parsed_element.is_absent() && p.at(self.separating_element_kind()) {
                // a missing element
                continue;
            }
            if self.recover(p, parsed_element).is_err() {
                break;
            }
        }
        self.finish_list(p, elements)
    }
}
