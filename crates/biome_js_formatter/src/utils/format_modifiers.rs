use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use crate::{AsFormat, IntoFormat};
use biome_formatter::{format_args, write};
use biome_js_syntax::JsSyntaxKind::JS_DECORATOR;
use biome_js_syntax::{JsLanguage, Modifier};
use biome_rowan::{AstNode, AstNodeList, NodeOrToken};

pub(crate) struct FormatModifiers<List> {
    pub(crate) list: List,
}

impl<List> FormatModifiers<List> {
    pub(crate) fn from(list: List) -> Self {
        Self { list }
    }
}

impl<List, Node> Format<JsFormatContext> for FormatModifiers<List>
where
    Node: AstNode<Language = JsLanguage> + AsFormat<JsFormatContext> + IntoFormat<JsFormatContext>,
    List: AstNodeList<Language = JsLanguage, Node = Node>,
    Modifier: for<'a> From<&'a Node>,
{
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let modifiers = sort_modifiers_by_precedence(&self.list);
        let should_expand = should_expand_decorators(&self.list);

        // Returning early here is important, because otherwise this node
        // returns a group that always has a soft line break, which causes
        // `may_directly_break` to return true, even if there is no
        // possibility for the break to be used, since it's at the start of
        // a line with no content before it. An example case this affects is:
        //
        // ```js
        // class Test {
        //   prop1 = // comment
        //     true;
        // }
        // ```
        //
        // Here, the modifier list is present before `prop1`, but part of the
        // statement as a whole. The statement checks if it _may_ break when
        // determining how to position the trailing comment. If it does break,
        // the comment is placed on a new line and the value on a line after
        // that. But if it doesn't break, then the whole statement can remnain
        // on a single line, which is desirable and important for preserving
        // semantics of things like ignore comments.
        //
        // ```js
        // class Test {
        //   prop1 = true; // comment
        // }
        if self.list.is_empty() {
            return Ok(());
        }

        // need to use peek the iterator to check if the current node is a decorator and don't advance the iterator
        let mut iter = modifiers.into_iter().peekable();
        let decorators = format_once(|f| {
            let mut join = f.join_nodes_with_soft_line();

            // join only decorators here
            while let Some(node) = iter.peek() {
                // check if the current node is a decorator
                match node.syntax().kind() {
                    JS_DECORATOR => {
                        join.entry(node.syntax(), &node.format());
                        // advance the iterator
                        iter.next();
                    }
                    _ => {
                        // if we encounter a non-decorator we break out of the loop
                        break;
                    }
                }
            }

            join.finish()
        });

        write!(
            f,
            [group(&format_args![decorators, soft_line_break_or_space()])
                .should_expand(should_expand)]
        )?;

        // join the rest of the modifiers
        f.join_with(&space()).entries(iter.formatted()).finish()
    }
}

/// This function expands decorators enclosing a group if there is a newline between decorators or after the last decorator.
pub(crate) fn should_expand_decorators<List, Node>(list: &List) -> bool
where
    Node: AstNode<Language = JsLanguage>,
    List: AstNodeList<Language = JsLanguage, Node = Node>,
{
    // we need to skip the first node because we look for newlines between decorators or after the last decorator
    for node in list.iter().skip(1) {
        match node.syntax().kind() {
            JS_DECORATOR => {
                if node.syntax().has_leading_newline() {
                    return true;
                }
            }
            _ => {
                // if we encounter a non-decorator with a leading newline after a decorator and the next modifier
                return node.syntax().has_leading_newline();
            }
        }
    }

    // if we encounter a non-decorator with a leading newline after a decorator and the next node or token
    list.syntax_list()
        .node()
        .next_sibling_or_token()
        .is_some_and(|node| match node {
            NodeOrToken::Node(node) => node.has_leading_newline(),
            NodeOrToken::Token(token) => token.has_leading_newline(),
        })
}
