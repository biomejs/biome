use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsExpression, AnyJsFunction, AnyJsMemberExpression,
    AnyJsRoot, JsAssignmentExpression, JsCallExpression, JsComputedMemberAssignment,
    JsExpressionStatement, JsGetterClassMember, JsGetterObjectMember, JsLanguage,
    JsMethodClassMember, JsMethodObjectMember, JsModuleItemList, JsPropertyClassMember,
    JsPropertyObjectMember, JsSetterClassMember, JsSetterObjectMember, JsStatementList,
    JsStaticMemberAssignment, JsSyntaxKind, JsSyntaxNode, JsVariableDeclarator, T,
};
use biome_rowan::TriviaPieceKind;
use biome_rowan::{AstNode, BatchMutation, BatchMutationExt, Direction, SyntaxTriviaPiece};
use rustc_hash::FxHashSet;
use std::{cell::RefCell, sync::Arc};

struct HeaderTrivia {
    declaration: Vec<(TriviaPieceKind, String)>,
    first_item: Vec<SyntaxTriviaPiece<JsLanguage>>,
}

#[derive(Clone)]
pub(crate) struct ModuleConstantFacts {
    occupied_reference_names: Arc<FxHashSet<String>>,
    has_unbound_direct_eval: bool,
}

struct CachedModuleConstantFacts {
    root: JsSyntaxNode,
    facts: ModuleConstantFacts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ModuleConstantNameKind {
    Binding,
    Function,
    Property,
    Member,
    Method,
    CallCallee,
}

pub(crate) struct ModuleConstantNameCandidate {
    pub(crate) kind: ModuleConstantNameKind,
    pub(crate) name: String,
}

/// Returns readable naming contexts found while walking from `node` toward its root.
///
/// Candidates are emitted in ancestor order. Within one ancestor, bindings, functions, methods,
/// properties, members, and call callees are emitted in that order.
pub(crate) fn module_constant_name_candidates(
    node: &JsSyntaxNode,
) -> Vec<ModuleConstantNameCandidate> {
    let mut candidates = Vec::new();

    for ancestor in node.ancestors().skip(1) {
        if let Some(declarator) = JsVariableDeclarator::cast(ancestor.clone())
            && let Some(binding) = declarator
                .id()
                .ok()
                .and_then(|pattern| pattern.as_any_js_binding().cloned())
            && let Some(name) = module_constant_binding_name(&binding)
        {
            candidates.push(ModuleConstantNameCandidate {
                kind: ModuleConstantNameKind::Binding,
                name,
            });
        }

        if let Some(function) = AnyJsFunction::cast(ancestor.clone())
            && let Some(name) = module_constant_function_name(&function)
        {
            candidates.push(ModuleConstantNameCandidate {
                kind: ModuleConstantNameKind::Function,
                name,
            });
        }

        if let Some(name) = module_constant_method_name(&ancestor) {
            candidates.push(ModuleConstantNameCandidate {
                kind: ModuleConstantNameKind::Method,
                name,
            });
        }

        if let Some(name) = module_constant_property_name(&ancestor) {
            candidates.push(ModuleConstantNameCandidate {
                kind: ModuleConstantNameKind::Property,
                name,
            });
        }

        if let Some(name) = module_constant_member_name(&ancestor) {
            candidates.push(ModuleConstantNameCandidate {
                kind: ModuleConstantNameKind::Member,
                name,
            });
        }

        if let Some(call) = JsCallExpression::cast(ancestor)
            && let Some(name) = call
                .callee()
                .ok()
                .and_then(|callee| callee.get_callee_member_name())
                .map(|token| token.text_trimmed().to_string())
        {
            candidates.push(ModuleConstantNameCandidate {
                kind: ModuleConstantNameKind::CallCallee,
                name,
            });
        }
    }

    candidates
}

/// Returns the identifier text for a simple binding pattern.
pub(crate) fn module_constant_binding_name(binding: &AnyJsBinding) -> Option<String> {
    binding
        .as_js_identifier_binding()?
        .name_token()
        .ok()
        .map(|token| token.text_trimmed().to_string())
}

/// Returns the declared name of a named function.
pub(crate) fn module_constant_function_name(function: &AnyJsFunction) -> Option<String> {
    let binding = match function {
        AnyJsFunction::JsFunctionDeclaration(function) => function.id().ok(),
        AnyJsFunction::JsFunctionExportDefaultDeclaration(function) => function.id(),
        AnyJsFunction::JsFunctionExpression(function) => function.id(),
        AnyJsFunction::JsArrowFunctionExpression(_) => None,
    }?;
    module_constant_binding_name(&binding)
}

/// Returns a readable object or class method name.
pub(crate) fn module_constant_method_name(node: &JsSyntaxNode) -> Option<String> {
    if let Some(method) = JsMethodObjectMember::cast(node.clone()) {
        return method.name().ok()?.name().map(|name| name.to_string());
    }
    if let Some(method) = JsGetterObjectMember::cast(node.clone()) {
        return method.name().ok()?.name().map(|name| name.to_string());
    }
    if let Some(method) = JsSetterObjectMember::cast(node.clone()) {
        return method.name().ok()?.name().map(|name| name.to_string());
    }
    if let Some(method) = JsMethodClassMember::cast(node.clone()) {
        return method
            .name()
            .ok()?
            .name()
            .map(|name| name.text().to_string());
    }
    if let Some(method) = JsGetterClassMember::cast(node.clone()) {
        return method
            .name()
            .ok()?
            .name()
            .map(|name| name.text().to_string());
    }
    if let Some(method) = JsSetterClassMember::cast(node.clone()) {
        return method
            .name()
            .ok()?
            .name()
            .map(|name| name.text().to_string());
    }
    None
}

/// Returns the readable name of an object or class property.
pub(crate) fn module_constant_property_name(node: &JsSyntaxNode) -> Option<String> {
    if let Some(property) = JsPropertyObjectMember::cast(node.clone()) {
        return property.name().ok()?.name().map(|name| name.to_string());
    }
    if let Some(property) = JsPropertyClassMember::cast(node.clone()) {
        return property
            .name()
            .ok()?
            .name()
            .map(|name| name.text().to_string());
    }
    None
}

/// Returns the readable name of a member expression or member assignment.
pub(crate) fn module_constant_member_name(node: &JsSyntaxNode) -> Option<String> {
    if let Some(assignment) = JsAssignmentExpression::cast(node.clone()) {
        return module_constant_member_name(&assignment.left().ok()?.into_syntax());
    }
    if let Some(member) = AnyJsMemberExpression::cast(node.clone()) {
        return member.member_name().map(|name| name.text().to_string());
    }
    if let Some(member) = JsStaticMemberAssignment::cast(node.clone()) {
        return member
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()
            .map(|token| token.text_trimmed().to_string());
    }
    if let Some(member) = JsComputedMemberAssignment::cast(node.clone()) {
        return member
            .member()
            .ok()?
            .as_static_value()
            .map(|value| value.text().to_string());
    }
    None
}

/// Converts text into an uppercase identifier component.
pub(crate) fn normalize_module_constant_name_component(
    text: &str,
    ensure_identifier_start: bool,
) -> Option<String> {
    let mut normalized = String::new();
    let mut previous_is_lowercase = false;

    for character in text.chars() {
        if character.is_ascii_alphanumeric() {
            if character.is_ascii_uppercase() && previous_is_lowercase {
                normalized.push('_');
            }
            normalized.push(character.to_ascii_uppercase());
            previous_is_lowercase = character.is_ascii_lowercase();
        } else {
            if !normalized.ends_with('_') {
                normalized.push('_');
            }
            previous_is_lowercase = false;
        }
    }

    let normalized = normalized.trim_matches('_');
    if normalized.is_empty() {
        return None;
    }

    // Numeric context names need a prefix before they can participate in a binding name.
    if ensure_identifier_start
        && normalized
            .as_bytes()
            .first()
            .is_some_and(|character| character.is_ascii_digit())
    {
        Some(format!("NUMBER_{normalized}"))
    } else {
        Some(normalized.to_string())
    }
}

/// Builds a stable uppercase name from the syntax surrounding a runtime number.
pub(crate) fn module_constant_numeric_name(target: &JsSyntaxNode, literal_text: &str) -> String {
    let mut variable = None;
    let mut property = None;
    let mut function = None;
    let mut call = None;

    for candidate in module_constant_name_candidates(target) {
        match candidate.kind {
            ModuleConstantNameKind::Binding if variable.is_none() => {
                variable = Some(candidate.name)
            }
            ModuleConstantNameKind::Property | ModuleConstantNameKind::Member
                if property.is_none() =>
            {
                property = Some(candidate.name)
            }
            ModuleConstantNameKind::Function if function.is_none() => {
                function = Some(candidate.name)
            }
            ModuleConstantNameKind::CallCallee if call.is_none() => call = Some(candidate.name),
            ModuleConstantNameKind::Binding
            | ModuleConstantNameKind::Function
            | ModuleConstantNameKind::Property
            | ModuleConstantNameKind::Member
            | ModuleConstantNameKind::Method
            | ModuleConstantNameKind::CallCallee => {}
        }
    }

    let mut parts = Vec::new();
    for context in [variable, property, function, call].into_iter().flatten() {
        if let Some(normalized) = normalize_module_constant_name_component(&context, true)
            && !parts.contains(&normalized)
        {
            parts.push(normalized);
        }
    }

    let value = normalize_module_constant_name_component(literal_text, false)
        .unwrap_or_else(|| "NUMBER".to_string());
    if parts.is_empty() {
        format!("NUMBER_{value}")
    } else {
        parts.push(value);
        parts.join("_")
    }
}

/// Builds a stable uppercase name for a regular-expression constant from its enclosing context.
pub(crate) fn module_constant_regex_name(target: &JsSyntaxNode, pattern: &str) -> String {
    for candidate in module_constant_name_candidates(target) {
        if matches!(
            candidate.kind,
            ModuleConstantNameKind::Binding
                | ModuleConstantNameKind::Function
                | ModuleConstantNameKind::Method
                | ModuleConstantNameKind::Property
        ) && let Some(name) = normalize_module_constant_name_component(&candidate.name, true)
        {
            return format!("{name}_REGEX");
        }
    }

    let pattern = normalize_module_constant_name_component(pattern, false);
    pattern.map_or_else(|| "REGEX".to_string(), |pattern| format!("REGEX_{pattern}"))
}

thread_local! {
    // Rule actions for one file share this bounded cache; replacing the entry keeps memory use
    // independent of the number of files analyzed on a worker thread.
    static MODULE_CONSTANT_FACTS: RefCell<Option<CachedModuleConstantFacts>> = const { RefCell::new(None) };
}

/// Returns the facts shared by module-constant extraction checks for `root`.
///
/// The facts are cached per thread and replaced when the syntax root changes.
pub(crate) fn module_constant_facts(
    root: &AnyJsRoot,
    model: &SemanticModel,
) -> ModuleConstantFacts {
    let root_syntax = root.syntax();
    MODULE_CONSTANT_FACTS.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(cached) = cache
            .as_ref()
            .filter(|cached| cached.root == root_syntax.clone())
        {
            return cached.facts.clone();
        }

        let facts = ModuleConstantFacts {
            occupied_reference_names: Arc::new(occupied_reference_names(model)),
            has_unbound_direct_eval: has_unbound_direct_eval(root, model),
        };
        *cache = Some(CachedModuleConstantFacts {
            root: root_syntax.clone(),
            facts: facts.clone(),
        });
        facts
    })
}

/// Builds a mutation that replaces `target` with a module-level constant reference.
///
/// Returns `None` when the target cannot be safely extracted or the generated declaration cannot be
/// placed in the root. The candidate name is checked against bindings and references in scope.
pub(crate) fn extract_module_constant(
    root: &AnyJsRoot,
    model: &SemanticModel,
    target: &JsSyntaxNode,
    value: AnyJsExpression,
    candidate_name: &str,
) -> Option<(BatchMutation<JsLanguage>, String)> {
    extract_module_constant_with_reserved_names(
        root,
        model,
        target,
        value,
        candidate_name,
        &FxHashSet::default(),
        true,
    )
}

/// Builds a constant-extraction mutation while excluding names already reserved by sibling fixes.
///
/// The mutation replaces the target, inserts a `const` declaration at the earliest safe slot, and
/// optionally transfers file-header trivia to that declaration. It returns `None` for unsupported
/// roots, unsafe dynamic scopes, ambiguous header trivia, or unavailable insertion points.
pub(crate) fn extract_module_constant_with_reserved_names(
    root: &AnyJsRoot,
    model: &SemanticModel,
    target: &JsSyntaxNode,
    value: AnyJsExpression,
    candidate_name: &str,
    reserved_names: &FxHashSet<String>,
    transfer_header: bool,
) -> Option<(BatchMutation<JsLanguage>, String)> {
    let facts = module_constant_facts(root, model);
    if !is_module_constant_extractable_with_facts(root, target, &facts) {
        return None;
    }

    let list = root_list(root)?;
    let top_level_item = target
        .ancestors()
        .find(|ancestor| ancestor.parent().is_some_and(|parent| parent == list))?;

    let name = collision_free_name(
        model,
        target,
        candidate_name,
        &facts.occupied_reference_names,
        reserved_names,
    );
    let insertion_slot = insertion_slot(&list, top_level_item.index())?;
    let first_token = if insertion_slot == 0 && transfer_header {
        Some(list.first_token()?)
    } else {
        None
    };
    let target_starts_with_first_token = first_token.as_ref().is_some_and(|first_token| {
        target
            .first_token()
            .is_some_and(|target_token| target_token == first_token.clone())
    });
    let replacement =
        make::js_identifier_expression(make::js_reference_identifier(make::ident(&name)))
            .into_syntax();
    let replacement = preserve_target_trivia(target, replacement, !target_starts_with_first_token)?;
    let line_ending = source_line_ending(&list);
    let value = trim_expression_trivia(value)?;
    let header_trivia = if insertion_slot == 0 && transfer_header {
        Some(split_header_trivia(&list)?)
    } else {
        None
    };
    let declaration = make_declaration(
        &name,
        value,
        header_trivia
            .as_ref()
            .map_or(&[][..], |trivia| trivia.declaration.as_slice()),
        header_trivia
            .as_ref()
            .map_or(&[][..], |trivia| trivia.first_item.as_slice()),
        insertion_slot == 0,
        line_ending,
    );

    let mut mutation = root.clone().begin();
    mutation.replace_element_discard_trivia(target.clone().into(), replacement.into());
    if let Some(first_token) = first_token.filter(|_| !target_starts_with_first_token) {
        // Header trivia belongs to the generated declaration; clear it from the actual first
        // item so attached comments are not moved or duplicated.
        mutation.clear_leading_trivia(first_token);
    }
    if insertion_slot == 0 && transfer_header {
        mutation.insert_element_with_header(list, insertion_slot, declaration.into());
    } else {
        mutation.insert_element(list, insertion_slot, declaration.into());
    }

    Some((mutation, name))
}

/// Chooses a name that is free in the target scopes and absent from known references and reservations.
///
/// If `candidate_name` is occupied, numeric suffixes starting at `_2` are tried until a free name is
/// found.
pub(crate) fn collision_free_module_constant_name_with_facts(
    model: &SemanticModel,
    target: &JsSyntaxNode,
    candidate_name: &str,
    reserved_names: &FxHashSet<String>,
    facts: &ModuleConstantFacts,
) -> String {
    collision_free_name(
        model,
        target,
        candidate_name,
        &facts.occupied_reference_names,
        reserved_names,
    )
}

/// Returns the top-level item list that can receive a generated runtime binding.
///
/// Expression and declaration-only roots return `None` because they have no mutable module or
/// script statement list.
fn root_list(root: &AnyJsRoot) -> Option<JsSyntaxNode> {
    // Expression snippets and declaration-only roots cannot receive a runtime binding.
    let list = match root {
        AnyJsRoot::JsModule(module) => module.items().into_syntax(),
        AnyJsRoot::JsScript(script) => script.statements().into_syntax(),
        _ => return None,
    };

    (JsModuleItemList::can_cast(list.kind()) || JsStatementList::can_cast(list.kind()))
        .then_some(list)
}

/// Returns `candidate_name` or the first suffixed name free from scope and reference collisions.
fn collision_free_name(
    model: &SemanticModel,
    target: &JsSyntaxNode,
    candidate_name: &str,
    occupied_reference_names: &FxHashSet<String>,
    reserved_names: &FxHashSet<String>,
) -> String {
    if name_is_free_in_target_scopes(
        model,
        target,
        candidate_name,
        occupied_reference_names,
        reserved_names,
    ) {
        return candidate_name.to_string();
    }

    let mut suffix = 2;
    loop {
        let name = format!("{candidate_name}_{suffix}");
        if name_is_free_in_target_scopes(
            model,
            target,
            &name,
            occupied_reference_names,
            reserved_names,
        ) {
            return name;
        }
        suffix += 1;
    }
}

/// Checks whether `name` is free in every scope enclosing `target` and absent from known references.
fn name_is_free_in_target_scopes(
    model: &SemanticModel,
    target: &JsSyntaxNode,
    name: &str,
    occupied_reference_names: &FxHashSet<String>,
    reserved_names: &FxHashSet<String>,
) -> bool {
    model
        .scope(target)
        .ancestors()
        .all(|scope| scope.get_binding(name).is_none())
        && !occupied_reference_names.contains(name)
        && !reserved_names.contains(name)
}

/// Collects names used by unresolved and configured-global references in the semantic model.
///
/// These names must not be captured by a generated top-level binding.
fn occupied_reference_names(model: &SemanticModel) -> FxHashSet<String> {
    let mut names = FxHashSet::default();
    for reference in model.all_unresolved_references() {
        if let Some(token) = reference.syntax().first_token() {
            names.insert(token.text_trimmed().to_string());
        }
    }
    for reference in model.all_global_references() {
        if let Some(token) = reference.syntax().first_token() {
            names.insert(token.text_trimmed().to_string());
        }
    }
    names
}

/// Reports whether an unbound direct `eval` can observe or introduce a generated binding.
///
/// Any such call makes extraction unsafe because its runtime scope effects cannot be determined
/// statically.
fn has_unbound_direct_eval(root: &AnyJsRoot, model: &SemanticModel) -> bool {
    root.syntax()
        .descendants()
        .filter_map(JsCallExpression::cast)
        .any(|call| {
            // A direct eval anywhere in the accepted root can observe a newly inserted
            // top-level binding, including from a sibling or nested function.
            is_direct_eval(model, &call)
        })
}

/// Checks whether `call` invokes the unbound identifier named `eval` directly.
///
/// Parenthesized callees still count as direct calls; property calls and bound identifiers do not.
fn is_direct_eval(model: &SemanticModel, call: &JsCallExpression) -> bool {
    let Some(reference) = call
        .callee()
        .ok()
        .map(|callee| callee.omit_parentheses())
        .and_then(|callee| callee.as_js_reference_identifier())
    else {
        return false;
    };

    reference
        .value_token()
        .is_ok_and(|token| token.text_trimmed() == "eval")
        && model.binding(&reference).is_none()
}

/// Copies the target's selected leading and trailing trivia onto a replacement node.
///
/// Leading trivia is copied only when requested; any invalid trivia attachment returns `None`.
fn preserve_target_trivia(
    target: &JsSyntaxNode,
    replacement: JsSyntaxNode,
    preserve_leading_trivia: bool,
) -> Option<JsSyntaxNode> {
    let replacement = if preserve_leading_trivia {
        if let Some(trivia) = target.first_leading_trivia() {
            replacement.with_leading_trivia_pieces(trivia.pieces())?
        } else {
            replacement
        }
    } else {
        replacement
    };

    if let Some(trivia) = target.last_trailing_trivia() {
        replacement.with_trailing_trivia_pieces(trivia.pieces())
    } else {
        Some(replacement)
    }
}

/// Removes leading and trailing trivia from an expression before embedding it in a declaration.
///
/// Returns `None` if the trivia edits no longer produce a valid expression node.
fn trim_expression_trivia(value: AnyJsExpression) -> Option<AnyJsExpression> {
    let value = value
        .into_syntax()
        .with_leading_trivia_pieces([])?
        .with_trailing_trivia_pieces([])?;

    AnyJsExpression::cast(value)
}

/// Separates file-header trivia from trivia attached to the first top-level item.
///
/// Returns `None` when comments occur after the first blank-line separator, because moving those
/// comments with a new declaration could change their attachment.
fn split_header_trivia(list: &JsSyntaxNode) -> Option<HeaderTrivia> {
    let first_item = list.first_child()?;
    let pieces = first_item
        .first_leading_trivia()
        .map(|trivia| trivia.pieces().collect::<Vec<_>>())
        .unwrap_or_default();
    let split_index = if pieces.iter().any(|piece| piece.kind().is_comment()) {
        find_first_blank_line_index(&pieces)?
    } else {
        pieces.len()
    };
    let first_item = pieces[split_index..].to_vec();
    if first_item.iter().any(|piece| piece.kind().is_comment()) {
        // Comments after the header separator belong to the original item and cannot safely be
        // relocated through a slot-zero insertion.
        return None;
    }

    Some(HeaderTrivia {
        declaration: pieces[..split_index]
            .iter()
            .map(|piece| (piece.kind(), piece.text().to_string()))
            .collect(),
        first_item,
    })
}

/// Finds the newline that ends the first blank line in a trivia sequence.
///
/// Whitespace between two newline pieces is treated as part of the blank line; missing separators
/// return `None`.
fn find_first_blank_line_index(pieces: &[SyntaxTriviaPiece<JsLanguage>]) -> Option<usize> {
    for (index, piece) in pieces.iter().enumerate() {
        if !piece.is_newline() {
            continue;
        }

        let mut next_index = index + 1;
        while pieces
            .get(next_index)
            .is_some_and(SyntaxTriviaPiece::is_whitespace)
        {
            next_index += 1;
        }
        if pieces
            .get(next_index)
            .is_some_and(SyntaxTriviaPiece::is_newline)
        {
            return Some(next_index);
        }
    }
    None
}

/// Computes the earliest declaration slot after imports and leading directives but before `target`.
///
/// Returns `None` when inserting there would place initialization after the target's top-level item.
fn insertion_slot(list: &JsSyntaxNode, target_index: usize) -> Option<usize> {
    let mut slot = 0;
    let mut in_directive_prologue = true;

    for child in list.children() {
        if is_import_like(&child) || in_directive_prologue && is_directive_statement(&child) {
            slot = child.index() + 1;
        } else {
            // A non-leading statement ends the prologue, so later string expressions are ordinary code.
            in_directive_prologue = false;
        }
    }

    // Inserting after the target could move initialization below code that uses it.
    (slot <= target_index).then_some(slot)
}

/// Returns the safe top-level insertion slot for extracting `target` from `root`.
///
/// Unsupported roots or targets outside the root's item list return `None`.
pub(crate) fn module_constant_insertion_slot(
    root: &AnyJsRoot,
    target: &JsSyntaxNode,
) -> Option<usize> {
    let list = root_list(root)?;
    let top_level_item = target
        .ancestors()
        .find(|ancestor| ancestor.parent().is_some_and(|parent| parent == list))?;
    insertion_slot(&list, top_level_item.index())
}

/// Reports whether `target` can be replaced by a generated module-level constant reference.
///
/// Targets under `with`, roots affected by unbound direct `eval`, and roots without a safe
/// insertion slot are rejected.
pub(crate) fn is_module_constant_extractable_with_facts(
    root: &AnyJsRoot,
    target: &JsSyntaxNode,
    facts: &ModuleConstantFacts,
) -> bool {
    if target
        .ancestors()
        .any(|ancestor| ancestor.kind() == JsSyntaxKind::JS_WITH_STATEMENT)
    {
        // A `with` scope can dynamically shadow any generated identifier.
        return false;
    }
    if facts.has_unbound_direct_eval {
        // Direct eval in a sloppy script can introduce a binding into an enclosing function.
        return false;
    }

    module_constant_insertion_slot(root, target).is_some()
}

/// Reports whether a top-level node is an import or a TypeScript import-equals declaration.
fn is_import_like(node: &JsSyntaxNode) -> bool {
    matches!(
        node.kind(),
        JsSyntaxKind::JS_IMPORT | JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION
    ) || (node.kind() == JsSyntaxKind::JS_EXPORT
        && node
            .descendants()
            .any(|descendant| descendant.kind() == JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION))
}

/// Reports whether a node is an expression statement containing a string literal directive.
fn is_directive_statement(node: &JsSyntaxNode) -> bool {
    let Some(statement) = JsExpressionStatement::cast_ref(node) else {
        return false;
    };

    statement.expression().ok().is_some_and(|expression| {
        expression
            .as_any_js_literal_expression()
            .is_some_and(|literal| literal.as_js_string_literal_expression().is_some())
    })
}

/// Constructs a `const` statement with the supplied value, trivia, separators, and line ending.
///
/// The returned statement owns the provided declaration trivia and adds line separators when the
/// insertion context requires them.
fn make_declaration(
    name: &str,
    value: AnyJsExpression,
    declaration_leading_trivia: &[(TriviaPieceKind, String)],
    first_item_leading_trivia: &[SyntaxTriviaPiece<JsLanguage>],
    insert_at_start: bool,
    line_ending: &str,
) -> JsSyntaxNode {
    let mut leading_trivia = declaration_leading_trivia.to_vec();
    // Inserted list items take the following item's leading trivia with them, so a nonzero-slot
    // declaration needs its own separator even when the preceding statement used ASI.
    if (!insert_at_start || !leading_trivia.is_empty())
        && !matches!(leading_trivia.last(), Some((TriviaPieceKind::Newline, _)))
    {
        leading_trivia.push((TriviaPieceKind::Newline, line_ending.to_string()));
    }

    let whitespace = [(TriviaPieceKind::Whitespace, " ")];
    let const_token = make::token(T![const])
        .with_leading_trivia(
            leading_trivia
                .iter()
                .map(|(kind, text)| (*kind, text.as_str())),
        )
        .with_trailing_trivia(whitespace);
    let name_token = make::ident(name).with_trailing_trivia(whitespace);
    let initializer =
        make::js_initializer_clause(make::token(T![=]).with_trailing_trivia(whitespace), value);
    // Identifier bindings are nested in the binding-pattern union in the CST schema.
    let binding = AnyJsBinding::JsIdentifierBinding(make::js_identifier_binding(name_token));
    let binding_pattern = AnyJsBindingPattern::AnyJsBinding(binding);
    let declarator = make::js_variable_declarator(binding_pattern)
        .with_initializer(initializer)
        .build();
    let declaration = make::js_variable_declaration(
        const_token,
        make::js_variable_declarator_list([declarator], []),
    )
    .build();

    let mut statement = make::js_variable_statement(declaration);
    let mut semicolon = make::token(T![;]);
    if insert_at_start {
        let mut trailing_trivia = vec![(TriviaPieceKind::Newline, line_ending.to_string())];
        trailing_trivia.extend(
            first_item_leading_trivia
                .iter()
                .map(|piece| (piece.kind(), piece.text().to_string())),
        );
        semicolon = semicolon.with_trailing_trivia(
            trailing_trivia
                .iter()
                .map(|(kind, text)| (*kind, text.as_str())),
        );
    }
    statement = statement.with_semicolon_token(semicolon);
    statement.build().into_syntax()
}

/// Returns the first newline convention found in source trivia, defaulting to LF.
///
/// Newlines inside comments and templates are ignored unless they are represented as newline
/// trivia pieces.
fn source_line_ending(list: &JsSyntaxNode) -> &'static str {
    // Detached factory tokens do not inherit the source file's line-separator convention.
    for token in list.descendants_tokens(Direction::Next) {
        for trivia in [token.leading_trivia(), token.trailing_trivia()] {
            // A comment or template token can contain a different separator than the
            // separators between source tokens. Only newline trivia establishes the file convention.
            for piece in trivia.pieces() {
                if !piece.is_newline() {
                    continue;
                }
                if let Some(line_ending) = line_ending_from_newline(piece.text()) {
                    return line_ending;
                }
            }
        }
    }

    "\n"
}

/// Returns the line-ending represented by a newline trivia piece.
fn line_ending_from_newline(text: &str) -> Option<&'static str> {
    match text {
        "\r\n" => Some("\r\n"),
        "\r" => Some("\r"),
        "\n" => Some("\n"),
        "\u{2028}" => Some("\u{2028}"),
        "\u{2029}" => Some("\u{2029}"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_module_constant, extract_module_constant_with_reserved_names};
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::{
        AnyJsExpression, JsFunctionDeclaration, JsModuleItemList, JsNumberLiteralExpression,
        JsReferenceIdentifier, JsRegexLiteralExpression,
    };
    use biome_languages::JsFileSource;
    use biome_rowan::{AstNode, AstNodeList, AstSeparatedList};
    use rustc_hash::FxHashSet;

    #[test]
    fn extracts_expression_after_imports_and_directives() {
        let parsed = parse(
            r#"import value from "module";
"use strict";
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, name) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();
        let transformed = parse(
            &output,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        assert_eq!(name, "REGEX");
        assert!(output.contains("const REGEX = /x/;"));
        assert!(output.contains("return REGEX.test(input);"));
        assert!(
            output.find("import value from \"module\";").unwrap()
                < output.find("const REGEX").unwrap()
        );
        assert!(output.find("\"use strict\";").unwrap() < output.find("const REGEX").unwrap());

        let module_items = transformed
            .syntax()
            .descendants()
            .find_map(JsModuleItemList::cast)
            .expect("expected a module item list");
        let generated_declaration = module_items
            .iter()
            .find_map(|item| {
                let statement = item
                    .as_any_js_statement()?
                    .as_js_variable_statement()?
                    .clone();
                let declarator = statement
                    .declaration()
                    .ok()?
                    .declarators()
                    .iter()
                    .next()?
                    .ok()?;
                let id = declarator.id().ok()?;
                let binding = id.as_any_js_binding()?.as_js_identifier_binding()?;
                (binding
                    .name_token()
                    .ok()
                    .is_some_and(|token| token.text_trimmed() == "REGEX"))
                .then_some(statement)
            })
            .expect("expected REGEX at module scope");
        assert!(
            generated_declaration
                .syntax()
                .parent()
                .is_some_and(|parent| JsModuleItemList::can_cast(parent.kind()))
        );
        assert!(
            generated_declaration
                .syntax()
                .descendants()
                .any(|node| JsRegexLiteralExpression::can_cast(node.kind()))
        );

        let function = transformed
            .syntax()
            .descendants()
            .find_map(JsFunctionDeclaration::cast)
            .expect("expected read function");
        assert!(
            function
                .syntax()
                .descendants()
                .filter_map(JsReferenceIdentifier::cast)
                .any(|reference| {
                    reference
                        .value_token()
                        .ok()
                        .is_some_and(|token| token.text_trimmed() == "REGEX")
                })
        );
        assert!(
            !function
                .syntax()
                .descendants()
                .any(|node| JsRegexLiteralExpression::can_cast(node.kind()))
        );
    }

    #[test]
    fn merges_expressions_after_semicolonless_import_and_directive() {
        let parsed = parse(
            r#""use strict"
import value from "module"

function read(input) {
    return /x/.test(input) && /y/.test(input);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let targets = parsed
            .syntax()
            .descendants()
            .filter_map(JsRegexLiteralExpression::cast)
            .collect::<Vec<_>>();
        assert_eq!(targets.len(), 2);

        let mut mutations = targets.into_iter().enumerate().map(|(index, target)| {
            let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");
            extract_module_constant(
                &parsed.tree(),
                &model,
                target.syntax(),
                value,
                &format!("REGEX_{}", index + 1),
            )
            .expect("expected a module-level extraction")
        });
        let (mut mutation, first_name) = mutations.next().expect("first extraction");
        let (second_mutation, second_name) = mutations.next().expect("second extraction");
        mutation.merge_actions(second_mutation);

        let output = mutation.commit().to_string();
        assert_eq!(first_name, "REGEX_1");
        assert_eq!(second_name, "REGEX_2");
        assert!(output.contains(
            "\"use strict\"\nimport value from \"module\"\nconst REGEX_1 = /x/;\nconst REGEX_2 = /y/;\n\nfunction"
        ));
        assert!(!output.contains("module\"const"));
        assert!(output.contains("return REGEX_1.test(input) && REGEX_2.test(input);"));
    }

    #[test]
    fn preserves_first_target_replacement_when_transferring_header() {
        let source = "// header\n\n31 + value;\n";
        let parsed = parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsNumberLiteralExpression::cast)
            .expect("expected a number literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("number expression");

        let (mutation, name) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "NUMBER")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(name, "NUMBER");
        assert_eq!(output.matches("const NUMBER = 31;").count(), 1);
        assert!(output.contains("NUMBER + value;"));
        assert!(!output.contains("31 + value;"));
    }

    #[test]
    fn merges_header_cleanup_with_first_target_replacement() {
        let source = "// header\n\n31 + 32;\n";
        let parsed = parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let targets = parsed
            .syntax()
            .descendants()
            .filter_map(JsNumberLiteralExpression::cast)
            .collect::<Vec<_>>();
        assert_eq!(targets.len(), 2);

        let mut mutations = targets.into_iter().enumerate().map(|(index, target)| {
            let value = AnyJsExpression::cast(target.syntax().clone()).expect("number expression");
            extract_module_constant(
                &parsed.tree(),
                &model,
                target.syntax(),
                value,
                &format!("NUMBER_{}", index + 31),
            )
            .expect("expected a module-level extraction")
        });
        let (mut mutation, first_name) = mutations.next().expect("first extraction");
        let (second_mutation, second_name) = mutations.next().expect("second extraction");
        mutation.merge_actions(second_mutation);

        let (committed_tree, text_edit) = mutation.clone().commit_with_text_range_and_edit(true);
        let output = mutation.commit().to_string();
        let (_, text_edit) = text_edit.expect("merged mutation should produce a text edit");

        assert_eq!(first_name, "NUMBER_31");
        assert_eq!(second_name, "NUMBER_32");
        assert!(output.contains("NUMBER_31 + NUMBER_32;"));
        assert!(!output.contains("31 + 32;"));
        assert_eq!(committed_tree.to_string(), output);
        assert_eq!(text_edit.new_string(source), output);
    }

    #[test]
    fn merges_two_extractions_for_fix_all() {
        let parsed = parse(
            r#"function read(input) {
    return /first/.test(input) && /second/.test(input);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let targets = parsed
            .syntax()
            .descendants()
            .filter_map(JsRegexLiteralExpression::cast)
            .collect::<Vec<_>>();
        assert_eq!(targets.len(), 2);

        let mut mutations = targets.into_iter().enumerate().map(|(index, target)| {
            let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");
            extract_module_constant(
                &parsed.tree(),
                &model,
                target.syntax(),
                value,
                &format!("REGEX_{}", index + 1),
            )
            .expect("expected a module-level extraction")
        });
        let (mut mutation, first_name) = mutations.next().expect("first extraction");
        let (second_mutation, second_name) = mutations.next().expect("second extraction");
        mutation.merge_actions(second_mutation);

        let output = mutation.commit().to_string();
        assert_eq!(first_name, "REGEX_1");
        assert_eq!(second_name, "REGEX_2");
        assert!(output.contains("const REGEX_1 = /first/;"));
        assert!(output.contains("const REGEX_2 = /second/;"));
        assert!(output.contains("return REGEX_1.test(input) && REGEX_2.test(input);"));
    }

    #[test]
    fn deduplicates_identical_extractions_for_fix_all() {
        let parsed = parse(
            r#"function read(input) {
    return input + 5 + 5;
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let targets = parsed
            .syntax()
            .descendants()
            .filter_map(JsNumberLiteralExpression::cast)
            .collect::<Vec<_>>();
        assert_eq!(targets.len(), 2);

        let mut mutations = targets.into_iter().map(|target| {
            let value = AnyJsExpression::cast(target.syntax().clone()).expect("number expression");
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "NUMBER")
                .expect("expected a module-level extraction")
                .0
        });
        let mut mutation = mutations.next().expect("first extraction");
        mutation.merge_actions(mutations.next().expect("second extraction"));

        let output = mutation.commit().to_string();
        assert_eq!(output.matches("const NUMBER = 5;").count(), 1);
        assert!(output.contains("return input + NUMBER + NUMBER;"));
    }

    #[test]
    fn coordinates_repeated_names_and_header_for_fix_all() {
        let source = "// header\n\nfunction read(input) {\n    return 5 + 5 + 5;\n}\n";
        let parsed = parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let targets = parsed
            .syntax()
            .descendants()
            .filter_map(JsNumberLiteralExpression::cast)
            .collect::<Vec<_>>();
        assert_eq!(targets.len(), 3);

        let mut reserved_names = FxHashSet::default();
        let mut mutations = targets
            .into_iter()
            .map(|target| {
                let value =
                    AnyJsExpression::cast(target.syntax().clone()).expect("number expression");
                let (mutation, name) = extract_module_constant_with_reserved_names(
                    &parsed.tree(),
                    &model,
                    target.syntax(),
                    value,
                    "NUMBER_5",
                    &reserved_names,
                    true,
                )
                .expect("expected a module-level extraction");
                reserved_names.insert(name.clone());
                (mutation, name)
            })
            .collect::<Vec<_>>();
        let (mut mutation, first_name) = mutations.remove(0);
        let (second_mutation, second_name) = mutations.remove(0);
        let (third_mutation, third_name) = mutations.remove(0);
        let standalone_second = second_mutation.clone().commit().to_string();
        assert!(standalone_second.starts_with("// header\nconst NUMBER_5_2 = 5;"));
        mutation.merge_actions(second_mutation);
        mutation.merge_actions(third_mutation);

        let (committed_tree, text_edit) = mutation.clone().commit_with_text_range_and_edit(true);
        let output = mutation.commit().to_string();
        let (_, text_edit) = text_edit.expect("merged mutation should produce a text edit");
        assert_eq!(first_name, "NUMBER_5");
        assert_eq!(second_name, "NUMBER_5_2");
        assert_eq!(third_name, "NUMBER_5_3");
        assert_eq!(output.matches("// header").count(), 1);
        assert!(output.contains("const NUMBER_5 = 5;"));
        assert!(output.contains("const NUMBER_5_2 = 5;"));
        assert!(output.contains("const NUMBER_5_3 = 5;"));
        assert!(output.contains("return NUMBER_5 + NUMBER_5_2 + NUMBER_5_3;"));
        assert!(output.find("// header").unwrap() < output.find("const NUMBER_5 =").unwrap());
        assert_eq!(committed_tree.to_string(), output);
        assert_eq!(text_edit.new_string(source), output);
    }

    #[test]
    fn extracts_script_expression_with_collision_suffix() {
        let parsed = parse(
            r#"const REGEX = 0;
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, name) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a script-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(name, "REGEX_2");
        assert!(output.contains("const REGEX_2 = /x/;"));
        assert!(output.contains("return REGEX_2.test(input);"));
    }

    #[test]
    fn transfers_header_trivia_when_target_is_not_first_item() {
        let source = "// header\n \t\nconst before = 0;\nfunction read(input) {\n    return /x/.test(input);\n}\n";
        let parsed = parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(output.matches("// header").count(), 1);
        assert!(output.starts_with("// header\n \t\nconst REGEX = /x/;\n\nconst before = 0;"));
    }

    #[test]
    fn rejects_attached_first_item_documentation() {
        let parsed = parse(
            r#"/** Documentation for before. */
const before = 0;
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn rejects_later_comment_blocks_after_header_separator() {
        let parsed = parse(
            r#"/** File header. */

/** Documentation for before. */
const before = 0;
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn avoids_shadowing_target_scope_bindings() {
        let parsed = parse(
            r#"function read(REGEX) {
    return /x/.test(REGEX);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, name) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(name, "REGEX_2");
        assert!(output.contains("const REGEX_2 = /x/;"));
        assert!(output.contains("return REGEX_2.test(REGEX);"));
    }

    #[test]
    fn avoids_capturing_unresolved_reference_elsewhere() {
        let parsed = parse(
            r#"function read(input) {
    return /x/.test(input);
}
console.log(REGEX);
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, name) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(name, "REGEX_2");
        assert!(output.contains("const REGEX_2 = /x/;"));
        assert!(output.contains("console.log(REGEX);"));
    }

    #[test]
    fn avoids_capturing_configured_global_reference_elsewhere() {
        let parsed = parse(
            r#"function read(input) {
    return /x/.test(input);
}
console.log(REGEX);
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let mut options = SemanticModelOptions::default();
        options.globals.insert("REGEX".to_string());
        let model = semantic_model(&parsed.tree(), options);
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, name) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(name, "REGEX_2");
        assert!(output.contains("const REGEX_2 = /x/;"));
        assert!(output.contains("console.log(REGEX);"));
    }

    #[test]
    fn rejects_target_under_dynamic_with() {
        let parsed = parse(
            r#"function read(input) {
    with (scope) {
        return /x/.test(input);
    }
}
"#,
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn rejects_target_affected_by_direct_eval_in_sloppy_script() {
        let parsed = parse(
            r#"function read(input) {
    eval("var REGEX = /other/;");
    return /x/.test(input);
}
"#,
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn rejects_script_target_with_sibling_nested_direct_eval() {
        let parsed = parse(
            r#"function other() {
    function nested() {
        eval("var REGEX = /other/;");
    }
    nested();
}
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn rejects_module_target_observable_to_direct_eval() {
        let parsed = parse(
            r#"function read(input) {
    eval("REGEX");
    return /x/.test(input);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn rejects_target_before_later_import() {
        let parsed = parse(
            r#"function read(input) {
    return /x/.test(input);
}
import value from "module";
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn places_constant_after_typescript_import_equals() {
        let parsed = parse(
            r#"import value = require("module");
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a TypeScript module-level extraction");
        let output = mutation.commit().to_string();

        assert!(
            output.find("import value = require(\"module\");").unwrap()
                < output.find("const REGEX").unwrap()
        );
        assert!(output.find("const REGEX").unwrap() < output.find("function read").unwrap());
    }

    #[test]
    fn places_constant_after_exported_typescript_import_equals() {
        let parsed = parse(
            r#"export import value = require("module");
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a TypeScript module-level extraction");
        let output = mutation.commit().to_string();

        assert!(
            output
                .find("export import value = require(\"module\");")
                .unwrap()
                < output.find("const REGEX").unwrap()
        );
        assert!(output.find("const REGEX").unwrap() < output.find("function read").unwrap());
    }

    #[test]
    fn preserves_target_comments_without_duplicating_them() {
        let parsed = parse(
            r#"function read(input) {
    return /* before */ /x/ /* after */.test(input);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(output.matches("/* before */").count(), 1);
        assert_eq!(output.matches("/* after */").count(), 1);
        assert!(output.contains("const REGEX = /x/;"));
        assert!(output.contains("return /* before */ REGEX /* after */.test(input);"));
    }

    #[test]
    fn places_script_constant_after_directives_with_crlf() {
        let parsed = parse(
            "\"use strict\";\r\nfunction read(input) {\r\n    return /x/.test(input);\r\n}\r\n",
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a script-level extraction");
        let output = mutation.commit().to_string();

        assert!(output.contains("\"use strict\";\r\nconst REGEX = /x/;\r\nfunction"));
        assert!(output.contains("return REGEX.test(input);"));
    }

    #[test]
    fn preserves_unicode_line_separators() {
        for separator in ["\u{2028}", "\u{2029}"] {
            let source = format!(
                "function read(input) {{{separator}    return /x/.test(input);{separator}}}{separator}"
            );
            let parsed = parse(
                &source,
                JsFileSource::js_script(),
                JsParserOptions::default(),
            );
            let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
            let target = parsed
                .syntax()
                .descendants()
                .find_map(JsRegexLiteralExpression::cast)
                .expect("expected a regex literal");
            let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

            let (mutation, _) =
                extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                    .expect("expected a script-level extraction");
            let output = mutation.commit().to_string();

            assert!(output.contains(&format!("const REGEX = /x/;{separator}function")));
        }
    }

    #[test]
    fn ignores_crlf_inside_multiline_comment_tokens() {
        let source = "const before = /* first line\r\nsecond line */ 0;\nfunction read(input) {\n    return /x/.test(input);\n}\n";
        let parsed = parse(
            source,
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a script-level extraction");
        let output = mutation.commit().to_string();

        assert!(output.contains("const REGEX = /x/;\nconst before"));
    }
}
