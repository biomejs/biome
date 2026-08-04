use crate::embedded::bindings::EmbeddedBinding;
use crate::embedded::references::{EmbeddedTypeReference, EmbeddedValueReference};
use biome_db::ParsedSource;
use biome_html_syntax::{
    AnyHtmlComponentObjectName, AnyHtmlTagName, AnySvelteBindingAssignmentBinding,
    AnySvelteBindingProperty, AnySvelteBlock, AnySvelteBlockItem, AnySvelteDestructuredName,
    AnySvelteDirective, AnySvelteEachName, AnyVueVForBinding, AnyVueVForBindingListElement,
    AnyVueVForDestructuredBinding, HtmlElement, HtmlRoot, HtmlSelfClosingElement,
    VueVForIdentifierBinding, VueVForValue,
};
use biome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayBindingPatternElement, AnyJsArrayElement,
    AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression,
    AnyJsIdentifierUsage, AnyJsModuleItem, AnyJsObjectAssignmentPatternMember,
    AnyJsObjectBindingPatternMember, AnyJsObjectMember, AnyJsRoot, AnyJsStatement,
    AnyTsIdentifierBinding, AnyTsType, JsAssignmentExpression, JsCallExpression, JsExport,
    JsExpressionStatement, JsIdentifierAssignment, JsImport, JsModuleItemList,
    JsReferenceIdentifier, JsStaticMemberExpression, JsSvelteDeclarationRoot, JsSvelteSnippetRoot,
    JsVariableStatement, JsxReferenceIdentifier,
};
use biome_languages::html::HtmlVariant;
use biome_languages::javascript::{JsEmbeddingKind, SvelteEmbeddingKind};
use biome_languages::{HtmlFileSource, JsFileSource, LanguageDb};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TokenText, WalkEvent};
use std::collections::VecDeque;

trait VisitNode {
    type Node: AstNode;

    /// Visits the specified node
    fn visit(node: Self::Node, context: &VisitContext, tracker: &mut impl TrackNode) -> Option<()>;
}

trait TrackNode {
    /// tracks an embedded reference
    fn track_reference(&mut self, reference: EmbeddedValueReference);

    /// Tracks an embedded binding
    fn track_binding(&mut self, binding: EmbeddedBinding);

    /// Tracks and embedded type reference
    fn track_type_reference(&mut self, ty: EmbeddedTypeReference);

    fn track_binding_token(&mut self, range: TextRange, text: TokenText) {
        self.track_binding(EmbeddedBinding {
            range,
            text,
            source: None,
        });
    }

    fn track_binding_token_with_source(
        &mut self,
        range: TextRange,
        text: TokenText,
        source: TokenText,
    ) {
        self.track_binding(EmbeddedBinding {
            range,
            text,
            source: Some(source),
        });
    }

    fn track_reference_token(&mut self, range: TextRange, text: TokenText) {
        self.track_reference(EmbeddedValueReference { range, text });
    }

    fn track_type_reference_token(&mut self, range: TextRange, text: TokenText) {
        self.track_type_reference(EmbeddedTypeReference { range, text });
    }
}

macro_rules! declare_node_visitor {
    ($name:ident($root:ty) { $($visitor:ty),+ $(,)? }) => {
        fn $name(root: &$root, context: &VisitContext, tracker: &mut impl TrackNode) {
            for event in root.syntax().preorder() {
                let WalkEvent::Enter(node) = event else {
                    continue;
                };

                $(
                    if let Some(node) = <<$visitor as VisitNode>::Node>::cast_ref(&node) {
                        let _ = <$visitor as VisitNode>::visit(node, context, tracker);
                    }
                )+
            }
        }
    };
}

struct VisitContext<'a> {
    host_source: &'a HtmlFileSource,
    snippet_source: Option<&'a JsFileSource>,
    svelte_kind: Option<&'a SvelteEmbeddingKind>,
    block_kind: Option<EmbeddedBlockKind>,
    /// Whether it should collect bindings
    collect_bindings: bool,
    /// Whether it should collect references
    collect_references: bool,
}

#[derive(Debug, Default)]
struct EmbeddedTracker {
    bindings: Vec<EmbeddedBinding>,
    value_references: Vec<EmbeddedValueReference>,
    type_references: Vec<EmbeddedTypeReference>,
}

impl TrackNode for EmbeddedTracker {
    fn track_reference(&mut self, reference: EmbeddedValueReference) {
        self.value_references.push(reference);
    }

    fn track_binding(&mut self, binding: EmbeddedBinding) {
        self.bindings.push(binding);
    }

    fn track_type_reference(&mut self, ty: EmbeddedTypeReference) {
        self.type_references.push(ty);
    }
}

declare_node_visitor!(visit_html_nodes(HtmlRoot) {
    VueVForValueVisitor,
    SvelteBlockVisitor,
    HtmlElementVisitor,
    HtmlSelfClosingElementVisitor,
    SvelteDirectiveVisitor,
});

declare_node_visitor!(visit_js_nodes(AnyJsRoot) {
    JsModuleItemListVisitor,
    JsCallExpressionVisitor,
    JsAssignmentExpressionVisitor,
    JsSvelteSnippetRootVisitor,
    JsSvelteDeclarationRootVisitor,
    JsxReferenceIdentifierVisitor,
    JsReferenceIdentifierVisitor,
    JsIdentifierAssignmentVisitor,
    JsStaticMemberExpressionVisitor,
});

#[derive(Debug, Default, Clone, Copy)]
enum EmbeddedBlockKind {
    Svelte(SvelteBlockKind),
    #[default]
    Neutral,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum SvelteBlockKind {
    Render,
    Snippet,
    Const,
    Declaration,
}

impl From<&AnySvelteBlock> for EmbeddedBlockKind {
    fn from(value: &AnySvelteBlock) -> Self {
        match value {
            AnySvelteBlock::SvelteAwaitBlock(_)
            | AnySvelteBlock::SvelteBogusBlock(_)
            | AnySvelteBlock::SvelteDebugBlock(_)
            | AnySvelteBlock::SvelteEachBlock(_)
            | AnySvelteBlock::SvelteHtmlBlock(_)
            | AnySvelteBlock::SvelteIfBlock(_)
            | AnySvelteBlock::SvelteKeyBlock(_) => Self::Neutral,
            AnySvelteBlock::SvelteConstBlock(_) => Self::Svelte(SvelteBlockKind::Const),
            AnySvelteBlock::SvelteDeclarationBlock(_) => Self::Svelte(SvelteBlockKind::Declaration),
            AnySvelteBlock::SvelteRenderBlock(_) => Self::Svelte(SvelteBlockKind::Render),
            AnySvelteBlock::SvelteSnippetBlock(_) => Self::Svelte(SvelteBlockKind::Snippet),
        }
    }
}

fn is_svelte_block(context: &VisitContext, kind: SvelteBlockKind) -> bool {
    matches!(context.block_kind, Some(EmbeddedBlockKind::Svelte(block_kind)) if block_kind == kind)
        || matches!(
            (context.svelte_kind, kind),
            (
                Some(SvelteEmbeddingKind::SnippetSignature),
                SvelteBlockKind::Snippet
            ) | (
                Some(SvelteEmbeddingKind::LegacyConst),
                SvelteBlockKind::Const
            ) | (
                Some(SvelteEmbeddingKind::Declaration),
                SvelteBlockKind::Declaration
            )
        )
}

#[salsa::tracked(returns(ref))]
pub(crate) fn embedded_bindings_from_source(
    db: &dyn LanguageDb,
    file: ParsedSource,
) -> Vec<Vec<EmbeddedBinding>> {
    let Some(host_source) = db.source_from_index(file.document_source_index(db)) else {
        return Vec::new();
    };
    let Some(host_file_source) = host_source.to_html_file_source() else {
        return Vec::new();
    };

    let html_root: HtmlRoot = file.parsed(db).tree();
    let mut tracker = EmbeddedTracker::default();

    visit_html_nodes(
        &html_root,
        &VisitContext {
            host_source: &host_file_source,
            snippet_source: None,
            svelte_kind: None,
            block_kind: None,
            collect_bindings: true,
            collect_references: false,
        },
        &mut tracker,
    );

    for snippet in file.snippets(db) {
        let Some(file_source) = db.source_from_index(snippet.document_source_index(db)) else {
            continue;
        };
        let Some(js_file_source) = file_source.to_js_file_source() else {
            continue;
        };

        if js_file_source.is_embedded_source()
            || host_file_source.is_svelte()
            || is_script_element_snippet(&html_root, snippet.content_range(db))
        {
            let block_kind = block_kind_from_js_source(&js_file_source)
                .or_else(|| block_kind_for_snippet(&html_root, snippet.content_range(db)));
            visit_js_nodes(
                &snippet.parsed(db).tree(),
                &VisitContext {
                    host_source: &host_file_source,
                    snippet_source: Some(&js_file_source),
                    svelte_kind: js_file_source
                        .as_embedding_kind()
                        .as_svelte_embedding_kind(),
                    block_kind,
                    collect_bindings: true,
                    collect_references: false,
                },
                &mut tracker,
            );
        }
    }

    vec![tracker.bindings]
}

#[salsa::tracked(returns(ref))]
pub fn embedded_references_from_source(
    db: &dyn LanguageDb,
    file: ParsedSource,
) -> Vec<Vec<EmbeddedValueReference>> {
    let Some(tracker) = collect_embedded_references(db, file) else {
        return Vec::new();
    };

    vec![tracker.value_references]
}

#[salsa::tracked(returns(ref))]
pub fn embedded_type_references_from_source(
    db: &dyn LanguageDb,
    file: ParsedSource,
) -> Vec<Vec<EmbeddedTypeReference>> {
    let Some(tracker) = collect_embedded_references(db, file) else {
        return Vec::new();
    };

    vec![tracker.type_references]
}

fn collect_embedded_references(db: &dyn LanguageDb, file: ParsedSource) -> Option<EmbeddedTracker> {
    let host_source = db.source_from_index(file.document_source_index(db))?;
    let host_file_source = host_source.to_html_file_source()?;
    let is_svelte = host_file_source.is_svelte();

    let mut tracker = EmbeddedTracker::default();

    for snippet in file.snippets(db) {
        let Some(file_source) = db.source_from_index(snippet.document_source_index(db)) else {
            continue;
        };
        let Some(js_file_source) = file_source.to_js_file_source() else {
            continue;
        };
        // Templates always; for Svelte also the sibling `<script>` blocks.
        // A Svelte component's `<script module>` and `<script>` compile to
        // one module and share a top-level scope, so a binding used only in
        // the other block must still count as used.
        if !js_file_source.is_embedded_source() || is_svelte {
            visit_js_nodes(
                &snippet.parsed(db).tree(),
                &VisitContext {
                    host_source: &host_file_source,
                    snippet_source: Some(&js_file_source),
                    svelte_kind: js_file_source
                        .as_embedding_kind()
                        .as_svelte_embedding_kind(),
                    block_kind: None,
                    collect_bindings: false,
                    collect_references: true,
                },
                &mut tracker,
            );
        }
    }

    if host_file_source.supports_components() {
        let html_root: HtmlRoot = file.parsed(db).tree();
        visit_html_nodes(
            &html_root,
            &VisitContext {
                host_source: &host_file_source,
                snippet_source: None,
                svelte_kind: None,
                block_kind: None,
                collect_bindings: false,
                collect_references: true,
            },
            &mut tracker,
        );
    }

    Some(tracker)
}

fn block_kind_from_js_source(source: &JsFileSource) -> Option<EmbeddedBlockKind> {
    match source.as_embedding_kind() {
        JsEmbeddingKind::Svelte {
            embedding_kind: SvelteEmbeddingKind::SnippetSignature,
            ..
        } => Some(EmbeddedBlockKind::Svelte(SvelteBlockKind::Snippet)),
        JsEmbeddingKind::Svelte {
            embedding_kind: SvelteEmbeddingKind::LegacyConst,
            ..
        } => Some(EmbeddedBlockKind::Svelte(SvelteBlockKind::Const)),
        JsEmbeddingKind::Svelte {
            embedding_kind: SvelteEmbeddingKind::Declaration,
            ..
        } => Some(EmbeddedBlockKind::Svelte(SvelteBlockKind::Declaration)),
        _ => None,
    }
}

fn block_kind_for_snippet(root: &HtmlRoot, content_range: TextRange) -> Option<EmbeddedBlockKind> {
    for node in root.syntax().descendants() {
        let Some(block) = AnySvelteBlock::cast_ref(&node) else {
            continue;
        };
        if block.range().contains_range(content_range) {
            return Some(EmbeddedBlockKind::from(&block));
        }
    }
    None
}

fn is_script_element_snippet(root: &HtmlRoot, content_range: TextRange) -> bool {
    root.syntax().descendants().any(|node| {
        HtmlElement::cast_ref(&node).is_some_and(|element| {
            element.is_script_tag() && element.range().contains_range(content_range)
        })
    })
}

struct VueVForValueVisitor;

impl VisitNode for VueVForValueVisitor {
    type Node = VueVForValue;

    fn visit(
        value: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        if !context.collect_bindings || !context.host_source.is_vue() {
            return None;
        }
        Self::visit_vue_v_for_binding(&value.binding().ok()?, tracker)
    }
}

struct SvelteBlockVisitor;

impl VisitNode for SvelteBlockVisitor {
    type Node = AnySvelteBlock;

    fn visit(
        block: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        if !context.collect_bindings || !context.host_source.is_svelte() {
            return None;
        }
        if let AnySvelteBlock::SvelteEachBlock(each_block) = block
            && let Ok(opening_block) = each_block.opening_block()
            && let Some(item) = opening_block.item()
        {
            match item {
                AnySvelteBlockItem::SvelteEachAsKeyedItem(as_keyed) => {
                    if let Ok(name) = as_keyed.name() {
                        Self::track_svelte_each_name_bindings(name, tracker);
                    }
                    if let Some(index) = as_keyed.index()
                        && let Ok(value) = index.value()
                        && let Ok(token) = value.ident_token()
                    {
                        tracker.track_binding_token(
                            token.text_trimmed_range(),
                            token.token_text_trimmed(),
                        );
                    }
                }
                AnySvelteBlockItem::SvelteEachKeyedItem(keyed) => {
                    if let Some(index) = keyed.index()
                        && let Ok(value) = index.value()
                        && let Ok(token) = value.ident_token()
                    {
                        tracker.track_binding_token(
                            token.text_trimmed_range(),
                            token.token_text_trimmed(),
                        );
                    }
                }
            }
        }
        Some(())
    }
}

struct JsModuleItemListVisitor;

impl VisitNode for JsModuleItemListVisitor {
    type Node = JsModuleItemList;

    fn visit(list: Self::Node, context: &VisitContext, tracker: &mut impl TrackNode) -> Option<()> {
        if !context.collect_bindings {
            return None;
        }
        for item in list {
            match item {
                AnyJsModuleItem::AnyJsStatement(statement) => match statement {
                    AnyJsStatement::JsVariableStatement(variable_statement) => {
                        JsVariableStatementVisitor::visit(
                            variable_statement.clone(),
                            context,
                            tracker,
                        );
                    }
                    AnyJsStatement::JsExpressionStatement(expr_statement) => {
                        JsExpressionStatementVisitor::visit(
                            expr_statement.clone(),
                            context,
                            tracker,
                        );
                    }
                    AnyJsStatement::JsFunctionDeclaration(decl) => {
                        if let Ok(binding) = decl.id() {
                            AnyJsBindingVisitor::visit(binding, context, tracker);
                        }
                    }
                    AnyJsStatement::JsClassDeclaration(decl) => {
                        if let Ok(binding) = decl.id() {
                            AnyJsBindingVisitor::visit(binding, context, tracker);
                        }
                    }
                    AnyJsStatement::TsEnumDeclaration(decl) => {
                        if let Ok(binding) = decl.id() {
                            AnyJsBindingVisitor::visit(binding, context, tracker);
                        }
                    }
                    AnyJsStatement::TsInterfaceDeclaration(decl) => {
                        if let Ok(binding) = decl.id() {
                            AnyTsIdentifierBindingVisitor::visit(binding, context, tracker);
                        }
                    }
                    AnyJsStatement::TsTypeAliasDeclaration(decl) => {
                        if let Ok(binding) = decl.binding_identifier() {
                            AnyTsIdentifierBindingVisitor::visit(binding, context, tracker);
                        }
                    }
                    AnyJsStatement::TsDeclareFunctionDeclaration(decl) => {
                        if let Ok(binding) = decl.id() {
                            AnyJsBindingVisitor::visit(binding, context, tracker);
                        }
                    }
                    _ => {}
                },
                AnyJsModuleItem::JsExport(export) => {
                    JsExportVisitor::visit(export, context, tracker);
                }
                AnyJsModuleItem::JsImport(import) => {
                    JsImportVisitor::visit(import, context, tracker);
                }
            }
        }
        Some(())
    }
}

struct JsCallExpressionVisitor;

impl VisitNode for JsCallExpressionVisitor {
    type Node = JsCallExpression;

    fn visit(call: Self::Node, context: &VisitContext, tracker: &mut impl TrackNode) -> Option<()> {
        if !context.collect_bindings {
            return None;
        }
        match context.host_source.variant() {
            HtmlVariant::Standard(_) | HtmlVariant::Astro => {}
            HtmlVariant::Vue => {
                Self::visit_define_props_call(&call, tracker);
            }
            HtmlVariant::Svelte => {
                Self::visit_svelte_block_call_expression(&call, context, tracker);
            }
            // TODO: Angular support
            HtmlVariant::Angular => {}
        }
        Some(())
    }
}

struct JsAssignmentExpressionVisitor;

impl VisitNode for JsAssignmentExpressionVisitor {
    type Node = JsAssignmentExpression;

    fn visit(
        assign: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        if !context.collect_bindings || !context.host_source.is_svelte() {
            return None;
        }
        if !is_svelte_block(context, SvelteBlockKind::Const) {
            return None;
        }
        let left = assign.left().ok()?;
        let ident = left.as_any_js_assignment()?.as_js_identifier_assignment()?;
        let token = ident.name_token().ok()?;
        tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
    }
}

struct JsSvelteSnippetRootVisitor;

impl VisitNode for JsSvelteSnippetRootVisitor {
    type Node = JsSvelteSnippetRoot;

    fn visit(root: Self::Node, context: &VisitContext, tracker: &mut impl TrackNode) -> Option<()> {
        if !context.collect_bindings || !context.host_source.is_svelte() {
            return None;
        }
        if !is_svelte_block(context, SvelteBlockKind::Snippet) {
            return None;
        }
        if let Ok(name) = root.name()
            && let Some(name) = name.as_js_identifier_binding()
            && let Ok(token) = name.name_token()
        {
            tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
        }
        if let Ok(parameters) = root.parameters() {
            for param in parameters.items().iter().flatten() {
                if let Some(formal) = param
                    .as_any_js_formal_parameter()
                    .and_then(|parameter| parameter.as_js_formal_parameter())
                    && let Ok(binding) = formal.binding()
                {
                    JsBindingPatternVisitor::visit(binding, context, tracker);
                }
            }
        }
        None
    }
}

struct JsSvelteDeclarationRootVisitor;

impl VisitNode for JsSvelteDeclarationRootVisitor {
    type Node = JsSvelteDeclarationRoot;

    fn visit(root: Self::Node, context: &VisitContext, tracker: &mut impl TrackNode) -> Option<()> {
        if !context.collect_bindings || !context.host_source.is_svelte() {
            return None;
        }
        if !is_svelte_block(context, SvelteBlockKind::Declaration) {
            return None;
        }
        for declarator in root.declaration().ok()?.declarators().iter().flatten() {
            JsBindingPatternVisitor::visit(declarator.id().ok()?, context, tracker)?;
        }
        Some(())
    }
}

struct JsImportVisitor;

impl VisitNode for JsImportVisitor {
    type Node = JsImport;

    fn visit(
        import: Self::Node,
        _context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let clause = import.import_clause().ok()?;
        if let Some(named_specifiers) = clause.named_specifiers() {
            let imported_names = named_specifiers
                .specifiers()
                .iter()
                .flatten()
                .map(|specifier| specifier.imported_name());

            for imported_name in imported_names {
                let Some(imported_name) = imported_name else {
                    continue;
                };
                tracker.track_binding_token(
                    imported_name.text_trimmed_range(),
                    imported_name.token_text_trimmed(),
                );
            }
        }

        if let Some(default_specifiers) = clause.default_specifiers() {
            let local_name = default_specifiers.local_name().ok()?;
            let local_name = local_name.as_js_identifier_binding()?;
            let local_name = local_name.name_token().ok()?;
            tracker.track_binding_token(
                local_name.text_trimmed_range(),
                local_name.token_text_trimmed(),
            );
        }

        if let Some(this_import) = clause.as_js_import_default_clause() {
            let name = this_import.default_specifier().ok()?;
            let name = name.local_name().ok()?;
            let name = name.as_js_identifier_binding()?;
            let name = name.name_token().ok()?;
            let source = import.source_text().ok()?;

            tracker.track_binding_token_with_source(
                name.text_trimmed_range(),
                name.token_text_trimmed(),
                source,
            );
        }

        if let Some(this_import) = clause.as_js_import_namespace_clause() {
            let specifier = this_import.namespace_specifier().ok()?;
            let name = specifier.local_name().ok()?;
            let name = name.as_js_identifier_binding()?;
            let name = name.name_token().ok()?;
            let source = import.source_text().ok()?;

            tracker.track_binding_token_with_source(
                name.text_trimmed_range(),
                name.token_text_trimmed(),
                source,
            );
        }

        Some(())
    }
}

struct JsExportVisitor;

impl VisitNode for JsExportVisitor {
    type Node = JsExport;

    fn visit(
        export: Self::Node,
        _context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let clause = export.export_clause().ok()?;
        let default_clause = clause.as_js_export_default_expression_clause()?;
        let expression = default_clause.expression().ok()?;
        let object_expr = expression.as_js_object_expression()?;

        for member in object_expr.members() {
            let props_value = if let Ok(AnyJsObjectMember::JsPropertyObjectMember(prop)) = member
                && let Ok(name) = prop.name()
                && name.name().as_deref() == Some("props")
                && let Ok(props_value) = prop.value()
            {
                props_value
            } else {
                continue;
            };

            match props_value {
                AnyJsExpression::JsObjectExpression(props_object) => {
                    for props_member in props_object.members() {
                        let Ok(AnyJsObjectMember::JsPropertyObjectMember(prop_entry)) =
                            props_member
                        else {
                            continue;
                        };
                        if let Ok(prop_name) = prop_entry.name()
                            && let Some(literal_name) = prop_name.as_js_literal_member_name()
                            && let Ok(token) = literal_name.value()
                        {
                            tracker.track_binding_token(
                                token.text_trimmed_range(),
                                token.token_text_trimmed(),
                            );
                        }
                    }
                }
                AnyJsExpression::JsArrayExpression(props_array) => {
                    use biome_js_syntax::{AnyJsArrayElement, AnyJsLiteralExpression};
                    for element in props_array.elements() {
                        let Ok(AnyJsArrayElement::AnyJsExpression(
                            AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsStringLiteralExpression(string_lit),
                            ),
                        )) = element
                        else {
                            continue;
                        };
                        if let Ok(inner) = string_lit.inner_string_text() {
                            tracker.track_binding_token(string_lit.range(), inner);
                        }
                    }
                }
                _ => {}
            }
        }

        Some(())
    }
}

struct AnyJsBindingVisitor;

impl VisitNode for AnyJsBindingVisitor {
    type Node = AnyJsBinding;

    fn visit(
        binding: Self::Node,
        _context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let identifier = binding.as_js_identifier_binding()?;
        let token = identifier.name_token().ok()?;
        tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
    }
}

struct AnyTsIdentifierBindingVisitor;

impl VisitNode for AnyTsIdentifierBindingVisitor {
    type Node = AnyTsIdentifierBinding;

    fn visit(
        binding: Self::Node,
        _context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let identifier = binding.as_ts_identifier_binding()?;
        let token = identifier.name_token().ok()?;
        tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
    }
}

struct JsVariableStatementVisitor;

impl VisitNode for JsVariableStatementVisitor {
    type Node = JsVariableStatement;

    fn visit(
        statement: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let declaration = statement.declaration().ok()?;
        for declarator in declaration.declarators().iter().flatten() {
            if let Some(initializer) = declarator.initializer()
                && let Ok(AnyJsExpression::JsCallExpression(call)) = initializer.expression()
            {
                JsCallExpressionVisitor::visit_define_props_call(&call, tracker);
            }

            let id = declarator.id().ok()?;
            JsBindingPatternVisitor::visit(id, context, tracker)?;
        }

        Some(())
    }
}

struct JsBindingPatternVisitor;

impl VisitNode for JsBindingPatternVisitor {
    type Node = AnyJsBindingPattern;

    fn visit(
        binding: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        match binding {
            AnyJsBindingPattern::AnyJsBinding(binding) => {
                let identifier = binding.as_js_identifier_binding()?;
                let token = identifier.name_token().ok()?;
                tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsBindingPattern::JsArrayBindingPattern(array_binding_pattern) => {
                for element in array_binding_pattern.elements().iter().flatten() {
                    match element {
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(element) => {
                            Self::visit(element.pattern().ok()?, context, tracker)?;
                        }
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(rest) => {
                            Self::visit(rest.pattern().ok()?, context, tracker)?;
                        }
                        AnyJsArrayBindingPatternElement::JsArrayHole(_) => {}
                    }
                }
            }
            AnyJsBindingPattern::JsObjectBindingPattern(object_binding_pattern) => {
                for property in object_binding_pattern.properties().iter().flatten() {
                    JsObjectBindingPatternMemberVisitor::visit(property, context, tracker)?;
                }
            }
        }

        Some(())
    }
}

struct JsExpressionStatementVisitor;

impl VisitNode for JsExpressionStatementVisitor {
    type Node = JsExpressionStatement;

    fn visit(
        statement: Self::Node,
        _context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let Ok(AnyJsExpression::JsCallExpression(call_expression)) = statement.expression() else {
            return None;
        };
        JsCallExpressionVisitor::visit_define_props_call(&call_expression, tracker)
    }
}

impl JsCallExpressionVisitor {
    fn visit_define_props_call(
        call_expression: &JsCallExpression,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let Ok(callee) = call_expression.callee() else {
            return None;
        };

        let callee_text = callee.syntax().text_trimmed();
        if callee_text != "defineProps" {
            return None;
        }

        if let Some(type_arguments) = call_expression.type_arguments()
            && let Some(Ok(AnyTsType::TsObjectType(object_type))) =
                type_arguments.ts_type_argument_list().iter().next()
        {
            for member in object_type.members() {
                if let biome_js_syntax::AnyTsTypeMember::TsPropertySignatureTypeMember(property) =
                    member
                    && let Ok(name) = property.name()
                    && let Some(literal_name) = name.as_js_literal_member_name()
                    && let Ok(value) = literal_name.value()
                {
                    tracker.track_binding_token(
                        value.text_trimmed_range(),
                        value.token_text_trimmed(),
                    );
                }
            }
            return Some(());
        }

        let Ok(arguments) = call_expression.arguments() else {
            return None;
        };
        let Some(Ok(first_arg)) = arguments.args().iter().next() else {
            return None;
        };
        let first_expr = first_arg.as_any_js_expression()?;

        match first_expr {
            AnyJsExpression::JsObjectExpression(obj) => {
                for member in obj.members() {
                    let Ok(AnyJsObjectMember::JsPropertyObjectMember(prop)) = member else {
                        continue;
                    };
                    if let Ok(name) = prop.name()
                        && let Some(literal_name) = name.as_js_literal_member_name()
                        && let Ok(token) = literal_name.value()
                    {
                        tracker.track_binding_token(
                            token.text_trimmed_range(),
                            token.token_text_trimmed(),
                        );
                    }
                }
            }
            AnyJsExpression::JsArrayExpression(arr) => {
                use biome_js_syntax::{AnyJsArrayElement, AnyJsLiteralExpression};
                for element in arr.elements() {
                    let Ok(AnyJsArrayElement::AnyJsExpression(
                        AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(string_lit),
                        ),
                    )) = element
                    else {
                        continue;
                    };
                    if let Ok(inner) = string_lit.inner_string_text() {
                        tracker.track_binding_token(string_lit.range(), inner);
                    }
                }
            }
            _ => {}
        }
        Some(())
    }
}

struct JsObjectBindingPatternMemberVisitor;

impl VisitNode for JsObjectBindingPatternMemberVisitor {
    type Node = AnyJsObjectBindingPatternMember;

    fn visit(
        property: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        match property {
            AnyJsObjectBindingPatternMember::JsBogusBinding(_) => {}
            AnyJsObjectBindingPatternMember::JsMetavariable(_) => {}
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                JsBindingPatternVisitor::visit(property.pattern().ok()?, context, tracker)?;
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                let binding = rest.binding().ok()?;
                let binding = binding.as_js_identifier_binding()?;
                let token = binding.name_token().ok()?;
                tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(property) => {
                let identifier = property.identifier().ok()?;
                let identifier = identifier.as_js_identifier_binding()?;
                let token = identifier.name_token().ok()?;
                tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
            }
        }

        Some(())
    }
}

struct HtmlElementVisitor;

impl VisitNode for HtmlElementVisitor {
    type Node = HtmlElement;

    fn visit(
        element: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        if !context.collect_references || element.is_script_tag() || element.is_style_tag() {
            return None;
        }
        Self::track_component_reference(&element.opening_element().ok()?.name().ok()?, tracker);
        Some(())
    }
}

struct HtmlSelfClosingElementVisitor;

impl VisitNode for HtmlSelfClosingElementVisitor {
    type Node = HtmlSelfClosingElement;

    fn visit(
        element: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        if !context.collect_references {
            return None;
        }
        HtmlElementVisitor::track_component_reference(&element.name().ok()?, tracker);
        Some(())
    }
}

struct SvelteDirectiveVisitor;

impl VisitNode for SvelteDirectiveVisitor {
    type Node = AnySvelteDirective;

    fn visit(
        directive: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        if !context.collect_references || !context.host_source.is_svelte() {
            return None;
        }
        let value = match directive {
            AnySvelteDirective::SvelteUseDirective(directive) => directive.value().ok()?,
            AnySvelteDirective::SvelteTransitionDirective(directive) => directive.value().ok()?,
            AnySvelteDirective::SvelteInDirective(directive) => directive.value().ok()?,
            AnySvelteDirective::SvelteOutDirective(directive) => directive.value().ok()?,
            AnySvelteDirective::SvelteAnimateDirective(directive) => directive.value().ok()?,
            AnySvelteDirective::SvelteBindDirective(directive) => {
                let value = directive.value().ok()?;
                if value.initializer().is_some() {
                    return None;
                }
                value
            }
            AnySvelteDirective::SvelteStyleDirective(_)
            | AnySvelteDirective::SvelteClassDirective(_) => return None,
        };

        Self::track_svelte_binding_property(value.property().ok(), tracker)
    }
}

impl SvelteDirectiveVisitor {
    fn track_svelte_binding_property(
        property: Option<AnySvelteBindingProperty>,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let token = match property? {
            AnySvelteBindingProperty::SvelteName(name) => name.ident_token().ok()?,
            AnySvelteBindingProperty::SvelteMemberProperty(_)
            | AnySvelteBindingProperty::SvelteLiteral(_) => return None,
        };

        tracker.track_reference_token(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
    }
}

impl HtmlElementVisitor {
    fn track_component_reference(name: &AnyHtmlTagName, tracker: &mut impl TrackNode) {
        match name {
            AnyHtmlTagName::HtmlComponentName(component) => {
                if let Ok(token) = component.value_token() {
                    tracker.track_reference_token(
                        token.text_trimmed_range(),
                        token.token_text_trimmed(),
                    );
                }
            }
            AnyHtmlTagName::HtmlMemberName(member) => {
                if let Ok(object) = member.object() {
                    Self::track_component_object(&object, tracker);
                }
            }
            AnyHtmlTagName::HtmlTagName(_) => {}
        }
    }

    fn track_component_object(object: &AnyHtmlComponentObjectName, tracker: &mut impl TrackNode) {
        match object {
            AnyHtmlComponentObjectName::HtmlTagName(tag) => {
                if let Ok(token) = tag.value_token() {
                    tracker.track_reference_token(
                        token.text_trimmed_range(),
                        token.token_text_trimmed(),
                    );
                }
            }
            AnyHtmlComponentObjectName::HtmlComponentName(component) => {
                if let Ok(token) = component.value_token() {
                    tracker.track_reference_token(
                        token.text_trimmed_range(),
                        token.token_text_trimmed(),
                    );
                }
            }
            AnyHtmlComponentObjectName::HtmlMemberName(member) => {
                if let Ok(object) = member.object() {
                    Self::track_component_object(&object, tracker);
                }
            }
        }
    }
}

struct JsxReferenceIdentifierVisitor;

impl VisitNode for JsxReferenceIdentifierVisitor {
    type Node = JsxReferenceIdentifier;

    fn visit(
        reference: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        if !context.collect_references {
            return None;
        }
        let name_token = reference.value_token().ok()?;
        tracker.track_reference_token(
            name_token.text_trimmed_range(),
            name_token.token_text_trimmed(),
        );
        Some(())
    }
}

struct JsReferenceIdentifierVisitor;

impl VisitNode for JsReferenceIdentifierVisitor {
    type Node = JsReferenceIdentifier;

    fn visit(
        reference: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        if !context.collect_references {
            return None;
        }
        Self::track_js_reference_identifier(reference, tracker)
    }
}

impl JsReferenceIdentifierVisitor {
    fn track_js_reference_identifier(
        reference: JsReferenceIdentifier,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let usage = AnyJsIdentifierUsage::from(reference.clone());
        let name_token = reference.value_token().ok()?;
        if usage.is_only_type() {
            tracker.track_type_reference_token(
                name_token.text_trimmed_range(),
                name_token.token_text_trimmed(),
            );
        } else {
            tracker.track_reference_token(
                name_token.text_trimmed_range(),
                name_token.token_text_trimmed(),
            );
        }
        Some(())
    }
}

struct JsIdentifierAssignmentVisitor;

impl VisitNode for JsIdentifierAssignmentVisitor {
    type Node = JsIdentifierAssignment;

    fn visit(
        assignment: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        if !context.collect_references
            || !matches!(
                context.snippet_source?.as_embedding_kind(),
                JsEmbeddingKind::Vue { .. }
            )
        {
            return None;
        }
        let name_token = assignment.name_token().ok()?;
        tracker.track_reference_token(
            name_token.text_trimmed_range(),
            name_token.token_text_trimmed(),
        );
        Some(())
    }
}

struct JsStaticMemberExpressionVisitor;

impl VisitNode for JsStaticMemberExpressionVisitor {
    type Node = JsStaticMemberExpression;

    fn visit(
        member: Self::Node,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        if !context.collect_references {
            return None;
        }
        let object = member.object().ok()?;
        if let Some(reference) = object.as_js_reference_identifier() {
            JsReferenceIdentifierVisitor::track_js_reference_identifier(
                reference.clone(),
                tracker,
            )?;
        }
        Some(())
    }
}

impl VueVForValueVisitor {
    fn visit_vue_v_for_binding(
        binding: &AnyVueVForBinding,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        match binding {
            AnyVueVForBinding::VueVForIdentifierBinding(binding) => {
                Self::track_vue_v_for_identifier_binding(binding, tracker);
            }
            AnyVueVForBinding::VueVForTupleBinding(tuple) => {
                Self::visit_vue_v_for_binding(&tuple.value().ok()?, tracker);
                if let Some(second) = tuple.second() {
                    Self::visit_vue_v_for_binding(&second.binding().ok()?, tracker);
                }
                if let Some(third) = tuple.third() {
                    Self::visit_vue_v_for_binding(&third.binding().ok()?, tracker);
                }
            }
            AnyVueVForBinding::AnyVueVForDestructuredBinding(binding) => {
                Self::visit_vue_v_for_destructured_binding(binding, tracker);
            }
        }
        Some(())
    }

    fn visit_vue_v_for_destructured_binding(
        binding: &AnyVueVForDestructuredBinding,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let bindings = match binding {
            AnyVueVForDestructuredBinding::VueVForArrayBinding(binding) => binding.bindings(),
            AnyVueVForDestructuredBinding::VueVForObjectBinding(binding) => binding.bindings(),
        };
        for binding in bindings.iter().flatten() {
            Self::visit_vue_v_for_binding_list_element(&binding, tracker);
        }
        Some(())
    }

    fn visit_vue_v_for_binding_list_element(
        binding: &AnyVueVForBindingListElement,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        match binding {
            AnyVueVForBindingListElement::VueVForIdentifierBinding(binding) => {
                Self::track_vue_v_for_identifier_binding(binding, tracker);
            }
            AnyVueVForBindingListElement::VueVForObjectPropertyBinding(binding) => {
                Self::visit_vue_v_for_binding(&binding.binding().ok()?, tracker);
            }
            AnyVueVForBindingListElement::VueVForRestBinding(binding) => {
                Self::track_vue_v_for_identifier_binding(&binding.binding().ok()?, tracker);
            }
            AnyVueVForBindingListElement::AnyVueVForDestructuredBinding(binding) => {
                Self::visit_vue_v_for_destructured_binding(binding, tracker);
            }
        }
        Some(())
    }

    fn track_vue_v_for_identifier_binding(
        binding: &VueVForIdentifierBinding,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let token = binding.name_token().ok()?;
        tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
    }
}

impl SvelteBlockVisitor {
    fn track_svelte_each_name_bindings(name: AnySvelteEachName, tracker: &mut impl TrackNode) {
        match name {
            AnySvelteEachName::SvelteName(ident) => {
                if let Ok(token) = ident.ident_token() {
                    tracker.track_binding_token(
                        token.text_trimmed_range(),
                        token.token_text_trimmed(),
                    );
                }
            }
            AnySvelteEachName::AnySvelteDestructuredName(destructured) => {
                Self::track_svelte_destructured_bindings(destructured, tracker);
            }
            AnySvelteEachName::HtmlTextExpression(_) => {}
        }
    }

    fn track_svelte_destructured_bindings(
        destructured: AnySvelteDestructuredName,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let mut queue = VecDeque::from([destructured]);
        while let Some(current) = queue.pop_front() {
            let list = match current {
                AnySvelteDestructuredName::SvelteCurlyDestructuredName(name) => name.names(),
                AnySvelteDestructuredName::SvelteSquareDestructuredName(name) => name.names(),
            };
            for binding in list.iter().flatten() {
                match binding {
                    AnySvelteBindingAssignmentBinding::SvelteName(ident) => {
                        let token = ident.ident_token().ok()?;
                        tracker.track_binding_token(
                            token.text_trimmed_range(),
                            token.token_text_trimmed(),
                        );
                    }
                    AnySvelteBindingAssignmentBinding::AnySvelteDestructuredName(nested) => {
                        queue.push_back(nested);
                    }
                    AnySvelteBindingAssignmentBinding::SvelteRestBinding(rest) => {
                        let token = rest.name().ok()?.ident_token().ok()?;
                        tracker.track_binding_token(
                            token.text_trimmed_range(),
                            token.token_text_trimmed(),
                        );
                    }
                    AnySvelteBindingAssignmentBinding::SvelteRenameBinding(rename) => {
                        match rename.name().ok()? {
                            AnySvelteBindingAssignmentBinding::SvelteName(ident) => {
                                let token = ident.ident_token().ok()?;
                                tracker.track_binding_token(
                                    token.text_trimmed_range(),
                                    token.token_text_trimmed(),
                                );
                            }
                            AnySvelteBindingAssignmentBinding::AnySvelteDestructuredName(
                                nested,
                            ) => {
                                queue.push_back(nested);
                            }
                            AnySvelteBindingAssignmentBinding::SvelteRestBinding(rest) => {
                                let token = rest.name().ok()?.ident_token().ok()?;
                                tracker.track_binding_token(
                                    token.text_trimmed_range(),
                                    token.token_text_trimmed(),
                                );
                            }
                            AnySvelteBindingAssignmentBinding::SvelteRenameBinding(_) => {}
                        }
                    }
                }
            }
        }
        Some(())
    }
}

impl JsCallExpressionVisitor {
    fn visit_svelte_block_call_expression(
        call: &JsCallExpression,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        match context.block_kind.as_ref()? {
            EmbeddedBlockKind::Svelte(SvelteBlockKind::Render) => {
                let token = call
                    .callee()
                    .ok()?
                    .as_js_identifier_expression()?
                    .name()
                    .ok()?
                    .value_token()
                    .ok()?;
                tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
            }
            EmbeddedBlockKind::Svelte(SvelteBlockKind::Snippet) => {
                let token = call
                    .callee()
                    .ok()?
                    .as_js_identifier_expression()?
                    .name()
                    .ok()?
                    .value_token()
                    .ok()?;
                tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
                for argument in call.arguments().ok()?.args().iter().flatten() {
                    match argument {
                        AnyJsCallArgument::AnyJsExpression(expression) => {
                            Self::visit_svelte_call_bindings(&expression, context, tracker);
                        }
                        AnyJsCallArgument::JsSpread(spread) => {
                            Self::visit_svelte_call_bindings(
                                &spread.argument().ok()?,
                                context,
                                tracker,
                            );
                        }
                    }
                }
            }
            EmbeddedBlockKind::Svelte(SvelteBlockKind::Const | SvelteBlockKind::Declaration)
            | EmbeddedBlockKind::Neutral => {}
        }
        None
    }

    fn visit_svelte_call_bindings(
        expression: &AnyJsExpression,
        context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        match expression {
            AnyJsExpression::JsIdentifierExpression(ident) => {
                let token = ident.name().ok()?.value_token().ok()?;
                tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsExpression::JsObjectExpression(object) => {
                for member in object.members().iter().flatten() {
                    match member {
                        AnyJsObjectMember::JsShorthandPropertyObjectMember(property) => {
                            let token = property.name().ok()?.value_token().ok()?;
                            tracker.track_binding_token(
                                token.text_trimmed_range(),
                                token.token_text_trimmed(),
                            );
                        }
                        AnyJsObjectMember::JsPropertyObjectMember(property) => {
                            Self::visit_svelte_call_bindings(
                                &property.value().ok()?,
                                context,
                                tracker,
                            );
                        }
                        AnyJsObjectMember::JsSpread(spread) => {
                            Self::visit_svelte_call_bindings(
                                &spread.argument().ok()?,
                                context,
                                tracker,
                            );
                        }
                        AnyJsObjectMember::JsBogusMember(_)
                        | AnyJsObjectMember::JsGetterObjectMember(_)
                        | AnyJsObjectMember::JsMetavariable(_)
                        | AnyJsObjectMember::JsMethodObjectMember(_)
                        | AnyJsObjectMember::JsSetterObjectMember(_) => {}
                    }
                }
            }
            AnyJsExpression::JsArrayExpression(array) => {
                for element in array.elements().iter().flatten() {
                    match element {
                        AnyJsArrayElement::AnyJsExpression(expression) => {
                            Self::visit_svelte_call_bindings(&expression, context, tracker);
                        }
                        AnyJsArrayElement::JsSpread(spread) => {
                            Self::visit_svelte_call_bindings(
                                &spread.argument().ok()?,
                                context,
                                tracker,
                            );
                        }
                        AnyJsArrayElement::JsArrayHole(_) => {}
                    }
                }
            }
            AnyJsExpression::JsAssignmentExpression(assignment) => {
                JsAssignmentPatternVisitor::visit(assignment.left().ok()?, context, tracker);
            }
            _ => {}
        }
        None
    }
}

struct JsAssignmentPatternVisitor;

impl VisitNode for JsAssignmentPatternVisitor {
    type Node = AnyJsAssignmentPattern;

    fn visit(
        pattern: Self::Node,
        _context: &VisitContext,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        let mut queue = VecDeque::from([pattern]);
        while let Some(current) = queue.pop_front() {
            Self::process_svelte_assignment_pattern_step(current, &mut queue, tracker);
        }
        Some(())
    }
}

impl JsAssignmentPatternVisitor {
    fn process_svelte_assignment_pattern_step(
        current: AnyJsAssignmentPattern,
        queue: &mut VecDeque<AnyJsAssignmentPattern>,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        match current {
            AnyJsAssignmentPattern::AnyJsAssignment(assignment) => {
                let token = assignment
                    .as_js_identifier_assignment()?
                    .name_token()
                    .ok()?;
                tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsAssignmentPattern::JsObjectAssignmentPattern(object) => {
                for member in object.properties().iter().flatten() {
                    Self::process_svelte_object_assignment_member(member, queue, tracker);
                }
            }
            AnyJsAssignmentPattern::JsArrayAssignmentPattern(array) => {
                for element in array.elements().iter().flatten() {
                    Self::process_svelte_array_assignment_element(element, queue);
                }
            }
        }
        Some(())
    }

    fn process_svelte_object_assignment_member(
        member: AnyJsObjectAssignmentPatternMember,
        queue: &mut VecDeque<AnyJsAssignmentPattern>,
        tracker: &mut impl TrackNode,
    ) -> Option<()> {
        match member {
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(property) => {
                queue.push_back(property.pattern().ok()?);
            }
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                property,
            ) => {
                let token = property.identifier().ok()?.name_token().ok()?;
                tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(rest) => {
                let token = rest
                    .target()
                    .ok()?
                    .as_js_identifier_assignment()?
                    .name_token()
                    .ok()?;
                tracker.track_binding_token(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsObjectAssignmentPatternMember::JsBogusAssignment(_) => {}
        }
        Some(())
    }

    fn process_svelte_array_assignment_element(
        element: AnyJsArrayAssignmentPatternElement,
        queue: &mut VecDeque<AnyJsAssignmentPattern>,
    ) -> Option<()> {
        match element {
            AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternElement(element) => {
                queue.push_back(element.pattern().ok()?);
            }
            AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(rest) => {
                queue.push_back(rest.pattern().ok()?);
            }
            AnyJsArrayAssignmentPatternElement::JsArrayHole(_) => {}
        }
        Some(())
    }
}
