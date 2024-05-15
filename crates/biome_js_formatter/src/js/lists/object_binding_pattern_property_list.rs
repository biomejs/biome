use crate::context::trailing_commas::FormatTrailingCommas;
use crate::prelude::*;
use biome_js_syntax::{AnyJsObjectBindingPatternMember, JsObjectBindingPatternPropertyList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectBindingPatternPropertyList;

impl FormatRule<JsObjectBindingPatternPropertyList> for FormatJsObjectBindingPatternPropertyList {
    type Context = JsFormatContext;

    fn fmt(
        &self,
        node: &JsObjectBindingPatternPropertyList,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        // The trailing separator is disallowed after a rest element
        let has_trailing_rest = match node.into_iter().last() {
            Some(elem) => matches!(
                elem?,
                AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(_)
            ),
            None => false,
        };

        let trailing_separator = if has_trailing_rest {
            TrailingSeparator::Disallowed
        } else {
            FormatTrailingCommas::ES5.trailing_separator(f.options())
        };

        let entries = node
            .format_separated(",")
            .with_trailing_separator(trailing_separator)
            .zip(node.iter());

        let mut join = f.join_nodes_with_soft_line();

        for (format_entry, node) in entries {
            join.entry(node?.syntax(), &format_entry);
        }

        join.finish()
    }
}
