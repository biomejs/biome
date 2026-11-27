//! Events emitted by the Parser which are then constructed into a syntax tree

use std::mem;
use std::num::NonZeroU32;

use crate::diagnostic::ParseDiagnostic;
use crate::tree_sink::TreeSink;
use biome_rowan::{SyntaxKind, TextSize};

/// Events emitted by the Parser, these events are later
/// made into a syntax tree with `process` into TreeSink.
#[derive(Debug, Clone)]
pub enum Event<K: SyntaxKind> {
    /// This event signifies the start of the node.
    /// It should be either abandoned (in which case the
    /// `kind` is `TOMBSTONE`, and the event is ignored),
    /// or completed via a `Finish` event.
    ///
    /// All tokens between a `Start` and a `Finish` would
    /// become the children of the respective node.
    Start {
        kind: K,
        forward_parent: Option<NonZeroU32>,
    },

    /// Complete the previous `Start` event
    Finish,

    /// Produce a single leaf-element.
    Token {
        kind: K,
        /// The end offset of this token.
        end: TextSize,
    },
}

impl<K: SyntaxKind> Event<K> {
    pub fn tombstone() -> Self {
        Self::Start {
            kind: K::TOMBSTONE,
            forward_parent: None,
        }
    }
}

/// Generate the syntax tree with the control of events.
#[inline]
pub fn process<K: SyntaxKind + PartialEq>(
    sink: &mut impl TreeSink<Kind = K>,
    mut events: Vec<Event<K>>,
    errors: Vec<ParseDiagnostic>,
) {
    sink.errors(errors);
    let mut forward_parents = Vec::new();

    for i in 0..events.len() {
        match &mut events[i] {
            Event::Start {
                kind,
                forward_parent,
                ..
            } => {
                if *kind == K::TOMBSTONE {
                    continue;
                }

                if forward_parent.is_none() {
                    sink.start_node(*kind);
                    continue;
                }

                // For events[A, B, C], B is A's forward_parent, C is B's forward_parent,
                // in the normal control flow, the parent-child relation: `A -> B -> C`,
                // while with the magic forward_parent, it writes: `C <- B <- A`.

                // append `A` into parents.
                forward_parents.push(*kind);
                let mut idx = i;
                let mut fp = *forward_parent;
                while let Some(fwd) = fp {
                    idx += u32::from(fwd) as usize;
                    // append `A`'s forward_parent `B`
                    fp = match mem::replace(&mut events[idx], Event::tombstone()) {
                        Event::Start {
                            kind,
                            forward_parent,
                            ..
                        } => {
                            if kind != K::TOMBSTONE {
                                forward_parents.push(kind);
                            }
                            forward_parent
                        }
                        _ => unreachable!(),
                    };
                    // append `B`'s forward_parent `C` in the next stage.
                }

                for kind in forward_parents.iter().rev() {
                    sink.start_node(*kind);
                }
                forward_parents.clear();
            }
            Event::Finish => sink.finish_node(),
            Event::Token { kind, end } => {
                sink.token(*kind, *end);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, process};
    use crate::diagnostic::ParseDiagnostic;
    use crate::tree_sink::TreeSink;
    use biome_rowan::TextSize;
    use biome_rowan::raw_language::RawLanguageKind::{self, *};
    use std::num::NonZeroU32;

    #[derive(Default)]
    struct RecordingSink(Vec<String>);

    impl TreeSink for RecordingSink {
        type Kind = RawLanguageKind;

        fn token(&mut self, kind: RawLanguageKind, end: TextSize) {
            self.0.push(format!("token {kind:?} {end:?}"));
        }

        fn start_node(&mut self, kind: RawLanguageKind) {
            self.0.push(format!("start {kind:?}"));
        }

        fn finish_node(&mut self) {
            self.0.push("finish".to_string());
        }

        fn errors(&mut self, _errors: Vec<ParseDiagnostic>) {}
    }

    fn start(kind: RawLanguageKind, forward_parent: u32) -> Event<RawLanguageKind> {
        Event::Start {
            kind,
            forward_parent: NonZeroU32::new(forward_parent),
        }
    }

    fn token(kind: RawLanguageKind, end: u32) -> Event<RawLanguageKind> {
        Event::Token {
            kind,
            end: TextSize::from(end),
        }
    }

    #[test]
    fn starts_forward_parents_before_the_node_that_points_to_them() {
        let events = vec![
            start(ROOT, 0),
            // Preceded twice: by an abandoned marker, which was then preceded by a list.
            start(LITERAL_EXPRESSION, 3),
            token(STRING_TOKEN, 1),
            Event::Finish,
            start(TOMBSTONE, 1),
            start(SEPARATED_EXPRESSION_LIST, 0),
            token(COMMA_TOKEN, 2),
            Event::Finish,
            Event::Finish,
        ];

        let mut sink = RecordingSink::default();
        process(&mut sink, events, Vec::new());

        assert_eq!(
            sink.0,
            [
                "start ROOT",
                "start SEPARATED_EXPRESSION_LIST",
                "start LITERAL_EXPRESSION",
                "token STRING_TOKEN 1",
                "finish",
                "token COMMA_TOKEN 2",
                "finish",
                "finish",
            ]
        );
    }
}
