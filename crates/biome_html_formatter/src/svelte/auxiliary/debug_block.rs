use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteDebugBlock, SvelteDebugBlockFields};
use biome_js_parser::syntax::ModuleNameSyntax;
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDebugBlock;
impl FormatNodeRule<SvelteDebugBlock> for FormatSvelteDebugBlock {
    fn fmt_fields(&self, node: &SvelteDebugBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteDebugBlockFields {
            sv_curly_at_token,
            debug_token,
            bindings,
            r_curly_token,
        } = node.as_fields();

        if !bindings.iter().all(|name| {
            name.and_then(|name| name.ident_token()).is_ok_and(|token| {
                ModuleNameSyntax::ReferenceIdentifier.accepts(token.text_trimmed())
            })
        }) {
            return format_html_source_node_preserving_line_endings(node.syntax())
                .without_comments()
                .fmt(f);
        }

        write!(f, [sv_curly_at_token.format(), debug_token.format(),])?;

        if !bindings.is_empty() {
            write!(f, [space()])?;
        }

        write!(f, [bindings.format()])?;

        write!(f, [r_curly_token.format()])
    }

    fn fmt_leading_comments(
        &self,
        _node: &SvelteDebugBlock,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // handled by element list formatter
        Ok(())
    }

    fn fmt_trailing_comments(
        &self,
        _node: &SvelteDebugBlock,
        _f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        // handled by element list formatter
        Ok(())
    }
}
