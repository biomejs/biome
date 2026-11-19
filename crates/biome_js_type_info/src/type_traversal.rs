use rustc_hash::FxHashSet;
use std::{hash::Hash, ops::ControlFlow};

enum TraversalEvent<N> {
    Visit(N),
    Finish(N),
}

/// Schedules children of the node currently passed to a [`DepthFirstVisitor`].
///
/// Children are visited in reverse scheduling order because traversal uses a
/// stack. Scheduling `[first, second]` visits `second` before `first`.
pub(crate) struct VisitContext<'a, N> {
    pending: &'a mut Vec<TraversalEvent<N>>,
}

impl<N> VisitContext<'_, N> {
    pub(crate) fn push(&mut self, child: N) {
        self.pending.push(TraversalEvent::Visit(child));
    }

    pub(crate) fn extend(&mut self, children: impl IntoIterator<Item = N>) {
        self.pending
            .extend(children.into_iter().map(TraversalEvent::Visit));
    }
}

/// Visits each distinct node in a depth-first traversal.
///
/// The traversal invokes `enter` only when a node is first entered. Nodes
/// reached again after their visit finishes are skipped. Reaching a node that
/// is still being visited is reported as a cycle in [`TraversalOutcome`].
pub(crate) trait DepthFirstVisitor<N>
where
    N: Copy + Eq + Hash,
{
    type Break;

    fn enter(&mut self, node: N, context: &mut VisitContext<'_, N>) -> ControlFlow<Self::Break>;

    /// Visits the graph reachable from `root` without recursion.
    ///
    /// `max_visited` counts distinct node entries. Completed duplicates, cycle
    /// edges, and internal finish events do not consume the limit.
    fn visit(&mut self, root: N, max_visited: usize) -> TraversalOutcome<Self::Break> {
        let mut active = FxHashSet::default();
        let mut completed = FxHashSet::default();
        let mut pending = vec![TraversalEvent::Visit(root)];
        let mut remaining = max_visited;
        let mut encountered_cycle = false;

        while let Some(event) = pending.pop() {
            let node = match event {
                TraversalEvent::Finish(node) => {
                    active.remove(&node);
                    completed.insert(node);
                    continue;
                }
                TraversalEvent::Visit(node) => node,
            };
            if completed.contains(&node) {
                continue;
            }
            if !active.insert(node) {
                encountered_cycle = true;
                continue;
            }
            if remaining == 0 {
                return TraversalOutcome::LimitExceeded;
            }
            remaining -= 1;
            pending.push(TraversalEvent::Finish(node));

            let mut context = VisitContext {
                pending: &mut pending,
            };
            if let ControlFlow::Break(result) = self.enter(node, &mut context) {
                return TraversalOutcome::Break(result);
            }
        }

        TraversalOutcome::Complete { encountered_cycle }
    }
}

/// Result of a bounded depth-first traversal.
pub(crate) enum TraversalOutcome<B> {
    /// The visitor stopped traversal with a conclusive result.
    Break(B),
    /// Every reachable node was processed.
    Complete {
        /// At least one edge reached a node on the active traversal path.
        encountered_cycle: bool,
    },
    /// Processing another distinct node would exceed the traversal limit.
    LimitExceeded,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestVisitor<'a> {
        edges: &'a [(u8, &'a [u8])],
        visited: Vec<u8>,
        break_at: Option<u8>,
    }

    impl DepthFirstVisitor<u8> for TestVisitor<'_> {
        type Break = u8;

        fn enter(
            &mut self,
            node: u8,
            context: &mut VisitContext<'_, u8>,
        ) -> ControlFlow<Self::Break> {
            self.visited.push(node);
            if self.break_at == Some(node) {
                return ControlFlow::Break(node);
            }
            if let Some((_, children)) = self.edges.iter().find(|(parent, _)| *parent == node) {
                context.extend(children.iter().copied());
            }
            ControlFlow::Continue(())
        }
    }

    #[test]
    fn distinguishes_cycles_from_completed_duplicates() {
        let mut cycle = TestVisitor {
            edges: &[(0, &[1]), (1, &[0])],
            visited: Vec::new(),
            break_at: None,
        };
        assert!(matches!(
            cycle.visit(0, 3),
            TraversalOutcome::Complete {
                encountered_cycle: true
            }
        ));
        assert_eq!(cycle.visited, [0, 1]);

        let mut diamond = TestVisitor {
            edges: &[(0, &[1, 2]), (1, &[3]), (2, &[3])],
            visited: Vec::new(),
            break_at: None,
        };
        assert!(matches!(
            diamond.visit(0, 4),
            TraversalOutcome::Complete {
                encountered_cycle: false
            }
        ));
        assert_eq!(diamond.visited, [0, 2, 3, 1]);
    }

    #[test]
    fn reports_limits_and_breaks() {
        let mut limited = TestVisitor {
            edges: &[(0, &[1]), (1, &[2])],
            visited: Vec::new(),
            break_at: None,
        };
        assert!(matches!(
            limited.visit(0, 2),
            TraversalOutcome::LimitExceeded
        ));
        assert_eq!(limited.visited, [0, 1]);

        let mut breaking = TestVisitor {
            edges: &[(0, &[1, 2])],
            visited: Vec::new(),
            break_at: Some(2),
        };
        assert!(matches!(breaking.visit(0, 3), TraversalOutcome::Break(2)));
        assert_eq!(breaking.visited, [0, 2]);
    }
}
