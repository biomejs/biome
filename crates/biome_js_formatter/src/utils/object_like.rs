use crate::prelude::*;
use crate::JsFormatContext;
use biome_formatter::write;
use biome_formatter::{Format, FormatResult};
use biome_js_syntax::{
    JsFormalParameter, JsObjectExpression, JsSyntaxToken, TsObjectType, TsTypeAnnotation,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, AstSeparatedList, SyntaxResult};

declare_node_union! {
    pub (crate) JsObjectLike = JsObjectExpression | TsObjectType
}
impl JsObjectLike {
    fn l_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsObjectLike::JsObjectExpression(oe) => oe.l_curly_token(),
            JsObjectLike::TsObjectType(ot) => ot.l_curly_token(),
        }
    }
    fn r_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsObjectLike::JsObjectExpression(oe) => oe.r_curly_token(),
            JsObjectLike::TsObjectType(ot) => ot.r_curly_token(),
        }
    }

    fn members_have_leading_newline(&self) -> bool {
        match self {
            JsObjectLike::JsObjectExpression(oe) => oe.members().syntax().has_leading_newline(),
            JsObjectLike::TsObjectType(ot) => ot.members().syntax().has_leading_newline(),
        }
    }

    fn members_are_empty(&self) -> bool {
        match self {
            JsObjectLike::JsObjectExpression(oe) => oe.members().is_empty(),
            JsObjectLike::TsObjectType(ot) => ot.members().is_empty(),
        }
    }

    fn write_members(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            JsObjectLike::JsObjectExpression(oe) => {
                write!(f, [oe.members().format()])
            }
            JsObjectLike::TsObjectType(ot) => {
                write!(f, [ot.members().format()])
            }
        }
    }
}

impl Format<JsFormatContext> for JsObjectLike {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let members = format_with(|f| self.write_members(f));

        write!(f, [self.l_curly_token().format(),])?;

        if self.members_are_empty() {
            write!(
                f,
                [format_dangling_comments(self.syntax()).with_block_indent(),]
            )?;
        } else {
            let should_insert_space_around_brackets = f.options().bracket_spacing().value();
            let should_expand = f.options().object_wrap().is_preserve()
                && self.members_have_leading_newline()
                // const fn = ({ foo }: { foo: string }) => { ... };
                //                      ^ do not break properties here
                && self
                    .parent::<TsTypeAnnotation>()
                    .is_none_or(|node| node.parent::<JsFormalParameter>().is_none());

            write!(
                f,
                [group(&soft_block_indent_with_maybe_space(
                    &members,
                    should_insert_space_around_brackets
                ))
                .should_expand(should_expand)]
            )?;
        }

        write!(f, [self.r_curly_token().format()])
    }
}
