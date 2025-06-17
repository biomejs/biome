use crate::JsFormatContext;
use crate::prelude::*;
use biome_formatter::{Expand, write};
use biome_formatter::{Format, FormatResult};
use biome_js_syntax::{
    JsFormalParameter, JsObjectExpression, JsParameterList, JsSyntaxToken, TsObjectType,
    TsTypeAnnotation,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxResult, declare_node_union};

declare_node_union! {
    pub (crate) JsObjectLike = JsObjectExpression | TsObjectType
}
impl JsObjectLike {
    fn l_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsObjectExpression(oe) => oe.l_curly_token(),
            Self::TsObjectType(ot) => ot.l_curly_token(),
        }
    }
    fn r_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsObjectExpression(oe) => oe.r_curly_token(),
            Self::TsObjectType(ot) => ot.r_curly_token(),
        }
    }

    fn members_have_leading_newline(&self) -> bool {
        match self {
            Self::JsObjectExpression(oe) => oe.members().syntax().has_leading_newline(),
            Self::TsObjectType(ot) => ot.members().syntax().has_leading_newline(),
        }
    }

    fn members_are_empty(&self) -> bool {
        match self {
            Self::JsObjectExpression(oe) => oe.members().is_empty(),
            Self::TsObjectType(ot) => ot.members().is_empty(),
        }
    }

    fn write_members(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            Self::JsObjectExpression(oe) => {
                write!(f, [oe.members().format()])
            }
            Self::TsObjectType(ot) => {
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
            let should_expand = (f.options().expand() == Expand::Auto
                && self.members_have_leading_newline())
                || f.options().expand() == Expand::Always;

            // If the object type is the type annotation of the only parameter in a function,
            // try to hug the parameter; we don't create a group and inline the contents here.
            //
            // For example:
            // ```ts
            // const fn = ({ foo }: { foo: string }) => { ... };
            //                      ^ do not break properties here
            // ```
            let should_hug = self.parent::<TsTypeAnnotation>().is_some_and(|node| {
                node.parent::<JsFormalParameter>().is_some_and(|node| {
                    node.parent::<JsParameterList>()
                        .is_some_and(|node| node.len() == 1)
                })
            });

            let inner =
                &soft_block_indent_with_maybe_space(&members, should_insert_space_around_brackets);

            if should_hug {
                write!(f, [inner])?;
            } else {
                write!(f, [group(inner).should_expand(should_expand)])?;
            }
        }

        write!(f, [self.r_curly_token().format()])
    }
}
