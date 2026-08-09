use crate::{
    FormatOwnedWithRule, FormatRefWithRule, FormatRule, FormatScopedOptions,
    FormatScopedOptionsExt as _, FormatWithRule, FormatWithScopedOptions, SyntaxToken,
};
use biome_rowan::Language;

/// Controls how a formatter rule transforms the case of source text.
///
/// Language formatters decide which syntax uses each policy and whether
/// [`Self::Auto`] should be reported as an unresolved decision in debug builds.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum TextCase {
    /// No explicit case policy was selected.
    ///
    /// Language rules define the release behavior and may report this as an
    /// unresolved decision in debug builds.
    #[default]
    Auto,
    /// Preserves the source text.
    Preserve,
    /// Converts the source text to lowercase.
    Lowercase,
}

/// Token formatting rule that supports an explicit text casing policy.
///
/// Implementing this trait exposes `with_text_case` on the borrowed and owned
/// formatted token wrappers.
pub trait FormatRuleWithTextCase<L>: FormatRule<SyntaxToken<L>>
where
    L: Language,
{
    /// Returns a rule using `case` to format token text.
    fn with_text_case(self, case: TextCase) -> Self;
}

impl<L, R> FormatRefWithRule<'_, SyntaxToken<L>, R>
where
    L: Language,
    R: FormatRuleWithTextCase<L>,
{
    /// Formats this syntax token with the selected text casing policy.
    #[inline]
    pub fn with_text_case(mut self, case: TextCase) -> Self {
        self.rule = self.rule.with_text_case(case);
        self
    }
}

impl<L, R> FormatOwnedWithRule<SyntaxToken<L>, R>
where
    L: Language,
    R: FormatRuleWithTextCase<L>,
{
    /// Formats this syntax token with the selected text casing policy.
    #[inline]
    pub fn with_text_case(mut self, case: TextCase) -> Self {
        self.rule = self.rule.with_text_case(case);
        self
    }
}

/// Adds an explicit text casing policy to an existing formatted item.
///
/// Language formatters implement [`FormatScopedOptions`] for [`TextCase`] to
/// define which part of an AST item consumes the policy.
pub trait FormatTextCaseExt<Context>: FormatWithRule<Context> + Sized {
    /// Formats this item with the selected text casing policy.
    fn with_text_case(self, case: TextCase) -> FormatWithScopedOptions<Self, TextCase>
    where
        TextCase: FormatScopedOptions<Context, Self::Item>,
    {
        self.with_scoped_options(case)
    }
}

impl<Formatted, Context> FormatTextCaseExt<Context> for Formatted where
    Formatted: FormatWithRule<Context>
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use crate::{
        FormatContext, FormatOwnedWithRule, FormatRefWithRule, FormatScopedOptions,
        SimpleFormatContext, SimpleFormatOptions, TransformSourceMap, format,
    };
    use biome_js_syntax::{JsSyntaxKind, JsSyntaxToken};

    #[derive(Debug, Clone, Copy, Default)]
    struct TokenCaseRule {
        case: TextCase,
    }

    impl FormatRule<JsSyntaxToken> for TokenCaseRule {
        type Context = SimpleFormatContext;

        fn fmt(&self, _item: &JsSyntaxToken, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
            let text = match self.case {
                TextCase::Auto => "auto",
                TextCase::Preserve => "preserve",
                TextCase::Lowercase => "lowercase",
            };

            token(text).fmt(f)
        }
    }

    impl FormatRuleWithTextCase<biome_js_syntax::JsLanguage> for TokenCaseRule {
        fn with_text_case(mut self, case: TextCase) -> Self {
            self.case = case;
            self
        }
    }

    #[test]
    fn syntax_token_wrappers_forward_case_to_the_rule() {
        let token = JsSyntaxToken::new_detached(JsSyntaxKind::IDENT, "VALUE", [], []);

        let reference = FormatRefWithRule::new(&token, TokenCaseRule::default())
            .with_text_case(TextCase::Lowercase);
        let formatted = format!(SimpleFormatContext::default(), [reference]).unwrap();
        assert_eq!(formatted.print().unwrap().as_code(), "lowercase");

        let owned = FormatOwnedWithRule::new(token, TokenCaseRule::default())
            .with_text_case(TextCase::Preserve);
        let formatted = format!(SimpleFormatContext::default(), [owned]).unwrap();
        assert_eq!(formatted.print().unwrap().as_code(), "preserve");
    }

    #[derive(Debug)]
    struct CaseContext {
        options: SimpleFormatOptions,
        case: TextCase,
    }

    impl Default for CaseContext {
        fn default() -> Self {
            Self {
                options: SimpleFormatOptions::default(),
                case: TextCase::Auto,
            }
        }
    }

    impl FormatContext for CaseContext {
        type Options = SimpleFormatOptions;

        fn options(&self) -> &Self::Options {
            &self.options
        }

        fn source_map(&self) -> Option<&TransformSourceMap> {
            None
        }
    }

    struct Identifier;

    #[derive(Debug, Clone, Copy)]
    struct IdentifierRule;

    impl FormatRule<Identifier> for IdentifierRule {
        type Context = CaseContext;

        fn fmt(&self, _item: &Identifier, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
            let text = match f.context().case {
                TextCase::Auto => "auto",
                TextCase::Preserve => "preserve",
                TextCase::Lowercase => "lowercase",
            };

            token(text).fmt(f)
        }
    }

    impl FormatScopedOptions<CaseContext, Identifier> for TextCase {
        type Restore = Self;

        fn enter(&self, _item: &Identifier, context: &mut CaseContext) -> Self::Restore {
            std::mem::replace(&mut context.case, *self)
        }

        fn exit(&self, restore: Self::Restore, context: &mut CaseContext) {
            context.case = restore;
        }
    }

    #[test]
    fn formatted_items_apply_text_case_through_scoped_options() {
        let identifier = Identifier;
        let formatted =
            FormatRefWithRule::new(&identifier, IdentifierRule).with_text_case(TextCase::Lowercase);

        let formatted = format!(CaseContext::default(), [formatted]).unwrap();

        assert_eq!(formatted.print().unwrap().as_code(), "lowercase");
        assert_eq!(formatted.context().case, TextCase::Auto);
    }
}
