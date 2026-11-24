use crate::{prelude::*, utils::formatters::FormatTokenAsLowercase};
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{HtmlDirective, HtmlDirectiveFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlDirective {
    comments_as_children: bool,
}

pub(crate) struct FormatHtmlDirectiveOptions {
    pub comments_as_children: bool,
}

impl FormatRuleWithOptions<HtmlDirective> for FormatHtmlDirective {
    type Options = FormatHtmlDirectiveOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.comments_as_children = options.comments_as_children;
        self
    }
}

impl FormatNodeRule<HtmlDirective> for FormatHtmlDirective {
    fn fmt_fields(&self, node: &HtmlDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlDirectiveFields {
            l_angle_token,
            excl_token,
            doctype_token,
            name_token,
            html_token,
            quirk_token,
            public_id_token,
            system_id_token,
            r_angle_token,
        } = node.as_fields();

        // The HTML5 doctype is spelled in lowercase, but only in a plain HTML
        // file: an `.astro` or `.vue` file keeps whatever the author wrote,
        // and so does any doctype that names a DTD rather than standing alone.
        let is_bare_html5_doctype = html_token.is_some()
            && quirk_token.is_none()
            && public_id_token.is_none()
            && system_id_token.is_none();
        let lowercase_doctype = is_bare_html5_doctype && f.options().file_source().is_html();

        write!(f, [l_angle_token.format(), excl_token.format()])?;
        if lowercase_doctype {
            write!(f, [FormatTokenAsLowercase::from(doctype_token?)])?;
        } else {
            write!(f, [doctype_token.format()])?;
        }
        if let Some(name) = name_token {
            write!(f, [space(), name.format()])?;
        }
        if let Some(html) = html_token {
            write!(f, [space(), FormatTokenAsLowercase::from(html)])?;
        }
        if let Some(quirk) = quirk_token {
            write!(f, [space()])?;
            quirk.format().fmt(f)?;
        }
        if let Some(public_id) = public_id_token {
            write!(f, [space()])?;
            public_id.format().fmt(f)?;
        }
        if let Some(system_id) = system_id_token {
            write!(f, [space()])?;
            system_id.format().fmt(f)?;
        }
        write!(f, [r_angle_token.format(), hard_line_break()])?;
        Ok(())
    }

    fn fmt_leading_comments(
        &self,
        node: &HtmlDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        if self.comments_as_children {
            Ok(())
        } else {
            format_html_leading_comments_for_block(node.syntax()).fmt(f)
        }
    }

    fn fmt_trailing_comments(
        &self,
        node: &HtmlDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        if self.comments_as_children {
            Ok(())
        } else {
            format_html_trailing_comments(node.syntax()).fmt(f)
        }
    }
}
