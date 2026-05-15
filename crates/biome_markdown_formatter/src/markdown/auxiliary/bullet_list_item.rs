use crate::markdown::lists::bullet_list::FormatMdBulletListOptions;
use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdBulletListItem, MdBulletListItemFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBulletListItem {
    inside_list: bool,
}
impl FormatNodeRule<MdBulletListItem> for FormatMdBulletListItem {
    fn fmt_fields(&self, node: &MdBulletListItem, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdBulletListItemFields { md_bullet_list } = node.as_fields();
        write!(
            f,
            [md_bullet_list
                .format()
                .with_options(FormatMdBulletListOptions {
                    inside_list: self.inside_list
                })]
        )
    }
}

pub(crate) struct FormatMdBulletListItemOptions {
    pub(crate) inside_list: bool,
}

impl FormatRuleWithOptions<MdBulletListItem> for FormatMdBulletListItem {
    type Options = FormatMdBulletListItemOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.inside_list = options.inside_list;
        self
    }
}
