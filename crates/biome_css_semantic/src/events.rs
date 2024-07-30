use std::collections::VecDeque;

#[derive(Debug)]
pub enum SemanticEvent {}

#[derive(Default, Debug)]
pub struct SemanticEventExtractor {
    stash: VecDeque<SemanticEvent>,
}

impl SemanticEventExtractor {
    pub fn enter(&mut self, _node: &biome_css_syntax::CssSyntaxNode) {}

    pub fn leave(&mut self, _node: &biome_css_syntax::CssSyntaxNode) {}

    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }
}
