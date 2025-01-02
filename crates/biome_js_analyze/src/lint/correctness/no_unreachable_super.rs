use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_control_flow::{
    builder::{BlockId, ROOT_BLOCK_ID},
    ExceptionHandlerKind, InstructionKind,
};
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsClass, AnyJsExpression, JsCallExpression, JsConstructorClassMember, JsSyntaxKind,
    JsThrowStatement, TextRange, WalkEvent,
};
use biome_rowan::{AstNode, NodeOrToken};
use rustc_hash::FxHashSet;

use crate::services::control_flow::{AnyJsControlFlowRoot, ControlFlowGraph};

declare_lint_rule! {
    /// Ensures the `super()` constructor is called exactly once on every code  path in a class constructor before `this` is accessed if the class has a superclass
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
    pub NoUnreachableSuper {
        version: "1.0.0",
        name: "noUnreachableSuper",
        language: "js",
        sources: &[RuleSource::Eslint("no-this-before-super")],
        recommended: true,
        severity: Severity::Error,
    }
}

pub enum RuleState {
    /// The constructor may call `super` multiple times
    DuplicateSuper { first: TextRange, second: TextRange },
    /// The constructor may read or write from `this` without calling `super`
    ThisWithoutSuper { this: TextRange },
    /// The constructor may return without calling `super`
    ReturnWithoutSuper { return_statement: Option<TextRange> },
}

/// A [BlockContext] consists of a block id and a context.
/// The context registers if a super() call was seen.
/// This allows traversing a control flow graph and retaining contextual information.
#[derive(Debug, Copy, Clone)]
struct BlockContext {
    block_id: BlockId,
    super_call: Option<TextRange>,
}

impl Rule for NoUnreachableSuper {
    type Query = ControlFlowGraph;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let cfg = ctx.query();
        if !JsConstructorClassMember::can_cast(cfg.node.kind()) {
            // Ignore non-constructor functions
            return None;
        }
        // Find the class this constructor belongs to
        let class = cfg
            .node
            .ancestors()
            .skip(1) // skip constructor
            .find_map(AnyJsClass::cast)?;

        // Do not run the rule if the class has no extends clause or is extending a literal expression
        let extends_clause = class.extends_clause()?;
        let super_class = extends_clause.super_class().ok()?;
        if matches!(super_class, AnyJsExpression::AnyJsLiteralExpression(_)) {
            return None;
        }

        // We traverse the control flow graph and register in a context if we saw `super`.
        // The context follows the traversal of every path (we process [BlockContext]).
        //
        // A block may be visiter twice: one time with an empty context,
        // and another time with a `super` in its context.
        //
        // As soon as we detect the use of this without calling first `super`

        // stack of block contexts to process
        let mut block_context_stack = vec![BlockContext {
            block_id: ROOT_BLOCK_ID,
            super_call: None,
        }];
        // Set of couples (block_id, was_visited_with_super_call)
        let mut visited_block_contexts = FxHashSet::from_iter([(ROOT_BLOCK_ID, false)]);
        while let Some(BlockContext {
            block_id,
            super_call: mut super_,
        }) = block_context_stack.pop()
        {
            let had_super = super_.is_some();
            let block = cfg.get(block_id);
            for instruction in block.instructions.iter() {
                if let Some(NodeOrToken::Node(ref block_node)) = instruction.node {
                    let mut iter = block_node.preorder();
                    while let Some(event) = iter.next() {
                        let WalkEvent::Enter(node) = event else {
                            continue;
                        };
                        match node.kind() {
                            JsSyntaxKind::JS_SUPER_EXPRESSION => {
                                let Some(parent) = node.parent() else {
                                    continue;
                                };
                                if !JsCallExpression::can_cast(parent.kind()) {
                                    // Ignore `super.method()` calls
                                    continue;
                                }
                                let range = node.text_trimmed_range();
                                if let Some(first) = super_ {
                                    return Some(RuleState::DuplicateSuper {
                                        first,
                                        second: range,
                                    });
                                }
                                super_ = Some(range);
                            }
                            JsSyntaxKind::JS_THIS_EXPRESSION => {
                                if super_.is_none() {
                                    return Some(RuleState::ThisWithoutSuper {
                                        this: node.text_trimmed_range(),
                                    });
                                }
                            }
                            _ if AnyJsControlFlowRoot::can_cast(node.kind()) => {
                                iter.skip_subtree();
                            }
                            _ => {}
                        }
                    }
                }
                match instruction.kind {
                    InstructionKind::Statement => {}
                    InstructionKind::Jump {
                        block: jump_block_id,
                        conditional,
                        ..
                    } => {
                        // Avoid cycles and redundant checks.
                        if visited_block_contexts.insert((jump_block_id, super_.is_some())) {
                            block_context_stack.push(BlockContext {
                                block_id: jump_block_id,
                                super_call: super_,
                            });
                        }
                        if !conditional {
                            // The next instructions are unreachable.
                            break;
                        }
                    }
                    InstructionKind::Return => {
                        if super_.is_none() {
                            let return_ = instruction.node.as_ref();
                            // ignore Throws
                            if return_.is_some_and(|node| JsThrowStatement::can_cast(node.kind())) {
                                break;
                            }
                            let return_ = return_.map(|node| node.text_trimmed_range());
                            return Some(RuleState::ReturnWithoutSuper {
                                return_statement: return_,
                            });
                        }
                        // The next instructions are unreachable.
                        break;
                    }
                }
            }
            for exception_handler in block.exception_handlers.iter() {
                // Ignore finally handler: they are already in the Control Flow Graph.
                if matches!(exception_handler.kind, ExceptionHandlerKind::Catch) {
                    // First, assume that an exception was raised at the start of the block
                    if !had_super && super_.is_some() {
                        // Avoid cycles and redundant checks.
                        if visited_block_contexts.insert((exception_handler.target, had_super)) {
                            block_context_stack.push(BlockContext {
                                block_id: exception_handler.target,
                                super_call: None,
                            });
                        }
                    }
                    // Now, assume that an exception was raised at the end of the block
                    // Avoid cycles and redundant checks.
                    if visited_block_contexts.insert((exception_handler.target, super_.is_some())) {
                        block_context_stack.push(BlockContext {
                            block_id: exception_handler.target,
                            super_call: super_,
                        });
                    }
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
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
            RuleState::ReturnWithoutSuper { return_statement: Some(range) } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().node.text_trimmed_range(),
                    markup! { "This constructor has code paths that return without calling `"<Emphasis>"super()"</Emphasis>"` first." },
                )
                .detail(range, markup! { "This statement returns from the constructor before `"<Emphasis>"super()"</Emphasis>"` has been called:" })
                .note("If this is intentional, add an explicit throw statement in unsupported paths."),
            ),
            RuleState::ReturnWithoutSuper { return_statement: None } => Some(
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
