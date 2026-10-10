//! The reference pass: finds every use of a name and resolves it to the
//! declaration it refers to.
//!
//! Names cross language boundaries in a Vue component. An identifier in a
//! template expression can be a `v-for` variable declared in HTML, a binding
//! of `<script setup>`, or a prop declared in a type. A tag name can be an
//! import. This pass walks one scope chain that spans all of them.
//!
//! It also records what each use does with the value, and follows the few
//! indirections Vue defines: `props.title` is a use of the prop `title`, and
//! `emit('change')` is a use of the event `change`.

use super::builder::{ModelBuilder, NewSymbol, synthetic_name};
use super::component::ScriptInput;
use super::layout::Layout;
use super::model::*;
use super::snippets::SfcSnippet;
use biome_js_semantic::SemanticModel as JsSemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, AnyJsRoot, JsCallExpression,
    JsComputedMemberAssignment, JsComputedMemberExpression, JsIdentifierAssignment,
    JsIdentifierBinding, JsReferenceIdentifier, JsStaticMemberAssignment, JsStaticMemberExpression,
    JsSyntaxKind, JsSyntaxNode, JsThisExpression, JsUnaryExpression, JsUnaryOperator,
};
use biome_rowan::{AstNode, AstSeparatedList, TextLen, TextRange, TextSize, TokenText};
use biome_string_case::Case;

/// Names Vue provides in every template.
const BUILTINS: &[&str] = &[
    "$attrs",
    "$data",
    "$el",
    "$emit",
    "$event",
    "$forceUpdate",
    "$nextTick",
    "$options",
    "$parent",
    "$props",
    "$refs",
    "$root",
    "$slots",
    "$watch",
];

/// JavaScript globals a template expression may use.
const TEMPLATE_GLOBALS: &[&str] = &[
    "Array",
    "BigInt",
    "Boolean",
    "Date",
    "Error",
    "Infinity",
    "Intl",
    "JSON",
    "Map",
    "Math",
    "NaN",
    "Number",
    "Object",
    "RegExp",
    "Set",
    "String",
    "Symbol",
    "console",
    "decodeURI",
    "decodeURIComponent",
    "encodeURI",
    "encodeURIComponent",
    "isFinite",
    "isNaN",
    "parseFloat",
    "parseInt",
    "undefined",
];

/// Tags Vue resolves itself.
const BUILTIN_COMPONENTS: &[&str] = &[
    "KeepAlive",
    "Suspense",
    "Teleport",
    "Transition",
    "TransitionGroup",
    "component",
    "keep-alive",
    "slot",
    "suspense",
    "teleport",
    "template",
    "transition",
    "transition-group",
];

const MUTATING_METHODS: &[&str] = &[
    "copyWithin",
    "fill",
    "pop",
    "push",
    "reverse",
    "shift",
    "sort",
    "splice",
    "unshift",
];

/// The scopes a single-file component's template resolves names through.
pub(crate) struct TemplateScopes {
    /// The component that renders the template.
    pub(crate) component: Option<ComponentId>,
    /// The innermost scope outside the template: the first one a name is
    /// looked up in after the scopes of elements.
    pub(crate) head: ScopeId,
    /// The scopes holding the top-level bindings of the script blocks the
    /// template can see, `<script setup>` first.
    pub(crate) modules: Vec<ScopeId>,
    pub(crate) builtins: ScopeId,
}

/// Creates the scope of the names Vue provides.
pub(crate) fn add_builtins_scope(builder: &mut ModelBuilder) -> ScopeId {
    let scope = builder.add_scope(ScopeKind::Builtins, None);
    for name in BUILTINS {
        builder.add_symbol(NewSymbol {
            name: synthetic_name(name),
            range: TextRange::default(),
            declaration: TextRange::default(),
            snippet: None,
            scope,
            namespace: Namespace::Binding,
            kind: SymbolKind::Builtin,
            component: None,
        });
    }
    scope
}

/// Records the variables elements introduce with `v-for` and `v-slot`, and
/// the names given to elements with a static `ref`.
pub(crate) fn collect_template_symbols(
    builder: &mut ModelBuilder,
    layout: &Layout,
    snippets: &[SfcSnippet],
    ids: &[SnippetId],
    scopes: &TemplateScopes,
) {
    for v_for in &layout.v_fors {
        let scope = builder.data.elements[v_for.element.index()].scope;
        for (name, range) in &v_for.bindings {
            builder.add_symbol(NewSymbol {
                name: name.clone(),
                range: *range,
                declaration: *range,
                snippet: None,
                scope,
                namespace: Namespace::Binding,
                kind: SymbolKind::VForAlias,
                component: None,
            });
        }
    }

    for (snippet, id) in snippets.iter().zip(ids) {
        let AnyJsRoot::JsVueSlotPropsRoot(root) = snippet.root else {
            continue;
        };
        let SnippetHost::DirectiveValue(attr) = builder.data.snippets[id.index()].host else {
            continue;
        };
        let element = builder.data.attrs[attr.index()].element;
        let scope = builder.data.elements[element.index()].scope;
        for binding in root
            .syntax()
            .descendants()
            .filter_map(JsIdentifierBinding::cast)
        {
            let Ok(token) = binding.name_token() else {
                continue;
            };
            let range = token.text_trimmed_range() + snippet.content_offset;
            builder.add_symbol(NewSymbol {
                name: token.token_text_trimmed(),
                range,
                declaration: range,
                snippet: Some(*id),
                scope,
                namespace: Namespace::Binding,
                kind: SymbolKind::SlotParam,
                component: None,
            });
        }
    }

    for (_, name, range) in &layout.template_refs {
        builder.add_symbol(NewSymbol {
            name: name.clone(),
            range: *range,
            declaration: *range,
            snippet: None,
            scope: scopes.builtins,
            namespace: Namespace::TemplateRef,
            kind: SymbolKind::TemplateRef,
            component: scopes.component,
        });
    }
}

/// What a use does with a value, read from the syntax around it.
struct Usage {
    path: Vec<TokenText>,
    path_open: bool,
    access: Access,
    call_args: Option<u8>,
    /// The call the value is the callee of.
    call: Option<JsCallExpression>,
    /// The outermost expression of the member chain that starts at the use.
    top: JsSyntaxNode,
}

/// Follows the member chain that starts at `start` and classifies what
/// happens to the value at its end.
fn usage_of(start: &JsSyntaxNode) -> Usage {
    let mut current = start.clone();
    let mut path = Vec::new();
    let mut path_open = false;
    let mut written = matches!(current.kind(), JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT);

    while let Some(parent) = current.parent() {
        match parent.kind() {
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
            | JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_SATISFIES_EXPRESSION => current = parent,
            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                let member = JsStaticMemberExpression::unwrap_cast(parent.clone());
                if member.object().ok().map(|object| object.into_syntax()) != Some(current.clone())
                {
                    break;
                }
                let Some(name) = member
                    .member()
                    .ok()
                    .and_then(|name| name.value_token().ok())
                else {
                    break;
                };
                path.push(name.token_text_trimmed());
                current = parent;
            }
            JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                let member = JsStaticMemberAssignment::unwrap_cast(parent.clone());
                if member.object().ok().map(|object| object.into_syntax()) != Some(current.clone())
                {
                    break;
                }
                if let Some(name) = member
                    .member()
                    .ok()
                    .and_then(|name| name.value_token().ok())
                {
                    path.push(name.token_text_trimmed());
                }
                written = true;
                current = parent;
                break;
            }
            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                let member = JsComputedMemberExpression::unwrap_cast(parent.clone());
                if member.object().ok().map(|object| object.into_syntax()) != Some(current.clone())
                {
                    break;
                }
                current = parent;
                match member.member().ok().map(AnyJsExpression::omit_parentheses) {
                    Some(AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(literal),
                    )) => match literal.inner_string_text() {
                        Ok(name) => path.push(name),
                        Err(_) => {
                            path_open = true;
                            break;
                        }
                    },
                    _ => {
                        path_open = true;
                        break;
                    }
                }
            }
            JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                let member = JsComputedMemberAssignment::unwrap_cast(parent.clone());
                if member.object().ok().map(|object| object.into_syntax()) != Some(current.clone())
                {
                    break;
                }
                path_open = true;
                written = true;
                current = parent;
                break;
            }
            _ => break,
        }
    }

    let mut usage = Usage {
        path,
        path_open,
        access: Access::Read,
        call_args: None,
        call: None,
        top: current.clone(),
    };
    if written {
        usage.access = Access::Write;
        return usage;
    }
    let Some(parent) = current.parent() else {
        return usage;
    };
    match parent.kind() {
        JsSyntaxKind::JS_CALL_EXPRESSION => {
            let call = JsCallExpression::unwrap_cast(parent);
            if call.callee().ok().map(|callee| callee.into_syntax()) == Some(current) {
                let args = call
                    .arguments()
                    .map_or(0, |arguments| arguments.args().len() as u8);
                if !usage.path_open
                    && usage
                        .path
                        .last()
                        .is_some_and(|name| MUTATING_METHODS.contains(&name.text()))
                {
                    usage.path.pop();
                    usage.access = Access::Mutate;
                } else {
                    usage.access = Access::Call;
                    usage.call_args = Some(args);
                    usage.call = Some(call);
                }
            }
        }
        JsSyntaxKind::JS_UNARY_EXPRESSION => {
            if JsUnaryExpression::unwrap_cast(parent)
                .operator()
                .is_ok_and(|operator| operator == JsUnaryOperator::Delete)
            {
                usage.access = Access::Write;
            }
        }
        JsSyntaxKind::JS_CALL_ARGUMENT_LIST
        | JsSyntaxKind::JS_SPREAD
        | JsSyntaxKind::JS_RETURN_STATEMENT
        | JsSyntaxKind::JS_INITIALIZER_CLAUSE
        | JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER
        | JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER
        | JsSyntaxKind::JS_ARRAY_ELEMENT_LIST
        | JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => usage.access = Access::Escape,
        _ => {}
    }
    usage
}

/// Returns the node a member chain starts at for a reference node: the
/// identifier expression that wraps a reference, or the node itself.
fn chain_start(node: &JsSyntaxNode) -> JsSyntaxNode {
    match node.parent() {
        Some(parent) if parent.kind() == JsSyntaxKind::JS_IDENTIFIER_EXPRESSION => parent,
        _ => node.clone(),
    }
}

/// Shared state of the reference pass over one JavaScript tree.
struct Walker<'a> {
    builder: &'a mut ModelBuilder,
    offset: TextSize,
    snippet: Option<SnippetId>,
}

impl Walker<'_> {
    fn host(&self, range: TextRange) -> TextRange {
        range + self.offset
    }

    /// Resolves `name` among the declarations of `component` in `namespace`.
    fn resolve_in_component(
        &self,
        component: Option<ComponentId>,
        namespace: Namespace,
        name: &str,
        kinds: &[SymbolKind],
    ) -> Resolution {
        self.builder
            .resolve_declaration(component, namespace, |symbol| {
                symbol.name.text() == name && (kinds.is_empty() || kinds.contains(&symbol.kind))
            })
    }

    fn mark_uses_open(&mut self, component: Option<ComponentId>, namespace: Namespace) {
        let Some(component) = component else {
            return;
        };
        // The use may also reach what the merged plain-block component
        // declares.
        let merged = self.builder.data.components[component.index()].merged;
        for id in std::iter::once(component).chain(merged) {
            self.builder.component_mut(id).uses_open[namespace.index()] = true;
        }
    }

    /// Records the use of a member of the component instance, as in
    /// `props.title`, `this.title` or `$props.title`.
    fn add_member_reference(
        &mut self,
        component: Option<ComponentId>,
        base: RefBase,
        usage: &Usage,
        skip: usize,
        kinds: &[SymbolKind],
        scope: Option<ScopeId>,
    ) {
        let Some(name) = usage.path.get(skip) else {
            // The object is used whole, so any member may be read through it.
            if usage.path_open || usage.access != Access::Read {
                self.mark_uses_open(component, Namespace::Binding);
            }
            return;
        };
        let resolution =
            self.resolve_in_component(component, Namespace::Binding, name.text(), kinds);
        let range = member_range(&usage.top, usage.path.len() - skip - 1).map_or_else(
            || self.host(usage.top.text_trimmed_range()),
            |range| self.host(range),
        );
        self.builder.add_reference(ReferenceData {
            range,
            name: name.clone(),
            snippet: self.snippet,
            site: RefSite::Identifier,
            namespace: Namespace::Binding,
            scope,
            base,
            path: usage.path[skip + 1..].into(),
            path_open: usage.path_open,
            access: usage.access,
            call_args: usage.call_args,
            resolution,
        });
    }

    /// Records the event an emit call names, as in `emit('change', id)`.
    fn add_event_reference(
        &mut self,
        component: Option<ComponentId>,
        base: RefBase,
        call: &JsCallExpression,
        scope: Option<ScopeId>,
    ) {
        let arguments: Vec<_> = call
            .arguments()
            .ok()
            .into_iter()
            .flat_map(|arguments| arguments.args().iter().flatten())
            .collect();
        let Some(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(literal),
        )) = arguments
            .first()
            .and_then(AnyJsCallArgument::as_any_js_expression)
            .map(|argument| argument.clone().omit_parentheses())
        else {
            self.mark_uses_open(component, Namespace::Emit);
            return;
        };
        let Ok(name) = literal.inner_string_text() else {
            return;
        };
        let mut resolution =
            self.resolve_in_component(component, Namespace::Emit, name.text(), &[]);
        // `defineModel('value')` declares the event `update:value`.
        if !matches!(resolution, Resolution::Symbol(_))
            && let Some(model) = name.text().strip_prefix("update:")
            && let Resolution::Symbol(symbol) = self.resolve_in_component(
                component,
                Namespace::Binding,
                model,
                &[SymbolKind::Model],
            )
        {
            resolution = Resolution::Symbol(symbol);
        }
        self.builder.add_reference(ReferenceData {
            range: self.host(literal.syntax().text_trimmed_range()),
            name,
            snippet: self.snippet,
            site: RefSite::EventName,
            namespace: Namespace::Emit,
            scope,
            base,
            path: Box::default(),
            path_open: false,
            access: Access::Call,
            call_args: Some(arguments.len().saturating_sub(1) as u8),
            resolution,
        });
    }

    /// Records the template ref a `$refs` member names, as in `$refs.input`.
    fn add_template_ref_reference(
        &mut self,
        component: Option<ComponentId>,
        name: TokenText,
        range: TextRange,
        scope: Option<ScopeId>,
    ) {
        let resolution =
            self.resolve_in_component(component, Namespace::TemplateRef, name.text(), &[]);
        self.builder.add_reference(ReferenceData {
            range,
            name,
            snippet: self.snippet,
            site: RefSite::TemplateRefKey,
            namespace: Namespace::TemplateRef,
            scope,
            base: RefBase::Builtin,
            path: Box::default(),
            path_open: false,
            access: Access::Read,
            call_args: None,
            resolution,
        });
    }

    /// Follows what Vue defines for a value of the given origin: a member of
    /// the props object is a prop, a call of the emit function names an
    /// event.
    fn project(
        &mut self,
        origin: Origin,
        via: RefBase,
        component: Option<ComponentId>,
        usage: &Usage,
        scope: Option<ScopeId>,
    ) {
        match origin {
            Origin::DefineProps | Origin::SetupProps => {
                self.add_member_reference(
                    component,
                    via,
                    usage,
                    0,
                    &[SymbolKind::Prop, SymbolKind::Model],
                    scope,
                );
            }
            Origin::DefineEmits | Origin::SetupEmit => match &usage.call {
                Some(call) if usage.path.is_empty() => {
                    self.add_event_reference(component, via, call, scope);
                }
                _ => {
                    if usage.access == Access::Escape {
                        self.mark_uses_open(component, Namespace::Emit);
                    }
                }
            },
            Origin::SetupContext => {
                if usage.path.first().is_some_and(|name| name.text() == "emit") {
                    match &usage.call {
                        Some(call) if usage.path.len() == 1 => {
                            self.add_event_reference(component, via, call, scope);
                        }
                        _ => self.mark_uses_open(component, Namespace::Emit),
                    }
                } else if usage.path.is_empty() && usage.access == Access::Escape {
                    self.mark_uses_open(component, Namespace::Emit);
                }
            }
            _ => {}
        }
    }

    /// Follows what Vue defines for a member of the component instance whose
    /// name starts with `$`. `skip` is the index of that name in the path.
    fn project_builtin(
        &mut self,
        component: Option<ComponentId>,
        base: RefBase,
        usage: &Usage,
        skip: usize,
        name: &str,
        scope: Option<ScopeId>,
    ) {
        match name {
            "$emit" => match &usage.call {
                Some(call) if usage.path.len() == skip => {
                    self.add_event_reference(component, base, call, scope);
                }
                _ => {
                    if usage.access == Access::Escape {
                        self.mark_uses_open(component, Namespace::Emit);
                    }
                }
            },
            "$props" => self.add_member_reference(
                component,
                base,
                usage,
                skip,
                &[SymbolKind::Prop, SymbolKind::Model],
                scope,
            ),
            "$refs" => match usage.path.get(skip) {
                Some(key) => {
                    let range = member_range(&usage.top, usage.path.len() - skip - 1).map_or_else(
                        || self.host(usage.top.text_trimmed_range()),
                        |range| self.host(range),
                    );
                    self.add_template_ref_reference(component, key.clone(), range, scope);
                }
                None => {
                    if usage.path_open || usage.access != Access::Read {
                        self.mark_uses_open(component, Namespace::TemplateRef);
                    }
                }
            },
            _ => {}
        }
    }
}

/// Returns the range of the member name `depth` levels below the top of a
/// member chain: depth 0 is the last name of `a.b.c`.
fn member_range(top: &JsSyntaxNode, depth: usize) -> Option<TextRange> {
    let mut node = top.clone();
    for _ in 0..depth {
        node = match node.kind() {
            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => JsStaticMemberExpression::cast(node)?
                .object()
                .ok()?
                .omit_parentheses()
                .into_syntax(),
            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => JsComputedMemberExpression::cast(node)?
                .object()
                .ok()?
                .omit_parentheses()
                .into_syntax(),
            JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => JsStaticMemberAssignment::cast(node)?
                .object()
                .ok()?
                .omit_parentheses()
                .into_syntax(),
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
            | JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_SATISFIES_EXPRESSION => node.first_child()?,
            _ => return None,
        };
    }
    match node.kind() {
        JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => Some(
            JsStaticMemberExpression::cast(node)?
                .member()
                .ok()?
                .syntax()
                .text_trimmed_range(),
        ),
        JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => Some(
            JsStaticMemberAssignment::cast(node)?
                .member()
                .ok()?
                .syntax()
                .text_trimmed_range(),
        ),
        JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => Some(
            JsComputedMemberExpression::cast(node)?
                .member()
                .ok()?
                .syntax()
                .text_trimmed_range(),
        ),
        _ => None,
    }
}

/// Returns the free identifiers of a tree: references that nothing in the
/// tree itself declares.
fn free_references(root: &AnyJsRoot, js: &JsSemanticModel) -> Vec<JsSyntaxNode> {
    root.syntax()
        .descendants()
        .filter(|node| {
            if let Some(reference) = JsReferenceIdentifier::cast_ref(node) {
                js.binding(&reference).is_none()
            } else if let Some(assignment) = JsIdentifierAssignment::cast_ref(node) {
                js.binding(&assignment).is_none()
            } else {
                false
            }
        })
        .collect()
}

/// Resolves the identifiers of every template expression.
pub(crate) fn collect_template_references(
    builder: &mut ModelBuilder,
    snippets: &[SfcSnippet],
    ids: &[SnippetId],
    scopes: &TemplateScopes,
) {
    for (snippet, id) in snippets.iter().zip(ids) {
        let host = builder.data.snippets[id.index()].host;
        let mut is_model_value = false;
        let scope = match host {
            SnippetHost::Script(_) => continue,
            SnippetHost::Interpolation(Some(element)) => {
                builder.data.elements[element.index()].scope
            }
            SnippetHost::DirectiveValue(attr) => {
                let attr = &builder.data.attrs[attr.index()];
                is_model_value = matches!(
                    attr.name,
                    AttrName::Directive {
                        kind: DirectiveKind::Model,
                        ..
                    }
                );
                builder.data.elements[attr.element.index()].scope
            }
            // The iterable of a `v-for` is evaluated before its variables
            // exist, so it resolves in the enclosing scope.
            SnippetHost::VForIterable(attr) => {
                let element = builder.data.attrs[attr.index()].element;
                let scope = builder.data.elements[element.index()].scope;
                builder.data.scopes[scope.index()].parent.unwrap_or(scope)
            }
            SnippetHost::StyleVBind(_)
            | SnippetHost::Interpolation(None)
            | SnippetHost::Unknown => scopes.head,
        };

        let root_expression = match snippet.root {
            AnyJsRoot::JsExpressionTemplateRoot(root) => root
                .expression()
                .map(|expression| expression.omit_parentheses().into_syntax()),
            _ => None,
        };

        let mut walker = Walker {
            builder,
            offset: snippet.content_offset,
            snippet: Some(*id),
        };
        for node in free_references(snippet.root, snippet.js) {
            let Some(token) = node.first_token() else {
                continue;
            };
            let name = token.token_text_trimmed();
            let mut usage = usage_of(&chain_start(&node));
            // `v-model="value"` assigns to its value.
            if is_model_value && Some(&usage.top) == root_expression.as_ref() {
                usage.access = Access::Write;
            }

            let found = lookup(&walker.builder.data, scope, name.text());
            let resolution = match found {
                Some(symbol) => Resolution::Symbol(symbol),
                None if TEMPLATE_GLOBALS.contains(&name.text()) => Resolution::Global,
                None if walker.builder.is_open(scopes.component, Namespace::Binding) => {
                    Resolution::Unknowable
                }
                None => Resolution::Unresolved,
            };
            walker.builder.add_reference(ReferenceData {
                range: walker.host(token.text_trimmed_range()),
                name: name.clone(),
                snippet: Some(*id),
                site: RefSite::Identifier,
                namespace: Namespace::Binding,
                scope: Some(scope),
                base: RefBase::Bare,
                path: usage.path.clone().into_boxed_slice(),
                path_open: usage.path_open,
                access: usage.access,
                call_args: usage.call_args,
                resolution,
            });

            let Some(symbol) = found else {
                continue;
            };
            let (kind, origin) = {
                let symbol = &walker.builder.data.symbols[symbol.index()];
                (symbol.kind, symbol.origin)
            };
            if kind == SymbolKind::Builtin {
                walker.project_builtin(
                    scopes.component,
                    RefBase::Builtin,
                    &usage,
                    0,
                    name.text(),
                    Some(scope),
                );
            } else {
                walker.project(
                    origin,
                    RefBase::Via(symbol),
                    scopes.component,
                    &usage,
                    Some(scope),
                );
            }
        }
    }
}

/// Looks `name` up in `scope` and its enclosing scopes.
fn lookup(data: &SemanticModelData, scope: ScopeId, name: &str) -> Option<SymbolId> {
    let mut current = Some(scope);
    while let Some(scope) = current {
        let scope = &data.scopes[scope.index()];
        let found = scope.symbols.iter().copied().find(|id| {
            let symbol = &data.symbols[id.index()];
            symbol.namespace == Namespace::Binding && symbol.name.text() == name
        });
        if found.is_some() {
            return found;
        }
        current = scope.parent;
    }
    None
}

/// Records how a script uses the declarations of the components it defines:
/// members of `this`, members of the props object, and emit calls.
pub(crate) fn collect_script_references(
    builder: &mut ModelBuilder,
    input: &ScriptInput,
    components: &[ComponentId],
) {
    // Everything this pass records belongs to a component, so a tree that
    // defines none is not walked.
    if components.is_empty() {
        return;
    }
    let content = input.root.syntax().text_trimmed_range() + input.offset;
    let mut walker = Walker {
        builder,
        offset: input.offset,
        snippet: input.snippet,
    };

    // Bindings with a role: the props object, the emit function, the setup
    // context, and variables destructured from props.
    let roles: Vec<_> = walker
        .builder
        .roles
        .iter()
        .filter(|(range, _)| content.contains_range(**range))
        .map(|(range, role)| (*range, *role))
        .collect();
    for (range, role) in roles {
        let Some(local) = range.checked_sub(input.offset) else {
            continue;
        };
        let Some(binding) = input
            .root
            .syntax()
            .covering_element(local)
            .ancestors()
            .find_map(JsIdentifierBinding::cast)
        else {
            continue;
        };
        let symbol = walker
            .builder
            .data
            .symbols
            .iter()
            .position(|symbol| symbol.range == range && symbol.namespace == Namespace::Binding)
            .map(SymbolId::new);
        for reference in input.js.as_binding(&binding).all_references() {
            let node = reference.syntax();
            let usage = usage_of(&chain_start(&node));
            match (role.origin, role.prop) {
                (Origin::PropsDestructure, Some(prop)) => {
                    let Some(token) = node.first_token() else {
                        continue;
                    };
                    walker.builder.add_reference(ReferenceData {
                        range: walker.host(token.text_trimmed_range()),
                        name: token.token_text_trimmed(),
                        snippet: input.snippet,
                        site: RefSite::Identifier,
                        namespace: Namespace::Binding,
                        scope: None,
                        base: RefBase::Bare,
                        path: usage.path.clone().into_boxed_slice(),
                        path_open: usage.path_open,
                        access: usage.access,
                        call_args: usage.call_args,
                        resolution: Resolution::Symbol(prop),
                    });
                }
                _ => {
                    let via = symbol.map_or(RefBase::Bare, RefBase::Via);
                    walker.project(role.origin, via, Some(role.component), &usage, None);
                }
            }
        }
    }

    // `this` inside an options component.
    let options_components: Vec<_> = components
        .iter()
        .copied()
        .filter(|id| walker.builder.data.components[id.index()].kind != ComponentKind::Setup)
        .collect();
    for node in input.root.syntax().descendants() {
        if let Some(this) = JsThisExpression::cast_ref(&node) {
            let range = walker.host(this.syntax().text_trimmed_range());
            // The innermost component whose definition contains the `this`.
            let Some(component) = options_components
                .iter()
                .copied()
                .filter(|id| {
                    walker.builder.data.components[id.index()]
                        .definition
                        .contains_range(range)
                })
                .min_by_key(|id| walker.builder.data.components[id.index()].definition.len())
            else {
                continue;
            };
            let definition = walker.builder.data.components[component.index()].definition;
            if !is_instance_this(&node, definition, input.offset) {
                continue;
            }
            let usage = usage_of(&node);
            let Some(name) = usage.path.first().cloned() else {
                if usage.path_open || usage.access != Access::Read {
                    walker.mark_uses_open(Some(component), Namespace::Binding);
                }
                continue;
            };
            if name.text().starts_with('$') {
                walker.project_builtin(
                    Some(component),
                    RefBase::This,
                    &usage,
                    1,
                    name.text(),
                    None,
                );
            } else {
                walker.add_member_reference(Some(component), RefBase::This, &usage, 0, &[], None);
            }
        } else if let Some(call) = JsCallExpression::cast_ref(&node) {
            collect_use_template_ref(&mut walker, input, &call, components);
        }
    }
}

/// Records the template ref a `useTemplateRef('name')` call names.
fn collect_use_template_ref(
    walker: &mut Walker,
    input: &ScriptInput,
    call: &JsCallExpression,
    components: &[ComponentId],
) {
    let Some(reference) = call
        .callee()
        .ok()
        .and_then(|callee| callee.omit_parentheses().as_js_reference_identifier())
    else {
        return;
    };
    if !reference.has_name("useTemplateRef")
        || input
            .js
            .binding(&reference)
            .is_some_and(|binding| !binding.is_imported())
    {
        return;
    }
    let Some(AnyJsExpression::AnyJsLiteralExpression(
        AnyJsLiteralExpression::JsStringLiteralExpression(literal),
    )) = call
        .arguments()
        .ok()
        .and_then(|arguments| arguments.args().iter().next()?.ok())
        .and_then(|argument| argument.as_any_js_expression().cloned())
        .map(AnyJsExpression::omit_parentheses)
    else {
        return;
    };
    let Ok(name) = literal.inner_string_text() else {
        return;
    };
    let range = walker.host(literal.syntax().text_trimmed_range());
    let component = components.iter().copied().find(|id| {
        walker.builder.data.components[id.index()]
            .definition
            .contains_range(range)
    });
    walker.add_template_ref_reference(component, name, range, None);
}

/// Returns `true` when a `this` expression refers to the component instance.
///
/// That is the case when exactly one function that binds its own `this`
/// separates the expression from the component's definition: the method,
/// hook or getter the expression is written in.
fn is_instance_this(this: &JsSyntaxNode, definition: TextRange, offset: TextSize) -> bool {
    let mut functions = 0;
    for ancestor in this.ancestors().skip(1) {
        if !definition.contains_range(ancestor.text_trimmed_range() + offset)
            || ancestor.text_trimmed_range() + offset == definition
        {
            break;
        }
        if matches!(
            ancestor.kind(),
            JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_FUNCTION_DECLARATION
                | JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
                | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
                | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
                | JsSyntaxKind::JS_METHOD_CLASS_MEMBER
                | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
                | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
                | JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER
        ) {
            functions += 1;
        }
    }
    functions == 1
}

/// Returns the spellings under which Vue looks a tag or directive name up:
/// the name as written, in camelCase, and in PascalCase.
fn asset_names(name: &str) -> [String; 3] {
    [
        name.to_string(),
        Case::Camel.convert(name),
        Case::Pascal.convert(name),
    ]
}

/// Resolves the names the template refers to outside of expressions: tags,
/// custom directives, slots and `is` values.
pub(crate) fn collect_markup_references(builder: &mut ModelBuilder, scopes: &TemplateScopes) {
    let Some(template) = builder
        .data
        .blocks
        .iter()
        .find(|block| block.kind == BlockKind::Template)
        .map(|block| builder.data.elements[block.element.index()].range)
    else {
        return;
    };
    let component = scopes.component;
    for index in 0..builder.data.elements.len() {
        let element = ElementId::new(index);
        let data = &builder.data.elements[index];
        if data.parent.is_none() || !template.contains_range(data.range) {
            continue;
        }
        let tag = data.tag.clone();
        let tag_range = TextRange::at(data.start_tag.start() + TextSize::from(1), tag.text_len());
        let attrs = data.attrs.clone();
        let is_component = data.is_component;

        if is_component {
            let resolution = resolve_component(
                builder,
                scopes,
                tag.text(),
                builder.is_open(component, Namespace::Component),
            );
            let id = builder.add_reference(ReferenceData {
                range: tag_range,
                name: tag.clone(),
                snippet: None,
                site: RefSite::TagName(element),
                namespace: Namespace::Component,
                scope: None,
                base: RefBase::Bare,
                path: Box::default(),
                path_open: false,
                access: Access::Read,
                call_args: None,
                resolution,
            });
            builder.data.elements[index].tag_ref = Some(id);
        }

        if tag.text() == "slot" {
            let handle = SemanticModelView { builder };
            match handle.effective(element, "name") {
                Effective::Dynamic => {
                    if let Some(component) = component {
                        builder.component_mut(component).uses_open[Namespace::Slot.index()] = true;
                    }
                }
                effective => {
                    let (name, range) = match effective {
                        Effective::Static(name, range) => (name, range),
                        _ => (synthetic_name("default"), tag_range),
                    };
                    let resolution =
                        resolve_named(builder, component, Namespace::Slot, name.text());
                    builder.add_reference(ReferenceData {
                        range,
                        name,
                        snippet: None,
                        site: RefSite::SlotName(element),
                        namespace: Namespace::Slot,
                        scope: None,
                        base: RefBase::Bare,
                        path: Box::default(),
                        path_open: false,
                        access: Access::Read,
                        call_args: None,
                        resolution,
                    });
                }
            }
        }

        for attr_index in attrs {
            let attr = AttrId(attr_index);
            let data = &builder.data.attrs[attr.index()];
            match &data.name {
                AttrName::Directive {
                    kind: DirectiveKind::Custom,
                    name,
                    ..
                } => {
                    let name = name.clone();
                    let range = TextRange::at(data.range.start(), name.text_len());
                    let resolution = resolve_directive(
                        builder,
                        scopes,
                        name.text(),
                        builder.is_open(component, Namespace::Directive),
                    );
                    let id = builder.add_reference(ReferenceData {
                        range,
                        name,
                        snippet: None,
                        site: RefSite::DirectiveName(attr),
                        namespace: Namespace::Directive,
                        scope: None,
                        base: RefBase::Bare,
                        path: Box::default(),
                        path_open: false,
                        access: Access::Read,
                        call_args: None,
                        resolution,
                    });
                    if let AttrName::Directive { name_ref, .. } =
                        &mut builder.data.attrs[attr.index()].name
                    {
                        *name_ref = Some(id);
                    }
                }
                AttrName::Plain(name) if name.text() == "is" => {
                    if let (AttrValueData::Static(value), Some(range)) =
                        (&data.value, data.value_range)
                    {
                        let value = value.clone();
                        let resolution = resolve_component(
                            builder,
                            scopes,
                            value.text().strip_prefix("vue:").unwrap_or(value.text()),
                            builder.is_open(component, Namespace::Component),
                        );
                        builder.add_reference(ReferenceData {
                            range,
                            name: value,
                            snippet: None,
                            site: RefSite::IsValue(attr),
                            namespace: Namespace::Component,
                            scope: None,
                            base: RefBase::Bare,
                            path: Box::default(),
                            path_open: false,
                            access: Access::Read,
                            call_args: None,
                            resolution,
                        });
                    }
                }
                AttrName::Directive {
                    kind: DirectiveKind::Bind | DirectiveKind::Is,
                    arg,
                    ..
                } => {
                    // A component chosen at runtime may be any registered
                    // one, so none of them can be called unused.
                    let is_dynamic_is = matches!(
                        &data.name,
                        AttrName::Directive {
                            kind: DirectiveKind::Is,
                            ..
                        }
                    ) || matches!(arg, DirectiveArg::Static(arg) if arg.text() == "is");
                    let is_literal = matches!(
                        data.value,
                        AttrValueData::Expr(snippet)
                            if matches!(builder.data.snippets[snippet.index()].root, RootShape::String(_))
                    );
                    if is_dynamic_is
                        && !is_literal
                        && let Some(component) = component
                    {
                        builder.component_mut(component).uses_open[Namespace::Component.index()] =
                            true;
                    }
                }
                _ => {}
            }
        }
    }
}

/// The value of an attribute while the model is still being built.
enum Effective {
    Absent,
    Static(TokenText, TextRange),
    Dynamic,
}

struct SemanticModelView<'a> {
    builder: &'a ModelBuilder,
}

impl SemanticModelView<'_> {
    fn effective(&self, element: ElementId, name: &str) -> Effective {
        for index in self.builder.data.elements[element.index()].attrs.clone() {
            let attr = &self.builder.data.attrs[index as usize];
            match &attr.name {
                AttrName::Plain(plain) if plain.text() == name => {
                    return match (&attr.value, attr.value_range) {
                        (AttrValueData::Static(text), Some(range)) => {
                            Effective::Static(text.clone(), range)
                        }
                        _ => Effective::Absent,
                    };
                }
                AttrName::Directive {
                    kind: DirectiveKind::Bind,
                    arg: DirectiveArg::Static(arg),
                    ..
                } if arg.text() == name => {
                    if let (AttrValueData::Expr(snippet), Some(range)) =
                        (&attr.value, attr.value_range)
                        && let RootShape::String(text) =
                            &self.builder.data.snippets[snippet.index()].root
                    {
                        return Effective::Static(synthetic_name(text), range);
                    }
                    return Effective::Dynamic;
                }
                _ => {}
            }
        }
        Effective::Absent
    }
}

fn resolve_named(
    builder: &ModelBuilder,
    component: Option<ComponentId>,
    namespace: Namespace,
    name: &str,
) -> Resolution {
    builder.resolve_declaration(component, namespace, |symbol| symbol.name.text() == name)
}

/// Finds a top-level script binding the template can see under one of
/// `names`, ignoring bindings that only exist as types.
fn module_binding(
    builder: &ModelBuilder,
    scopes: &TemplateScopes,
    names: &[String],
) -> Option<SymbolId> {
    scopes.modules.iter().find_map(|scope| {
        builder.data.scopes[scope.index()]
            .symbols
            .iter()
            .copied()
            .find(|id| {
                let symbol = &builder.data.symbols[id.index()];
                !symbol.type_only && names.iter().any(|name| name == symbol.name.text())
            })
    })
}

fn resolve_component(
    builder: &ModelBuilder,
    scopes: &TemplateScopes,
    tag: &str,
    open: bool,
) -> Resolution {
    // `<Card.Header>` is a member of the binding `Card`.
    let head = tag.split('.').next().unwrap_or(tag);
    let names = asset_names(head);
    if let Some(symbol) = module_binding(builder, scopes, &names) {
        return Resolution::Symbol(symbol);
    }
    let registration =
        builder.resolve_declaration(scopes.component, Namespace::Component, |symbol| {
            names.iter().any(|name| name == symbol.name.text())
        });
    if matches!(registration, Resolution::Symbol(_)) {
        return registration;
    }
    // A component can render itself under its own name.
    if scopes.component.is_some_and(|component| {
        builder.data.components[component.index()]
            .name
            .as_ref()
            .is_some_and(|(name, _)| names.iter().any(|candidate| candidate == name.text()))
    }) {
        return Resolution::Global;
    }
    if BUILTIN_COMPONENTS.contains(&tag) {
        Resolution::Global
    } else if open {
        Resolution::Unknowable
    } else {
        Resolution::Unresolved
    }
}

fn resolve_directive(
    builder: &ModelBuilder,
    scopes: &TemplateScopes,
    directive: &str,
    open: bool,
) -> Resolution {
    let Some(name) = directive.strip_prefix("v-") else {
        return Resolution::Unresolved;
    };
    // `v-click-outside` is the `<script setup>` binding `vClickOutside`.
    let binding = [format!("v{}", Case::Pascal.convert(name))];
    if let Some(symbol) = module_binding(builder, scopes, &binding) {
        return Resolution::Symbol(symbol);
    }
    let names = asset_names(name);
    let registration =
        builder.resolve_declaration(scopes.component, Namespace::Directive, |symbol| {
            names.iter().any(|name| name == symbol.name.text())
        });
    if matches!(registration, Resolution::Symbol(_)) {
        return registration;
    }
    if open {
        Resolution::Unknowable
    } else {
        Resolution::Unresolved
    }
}
