use std::any::TypeId;

use biome_analyze::{merge_node_visitors, Visitor, VisitorContext};
use biome_js_syntax::{
    AnyJsFunction, JsConstructorClassMember, JsGetterClassMember, JsGetterObjectMember, JsLanguage,
    JsMethodClassMember, JsMethodObjectMember, JsModule, JsScript, JsSetterClassMember,
    JsSetterObjectMember, JsStaticInitializationBlockClassMember, TsModuleDeclaration,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxError, SyntaxResult};

use crate::ControlFlowGraph;

use super::{nodes::*, FunctionBuilder};

/// Return a new instance of the [ControlFlowVisitor]
pub(crate) fn make_visitor() -> impl Visitor<Language = JsLanguage> {
    ControlFlowVisitor::new()
}

/// Wrapper macro for [merge_node_visitors], implements additional control
/// flow-related utilities on top of the generated visitor
macro_rules! declare_visitor {
    ( $vis:vis $name:ident { $( $id:ident: $visitor:ty, )* } ) => {
        merge_node_visitors! {
            $vis $name {
                function: FunctionVisitor,
                $( $id: VisitorAdapter<$visitor>, )*
            }
        }

        /// Slice of the merged visitor state stack cut off at the current function
        pub(super) struct StatementStack<'a> {
            pub(super) stack: &'a mut [(TypeId, usize)],
            $(
                #[cfg(debug_assertions)]
                $id: (usize, &'a mut [(usize, VisitorAdapter<$visitor>)]),
                #[cfg(not(debug_assertions))]
                $id: &'a mut [(usize, VisitorAdapter<$visitor>)],
            )*
        }

        impl<'a> StatementStack<'a> {
            /// Split the visitor state at the topmost function, returning the
            /// corresponding function visitor and the rest of the stack above it
            fn new(visitor: &'a mut $name) -> Option<(&'a mut FunctionVisitor, Self)> {
                let (index, builder) = visitor.function.last_mut()?;

                Some((builder, Self {
                    stack: {
                        let stack_len = visitor.stack.len();
                        visitor.stack.get_mut(*index + 1..).unwrap_or_else(|| panic!("stack index out of bounds: {} >= {stack_len}", *index + 1))
                    },
                    $(
                        // For safety, cut off the stack slices below the start
                        // of the current function in debug mode
                        #[cfg(debug_assertions)]
                        $id: (
                            visitor
                                .$id
                                .iter()
                                .rposition(|(stack_index, _)| *stack_index < *index)
                                .map_or(0, |index| (index + 1).min(visitor.$id.len().saturating_sub(1))),
                            &mut visitor.$id,
                        ),
                        #[cfg(not(debug_assertions))]
                        $id: &mut visitor.$id,
                    )*
                }))
            }
        }

        $( impl<'a> MergedVisitor<'a, $visitor> for StatementStack<'a> {
            fn read_top(self) -> SyntaxResult<&'a mut $visitor> {
                #[cfg(debug_assertions)]
                let (_, visitor) =
                    self.$id.1.last_mut().ok_or(::biome_rowan::SyntaxError::MissingRequiredChild)?;
                #[cfg(not(debug_assertions))]
                let (_, visitor) =
                    self.$id.last_mut().ok_or(::biome_rowan::SyntaxError::MissingRequiredChild)?;

                let VisitorAdapter(visitor) = visitor;
                let visitor = visitor.as_mut().map_err(|err| *err)?;
                Ok(visitor)
            }

            fn try_downcast(&'a self, type_id: TypeId, index: usize) -> Option<&'a $visitor> {
                if type_id != TypeId::of::<VisitorAdapter<$visitor>>() {
                    return None;
                }

                #[cfg(debug_assertions)]
                let (_, visitor) = index.checked_sub(self.$id.0)
                    .and_then(|index| self.$id.1.get(index))
                    .unwrap_or_else(|| panic!(concat!(stringify!($id), " index out of bounds: {} + {} >= {}"), index, self.$id.0, self.$id.1.len()));
                #[cfg(not(debug_assertions))]
                let (_, visitor) = self.$id.get(index)
                    .unwrap_or_else(|| panic!(concat!(stringify!($id), " index out of bounds: {} >= {}"), index, self.$id.len()));

                let VisitorAdapter(visitor) = visitor;
                let visitor = visitor.as_ref().ok()?;
                Some(visitor)
            }
        } )*
    };
}

declare_visitor! {
    ControlFlowVisitor {
        statement: StatementVisitor,
        block: BlockVisitor,
        try_stmt: TryVisitor,
        catch: CatchVisitor,
        finally: FinallyVisitor,
        if_stmt: IfVisitor,
        else_stmt: ElseVisitor,
        switch: SwitchVisitor,
        case: CaseVisitor,
        for_stmt: ForVisitor,
        for_in: ForInVisitor,
        for_of: ForOfVisitor,
        while_stmt: WhileVisitor,
        do_while: DoWhileVisitor,
        break_stmt: BreakVisitor,
        continue_stmt: ContinueVisitor,
        return_stmt: ReturnVisitor,
        throw: ThrowVisitor,
        variable: VariableVisitor,
        bogus: BogusVisitor,
    }
}

/// Utility implemented for [StatementStack] in the [declare_visitor] macro,
/// allows type checked access into the visitor state stack
pub(super) trait MergedVisitor<'a, N> {
    fn read_top(self) -> SyntaxResult<&'a mut N>;
    fn try_downcast(&'a self, type_id: TypeId, index: usize) -> Option<&'a N>;
}

// Wrapper methods on top of the `MergedVisitor` trait to support for the
// "turbofish" (`::<>`) syntax
impl<'a> StatementStack<'a> {
    pub(super) fn read_top<N>(self) -> SyntaxResult<&'a mut N>
    where
        Self: MergedVisitor<'a, N>,
    {
        MergedVisitor::read_top(self)
    }

    pub(super) fn try_downcast<N>(&'a self, type_id: TypeId, index: usize) -> Option<&'a N>
    where
        Self: MergedVisitor<'a, N>,
    {
        MergedVisitor::try_downcast(self, type_id, index)
    }
}

pub(super) struct FunctionVisitor {
    builder: Option<FunctionBuilder>,
}

declare_node_union! {
    pub AnyJsControlFlowRoot = JsModule
        | JsScript
        | AnyJsFunction
        | JsGetterObjectMember
        | JsSetterObjectMember
        | JsMethodObjectMember
        | JsConstructorClassMember
        | JsMethodClassMember
        | JsGetterClassMember
        | JsSetterClassMember
        | JsStaticInitializationBlockClassMember
        | TsModuleDeclaration
}

impl biome_analyze::NodeVisitor<ControlFlowVisitor> for FunctionVisitor {
    type Node = AnyJsControlFlowRoot;

    fn enter(
        node: Self::Node,
        _: &mut VisitorContext<JsLanguage>,
        _: &mut ControlFlowVisitor,
    ) -> Self {
        Self {
            builder: Some(FunctionBuilder::new(node.into_syntax())),
        }
    }

    fn exit(self, _: Self::Node, ctx: &mut VisitorContext<JsLanguage>, _: &mut ControlFlowVisitor) {
        if let Some(builder) = self.builder {
            ctx.match_query(ControlFlowGraph {
                graph: builder.finish(),
            });
        }
    }
}

/// Wrapper trait for [biome_analyze::NodeVisitor] adding control flow specific
/// utilities (error handling and automatic [FunctionBuilder] injection)
pub(super) trait NodeVisitor: Sized {
    type Node: AstNode<Language = JsLanguage>;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self>;

    fn exit(self, _: Self::Node, _: &mut FunctionBuilder, _: StatementStack) -> SyntaxResult<()> {
        Ok(())
    }
}

/// Wrapper type implementing [biome_analyze::NodeVisitor] for types
/// implementing the control-flow specific [NodeVisitor] trait
pub(super) struct VisitorAdapter<V>(SyntaxResult<V>);

impl<V> biome_analyze::NodeVisitor<ControlFlowVisitor> for VisitorAdapter<V>
where
    V: NodeVisitor,
{
    type Node = V::Node;

    fn enter(
        node: Self::Node,
        _: &mut VisitorContext<JsLanguage>,
        stack: &mut ControlFlowVisitor,
    ) -> Self {
        let (visitor, stack) = match StatementStack::new(stack) {
            Some((builder, stack)) => (builder, stack),
            None => return Self(Err(SyntaxError::MissingRequiredChild)),
        };

        let result = if let Some(builder) = visitor.builder.as_mut() {
            let result = V::enter(node, builder, stack);

            if result.is_err() {
                visitor.builder.take();
            }

            result
        } else {
            Err(SyntaxError::MissingRequiredChild)
        };

        Self(result)
    }

    fn exit(
        self,
        node: Self::Node,
        _: &mut VisitorContext<JsLanguage>,
        stack: &mut ControlFlowVisitor,
    ) {
        let state = match self {
            Self(Ok(state)) => state,
            _ => return,
        };

        let (visitor, stack) = match StatementStack::new(stack) {
            Some((builder, stack)) => (builder, stack),
            None => return,
        };

        if let Some(builder) = visitor.builder.as_mut() {
            if state.exit(node, builder, stack).is_err() {
                visitor.builder.take();
            }
        }
    }
}
