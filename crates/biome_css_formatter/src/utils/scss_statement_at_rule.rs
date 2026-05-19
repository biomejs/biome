use crate::prelude::*;
use biome_css_syntax::CssSyntaxToken;
use biome_formatter::{Format, write};

/// Formats SCSS statement semicolons omitted before `}`.
///
/// Example: `@mixin x { @content }` prints `@content;`.
pub(crate) fn format_scss_statement_at_rule_semicolon(
    semicolon: Option<CssSyntaxToken>,
) -> FormatScssStatementAtRuleSemicolon {
    FormatScssStatementAtRuleSemicolon { semicolon }
}

pub(crate) struct FormatScssStatementAtRuleSemicolon {
    semicolon: Option<CssSyntaxToken>,
}

impl Format<CssFormatContext> for FormatScssStatementAtRuleSemicolon {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        if let Some(semicolon) = &self.semicolon {
            write!(f, [semicolon.format()])
        } else {
            write!(f, [token(";")])
        }
    }
}
