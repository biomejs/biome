use crate::prelude::*;
use biome_formatter::trivia::{FormatToken, format_skipped_token_trivia};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use biome_grit_syntax::{GritLanguage, GritSyntaxNode, GritSyntaxToken, map_syntax_node};
use biome_rowan::SyntaxToken;

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatGritSyntaxNode;

impl FormatRule<GritSyntaxNode> for FormatGritSyntaxNode {
    type Context = GritFormatContext;

    fn fmt(&self, node: &GritSyntaxNode, f: &mut GritFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<GritFormatContext> for GritSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatGritSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatGritSyntaxNode)
    }
}

impl IntoFormat<GritFormatContext> for GritSyntaxNode {
    type Format = FormatOwnedWithRule<Self, FormatGritSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatGritSyntaxNode)
    }
}

/// Format implementation specific to GritQL tokens.
#[derive(Debug, Default)]
pub(crate) struct FormatGritSyntaxToken;

impl FormatRule<GritSyntaxToken> for FormatGritSyntaxToken {
    type Context = GritFormatContext;

    fn fmt(&self, token: &GritSyntaxToken, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        f.state_mut().track_token(token);

        self.format_skipped_token_trivia(token, f)?;
        self.format_trimmed_token_trivia(token, f)?;

        Ok(())
    }
}

impl FormatToken<GritLanguage, GritFormatContext> for FormatGritSyntaxToken {
    fn format_skipped_token_trivia(
        &self,
        token: &SyntaxToken<GritLanguage>,
        f: &mut Formatter<GritFormatContext>,
    ) -> FormatResult<()> {
        format_skipped_token_trivia(token).fmt(f)
    }
}

impl AsFormat<GritFormatContext> for GritSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatGritSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatGritSyntaxToken)
    }
}

impl IntoFormat<GritFormatContext> for GritSyntaxToken {
    type Format = FormatOwnedWithRule<Self, FormatGritSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatGritSyntaxToken)
    }
}
