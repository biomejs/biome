use crate::bullet_list::BulletListPrinter;
use crate::prelude::*;
use biome_formatter::FormatRuleWithOptions;
use biome_markdown_syntax::MdBulletList;
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBulletList {
    pub(crate) inside_list: bool,
}
impl FormatRule<MdBulletList> for FormatMdBulletList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdBulletList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let list_printer = BulletListPrinter::new(node, false);

        list_printer.fmt(f)
    }
}

pub(crate) struct FormatMdBulletListOptions {
    pub(crate) inside_list: bool,
}

impl FormatRuleWithOptions<MdBulletList> for FormatMdBulletList {
    type Options = FormatMdBulletListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.inside_list = options.inside_list;
        self
    }
}
