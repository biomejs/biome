use std::any::TypeId;

use biome_js_syntax::{
    AnyJsFunction, JsConstructorClassMember, JsGetterClassMember, JsGetterObjectMember, JsLanguage,
    JsMethodClassMember, JsMethodObjectMember, JsModule, JsScript, JsSetterClassMember,
    JsSetterObjectMember, JsStaticInitializationBlockClassMember, JsSyntaxNode,
    TsModuleDeclaration, TsPropertySignatureTypeMember,
};
use biome_rowan::{AstNode, SyntaxError, SyntaxResult, WalkEvent, declare_node_union};

use crate::JsControlFlowGraph;

use super::{FunctionBuilder, nodes::*};

/// Keeps typed statement stacks aligned with the syntax traversal and isolates
/// each execution root from its enclosing statements.
macro_rules! declare_visitor {
    ( $vis:vis $name:ident { $( $id:ident: $visitor:ty, )* } ) => {
        $vis struct $name {
            stack: Vec<(TypeId, usize)>,
            function: Vec<(usize, FunctionVisitor)>,
            $( $id: Vec<(usize, VisitorAdapter<$visitor>)>, )*
        }

        impl $name {
            pub(crate) fn new() -> Self {
                Self {
                    stack: Vec::new(),
                    function: Vec::new(),
                    $( $id: Vec::new(), )*
                }
            }

            pub(crate) fn visit(&mut self, event: WalkEvent<JsSyntaxNode>, graphs: &mut Vec<JsControlFlowGraph>) {
                match event {
                    WalkEvent::Enter(node) => {
                        if let Some(root) = AnyJsControlFlowRoot::cast_ref(&node) {
                            let state = FunctionVisitor::enter(root);
                            self.stack.push((TypeId::of::<FunctionVisitor>(), self.function.len()));
                            self.function.push((self.stack.len() - 1, state));
                            return;
                        }
                        $(
                            if let Some(node) = <$visitor as NodeVisitor>::Node::cast_ref(&node) {
                                let state = VisitorAdapter::<$visitor>::enter(node, self);
                                self.stack.push((TypeId::of::<VisitorAdapter<$visitor>>(), self.$id.len()));
                                self.$id.push((self.stack.len() - 1, state));
                                return;
                            }
                        )*
                    }
                    WalkEvent::Leave(node) => {
                        if AnyJsControlFlowRoot::can_cast(node.kind()) {
                            self.stack.pop();
                            let state = self.function.pop();
                            debug_assert!(state.is_some(), "execution root must have an entry state");
                            let Some((_, state)) = state else { return; };
                            if let Some(builder) = state.builder {
                                graphs.push(builder.finish());
                            }
                            return;
                        }
                        $(
                            if let Some(node) = <$visitor as NodeVisitor>::Node::cast_ref(&node) {
                                self.stack.pop();
                                let state = self.$id.pop();
                                debug_assert!(state.is_some(), "statement must have an entry state");
                                let Some((_, state)) = state else { return; };
                                state.exit(node, self);
                                return;
                            }
                        )*
                    }
                }
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
                    stack: visitor.stack.get_mut(*index + 1..)?,
                    $(
                        // For safety, cut off the stack slices below the start
                        // of the current function in debug mode
                        #[cfg(debug_assertions)]
                        $id: {
                            let start = visitor
                                .$id
                                .iter()
                                .rposition(|(stack_index, _)| *stack_index < *index)
                                .map_or(0, |index| index + 1);
                            (start, visitor.$id.get_mut(start..)?)
                        },
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
                    .and_then(|index| self.$id.1.get(index))?;
                #[cfg(not(debug_assertions))]
                let (_, visitor) = self.$id.get(index)?;

                let VisitorAdapter(visitor) = visitor;
                let visitor = visitor.as_ref().ok()?;
                Some(visitor)
            }
        } )*
    };
}

declare_visitor! {
    pub(crate) ControlFlowVisitor {
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
        | TsPropertySignatureTypeMember
}

impl FunctionVisitor {
    fn enter(node: AnyJsControlFlowRoot) -> Self {
        Self {
            builder: Some(FunctionBuilder::new(node.into_syntax())),
        }
    }
}

/// Builds statement control flow inside the current execution root.
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

/// Aborts the current graph when a statement visitor encounters malformed syntax.
pub(super) struct VisitorAdapter<V>(SyntaxResult<V>);

impl<V> VisitorAdapter<V>
where
    V: NodeVisitor,
{
    fn enter(node: V::Node, stack: &mut ControlFlowVisitor) -> Self {
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

    fn exit(self, node: V::Node, stack: &mut ControlFlowVisitor) {
        let state = match self {
            Self(Ok(state)) => state,
            _ => return,
        };

        let (visitor, stack) = match StatementStack::new(stack) {
            Some((builder, stack)) => (builder, stack),
            None => return,
        };

        if let Some(builder) = visitor.builder.as_mut()
            && state.exit(node, builder, stack).is_err()
        {
            visitor.builder.take();
        }
    }
}
