use grit_util::{AstCursor, AstNode as GritAstNode, ByteRange, CodeRange};
use std::{borrow::Cow, str::Utf8Error};

#[derive(Clone, Debug)]
pub(crate) struct GritNode;

impl GritAstNode for GritNode {
    fn ancestors(&self) -> impl Iterator<Item = Self> {
        TodoIterator
    }

    fn children(&self) -> impl Iterator<Item = Self> {
        TodoIterator
    }

    fn parent(&self) -> Option<Self> {
        todo!()
    }

    fn next_named_node(&self) -> Option<Self> {
        todo!()
    }

    fn previous_named_node(&self) -> Option<Self> {
        todo!()
    }

    fn next_sibling(&self) -> Option<Self> {
        todo!()
    }

    fn previous_sibling(&self) -> Option<Self> {
        todo!()
    }

    fn text(&self) -> Result<Cow<str>, Utf8Error> {
        todo!()
    }

    fn byte_range(&self) -> ByteRange {
        todo!()
    }

    fn code_range(&self) -> CodeRange {
        todo!()
    }

    fn full_source(&self) -> &str {
        todo!()
    }

    fn walk(&self) -> impl AstCursor<Node = Self> {
        TodoCursor
    }
}

#[derive(Clone)]
struct TodoIterator;

impl Iterator for TodoIterator {
    type Item = GritNode;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Clone)]
struct TodoCursor;

impl AstCursor for TodoCursor {
    type Node = GritNode;

    fn goto_first_child(&mut self) -> bool {
        todo!()
    }

    fn goto_parent(&mut self) -> bool {
        todo!()
    }

    fn goto_next_sibling(&mut self) -> bool {
        todo!()
    }

    fn node(&self) -> Self::Node {
        todo!()
    }
}
