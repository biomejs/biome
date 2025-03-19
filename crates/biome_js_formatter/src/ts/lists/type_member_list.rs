use crate::prelude::*;
use biome_formatter::{Buffer, write};
use biome_js_syntax::{AnyTsTypeMember, TsTypeMemberList};

use crate::context::Semicolons;
use biome_rowan::AstNodeList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeMemberList;

impl FormatRule<TsTypeMemberList> for FormatTsTypeMemberList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsTypeMemberList, f: &mut JsFormatter) -> FormatResult<()> {
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

        joiner.finish()
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
                        write!(f, [if_group_breaks(&text(";"))])?;
                    } else {
                        text(";").fmt(f)?;
                    }
                }
                Semicolons::AsNeeded => {
                    if !self.last {
                        write!(f, [if_group_fits_on_line(&text(";"))])?;
                    }
                }
            }
        }

        Ok(())
    }
}
