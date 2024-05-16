use crate::js::bindings::parameters::ParameterLayout;
use crate::prelude::*;

use crate::context::trailing_commas::FormatTrailingCommas;
use biome_js_syntax::parameter_ext::{AnyJsParameterList, AnyParameter};
use biome_js_syntax::{AnyJsConstructorParameter, AnyJsParameter, JsParameterList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsParameterList;

impl FormatRule<JsParameterList> for FormatJsParameterList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsParameterList, f: &mut JsFormatter) -> FormatResult<()> {
        FormatJsAnyParameterList::with_layout(
            &AnyJsParameterList::from(node.clone()),
            ParameterLayout::Default,
        )
        .fmt(f)
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct FormatJsAnyParameterList<'a> {
    list: &'a AnyJsParameterList,
    layout: Option<ParameterLayout>,
}

impl<'a> FormatJsAnyParameterList<'a> {
    pub fn with_layout(list: &'a AnyJsParameterList, layout: ParameterLayout) -> Self {
        Self {
            list,
            layout: Some(layout),
        }
    }
}

impl Format<JsFormatContext> for FormatJsAnyParameterList<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self.layout {
            None | Some(ParameterLayout::Default | ParameterLayout::NoParameters) => {
                let has_trailing_rest = match self.list.last() {
                    Some(elem) => matches!(
                        elem?,
                        AnyParameter::AnyJsParameter(AnyJsParameter::JsRestParameter(_))
                            | AnyParameter::AnyJsConstructorParameter(
                                AnyJsConstructorParameter::JsRestParameter(_)
                            )
                    ),
                    None => false,
                };

                // If it's a rest parameter, the assumption is no more
                // parameters could be added afterward, so no separator is
                // added there either.
                let trailing_separator = if has_trailing_rest {
                    TrailingSeparator::Disallowed
                } else {
                    FormatTrailingCommas::All.trailing_separator(f.options())
                };

                let has_modifiers = self.list.iter().any(|node| {
                    matches!(
                        node,
                        Ok(AnyParameter::AnyJsConstructorParameter(
                            AnyJsConstructorParameter::TsPropertyParameter(_),
                        ))
                    )
                });
                let mut joiner = if has_modifiers {
                    f.join_nodes_with_hardline()
                } else {
                    f.join_nodes_with_soft_line()
                };
                join_parameter_list(&mut joiner, self.list, trailing_separator)?;
                joiner.finish()
            }
            Some(ParameterLayout::Hug) => {
                let mut join = f.join_with(space());

                match self.list {
                    AnyJsParameterList::JsParameterList(list) => join.entries(
                        list.format_separated(",")
                            .with_trailing_separator(TrailingSeparator::Omit),
                    ),
                    AnyJsParameterList::JsConstructorParameterList(list) => join.entries(
                        list.format_separated(",")
                            .with_trailing_separator(TrailingSeparator::Omit),
                    ),
                };

                join.finish()
            }
        }
    }
}

fn join_parameter_list<S>(
    joiner: &mut JoinNodesBuilder<'_, '_, S, JsFormatContext>,
    list: &AnyJsParameterList,
    trailing_separator: TrailingSeparator,
) -> FormatResult<()>
where
    S: Format<JsFormatContext>,
{
    match list {
        AnyJsParameterList::JsParameterList(list) => {
            let entries = list
                .format_separated(",")
                .with_trailing_separator(trailing_separator)
                .zip(list.iter());

            for (format_entry, node) in entries {
                joiner.entry(node?.syntax(), &format_entry);
            }
        }
        AnyJsParameterList::JsConstructorParameterList(list) => {
            let entries = list
                .format_separated(",")
                .with_trailing_separator(trailing_separator)
                .zip(list.iter());

            for (format_entry, node) in entries {
                joiner.entry(node?.syntax(), &format_entry);
            }
        }
    }

    Ok(())
}
