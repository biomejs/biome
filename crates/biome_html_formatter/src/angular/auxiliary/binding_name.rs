use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{AngularBindingName, AngularBindingNameFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularBindingName;
impl FormatNodeRule<AngularBindingName> for FormatAngularBindingName {
    fn fmt_fields(&self, node: &AngularBindingName, f: &mut HtmlFormatter) -> FormatResult<()> {
        let AngularBindingNameFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
