use crate::{
    matcher::{MatchQueryParams, Query},
    registry::{NodeLanguage, Phases},
    AnalyzerOptions, LanguageRoot, QueryMatch, QueryMatcher, ServiceBag, SignalEntry,
    SuppressionAction,
};
use biome_rowan::{AstNode, Language, SyntaxNode, TextRange, WalkEvent};
use std::collections::BinaryHeap;

/// Mutable context objects shared by all visitors
pub struct VisitorContext<'phase, 'query, L: Language> {
    pub phase: Phases,
    pub root: &'phase LanguageRoot<L>,
    pub services: &'phase ServiceBag,
    pub suppression_action: &'phase dyn SuppressionAction<Language = L>,
    pub range: Option<TextRange>,
    pub(crate) query_matcher: &'query mut dyn QueryMatcher<L>,
    pub(crate) signal_queue: &'query mut BinaryHeap<SignalEntry<'phase, L>>,
    pub options: &'phase AnalyzerOptions,
}

impl<L: Language> VisitorContext<'_, '_, L> {
    pub fn match_query<T: QueryMatch>(&mut self, query: T) {
        self.query_matcher.match_query(MatchQueryParams {
            phase: self.phase,
            root: self.root,
            query: Query::new(query),
            services: self.services,
            signal_queue: self.signal_queue,
            suppression_action: self.suppression_action,
            options: self.options,
        })
    }
}

/// Mutable context objects provided to the finish hook of visitors
pub struct VisitorFinishContext<'a, L: Language> {
    pub root: &'a LanguageRoot<L>,
    pub services: &'a mut ServiceBag,
}

/// Visitors are the main building blocks of the analyzer: they receive syntax
/// [WalkEvent]s, process these events to build secondary data structures from
/// the syntax tree, and emit rule query matches through the [crate::RuleRegistry]
pub trait Visitor {
    type Language: Language;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        ctx: VisitorContext<Self::Language>,
    );

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<Self::Language>) {
        let _ = ctx;
    }
}

/// A node visitor is a special kind of visitor that does not have a persistent
/// state for the entire run of the analyzer. Instead these visitors are
/// transient, they get instantiated when the traversal enters the
/// corresponding node type and destroyed when the corresponding node exits
///
/// Due to these specificities node visitors do not implement [Visitor]
/// directly, instead one or more of these must the merged into a single
/// visitor type using the [crate::merge_node_visitors] macro
pub trait NodeVisitor<V>: Sized {
    type Node: AstNode;

    fn enter(
        node: Self::Node,
        ctx: &mut VisitorContext<NodeLanguage<Self::Node>>,
        stack: &mut V,
    ) -> Self;

    fn exit(
        self,
        node: Self::Node,
        ctx: &mut VisitorContext<NodeLanguage<Self::Node>>,
        stack: &mut V,
    );
}

/// Creates a single struct implementing [Visitor] over a collection of type
/// implementing the [NodeVisitor] helper trait. Unlike the global [Visitor],
/// node visitors are transient: they get instantiated when the traversal
/// enters the corresponding node and destroyed when the node is exited. They
/// are intended as a building blocks for creating and managing the state of
/// complex visitors by allowing the implementation to be split over multiple
/// smaller components.
///
/// # Example
///
/// ```ignore
/// struct BinaryVisitor;
///
/// impl NodeVisitor for BinaryVisitor {
///     type Node = BinaryExpression;
/// }
///
/// struct UnaryVisitor;
///
/// impl NodeVisitor for UnaryVisitor {
///     type Node = UnaryExpression;
/// }
///
/// merge_node_visitors! {
///     // This declares a new `ExpressionVisitor` struct that implements
///     // `Visitor` and manages instances of `BinaryVisitor` and
///     // `UnaryVisitor`
///     pub(crate) ExpressionVisitor {
///         binary: BinaryVisitor,
///         unary: UnaryVisitor,
///     }
/// }
/// ```
#[macro_export]
macro_rules! merge_node_visitors {
    ( $vis:vis $name:ident { $( $id:ident: $visitor:ty, )+ } ) => {
        $vis struct $name {
            stack: Vec<(::std::any::TypeId, usize)>,
            $( $vis $id: Vec<(usize, $visitor)>, )*
        }

        impl $name {
            $vis fn new() -> Self {
                Self {
                    stack: Vec::new(),
                    $( $id: Vec::new(), )*
                }
            }
        }

        impl $crate::Visitor for $name {
            type Language = <( $( <$visitor as $crate::NodeVisitor<$name>>::Node, )* ) as ::biome_rowan::macros::UnionLanguage>::Language;

            fn visit(
                &mut self,
                event: &::biome_rowan::WalkEvent<::biome_rowan::SyntaxNode<Self::Language>>,
                mut ctx: $crate::VisitorContext<Self::Language>,
            ) {
                match event {
                    ::biome_rowan::WalkEvent::Enter(node) => {
                        let kind = node.kind();

                        $(
                            if <<$visitor as $crate::NodeVisitor<$name>>::Node as ::biome_rowan::AstNode>::can_cast(kind) {
                                let node = <<$visitor as $crate::NodeVisitor<$name>>::Node as ::biome_rowan::AstNode>::unwrap_cast(node.clone());
                                let state = <$visitor as $crate::NodeVisitor<$name>>::enter(node, &mut ctx, self);

                                let stack_index = self.stack.len();
                                let ty_index = self.$id.len();

                                self.$id.push((stack_index, state));
                                self.stack.push((::std::any::TypeId::of::<$visitor>(), ty_index));
                                return;
                            }
                        )*
                    }
                    ::biome_rowan::WalkEvent::Leave(node) => {
                        let kind = node.kind();

                        $(
                            if <<$visitor as $crate::NodeVisitor<$name>>::Node as ::biome_rowan::AstNode>::can_cast(kind) {
                                self.stack.pop().unwrap();
                                let (_, state) = self.$id.pop().unwrap();

                                let node = <<$visitor as $crate::NodeVisitor<$name>>::Node as ::biome_rowan::AstNode>::unwrap_cast(node.clone());
                                <$visitor as $crate::NodeVisitor<$name>>::exit(state, node, &mut ctx, self);
                                return;
                            }
                        )*
                    }
                }
            }
        }
    };
}
