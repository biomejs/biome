use std::collections::VecDeque;

use biome_css_syntax::CssSyntaxKind::*;
use biome_rowan::{TextRange, TokenText};
use rustc_hash::FxHashMap;

type BindingName = TokenText;

#[derive(Debug, Clone)]
struct BindingInfo {
    /// range of the name
    range: TextRange,
}

#[derive(Debug)]
struct Scope {
    scope_id: usize,
    /// All bindings declared inside this scope
    bindings: Vec<BindingName>,
}

#[derive(Debug)]
pub enum CssSemanticEvent {
    /// Tracks where a new symbol declaration is found.
    /// Generated for:
    /// - Css Custom Variable Declarations
    /// - @property rules
    DeclarationFound {
        range: TextRange,
        scope_id: usize,
        hoisted_scope_id: Option<usize>,
    },

    /// Tracks where a symbol is read, but only if its declaration is before this reference.
    /// Generated for:
    /// - All reference identifiers
    Read {
        range: TextRange,
        declared_at: TextRange,
        scope_id: usize,
    },

    /// Tracks references that do no have any matching binding
    /// Generated for:
    /// - Unmatched reference identifiers
    UnresolvedReference { is_read: bool, range: TextRange },

    /// Tracks where a new scope starts
    /// Generated for:
    /// - Selectors
    /// - Nested selectors
    /// - @media rules
    /// - @supports rules
    ScopeStarted {
        /// Scope range
        range: TextRange,
        scope_id: usize,
        parent_scope_id: Option<usize>,
    },

    /// Tracks where a scope ends
    /// Generated for:
    /// - Blocks
    /// - Function body
    ScopeEnded {
        /// Scope range
        range: TextRange,
        scope_id: usize,
    },
}

#[derive(Default, Debug)]
pub struct CssSemanticEventExtractor {
    /// Event queue
    stash: VecDeque<CssSemanticEvent>,
    /// Stack of scopes
    scopes: Vec<Scope>,
    /// Number of generated scopes
    /// This allows assigning a unique scope id to every scope.
    scope_count: usize,
    /// At any point this is the set of available bindings and their range in the current scope
    bindings: FxHashMap<TokenText, BindingInfo>,
}

impl CssSemanticEventExtractor {
    pub fn new() -> Self {
        Self {
            stash: VecDeque::new(),
            scopes: vec![],
            scope_count: 0,
            bindings: FxHashMap::default(),
        }
    }
    pub fn pop(&mut self) -> Option<CssSemanticEvent> {
        self.stash.pop_front()
    }

    pub fn enter(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        match node.kind() {
            CSS_SELECTOR_LIST
            | CSS_RELATIVE_SELECTOR_LIST
            | CSS_SUPPORTS_AT_RULE
            | CSS_MEDIA_AT_RULE => self.push_scope(node.text_range()),
            _ => {}
        }
    }

    pub fn leave(&mut self, node: &biome_css_syntax::CssSyntaxNode) {
        match node.kind() {
            CSS_SELECTOR_LIST
            | CSS_RELATIVE_SELECTOR_LIST
            | CSS_SUPPORTS_AT_RULE
            | CSS_MEDIA_AT_RULE => self.pop_scope(node.text_range()),
            _ => {}
        }
    }

    fn push_scope(&mut self, range: TextRange) {
        let scope_id = self.scope_count;
        self.scope_count += 1;
        self.stash.push_back(CssSemanticEvent::ScopeStarted {
            range,
            scope_id,
            parent_scope_id: self.scopes.iter().last().map(|x| x.scope_id),
        });
        self.scopes.push(Scope {
            scope_id,
            bindings: vec![],
        });
    }

    fn pop_scope(&mut self, range: TextRange) {
        debug_assert!(!self.scopes.is_empty());
        let scope = self.scopes.pop().unwrap();
        let scope_id = scope.scope_id;

        self.stash
            .push_back(CssSemanticEvent::ScopeEnded { range, scope_id });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use biome_css_parser::{parse_css, CssParserOptions};

    #[test]
    fn test_css_semantic_event_extractor() {
        let tree = parse_css(
            r#".parent {
    .child {
        color: red;
    }
}"#,
            CssParserOptions::default(),
        );
        let mut extractor = CssSemanticEventExtractor::new();
        for e in tree.syntax().preorder() {
            match e {
                biome_rowan::WalkEvent::Enter(node) => extractor.enter(&node),
                biome_rowan::WalkEvent::Leave(node) => extractor.leave(&node),
            }

            while let Some(e) = extractor.pop() {
                dbg!(e);
            }
        }
    }
}
