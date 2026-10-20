use crate::services::semantic::Semantic;
use crate::utils::same_reference::is_same_reference;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_control_flow::{ExceptionHandlerKind, InstructionKind, builder::BlockId};
use biome_db::AnyParsedSource;
use biome_diagnostics::Severity;
use biome_js_control_flow::{
    AnyJsControlFlowRoot, JsControlFlowGraph, control_flow_model, js_control_flow_model,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBindingPattern, AnyJsClass, AnyJsExpression,
    AnyJsForInOrOfInitializer, AnyJsFunction, AnyJsMemberExpression, AnyTsName, AnyTsType,
    JsCallExpression, JsConditionalExpression, JsExpressionStatement, JsForInStatement,
    JsForOfStatement, JsGetterObjectMember, JsIdentifierBinding, JsLanguage, JsMethodObjectMember,
    JsReferenceIdentifier, JsSetterObjectMember, JsSyntaxNode, JsVariableDeclarator, T,
};
use biome_languages::LanguageDb;
use biome_rowan::{AstNode, AstSeparatedList, NodeOrToken, SyntaxKindSet, WalkEvent};
use biome_rule_options::no_loop_iterable_mutation::NoLoopIterableMutationOptions;
use roaring::RoaringBitmap;
use rustc_hash::FxHashMap;
use std::cell::OnceCell;
use std::rc::Rc;

declare_lint_rule! {
    /// Disallow changing a collection while a `for...of` loop iterates over it.
    ///
    /// When a `for...of` loop goes through an array, a `Set`, or a `Map`, adding, removing,
    /// or reordering its items during the loop can make the loop skip items, visit items more
    /// than once, or never end.
    ///
    /// This rule reports calls that change the collection inside a loop over that same
    /// collection, such as `push()`, `splice()`, `sort()`, `add()`, `delete()`, `set()`, and
    /// `clear()`. Loops over `.keys()`, `.values()`, and `.entries()` of the collection are
    /// checked too.
    ///
    /// Some changes don't affect which items the loop visits, so they are allowed:
    ///
    /// - Removing the current item from a `Set` or the current key from a `Map`.
    /// - Adding the current item to a `Set`, or setting the value of the current key in a
    ///   `Map`, since the collection already contains it. If the item was removed earlier in
    ///   the same pass of the loop, adding it back is reported, because the loop then visits it
    ///   again.
    ///
    /// These changes are only allowed when the loop variable is declared with `const`. A
    /// variable declared with `let` can be changed to hold a different item, so every change
    /// is reported in those loops.
    ///
    /// Code inside functions and classes created in the loop isn't checked, since it may run
    /// after the loop ends.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (const item of items) {
    ///     items.push(item.clone());
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (const value of values) {
    ///     values.shift();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (const value of set) {
    ///     set.delete(value);
    ///     set.add(value);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// for (const item of [...items]) {
    ///     items.push(item.clone());
    /// }
    ///
    /// for (const key of Object.keys(object)) {
    ///     delete object[key];
    /// }
    ///
    /// for (const value of set) {
    ///     set.add(value);
    /// }
    ///
    /// for (const key of map.keys()) {
    ///     map.set(key, newValue);
    /// }
    ///
    /// for (const [key] of map) {
    ///     map.delete(key);
    /// }
    /// ```
    ///
    pub NoLoopIterableMutation {
        version: "next",
        name: "noLoopIterableMutation",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-loop-iterable-mutation").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

/// A call that changes the collection of a loop that contains it.
pub struct Mutation {
    call: JsCallExpression,
    /// The innermost loop over the changed collection that reports the call.
    for_of: JsForOfStatement,
}

impl Rule for NoLoopIterableMutation {
    type Query = Semantic<AnyJsControlFlowRoot>;
    type State = Mutation;
    type Signals = Box<[Self::State]>;
    type Options = NoLoopIterableMutationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let mut deletes = CurrentDeletes::new(ctx);
        let mut mutations = Vec::new();
        let mut events = root.syntax().preorder();
        while let Some(event) = events.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if &node != root.syntax() && AnyJsControlFlowRoot::KIND_SET.matches(node.kind()) {
                // Nested functions are queried separately, with their own control flow graph.
                events.skip_subtree();
            } else if let Some(call) = JsCallExpression::cast(node)
                && let Some(for_of) = mutated_loop(ctx.model(), &call, &mut deletes)
            {
                mutations.push(Mutation { call, for_of });
            }
        }
        mutations.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, mutation: &Self::State) -> Option<RuleDiagnostic> {
        let Mutation { call, for_of } = mutation;
        let member = callee_member(call)?;
        let range = match &member {
            AnyJsMemberExpression::JsStaticMemberExpression(member) => {
                member.member().ok()?.range()
            }
            AnyJsMemberExpression::JsComputedMemberExpression(member) => {
                member.member().ok()?.range()
            }
        };
        let object = member.object().ok()?.omit_type_wrappers()?;
        let object = object.syntax().text_trimmed();
        let iterable = for_of.expression().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This changes "<Emphasis>{format_args!("{object}")}</Emphasis>" while a loop is iterating over it."
                },
            )
            .detail(
                iterable.range(),
                markup! { "The loop iterates over it here." },
            )
            .note(markup! {
                "Adding, removing, or reordering items during the loop can make it skip items, visit items more than once, or never end."
            })
            .note(markup! {
                "Loop over a copy, such as "<Emphasis>"[..."{format_args!("{}", iterable.syntax().text_trimmed())}"]"</Emphasis>", or make the changes after the loop."
            }),
        )
    }
}

/// Functions and classes created inside a loop may run after the loop ends.
const SCOPE_BOUNDARY_KINDS: SyntaxKindSet<JsLanguage> = AnyJsFunction::KIND_SET
    .union(AnyJsClass::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);

/// Returns the loop that `call` changes the collection of, if the change should be reported.
fn mutated_loop(
    model: &SemanticModel,
    call: &JsCallExpression,
    deletes: &mut CurrentDeletes,
) -> Option<JsForOfStatement> {
    let member = callee_member(call)?;
    let method = MutationMethod::from_name(member.member_name()?.text())?;
    let object = member.object().ok()?;

    // The innermost loop over the same collection. Calls that add its current item are
    // allowed in the outer loops too.
    let mut nested_loop = None;
    let mut child = call.syntax().clone();
    for ancestor in call.syntax().ancestors().skip(1) {
        if SCOPE_BOUNDARY_KINDS.matches(ancestor.kind()) {
            return None;
        }
        if let Some(for_of) = JsForOfStatement::cast_ref(&ancestor)
            && for_of.await_token().is_none()
            && for_of.body().is_ok_and(|body| body.syntax() == &child)
            && let Some(loop_info) = LoopInfo::from_loop(model, &for_of, &object)
        {
            if is_reported_mutation(
                model,
                call,
                method,
                &loop_info,
                nested_loop.as_ref(),
                deletes,
            ) {
                return Some(for_of);
            }
            nested_loop.get_or_insert(loop_info);
        }
        child = ancestor;
    }
    None
}

/// Kinds of collections that the rule can recognize from a variable declaration.
#[derive(Clone, Copy, Eq, PartialEq)]
enum CollectionKind {
    Array,
    Set,
    Map,
}

/// How a loop reads the collection it iterates over.
#[derive(Clone, Copy, Eq, PartialEq)]
enum IterationMethod {
    /// `for (const item of items)`
    Direct,
    /// `for (const key of items.keys())`
    Keys,
    /// `for (const value of items.values())`
    Values,
    /// `for (const [key, value] of items.entries())`
    Entries,
}

/// Methods that change an array, a `Set`, or a `Map`.
#[derive(Clone, Copy, Eq, PartialEq)]
enum MutationMethod {
    Add,
    Clear,
    CopyWithin,
    Delete,
    Fill,
    Pop,
    Push,
    Reverse,
    Set,
    Shift,
    Sort,
    Splice,
    Unshift,
}

impl MutationMethod {
    fn from_name(name: &str) -> Option<Self> {
        let index = MUTATION_METHODS
            .binary_search_by(|(method_name, _)| method_name.cmp(&name))
            .ok()?;
        Some(MUTATION_METHODS[index].1)
    }

    /// Returns whether the method changes a collection of the given kind. An unknown kind
    /// can be any collection.
    fn changes(self, kind: Option<CollectionKind>) -> bool {
        match (self, kind) {
            (_, None) => true,
            (Self::Add, Some(kind)) => kind == CollectionKind::Set,
            (Self::Clear | Self::Delete, Some(kind)) => kind != CollectionKind::Array,
            (Self::Set, Some(kind)) => kind == CollectionKind::Map,
            (_, Some(kind)) => kind == CollectionKind::Array,
        }
    }
}

/// Sorted by name, so it can be binary searched.
const MUTATION_METHODS: &[(&str, MutationMethod)] = &[
    ("add", MutationMethod::Add),
    ("clear", MutationMethod::Clear),
    ("copyWithin", MutationMethod::CopyWithin),
    ("delete", MutationMethod::Delete),
    ("fill", MutationMethod::Fill),
    ("pop", MutationMethod::Pop),
    ("push", MutationMethod::Push),
    ("reverse", MutationMethod::Reverse),
    ("set", MutationMethod::Set),
    ("shift", MutationMethod::Shift),
    ("sort", MutationMethod::Sort),
    ("splice", MutationMethod::Splice),
    ("unshift", MutationMethod::Unshift),
];

/// What the rule knows about a `for...of` loop over a collection.
struct LoopInfo {
    /// The collection, for example `items` in `for (const item of items.values())`.
    collection: AnyJsExpression,
    kind: Option<CollectionKind>,
    for_of: JsForOfStatement,
    /// The loop variable holding the current item, for which `add()` doesn't change the
    /// collection.
    current_item: Option<JsIdentifierBinding>,
    /// The loop variable holding the current key, for which `set()` doesn't change the
    /// collection.
    current_key: Option<JsIdentifierBinding>,
    /// The loop variable for which `delete()` removes the current item.
    current_deletable: Option<JsIdentifierBinding>,
}

impl LoopInfo {
    /// Returns the information about `for_of` if it iterates over the same collection as
    /// `object`.
    fn from_loop(
        model: &SemanticModel,
        for_of: &JsForOfStatement,
        object: &AnyJsExpression,
    ) -> Option<Self> {
        let (collection, method) = loop_collection(for_of.expression().ok()?)?;
        if !is_same_variable(model, object, &collection) {
            return None;
        }
        let kind = collection_kind(model, &collection);

        let binding = match for_of.initializer().ok()? {
            AnyJsForInOrOfInitializer::JsForVariableDeclaration(declaration)
                if declaration.is_const() =>
            {
                declaration.declarator().ok()?.id().ok()
            }
            _ => None,
        };
        let Some(binding) = binding else {
            return Some(Self {
                collection,
                kind,
                for_of: for_of.clone(),
                current_item: None,
                current_key: None,
                current_deletable: None,
            });
        };

        let current_item = match (method, kind) {
            (IterationMethod::Entries, _) => first_element_identifier(&binding),
            (IterationMethod::Direct, Some(CollectionKind::Map)) => None,
            _ => identifier(&binding),
        };
        let current_key = match (method, kind) {
            (IterationMethod::Keys, _) => identifier(&binding),
            (IterationMethod::Entries, _) => first_element_identifier(&binding),
            (IterationMethod::Direct, kind) if kind != Some(CollectionKind::Set) => {
                first_element_identifier(&binding)
            }
            _ => None,
        };
        let current_deletable = match (method, kind) {
            (IterationMethod::Direct | IterationMethod::Values, Some(CollectionKind::Set)) => {
                identifier(&binding)
            }
            // `map.delete(entry)` with the whole `[key, value]` pair removes nothing, so it
            // doesn't change the collection either.
            (IterationMethod::Direct | IterationMethod::Entries, _) => {
                identifier(&binding).or_else(|| first_element_identifier(&binding))
            }
            _ => current_key.clone(),
        };

        Some(Self {
            collection,
            kind,
            for_of: for_of.clone(),
            current_item,
            current_key,
            current_deletable,
        })
    }
}

/// Returns the member expression that `call` calls, like `items.push` in `items.push(item)`.
fn callee_member(call: &JsCallExpression) -> Option<AnyJsMemberExpression> {
    AnyJsMemberExpression::cast(call.callee().ok()?.omit_type_wrappers()?.into_syntax())
}

/// Returns the collection that a `for...of` loop iterates over, and how it reads it.
///
/// Returns `None` when the loop iterates over something else, such as a copy of the
/// collection or the result of a function call.
fn loop_collection(expression: AnyJsExpression) -> Option<(AnyJsExpression, IterationMethod)> {
    let expression = expression.omit_type_wrappers()?;
    if is_variable_or_property(&expression) {
        return Some((expression, IterationMethod::Direct));
    }

    let call = expression.as_js_call_expression()?;
    if call.is_optional_chain() || call.arguments().ok()?.args().len() > 0 {
        return None;
    }
    let member = AnyJsMemberExpression::cast(call.callee().ok()?.into_syntax())?;
    let method = match member.member_name()?.text() {
        "entries" => IterationMethod::Entries,
        "keys" => IterationMethod::Keys,
        "values" => IterationMethod::Values,
        _ => return None,
    };
    let collection = member.object().ok()?.omit_type_wrappers()?;
    is_variable_or_property(&collection).then_some((collection, method))
}

/// Returns whether `expression` reads a variable, `this`, or a property of one, without
/// calls or optional chaining (`?.`).
fn is_variable_or_property(expression: &AnyJsExpression) -> bool {
    let mut expression = expression.clone();
    loop {
        let Some(unwrapped) = expression.omit_type_wrappers() else {
            return false;
        };
        expression = match unwrapped {
            AnyJsExpression::JsIdentifierExpression(_) | AnyJsExpression::JsThisExpression(_) => {
                return true;
            }
            AnyJsExpression::JsStaticMemberExpression(member) if !member.is_optional() => {
                match member.object() {
                    Ok(object) => object,
                    Err(_) => return false,
                }
            }
            AnyJsExpression::JsComputedMemberExpression(member) if !member.is_optional() => {
                match member.object() {
                    Ok(object) => object,
                    Err(_) => return false,
                }
            }
            _ => return false,
        };
    }
}

/// Returns whether `left` and `right` read the same value, and every variable they read is
/// the same variable on both sides.
fn is_same_variable(
    model: &SemanticModel,
    left: &AnyJsExpression,
    right: &AnyJsExpression,
) -> bool {
    is_same_reference(left.clone(), right.clone()) == Some(true)
        && has_same_bindings(model, left, right).unwrap_or(false)
}

/// Returns whether the variables read by `left` and `right` resolve to the same
/// declarations, assuming the two expressions have the same shape.
fn has_same_bindings(
    model: &SemanticModel,
    left: &AnyJsExpression,
    right: &AnyJsExpression,
) -> Option<bool> {
    let left = left.clone().omit_type_wrappers()?;
    let right = right.clone().omit_type_wrappers()?;
    match (&left, &right) {
        (
            AnyJsExpression::JsIdentifierExpression(left),
            AnyJsExpression::JsIdentifierExpression(right),
        ) => {
            let left = model.binding(&left.name().ok()?);
            let right = model.binding(&right.name().ok()?);
            Some(match (left, right) {
                (Some(left), Some(right)) => left.syntax() == right.syntax(),
                (None, None) => true,
                _ => false,
            })
        }
        _ => {
            let (Some(left), Some(right)) = (
                AnyJsMemberExpression::cast_ref(left.syntax()),
                AnyJsMemberExpression::cast_ref(right.syntax()),
            ) else {
                return Some(true);
            };
            if !has_same_bindings(model, &left.object().ok()?, &right.object().ok()?)? {
                return Some(false);
            }
            // When only one key is computed, it's a literal that matches the other key, like
            // `"b"` in `a["b"]` and `a.b`, so it reads no variable.
            match (left, right) {
                (
                    AnyJsMemberExpression::JsComputedMemberExpression(left),
                    AnyJsMemberExpression::JsComputedMemberExpression(right),
                ) => has_same_bindings(model, &left.member().ok()?, &right.member().ok()?),
                _ => Some(true),
            }
        }
    }
}

/// Returns the kind of collection that `collection` holds, when its declaration shows it.
fn collection_kind(model: &SemanticModel, collection: &AnyJsExpression) -> Option<CollectionKind> {
    let reference = collection.as_js_identifier_expression()?.name().ok()?;
    let declaration = model.binding(&reference)?.tree().declaration()?;
    match declaration {
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            if let Some(annotation) = declarator.variable_annotation() {
                return type_collection_kind(model, annotation.type_annotation().ok()??.ty().ok()?);
            }
            if !declarator.declaration()?.is_const() {
                return None;
            }
            let initializer = declarator.initializer()?.expression().ok()?;
            match initializer.omit_type_wrappers()? {
                AnyJsExpression::JsArrayExpression(_) => Some(CollectionKind::Array),
                AnyJsExpression::JsNewExpression(new_expression) => {
                    let callee = new_expression.callee().ok()?;
                    global_collection_kind(
                        model,
                        &callee.as_js_identifier_expression()?.name().ok()?,
                    )
                }
                _ => None,
            }
        }
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => {
            type_collection_kind(model, parameter.type_annotation()?.ty().ok()?)
        }
        _ => None,
    }
}

/// Returns the kind of collection described by a type annotation, like `Set<string>` or
/// `string[]`.
fn type_collection_kind(model: &SemanticModel, ty: AnyTsType) -> Option<CollectionKind> {
    match ty.omit_parentheses() {
        AnyTsType::TsArrayType(_) => Some(CollectionKind::Array),
        AnyTsType::TsTypeOperatorType(operator)
            if operator.operator_token().ok()?.kind() == T![readonly] =>
        {
            type_collection_kind(model, operator.ty().ok()?)
        }
        AnyTsType::TsReferenceType(reference) => match reference.name().ok()? {
            AnyTsName::JsReferenceIdentifier(name) => global_collection_kind(model, &name),
            AnyTsName::TsQualifiedName(_) => None,
        },
        _ => None,
    }
}

/// Returns the collection kind of a built-in class or type name, like `Set` or
/// `ReadonlyMap`, unless a local declaration shadows it.
fn global_collection_kind(
    model: &SemanticModel,
    name: &JsReferenceIdentifier,
) -> Option<CollectionKind> {
    let kind = match name.value_token().ok()?.text_trimmed() {
        "Array" | "ReadonlyArray" => CollectionKind::Array,
        "Set" | "ReadonlySet" => CollectionKind::Set,
        "Map" | "ReadonlyMap" => CollectionKind::Map,
        _ => return None,
    };
    model.binding(name).is_none().then_some(kind)
}

/// Returns the variable declared by `pattern` if it is a plain identifier.
fn identifier(pattern: &AnyJsBindingPattern) -> Option<JsIdentifierBinding> {
    pattern
        .as_any_js_binding()?
        .as_js_identifier_binding()
        .cloned()
}

/// Returns `key` in `[key, value]`, if it has no default value.
fn first_element_identifier(pattern: &AnyJsBindingPattern) -> Option<JsIdentifierBinding> {
    let first = pattern
        .as_js_array_binding_pattern()?
        .elements()
        .iter()
        .next()?
        .ok()?;
    let AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(element) = first else {
        return None;
    };
    if element.init().is_some() {
        return None;
    }
    identifier(&element.pattern().ok()?)
}

/// Returns whether `call`, which calls `method` on the collection of `loop_info`, should be
/// reported.
///
/// `nested_loop` is a loop over the same collection that sits between the call and the loop
/// of `loop_info`. Adding its current item is allowed too.
fn is_reported_mutation(
    model: &SemanticModel,
    call: &JsCallExpression,
    method: MutationMethod,
    loop_info: &LoopInfo,
    nested_loop: Option<&LoopInfo>,
    deletes: &mut CurrentDeletes,
) -> bool {
    if !method.changes(loop_info.kind) {
        return false;
    }
    if method == MutationMethod::Delete
        && is_first_argument(model, call, loop_info.current_deletable.as_ref())
    {
        return false;
    }
    if is_current_add_or_set(model, call, method, loop_info)
        && !deletes.may_run_after_delete(model, call, loop_info)
    {
        return false;
    }
    !nested_loop.is_some_and(|nested_loop| is_current_add_or_set(model, call, method, nested_loop))
}

/// Returns whether `call` adds the current item or sets the current key again.
fn is_current_add_or_set(
    model: &SemanticModel,
    call: &JsCallExpression,
    method: MutationMethod,
    loop_info: &LoopInfo,
) -> bool {
    match method {
        MutationMethod::Add => is_first_argument(model, call, loop_info.current_item.as_ref()),
        MutationMethod::Set => is_first_argument(model, call, loop_info.current_key.as_ref()),
        _ => false,
    }
}

/// Returns whether the first argument of `call` reads the variable `binding`.
fn is_first_argument(
    model: &SemanticModel,
    call: &JsCallExpression,
    binding: Option<&JsIdentifierBinding>,
) -> bool {
    let Some(binding) = binding else {
        return false;
    };
    call.arguments()
        .ok()
        .and_then(|arguments| arguments.args().iter().next()?.ok())
        .and_then(|argument| {
            argument
                .as_any_js_expression()?
                .clone()
                .omit_type_wrappers()
        })
        .and_then(|argument| argument.as_js_identifier_expression()?.name().ok())
        .and_then(|reference| model.binding(&reference))
        .is_some_and(|declared| &declared.syntax() == binding.syntax())
}

/// Returns whether `call` is `collection.delete(current)`.
fn is_current_delete_call(
    model: &SemanticModel,
    call: &JsCallExpression,
    loop_info: &LoopInfo,
) -> bool {
    callee_member(call).is_some_and(|member| {
        member
            .member_name()
            .is_some_and(|name| name.text() == "delete")
            && member
                .object()
                .is_ok_and(|object| is_same_variable(model, &object, &loop_info.collection))
            && is_first_argument(model, call, loop_info.current_deletable.as_ref())
    })
}

/// Finds the code that can run after a loop deletes its current item, before the loop moves
/// to its next item, using the control flow graph of the queried function or module.
struct CurrentDeletes<'a> {
    ctx: &'a RuleContext<'a, NoLoopIterableMutation>,
    /// Built when a call first needs it, since most functions don't contain such calls.
    graph: OnceCell<Option<IndexedGraph>>,
    /// The result of [IndexedGraph::after_delete] for each loop.
    after_deletes: FxHashMap<JsForOfStatement, AfterDelete>,
}

impl<'a> CurrentDeletes<'a> {
    fn new(ctx: &'a RuleContext<'a, NoLoopIterableMutation>) -> Self {
        Self {
            ctx,
            graph: OnceCell::new(),
            after_deletes: FxHashMap::default(),
        }
    }

    /// Returns whether `call` can run after the current item of `loop_info` is deleted,
    /// before the loop moves to its next item.
    fn may_run_after_delete(
        &mut self,
        model: &SemanticModel,
        call: &JsCallExpression,
        loop_info: &LoopInfo,
    ) -> bool {
        let Self {
            ctx,
            graph,
            after_deletes,
        } = self;
        let Some(graph) = graph.get_or_init(|| IndexedGraph::new(ctx)) else {
            return false;
        };
        let Some(instruction) = graph.instruction_of(call) else {
            return false;
        };
        let after_delete = after_deletes
            .entry(loop_info.for_of.clone())
            .or_insert_with(|| graph.after_delete(model, loop_info));
        if after_delete.reached.contains(instruction) {
            return true;
        }
        let deletes = &after_delete.deletes;
        let first = deletes.partition_point(|(number, _)| *number < instruction);
        deletes[first..]
            .iter()
            .take_while(|(number, _)| *number == instruction)
            .any(|(_, delete)| runs_after_in_same_instruction(delete, call))
    }
}

/// The code that can run after a loop deletes its current item, before the loop moves to its
/// next item.
struct AfterDelete {
    /// The numbers of the instructions that can run after one of the deletes.
    reached: RoaringBitmap,
    /// Each delete call with the number of its instruction, sorted by that number.
    deletes: Vec<(u32, JsCallExpression)>,
}

/// A place where [IndexedGraph::after_delete] continues its search.
struct Visit {
    block: usize,
    /// The instruction to start from.
    start: usize,
    /// Set if the instruction at `start` contains a delete call. It's `true` if the
    /// instruction runs more code after that call, which can throw.
    delete: Option<bool>,
    /// Whether a `finally` clause in the path runs because the code is leaving its `try`
    /// statement.
    unwinding: bool,
}

/// A control flow graph that numbers all its instructions in order.
struct IndexedGraph {
    graph: JsControlFlowGraph,
    /// The number of the first instruction of each block.
    block_starts: Vec<u32>,
    /// The number of the instruction that evaluates each node.
    instructions: FxHashMap<JsSyntaxNode, u32>,
    /// For the first block of each loop, the number of the jump that enters the loop from the
    /// code before it.
    loop_entries: FxHashMap<u32, u32>,
}

impl IndexedGraph {
    fn new(ctx: &RuleContext<NoLoopIterableMutation>) -> Option<Self> {
        let root = ctx.query();
        let graph = if let Some(db) = ctx.get_service::<Rc<dyn LanguageDb>>()
            && let Some(source) = ctx.get_service::<AnyParsedSource>()
        {
            js_control_flow_model(db.as_ref(), source).graph(root)
        } else {
            // Fix passes and standalone callers analyze trees that are not stored in the
            // database.
            control_flow_model(&ctx.root()).graph(root)
        }?;

        let mut block_starts = Vec::with_capacity(graph.blocks.len());
        let mut instructions = FxHashMap::default();
        let mut loop_entries = FxHashMap::default();
        let mut number = 0;
        for (index, block) in graph.blocks.iter().enumerate() {
            block_starts.push(number);
            for instruction in &block.instructions {
                match &instruction.node {
                    Some(NodeOrToken::Node(node)) => {
                        instructions.entry(node.clone()).or_insert(number);
                    }
                    // Blocks are numbered in source order, so the jump that enters a loop is
                    // the only jump without a node that goes forward to the loop's first
                    // block. Jumps back to it come from the loop body.
                    None => {
                        if let InstructionKind::Jump {
                            conditional: false,
                            block: target,
                            ..
                        } = instruction.kind
                            && target.index() as usize > index
                        {
                            loop_entries.insert(target.index(), number);
                        }
                    }
                    Some(NodeOrToken::Token(_)) => {}
                }
                number += 1;
            }
        }
        Some(Self {
            graph,
            block_starts,
            instructions,
            loop_entries,
        })
    }

    /// Returns the number of the instruction that evaluates `call`.
    fn instruction_of(&self, call: &JsCallExpression) -> Option<u32> {
        let mut child = call.syntax().clone();
        for node in call.syntax().ancestors() {
            if let Some(number) = self.instructions.get(&node) {
                return Some(*number);
            }
            // Default values in a declared pattern run right after the initializer.
            if let Some(declarator) = JsVariableDeclarator::cast_ref(&node)
                && let Some(initializer) = declarator.initializer()
            {
                let expression = initializer.expression().ok()?;
                return self.instructions.get(expression.syntax()).copied();
            }
            // The parts of a nested loop that have no instruction of their own: the
            // collection runs once before the loop, and default values in the loop variable
            // run when the loop moves to an item.
            let parts = JsForOfStatement::cast_ref(&node)
                .and_then(|for_of| Some((for_of.initializer().ok()?, for_of.expression().ok()?)))
                .or_else(|| {
                    JsForInStatement::cast_ref(&node).and_then(|for_in| {
                        Some((for_in.initializer().ok()?, for_in.expression().ok()?))
                    })
                });
            if let Some((initializer, collection)) = parts {
                let header = *self.instructions.get(initializer.syntax())?;
                if collection.syntax() == &child {
                    let block = self.block_of(header) as u32;
                    return self.loop_entries.get(&block).copied();
                }
                return Some(header);
            }
            child = node;
        }
        None
    }

    /// Returns the index of the block that contains the instruction `number`.
    fn block_of(&self, number: u32) -> usize {
        self.block_starts
            .partition_point(|start| *start <= number)
            .saturating_sub(1)
    }

    /// Returns the code that can run after the current item of `loop_info` is deleted,
    /// before the loop moves to its next item.
    fn after_delete(&self, model: &SemanticModel, loop_info: &LoopInfo) -> AfterDelete {
        let mut after_delete = AfterDelete {
            reached: RoaringBitmap::new(),
            deletes: Vec::new(),
        };
        let Ok(body) = loop_info.for_of.body() else {
            return after_delete;
        };
        let body_range = body.syntax().text_trimmed_range();

        let mut first_deletes = FxHashMap::default();
        let mut events = body.syntax().preorder();
        while let Some(event) = events.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if SCOPE_BOUNDARY_KINDS.matches(node.kind()) {
                events.skip_subtree();
            } else if let Some(call) = JsCallExpression::cast(node)
                && is_current_delete_call(model, &call, loop_info)
                && let Some(number) = self.instruction_of(&call)
            {
                let block = self.block_of(number);
                let start = (number - self.block_starts[block]) as usize;
                let instruction_node = self.graph.blocks[block].instructions[start]
                    .node
                    .as_ref()
                    .and_then(NodeOrToken::as_node);
                // Only the code that the instruction evaluates after the call can throw.
                let is_whole_instruction = instruction_node.is_some_and(|node| {
                    node == call.syntax()
                        || JsExpressionStatement::cast_ref(node)
                            .and_then(|statement| statement.expression().ok()?.omit_type_wrappers())
                            .is_some_and(|expression| expression.syntax() == call.syntax())
                });
                // The search from the first delete of a block covers the later ones.
                let first = first_deletes.entry(block).or_insert((start, false));
                if start < first.0 {
                    *first = (start, false);
                }
                if start == first.0 {
                    first.1 |= !is_whole_instruction;
                }
                after_delete.deletes.push((number, call));
            }
        }
        after_delete
            .deletes
            .sort_unstable_by_key(|(number, _)| *number);
        let mut queue: Vec<_> = first_deletes
            .into_iter()
            .map(|(block, (start, throws_after_delete))| Visit {
                block,
                start,
                delete: Some(throws_after_delete),
                unwinding: false,
            })
            .collect();

        let mut entered = RoaringBitmap::new();
        'blocks: while let Some(visit) = queue.pop() {
            let data = &self.graph.blocks[visit.block];
            let mut enter = |target: BlockId, unwinding: bool| {
                if entered.insert(target.index() * 2 + u32::from(unwinding)) {
                    queue.push(Visit {
                        block: target.index() as usize,
                        start: 0,
                        delete: None,
                        unwinding,
                    });
                }
            };

            let mut can_throw = false;
            for (index, instruction) in data.instructions.iter().enumerate().skip(visit.start) {
                // Code outside the body runs after the loop ends or when it moves to its next
                // item, and leads back into the body only through the next item.
                let range = instruction.node.as_ref().map(|node| match node {
                    NodeOrToken::Node(node) => node.text_trimmed_range(),
                    NodeOrToken::Token(token) => token.text_trimmed_range(),
                });
                if range.is_some_and(|range| !body_range.contains_range(range)) {
                    continue 'blocks;
                }
                let throws = match visit.delete {
                    Some(throws_after_delete) if index == visit.start => throws_after_delete,
                    _ => {
                        after_delete
                            .reached
                            .insert(self.block_starts[visit.block] + index as u32);
                        true
                    }
                };
                match instruction.kind {
                    InstructionKind::Statement => can_throw |= throws,
                    InstructionKind::Return => {
                        can_throw |= throws;
                        break;
                    }
                    InstructionKind::Jump {
                        conditional,
                        block: target,
                        finally_fallthrough,
                    } => {
                        can_throw |= conditional && throws;
                        if finally_fallthrough && visit.unwinding {
                            // A `finally` clause that runs while leaving its `try` statement
                            // continues to the outer `finally` clauses, not to the code
                            // after the statement.
                            for handler in &data.cleanup_handlers {
                                enter(handler.target, true);
                            }
                        } else {
                            enter(target, visit.unwinding);
                        }
                        if !conditional {
                            break;
                        }
                    }
                }
            }
            if can_throw {
                for handler in &data.exception_handlers {
                    enter(
                        handler.target,
                        handler.kind == ExceptionHandlerKind::Finally,
                    );
                }
            }
            // `return`, `break`, and `continue` run the `finally` clauses of the `try`
            // statements they leave.
            for handler in &data.cleanup_handlers {
                enter(handler.target, true);
            }
        }
        after_delete
    }
}

/// Returns whether `call` runs after `delete` when both are in the same instruction, which
/// evaluates them from left to right.
fn runs_after_in_same_instruction(delete: &JsCallExpression, call: &JsCallExpression) -> bool {
    let call_range = call.syntax().text_trimmed_range();
    if delete.syntax().text_trimmed_range().end() > call_range.start() {
        return false;
    }
    // Only one branch of `condition ? a : b` runs.
    let Some(common) = delete
        .syntax()
        .ancestors()
        .find(|node| node.text_trimmed_range().contains_range(call_range))
    else {
        return true;
    };
    !JsConditionalExpression::cast(common).is_some_and(|conditional| {
        conditional
            .consequent()
            .is_ok_and(|consequent| consequent.range().contains_range(delete.range()))
    })
}

#[cfg(test)]
mod tests {
    use super::MUTATION_METHODS;

    #[test]
    fn mutation_methods_are_sorted() {
        for pair in MUTATION_METHODS.windows(2) {
            assert!(
                pair[0].0 < pair[1].0,
                "{} must come after {}",
                pair[0].0,
                pair[1].0
            );
        }
    }
}
