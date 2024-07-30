use std::collections::VecDeque;

use biome_rowan::TextRange;

#[derive(Debug)]
pub enum SemanticEvent {
    SelectorDeclaration { range: TextRange },
}

#[derive(Default, Debug)]
pub struct SemanticEventExtractor {
    stash: VecDeque<SemanticEvent>,
}

impl SemanticEventExtractor {
    pub fn enter(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        match node.kind() {
            biome_css_syntax::CssSyntaxKind::CSS_SELECTOR_LIST => {
                self.stash.push_back(SemanticEvent::SelectorDeclaration {
                    range: node.text_range(),
                });
            }
            _ => {}
        }
    }

    pub fn leave(&mut self, _node: &biome_css_syntax::CssSyntaxNode) {}

    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }
}
