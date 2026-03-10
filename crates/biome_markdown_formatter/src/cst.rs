use crate::{MdFormatContext, prelude::*};
use biome_formatter::trivia::{FormatToken, format_skipped_token_trivia};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use biome_markdown_syntax::{
    MarkdownLanguage, MarkdownSyntaxNode, MarkdownSyntaxToken, map_syntax_node,
};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatMdSyntaxToken;

impl FormatRule<MarkdownSyntaxToken> for FormatMdSyntaxToken {
    type Context = MdFormatContext;

    fn fmt(
        &self,
        token: &MarkdownSyntaxToken,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        f.state_mut().track_token(token);

        self.format_skipped_token_trivia(token, f)?;
        self.format_trimmed_token_trivia(token, f)
    }
}

impl FormatToken<MarkdownLanguage, MdFormatContext> for FormatMdSyntaxToken {
    fn format_skipped_token_trivia(
        &self,
        token: &MarkdownSyntaxToken,
        f: &mut Formatter<MdFormatContext>,
    ) -> FormatResult<()> {
        format_skipped_token_trivia(token).fmt(f)
    }
}

impl FormatRule<MarkdownSyntaxNode> for FormatMdSyntaxToken {
    type Context = MdFormatContext;

    fn fmt(&self, node: &MarkdownSyntaxNode, f: &mut MarkdownFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<MdFormatContext> for MarkdownSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatMdSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatMdSyntaxToken)
    }
}

impl IntoFormat<MdFormatContext> for MarkdownSyntaxNode {
    type Format = FormatOwnedWithRule<Self, FormatMdSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatMdSyntaxToken)
    }
}

impl AsFormat<MdFormatContext> for MarkdownSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatMdSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatMdSyntaxToken)
    }
}

impl IntoFormat<MdFormatContext> for MarkdownSyntaxToken {
    type Format = FormatOwnedWithRule<Self, FormatMdSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatMdSyntaxToken)
    }
}
