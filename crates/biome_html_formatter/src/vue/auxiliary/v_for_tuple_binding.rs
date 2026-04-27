use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueVForTupleBinding, VueVForTupleBindingFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForTupleBinding;
impl FormatNodeRule<VueVForTupleBinding> for FormatVueVForTupleBinding {
    fn fmt_fields(&self, node: &VueVForTupleBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueVForTupleBindingFields {
            l_paren_token,
            value,
            second,
            third,
            r_paren_token,
        } = node.as_fields();

        write!(f, [l_paren_token.format(), value.format()])?;

        if let Some(second) = second {
            write!(f, [second.format()])?;
        }

        if let Some(third) = third {
            write!(f, [third.format()])?;
        }

        write!(f, [r_paren_token.format()])
    }
}
