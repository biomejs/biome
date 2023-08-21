use std::{collections::VecDeque, iter, slice};

use roaring::bitmap::RoaringBitmap;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_control_flow::InstructionKind;
use rome_js_syntax::{
    AnyJsClass, AnyJsExpression, JsConstructorClassMember, JsSuperExpression, JsSyntaxElement,
    JsThisExpression, JsThrowStatement, TextRange, WalkEvent,
};
use rome_rowan::AstNode;

use crate::control_flow::{AnyJsControlFlowRoot, BasicBlock, ControlFlowGraph};

declare_rule! {
    /// Ensures the `super()` constructor is called exactly once on every code
    /// path in a class constructor before `this` is accessed if the class has
    /// a superclass
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A extends B {
    ///     constructor() {}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class A extends B {
    ///     constructor(value) {
    ///         this.prop = value;
    ///         super();
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class A extends B {
    ///     constructor(cond) {
    ///         if(cond) {
    ///             super();
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export default class A extends B {
    ///     constructor() {
    ///         super();
    ///     }
    /// }
    /// ```
    ///
    /// ```js
    /// export class A {
    ///     constructor() {}
    /// }
    /// ```
    ///
    pub(crate) NoUnreachableSuper {
        version: "1.0.0",
        name: "noUnreachableSuper",
        recommended: true,
    }
}

#[allow(clippy::enum_variant_names)]
pub(crate) enum RuleState {
    /// The constructor reads or write from `this` before calling `super`
    ThisBeforeSuper { this: TextRange, super_: TextRange },
    /// The constructor may call `super` multiple times
    DuplicateSuper { first: TextRange, second: TextRange },
    /// The constructor may read or write from `this` without calling `super`
    ThisWithoutSuper { this: TextRange },
    /// The constructor may return without calling `super`
    ReturnWithoutSuper { return_: Option<TextRange> },
}

impl Rule for NoUnreachableSuper {
    type Query = ControlFlowGraph;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let cfg = ctx.query();

        // Ignore non-constructor functions
        let constructor = JsConstructorClassMember::cast_ref(&cfg.node)?;

        // Find the class this constructor belongs to
        let class = constructor
            .syntax()
            .ancestors()
            .find_map(AnyJsClass::cast)?;

        // Do not run the rule if the class has no extends clause or is extending a literal expression
        let extends_clause = class.extends_clause()?;
        let super_class = extends_clause.super_class().ok()?;
        if matches!(super_class, AnyJsExpression::AnyJsLiteralExpression(_)) {
            return None;
        }

        // Iterate over all the blocks, performing block-local checks and
        // collecting metadata on the control flow graph in various
        // acceleration structures
        let mut outgoing_edges = BlockMap::default();
        let mut incoming_edges = BlockMap::default();

        let mut this_blocks = BlockMap::default();
        let mut super_blocks = BlockMap::default();
        let mut return_blocks = BlockMap::default();

        for (block_index, block) in cfg.blocks.iter().enumerate() {
            let signal = inspect_block(
                &mut outgoing_edges,
                &mut incoming_edges,
                &mut this_blocks,
                &mut super_blocks,
                &mut return_blocks,
                block,
                block_index.try_into().expect("integer overflow"),
            );

            if let Some(signal) = signal {
                return Some(signal);
            }
        }

        // Traverse the control flow graph "downwards" starting from blocks
        // containing a `super()` call towards the return points of the constructor
        let mut queue = VecDeque::new();

        for (block_id, super_expression) in &super_blocks {
            if let Some(outgoing_edges) = outgoing_edges.get(block_id) {
                for next_block in outgoing_edges {
                    queue.push_back((next_block, block_id, super_expression));
                }
            }
        }

        // During the traversal, all the `super()` expressions that precede a
        // given block are collected into the `predecessors`
        let mut predecessors = BlockMap::default();

        while let Some((block_id, prev_block, super_expression)) = queue.pop_front() {
            let visited = predecessors
                .entry(block_id)
                .get_or_insert_with(BlockMap::<&JsSuperExpression>::default);

            let previous_node = visited
                .insert(prev_block, super_expression)
                .filter(|previous_node| *previous_node == super_expression);

            if previous_node.is_some() {
                continue;
            }

            if let Some(outgoing_edges) = outgoing_edges.get(block_id) {
                for next_block in outgoing_edges {
                    queue.push_back((next_block, block_id, super_expression));
                }
            }
        }

        // Check all the blocks containing a `super()` expression and emit an
        // error if they have a predecessor (as it means `super()` may have
        // already been called)
        for (block_id, second) in &super_blocks {
            if let Some(predecessors) = predecessors.get(block_id) {
                if let Some(first) = predecessors.values().next() {
                    return Some(RuleState::DuplicateSuper {
                        first: first.syntax().text_trimmed_range(),
                        second: second.syntax().text_trimmed_range(),
                    });
                }
            }
        }

        // For each block containing a `this`, check that it has a predecessor for each of its incoming edges
        for (block_id, this_expression) in &this_blocks {
            if super_blocks.contains_key(block_id) {
                continue;
            }

            if let Some(predecessors) = predecessors.get(block_id) {
                if let Some(incoming_edges) = incoming_edges.get(block_id) {
                    if predecessors.len() != incoming_edges.len() {
                        return Some(RuleState::ThisWithoutSuper {
                            this: this_expression.syntax().text_trimmed_range(),
                        });
                    }
                }
            } else {
                return Some(RuleState::ThisWithoutSuper {
                    this: this_expression.syntax().text_trimmed_range(),
                });
            }
        }

        // For each block containing a return instruction, check that it has a predecessor for each of its incoming edges
        for (block_id, return_range) in &return_blocks {
            if super_blocks.contains_key(block_id) {
                continue;
            }

            if let Some(predecessors) = predecessors.get(block_id) {
                if let Some(incoming_edges) = incoming_edges.get(block_id) {
                    if predecessors.len() != incoming_edges.len() {
                        return Some(RuleState::ReturnWithoutSuper {
                            return_: *return_range,
                        });
                    }
                }
            } else {
                return Some(RuleState::ReturnWithoutSuper {
                    return_: *return_range,
                });
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            RuleState::ThisBeforeSuper { this, super_ } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().node.text_trimmed_range(),
                    markup! { "This constructor has code paths accessing `"<Emphasis>"this"</Emphasis>"` before `"<Emphasis>"super()"</Emphasis>"` is called." },
                )
                .detail(this, markup! { "`"<Emphasis>"this"</Emphasis>"` is accessed here:" })
                .detail(super_, markup! { "`"<Emphasis>"super()"</Emphasis>"` is only called here:" })
                .note("If this is intentional, add an explicit throw statement in unsupported paths."),
            ),

            RuleState::ThisWithoutSuper { this } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().node.text_trimmed_range(),
                    markup! { "This constructor has code paths accessing `"<Emphasis>"this"</Emphasis>"` without calling `"<Emphasis>"super()"</Emphasis>"` first." },
                )
                .detail(this, markup! { "`"<Emphasis>"this"</Emphasis>"` is accessed here:" })
                .note("If this is intentional, add an explicit throw statement in unsupported paths."),
            ),

            RuleState::DuplicateSuper { first, second } if *first == *second => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().node.text_trimmed_range(),
                    markup! { "This constructor calls `"<Emphasis>"super()"</Emphasis>"` in a loop." },
                )
                .detail(first, markup! { "`"<Emphasis>"super()"</Emphasis>"` is called here:" }),
            ),
            RuleState::DuplicateSuper { first, second } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().node.text_trimmed_range(),
                    markup! { "This constructor has code paths where `"<Emphasis>"super()"</Emphasis>"` is called more than once." },
                )
                .detail(first, markup! { "`"<Emphasis>"super()"</Emphasis>"` is first called here:" })
                .detail(second, markup! { "`"<Emphasis>"super()"</Emphasis>"` is then called again here:" }),
            ),

            RuleState::ReturnWithoutSuper { return_: Some(range) } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().node.text_trimmed_range(),
                    markup! { "This constructor has code paths that return without calling `"<Emphasis>"super()"</Emphasis>"` first." },
                )
                .detail(range, markup! { "This statement returns from the constructor before `"<Emphasis>"super()"</Emphasis>"` has been called:" })
                .note("If this is intentional, add an explicit throw statement in unsupported paths."),
            ),
            RuleState::ReturnWithoutSuper { return_: None } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().node.text_trimmed_range(),
                    markup! { "This constructor has code paths that return without calling `"<Emphasis>"super()"</Emphasis>"`." },
                )
                .note("If this is intentional, add an explicit throw statement in unsupported paths."),
            ),
        }
    }
}

/// Performs block-local control flow checks to ensure `super()` is only called once, and
/// always before all `this` expressions or return instructions within a single block
///
/// This function also collects various acceleration structures for the graph-wide analysis step:
/// - `outgoing_edges` and `incoming_edges` map the index of a block to the set of the indices of
/// all the blocks that have a jump coming from- or pointing to this block respectively
/// - `this_blocks` maps the index of a block to the first `this` expression it contains if it has any
/// - `super_blocks` maps the index of a block to the first `super()` expression it contains if it has any
/// - `return_blocks` maps the index of a block to the first return instruction it contains if it has any,
/// with an optional text range if the instruction was created from an explicit `return`
fn inspect_block(
    outgoing_edges: &mut BlockMap<RoaringBitmap>,
    incoming_edges: &mut BlockMap<RoaringBitmap>,
    this_blocks: &mut BlockMap<JsThisExpression>,
    super_blocks: &mut BlockMap<JsSuperExpression>,
    return_blocks: &mut BlockMap<Option<TextRange>>,
    block: &BasicBlock,
    block_index: u32,
) -> Option<RuleState> {
    let mut has_this = None;
    let mut has_super: Option<(usize, JsSuperExpression)> = None;
    let mut has_return = None;

    // Iterator over all the instructions in the block
    for (inst_index, inst) in block.instructions.iter().enumerate() {
        // If the instruction has a corresponding node, visit its descendants
        // to detect any `super()` or `this` expression
        if let Some(node) = inst.node.as_ref().and_then(JsSyntaxElement::as_node) {
            let mut iter = node.preorder();
            while let Some(event) = iter.next() {
                let node = match event {
                    WalkEvent::Enter(node) => {
                        if AnyJsControlFlowRoot::can_cast(node.kind()) {
                            iter.skip_subtree();
                            continue;
                        }

                        node
                    }
                    WalkEvent::Leave(_) => continue,
                };

                // If we find a `super()` node but the block already has one, exit with an error immediately
                if let Some(super_node) = JsSuperExpression::cast_ref(&node) {
                    if let Some((prev_index, prev_super)) = &has_super {
                        if *prev_index < inst_index {
                            return Some(RuleState::DuplicateSuper {
                                first: prev_super.syntax().text_trimmed_range(),
                                second: super_node.syntax().text_trimmed_range(),
                            });
                        }
                    } else {
                        has_super = Some((inst_index, super_node));
                    }
                }

                has_this = has_this.or_else(|| {
                    let node = JsThisExpression::cast_ref(&node)?;
                    Some((inst_index, node))
                });
            }
        }

        match inst.kind {
            InstructionKind::Statement => {}

            // If the instruction is a jump, stores the metadata about this edge
            // and stop analyzing the block if its unconditional
            InstructionKind::Jump {
                block, conditional, ..
            } => {
                outgoing_edges
                    .entry(block_index)
                    .get_or_insert_with(RoaringBitmap::default)
                    .insert(block.index());

                incoming_edges
                    .entry(block.index())
                    .get_or_insert_with(RoaringBitmap::default)
                    .insert(block_index);

                if !conditional {
                    break;
                }
            }

            // If the instruction is a return, store its optional text range and stop analyzing the block
            InstructionKind::Return => {
                if let Some(node) = &inst.node {
                    if !JsThrowStatement::can_cast(node.kind()) {
                        has_return = Some(Some(node.text_trimmed_range()));
                    }
                } else {
                    has_return = Some(None);
                }
                break;
            }
        }
    }

    // If the block has a `super()` node and at least one `this` expression,
    // check that the first `this` node comes after the call to `super()`
    //
    // NOTE: The CFG has no representation of control flow within expressions
    // at the moment, meaning the ordering of `super()` and `this` within the
    // same expression statement is *NOT* checked (for instance the statement
    // `this.value && super();` is allowed)
    if let (Some((this_index, this_node)), Some((super_index, super_node))) =
        (&has_this, &has_super)
    {
        if this_index < super_index {
            return Some(RuleState::ThisBeforeSuper {
                this: this_node.syntax().text_trimmed_range(),
                super_: super_node.syntax().text_trimmed_range(),
            });
        }
    }

    if let Some((_, node)) = has_this {
        this_blocks.insert(block_index, node);
    }
    if let Some((_, node)) = has_super {
        super_blocks.insert(block_index, node);
    }
    if let Some(return_range) = has_return {
        return_blocks.insert(block_index, return_range);
    }

    None
}

/// Fast implementation of `Map<u32, T>` backed by a vector
struct BlockMap<T> {
    storage: Vec<Option<T>>,
}

impl<T> Default for BlockMap<T> {
    fn default() -> Self {
        Self {
            storage: Vec::new(),
        }
    }
}

impl<T> BlockMap<T> {
    /// Insert `value` into the map at the position `key`
    ///
    /// If the map did not have this key present, None is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old value is returned.
    fn insert(&mut self, key: u32, value: T) -> Option<T> {
        let index = usize::try_from(key).expect("integer overflow");

        if self.storage.len() <= index {
            self.storage.resize_with(index + 1, || None);
        }

        self.storage[index].replace(value)
    }

    /// Gets the given key’s corresponding entry in the map for in-place manipulation.
    fn entry(&mut self, key: u32) -> &mut Option<T> {
        let index = usize::try_from(key).expect("integer overflow");

        if self.storage.len() <= index {
            self.storage.resize_with(index + 1, || None);
        }

        &mut self.storage[index]
    }

    /// Returns a reference to the value corresponding to the key.
    fn get(&self, key: u32) -> Option<&T> {
        let index = usize::try_from(key).expect("integer overflow");
        self.storage.get(index)?.as_ref()
    }

    /// Returns true if the map contains a value for the specified key.
    fn contains_key(&self, key: u32) -> bool {
        self.get(key).is_some()
    }

    /// Returns the number of elements in the map.
    fn len(&self) -> u64 {
        self.values().count().try_into().expect("integer overflow")
    }

    /// An iterator visiting all values in the map, sorted by their key in ascending order
    fn values(&self) -> impl Iterator<Item = &T> {
        self.storage.iter().filter_map(Option::as_ref)
    }
}

impl<'a, T> IntoIterator for &'a BlockMap<T> {
    type Item = (u32, &'a T);
    type IntoIter = iter::FilterMap<
        iter::Enumerate<slice::Iter<'a, Option<T>>>,
        fn((usize, &Option<T>)) -> Option<(u32, &T)>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.storage.iter().enumerate().filter_map(|(index, slot)| {
            Some((index.try_into().expect("integer overflow"), slot.as_ref()?))
        })
    }
}
