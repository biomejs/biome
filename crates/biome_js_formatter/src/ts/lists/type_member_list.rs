use crate::context::Semicolons;
use crate::context::{ConsistentQuoteProperties, QuoteProperties};
use crate::prelude::*;
use crate::utils::object::type_member_list_requires_quotes;
use biome_formatter::{Buffer, write};
use biome_js_syntax::{AnyTsTypeMember, TsTypeMemberList};

use biome_rowan::AstNodeList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeMemberList;

impl FormatRule<TsTypeMemberList> for FormatTsTypeMemberList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsTypeMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let needs_quotes = f.options().quote_properties() == QuoteProperties::Consistent
            && type_member_list_requires_quotes(node, f.options().source_type().into());

        let previous = f.context().consistent_quote_properties();
        f.context_mut()
            .set_consistent_quote_properties(Some(ConsistentQuoteProperties::new(
                node.syntax().clone(),
                needs_quotes,
            )));

        let items = node.iter();
        let last_index = items.len().saturating_sub(1);

        let mut joiner = f.join_nodes_with_soft_line();

        for (index, member) in items.enumerate() {
            joiner.entry(
                member.syntax(),
                &TsTypeMemberItem {
                    last: index == last_index,
                    member: &member,
                },
            )
        }

        let result = joiner.finish();

        f.context_mut().set_consistent_quote_properties(previous);

        result
    }
}

struct TsTypeMemberItem<'a> {
    last: bool,
    member: &'a AnyTsTypeMember,
}

impl Format<JsFormatContext> for TsTypeMemberItem<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let mut is_verbatim = false;

        write!(
            f,
            [group(&format_once(|f| {
                let mut recording = f.start_recording();
                write!(recording, [self.member.format()])?;

                is_verbatim = recording.stop().end_tag(TagKind::Verbatim).is_some();

                Ok(())
            }))]
        )?;

        if !is_verbatim {
            // Children don't format the separator on purpose, so it's up to the parent - this node,
            // to decide to print their separator
            match f.options().semicolons() {
                Semicolons::Always => {
                    if self.last {
                        write!(f, [if_group_breaks(&token(";"))])?;
                    } else {
                        token(";").fmt(f)?;
                    }
                }
                Semicolons::AsNeeded => {
                    if !self.last {
                        write!(f, [if_group_fits_on_line(&token(";"))])?;
                    }
                }
            }
        }

        Ok(())
    }
}
