use crate::embedded::bindings::EmbeddedBinding;
use crate::embedded::references::EmbeddedValueReference;
use biome_db::ParsedSource;
use biome_html_syntax::{
    AnyHtmlComponentObjectName, AnyHtmlTagName, AnySvelteBindingAssignmentBinding, AnySvelteBlock,
    AnySvelteBlockItem, AnySvelteDestructuredName, AnySvelteEachName, AnyVueVForBinding,
    AnyVueVForBindingListElement, AnyVueVForDestructuredBinding, HtmlElement, HtmlRoot,
    HtmlSelfClosingElement, VueVForIdentifierBinding, VueVForValue,
};
use biome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayBindingPatternElement, AnyJsArrayElement,
    AnyJsAssignmentPattern, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression,
    AnyJsIdentifierUsage, AnyJsModuleItem, AnyJsObjectAssignmentPatternMember,
    AnyJsObjectBindingPatternMember, AnyJsObjectMember, AnyJsRoot, AnyJsStatement,
    AnyTsIdentifierBinding, AnyTsType, JsAssignmentExpression, JsCallExpression, JsExport,
    JsImport, JsModuleItemList, JsReferenceIdentifier, JsStaticMemberExpression,
    JsSvelteSnippetRoot, JsVariableStatement, JsxReferenceIdentifier,
};
use biome_languages::html::HtmlVariant;
use biome_languages::javascript::JsEmbeddingKind;
use biome_languages::{HtmlFileSource, JsFileSource, LanguageDb};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TokenText, WalkEvent};
use std::collections::VecDeque;

#[derive(Debug, Default, Clone, Copy)]
enum EmbeddedBlockKind {
    Svelte(SvelteBlockKind),
    #[default]
    Neutral,
}

#[derive(Debug, Clone, Copy)]
enum SvelteBlockKind {
    Render,
    Snippet,
    Const,
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
            AnySvelteBlock::SvelteRenderBlock(_) => Self::Svelte(SvelteBlockKind::Render),
            AnySvelteBlock::SvelteSnippetBlock(_) => Self::Svelte(SvelteBlockKind::Snippet),
        }
    }
}

#[salsa::tracked(returns(ref))]
pub fn embedded_bindings_from_source(
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
    let mut builder = EmbeddedBindingsBuilder::new();

    if host_file_source.is_vue() {
        builder.visit_vue_html_root(&html_root);
    } else if host_file_source.is_svelte() {
        builder.visit_svelte_html_root(&html_root);
    }

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
            builder.visit_js_source_snippet(
                &snippet.parsed(db).tree(),
                &host_file_source,
                block_kind.as_ref(),
            );
        }
    }

    vec![
        builder
            .js_bindings
            .into_iter()
            .map(|(range, text, source)| EmbeddedBinding {
                range,
                text,
                source,
            })
            .collect(),
    ]
}

#[salsa::tracked(returns(ref))]
pub fn embedded_references_from_source(
    db: &dyn LanguageDb,
    file: ParsedSource,
) -> Vec<Vec<EmbeddedValueReference>> {
    let Some(host_source) = db.source_from_index(file.document_source_index(db)) else {
        return Vec::new();
    };

    let mut references = Vec::new();
    let mut builder = EmbeddedReferencesBuilder::new();

    for snippet in file.snippets(db) {
        let Some(file_source) = db.source_from_index(snippet.document_source_index(db)) else {
            continue;
        };
        let Some(js_file_source) = file_source.to_js_file_source() else {
            continue;
        };
        if !js_file_source.is_embedded_source() {
            builder.visit_non_source_snippet(&snippet.parsed(db).tree());
        }
    }

    if let Some(html_file_source) = host_source.to_html_file_source()
        && html_file_source.supports_components()
    {
        let html_root: HtmlRoot = file.parsed(db).tree();
        builder.visit_html_root(&html_root);
    }

    references.push(
        builder
            .references
            .into_iter()
            .map(|(range, text)| EmbeddedValueReference { range, text })
            .collect(),
    );
    references
}

fn block_kind_from_js_source(source: &JsFileSource) -> Option<EmbeddedBlockKind> {
    match source.as_embedding_kind() {
        JsEmbeddingKind::Svelte {
            is_function_signature: true,
            ..
        } => Some(EmbeddedBlockKind::Svelte(SvelteBlockKind::Snippet)),
        JsEmbeddingKind::Svelte {
            is_const_block: true,
            ..
        } => Some(EmbeddedBlockKind::Svelte(SvelteBlockKind::Const)),
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

#[derive(Debug)]
struct EmbeddedBindingsBuilder {
    js_bindings: Vec<(TextRange, TokenText, Option<TokenText>)>,
}

impl EmbeddedBindingsBuilder {
    fn new() -> Self {
        Self {
            js_bindings: Vec::default(),
        }
    }

    fn register_binding(&mut self, range: TextRange, text: TokenText) {
        self.js_bindings.push((range, text, None));
    }

    fn register_binding_with_source(
        &mut self,
        range: TextRange,
        text: TokenText,
        source: TokenText,
    ) {
        self.js_bindings.push((range, text, Some(source)));
    }

    fn visit_vue_html_root(&mut self, root: &HtmlRoot) {
        for node in root.syntax().descendants() {
            if let Some(value) = VueVForValue::cast_ref(&node) {
                self.visit_vue_v_for_value(&value);
            }
        }
    }

    fn visit_svelte_html_root(&mut self, root: &HtmlRoot) {
        for node in root.syntax().descendants() {
            let Some(block) = AnySvelteBlock::cast_ref(&node) else {
                continue;
            };
            if let AnySvelteBlock::SvelteEachBlock(each_block) = block
                && let Ok(opening_block) = each_block.opening_block()
                && let Some(item) = opening_block.item()
            {
                match item {
                    AnySvelteBlockItem::SvelteEachAsKeyedItem(as_keyed) => {
                        if let Ok(name) = as_keyed.name() {
                            self.register_svelte_each_name_bindings(name);
                        }
                        if let Some(index) = as_keyed.index()
                            && let Ok(value) = index.value()
                            && let Ok(token) = value.ident_token()
                        {
                            self.register_binding(
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
                            self.register_binding(
                                token.text_trimmed_range(),
                                token.token_text_trimmed(),
                            );
                        }
                    }
                }
            }
        }
    }

    fn visit_vue_v_for_value(&mut self, value: &VueVForValue) -> Option<()> {
        let binding = value.binding().ok()?;
        self.visit_vue_v_for_binding(&binding)
    }

    fn visit_vue_v_for_binding(&mut self, binding: &AnyVueVForBinding) -> Option<()> {
        match binding {
            AnyVueVForBinding::VueVForIdentifierBinding(binding) => {
                self.register_vue_v_for_identifier_binding(binding);
            }
            AnyVueVForBinding::VueVForTupleBinding(tuple) => {
                self.visit_vue_v_for_binding(&tuple.value().ok()?);
                if let Some(second) = tuple.second() {
                    self.visit_vue_v_for_binding(&second.binding().ok()?);
                }
                if let Some(third) = tuple.third() {
                    self.visit_vue_v_for_binding(&third.binding().ok()?);
                }
            }
            AnyVueVForBinding::AnyVueVForDestructuredBinding(binding) => {
                self.visit_vue_v_for_destructured_binding(binding);
            }
        }

        Some(())
    }

    fn visit_vue_v_for_destructured_binding(
        &mut self,
        binding: &AnyVueVForDestructuredBinding,
    ) -> Option<()> {
        let bindings = match binding {
            AnyVueVForDestructuredBinding::VueVForArrayBinding(binding) => binding.bindings(),
            AnyVueVForDestructuredBinding::VueVForObjectBinding(binding) => binding.bindings(),
        };

        for binding in bindings.iter().flatten() {
            self.visit_vue_v_for_binding_list_element(&binding);
        }

        Some(())
    }

    fn visit_vue_v_for_binding_list_element(
        &mut self,
        binding: &AnyVueVForBindingListElement,
    ) -> Option<()> {
        match binding {
            AnyVueVForBindingListElement::VueVForIdentifierBinding(binding) => {
                self.register_vue_v_for_identifier_binding(binding);
            }
            AnyVueVForBindingListElement::VueVForObjectPropertyBinding(binding) => {
                self.visit_vue_v_for_binding(&binding.binding().ok()?);
            }
            AnyVueVForBindingListElement::VueVForRestBinding(binding) => {
                self.register_vue_v_for_identifier_binding(&binding.binding().ok()?);
            }
            AnyVueVForBindingListElement::AnyVueVForDestructuredBinding(binding) => {
                self.visit_vue_v_for_destructured_binding(binding);
            }
        }

        Some(())
    }

    fn register_vue_v_for_identifier_binding(
        &mut self,
        binding: &VueVForIdentifierBinding,
    ) -> Option<()> {
        let token = binding.name_token().ok()?;
        self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
    }

    fn register_svelte_each_name_bindings(&mut self, name: AnySvelteEachName) {
        match name {
            AnySvelteEachName::SvelteName(ident) => {
                if let Ok(token) = ident.ident_token() {
                    self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
                }
            }
            AnySvelteEachName::AnySvelteDestructuredName(destructured) => {
                self.register_svelte_destructured_bindings(destructured);
            }
            AnySvelteEachName::HtmlTextExpression(_) => {}
        }
    }

    fn register_svelte_destructured_bindings(
        &mut self,
        destructured: AnySvelteDestructuredName,
    ) -> Option<()> {
        let mut queue: VecDeque<AnySvelteDestructuredName> = VecDeque::new();
        queue.push_back(destructured);

        while let Some(current) = queue.pop_front() {
            let list = match current {
                AnySvelteDestructuredName::SvelteCurlyDestructuredName(n) => n.names(),
                AnySvelteDestructuredName::SvelteSquareDestructuredName(n) => n.names(),
            };
            for binding in list.iter().flatten() {
                match binding {
                    AnySvelteBindingAssignmentBinding::SvelteName(ident) => {
                        let token = ident.ident_token().ok()?;
                        self.register_binding(
                            token.text_trimmed_range(),
                            token.token_text_trimmed(),
                        );
                    }
                    AnySvelteBindingAssignmentBinding::AnySvelteDestructuredName(nested) => {
                        queue.push_back(nested);
                    }
                    AnySvelteBindingAssignmentBinding::SvelteRestBinding(rest) => {
                        let name = rest.name().ok()?;
                        let token = name.ident_token().ok()?;
                        self.register_binding(
                            token.text_trimmed_range(),
                            token.token_text_trimmed(),
                        );
                    }
                    AnySvelteBindingAssignmentBinding::SvelteRenameBinding(rename) => {
                        match rename.name().ok()? {
                            AnySvelteBindingAssignmentBinding::SvelteName(ident) => {
                                let token = ident.ident_token().ok()?;
                                self.register_binding(
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
                                let name = rest.name().ok()?;
                                let token = name.ident_token().ok()?;
                                self.register_binding(
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

    fn visit_js_source_snippet(
        &mut self,
        root: &AnyJsRoot,
        host_file_source: &HtmlFileSource,
        embed_block_kind: Option<&EmbeddedBlockKind>,
    ) {
        let preorder = root.syntax().preorder();

        for event in preorder {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(module_item) = JsModuleItemList::cast_ref(&node) {
                        self.visit_module_item_list(module_item);
                    } else if let Some(expr) = JsCallExpression::cast_ref(&node) {
                        match host_file_source.variant() {
                            HtmlVariant::Standard(_) | HtmlVariant::Astro => {}
                            HtmlVariant::Vue => {
                                self.visit_define_props_call(&expr);
                            }
                            HtmlVariant::Svelte => {
                                self.visit_svelte_block_call_expressions(&expr, embed_block_kind);
                            }
                        }
                    } else if let Some(assign) = JsAssignmentExpression::cast_ref(&node)
                        && host_file_source.is_svelte()
                    {
                        self.visit_svelte_const_assignment(&assign, embed_block_kind);
                    } else if let Some(root) = JsSvelteSnippetRoot::cast_ref(&node)
                        && host_file_source.is_svelte()
                    {
                        self.visit_svelte_snippet_declaration(&root, embed_block_kind);
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }
    }

    fn visit_svelte_const_assignment(
        &mut self,
        assign: &JsAssignmentExpression,
        embed_block_kind: Option<&EmbeddedBlockKind>,
    ) -> Option<()> {
        let EmbeddedBlockKind::Svelte(SvelteBlockKind::Const) = embed_block_kind? else {
            return None;
        };
        let left = assign.left().ok()?;
        let ident = left.as_any_js_assignment()?.as_js_identifier_assignment()?;
        let token = ident.name_token().ok()?;
        self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
    }

    fn visit_svelte_snippet_declaration(
        &mut self,
        root: &JsSvelteSnippetRoot,
        embed_block_kind: Option<&EmbeddedBlockKind>,
    ) -> Option<()> {
        let EmbeddedBlockKind::Svelte(SvelteBlockKind::Snippet) = embed_block_kind? else {
            return None;
        };

        if let Ok(name) = root.name()
            && let Some(name) = name.as_js_identifier_binding()
            && let Ok(token) = name.name_token()
        {
            self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
        }

        if let Ok(parameters) = root.parameters() {
            for param in parameters.items().iter().flatten() {
                if let Some(formal) = param
                    .as_any_js_formal_parameter()
                    .and_then(|fp| fp.as_js_formal_parameter())
                    && let Ok(binding) = formal.binding()
                {
                    self.visit_any_js_binding_pattern(&binding);
                }
            }
        }

        None
    }

    fn visit_svelte_block_call_expressions(
        &mut self,
        call_expression: &JsCallExpression,
        embed_block_kind: Option<&EmbeddedBlockKind>,
    ) -> Option<()> {
        let embed_block_kind = embed_block_kind?;
        match embed_block_kind {
            EmbeddedBlockKind::Svelte(svelte) => match svelte {
                SvelteBlockKind::Render => {
                    let callee = call_expression.callee().ok()?;
                    let ident = callee.as_js_identifier_expression()?;
                    let token = ident.name().ok()?.value_token().ok()?;
                    self.register_binding(token.text_trimmed_range(), token.token_text_trimmed())
                }
                SvelteBlockKind::Snippet => {
                    let callee = call_expression.callee().ok()?;
                    let ident = callee.as_js_identifier_expression()?;
                    let token = ident.name().ok()?.value_token().ok()?;
                    self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());

                    let arguments = call_expression.arguments().ok()?;

                    for argument in arguments.args().iter().flatten() {
                        match argument {
                            AnyJsCallArgument::AnyJsExpression(expr) => {
                                self.visit_svelte_call_bindings(&expr);
                            }
                            AnyJsCallArgument::JsSpread(spread) => {
                                let expr = spread.argument().ok()?;
                                self.visit_svelte_call_bindings(&expr);
                            }
                        }
                    }
                }
                SvelteBlockKind::Const => {}
            },
            EmbeddedBlockKind::Neutral => return None,
        }

        None
    }

    fn visit_svelte_call_bindings(&mut self, expression: &AnyJsExpression) -> Option<()> {
        match expression {
            AnyJsExpression::JsIdentifierExpression(ident) => {
                let token = ident.name().ok()?.value_token().ok()?;
                self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsExpression::JsObjectExpression(object) => {
                for member in object.members().iter().flatten() {
                    match member {
                        AnyJsObjectMember::JsShorthandPropertyObjectMember(prop) => {
                            let token = prop.name().ok()?.value_token().ok()?;
                            self.register_binding(
                                token.text_trimmed_range(),
                                token.token_text_trimmed(),
                            );
                        }
                        AnyJsObjectMember::JsPropertyObjectMember(prop) => {
                            let value = prop.value().ok()?;
                            self.visit_svelte_call_bindings(&value);
                        }
                        AnyJsObjectMember::JsSpread(spread) => {
                            let argument = spread.argument().ok()?;
                            self.visit_svelte_call_bindings(&argument);
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
                        AnyJsArrayElement::AnyJsExpression(expr) => {
                            self.visit_svelte_call_bindings(&expr);
                        }
                        AnyJsArrayElement::JsSpread(spread) => {
                            let argument = spread.argument().ok()?;
                            self.visit_svelte_call_bindings(&argument);
                        }
                        AnyJsArrayElement::JsArrayHole(_) => {}
                    }
                }
            }
            AnyJsExpression::JsAssignmentExpression(assign) => {
                let left = assign.left().ok()?;
                self.visit_svelte_assignment_pattern(left);
            }
            _ => {}
        }

        None
    }

    fn visit_svelte_assignment_pattern(&mut self, pattern: AnyJsAssignmentPattern) -> Option<()> {
        let mut queue: VecDeque<AnyJsAssignmentPattern> = VecDeque::new();
        queue.push_back(pattern);

        while let Some(current) = queue.pop_front() {
            self.process_svelte_assignment_pattern_step(current, &mut queue);
        }

        Some(())
    }

    fn process_svelte_assignment_pattern_step(
        &mut self,
        current: AnyJsAssignmentPattern,
        queue: &mut VecDeque<AnyJsAssignmentPattern>,
    ) -> Option<()> {
        match current {
            AnyJsAssignmentPattern::AnyJsAssignment(assignment) => {
                let ident = assignment.as_js_identifier_assignment()?;
                let token = ident.name_token().ok()?;
                self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsAssignmentPattern::JsObjectAssignmentPattern(object) => {
                for member in object.properties().iter().flatten() {
                    self.process_svelte_object_assignment_member(member, queue);
                }
            }
            AnyJsAssignmentPattern::JsArrayAssignmentPattern(array) => {
                for element in array.elements().iter().flatten() {
                    self.process_svelte_array_assignment_element(element, queue);
                }
            }
        }
        Some(())
    }

    fn process_svelte_object_assignment_member(
        &mut self,
        member: AnyJsObjectAssignmentPatternMember,
        queue: &mut VecDeque<AnyJsAssignmentPattern>,
    ) -> Option<()> {
        match member {
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(prop) => {
                let inner = prop.pattern().ok()?;
                queue.push_back(inner);
            }
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                prop,
            ) => {
                let identifier = prop.identifier().ok()?;
                let token = identifier.name_token().ok()?;
                self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(rest) => {
                let target = rest.target().ok()?;
                let ident = target.as_js_identifier_assignment()?;
                let token = ident.name_token().ok()?;
                self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsObjectAssignmentPatternMember::JsBogusAssignment(_) => {}
        }
        Some(())
    }

    fn process_svelte_array_assignment_element(
        &mut self,
        element: AnyJsArrayAssignmentPatternElement,
        queue: &mut VecDeque<AnyJsAssignmentPattern>,
    ) -> Option<()> {
        match element {
            AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternElement(el) => {
                let inner = el.pattern().ok()?;
                queue.push_back(inner);
            }
            AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(rest) => {
                let inner = rest.pattern().ok()?;
                queue.push_back(inner);
            }
            AnyJsArrayAssignmentPatternElement::JsArrayHole(_) => {}
        }
        Some(())
    }

    fn visit_module_item_list(&mut self, list: JsModuleItemList) {
        for item in list {
            match item {
                AnyJsModuleItem::AnyJsStatement(statement) => match statement {
                    AnyJsStatement::JsVariableStatement(variable_statement) => {
                        self.visit_js_variable_statement(variable_statement.clone());
                    }
                    AnyJsStatement::JsExpressionStatement(expr_statement) => {
                        self.visit_js_expression_statement(expr_statement.clone());
                    }
                    AnyJsStatement::JsFunctionDeclaration(decl) => {
                        self.register_js_binding(decl.id());
                    }
                    AnyJsStatement::JsClassDeclaration(decl) => {
                        self.register_js_binding(decl.id());
                    }
                    AnyJsStatement::TsEnumDeclaration(decl) => {
                        self.register_js_binding(decl.id());
                    }
                    AnyJsStatement::TsInterfaceDeclaration(decl) => {
                        self.register_ts_identifier_binding(decl.id());
                    }
                    AnyJsStatement::TsTypeAliasDeclaration(decl) => {
                        self.register_ts_identifier_binding(decl.binding_identifier());
                    }
                    AnyJsStatement::TsDeclareFunctionDeclaration(decl) => {
                        self.register_js_binding(decl.id());
                    }
                    _ => {}
                },
                AnyJsModuleItem::JsExport(export) => {
                    self.visit_js_export(export);
                }
                AnyJsModuleItem::JsImport(import) => {
                    self.visit_js_import(import);
                }
            }
        }
    }

    fn visit_js_import(&mut self, import: JsImport) -> Option<()> {
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
                self.register_binding(
                    imported_name.text_trimmed_range(),
                    imported_name.token_text_trimmed(),
                );
            }
        }

        if let Some(default_specifiers) = clause.default_specifiers() {
            let local_name = default_specifiers.local_name().ok()?;
            let local_name = local_name.as_js_identifier_binding()?;
            let local_name = local_name.name_token().ok()?;
            self.register_binding(
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

            self.register_binding_with_source(
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

            self.register_binding_with_source(
                name.text_trimmed_range(),
                name.token_text_trimmed(),
                source,
            );
        }

        Some(())
    }

    fn visit_js_export(&mut self, export: JsExport) -> Option<()> {
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
                            self.register_binding(
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
                            self.register_binding(string_lit.range(), inner);
                        }
                    }
                }
                _ => {}
            }
        }

        Some(())
    }

    fn register_js_binding(
        &mut self,
        result: biome_rowan::SyntaxResult<biome_js_syntax::AnyJsBinding>,
    ) -> Option<()> {
        let binding = result.ok()?;
        let identifier = binding.as_js_identifier_binding()?;
        let token = identifier.name_token().ok()?;
        self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
    }

    fn register_ts_identifier_binding(
        &mut self,
        result: biome_rowan::SyntaxResult<AnyTsIdentifierBinding>,
    ) -> Option<()> {
        let binding = result.ok()?;
        let identifier = binding.as_ts_identifier_binding()?;
        let token = identifier.name_token().ok()?;
        self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
    }

    fn visit_js_variable_statement(&mut self, statement: JsVariableStatement) -> Option<()> {
        let declaration = statement.declaration().ok()?;
        for declarator in declaration.declarators().iter().flatten() {
            if let Some(initializer) = declarator.initializer()
                && let Ok(AnyJsExpression::JsCallExpression(call)) = initializer.expression()
            {
                self.visit_define_props_call(&call);
            }

            let id = declarator.id().ok()?;
            self.visit_any_js_binding_pattern(&id)?;
        }

        Some(())
    }

    fn visit_any_js_binding_pattern(&mut self, binding: &AnyJsBindingPattern) -> Option<()> {
        match binding {
            AnyJsBindingPattern::AnyJsBinding(binding) => {
                let identifier = binding.as_js_identifier_binding()?;
                let token = identifier.name_token().ok()?;
                self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsBindingPattern::JsArrayBindingPattern(array_binding_pattern) => {
                for element in array_binding_pattern.elements().iter().flatten() {
                    match element {
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(element) => {
                            self.track_any_js_binding_pattern(VecDeque::from([element
                                .pattern()
                                .ok()?]))?;
                        }
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(rest) => {
                            self.track_any_js_binding_pattern(VecDeque::from([rest
                                .pattern()
                                .ok()?]))?;
                        }
                        AnyJsArrayBindingPatternElement::JsArrayHole(_) => {}
                    }
                }
            }
            AnyJsBindingPattern::JsObjectBindingPattern(object_binding_pattern) => {
                for property in object_binding_pattern.properties().iter().flatten() {
                    self.visit_object_binding_pattern_member(property)?;
                }
            }
        }

        Some(())
    }

    fn visit_js_expression_statement(&mut self, statement: biome_js_syntax::JsExpressionStatement) {
        let Ok(AnyJsExpression::JsCallExpression(call_expression)) = statement.expression() else {
            return;
        };
        self.visit_define_props_call(&call_expression);
    }

    fn visit_define_props_call(&mut self, call_expression: &JsCallExpression) {
        let Ok(callee) = call_expression.callee() else {
            return;
        };

        let callee_text = callee.syntax().text_trimmed();
        if callee_text != "defineProps" {
            return;
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
                    self.register_binding(value.text_trimmed_range(), value.token_text_trimmed());
                }
            }
            return;
        }

        let Ok(arguments) = call_expression.arguments() else {
            return;
        };
        let Some(Ok(first_arg)) = arguments.args().iter().next() else {
            return;
        };
        let Some(first_expr) = first_arg.as_any_js_expression() else {
            return;
        };

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
                        self.register_binding(
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
                        self.register_binding(string_lit.range(), inner);
                    }
                }
            }
            _ => {}
        }
    }

    fn visit_object_binding_pattern_member(
        &mut self,
        property: AnyJsObjectBindingPatternMember,
    ) -> Option<()> {
        match property {
            AnyJsObjectBindingPatternMember::JsBogusBinding(_) => {}
            AnyJsObjectBindingPatternMember::JsMetavariable(_) => {}
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                self.track_any_js_binding_pattern(VecDeque::from([property.pattern().ok()?]))?;
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                let binding = rest.binding().ok()?;
                let binding = binding.as_js_identifier_binding()?;
                let token = binding.name_token().ok()?;
                self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(property) => {
                let identifier = property.identifier().ok()?;
                let identifier = identifier.as_js_identifier_binding()?;
                let token = identifier.name_token().ok()?;
                self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
            }
        }

        Some(())
    }

    fn track_any_js_binding_pattern(
        &mut self,
        mut queue: VecDeque<AnyJsBindingPattern>,
    ) -> Option<()> {
        while let Some(pattern) = queue.pop_front() {
            match pattern {
                AnyJsBindingPattern::AnyJsBinding(binding) => {
                    let identifier = binding.as_js_identifier_binding()?;
                    let token = identifier.name_token().ok()?;
                    self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
                }
                AnyJsBindingPattern::JsArrayBindingPattern(binding_pattern) => {
                    for element in binding_pattern.elements().iter().flatten() {
                        match element {
                            AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(
                                element,
                            ) => {
                                let pattern = element.pattern().ok()?;
                                queue.push_back(pattern);
                            }
                            AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                                rest_element,
                            ) => {
                                let pattern = rest_element.pattern().ok()?;
                                queue.push_back(pattern);
                            }
                            AnyJsArrayBindingPatternElement::JsArrayHole(_) => {}
                        }
                    }
                }
                AnyJsBindingPattern::JsObjectBindingPattern(object) => {
                    for property in object.properties().iter().flatten() {
                        self.visit_object_binding_pattern_member(property)?;
                    }
                }
            }
        }

        Some(())
    }
}

#[derive(Debug)]
struct EmbeddedReferencesBuilder {
    references: Vec<(TextRange, TokenText)>,
}

impl EmbeddedReferencesBuilder {
    fn new() -> Self {
        Self {
            references: Vec::default(),
        }
    }

    fn register_reference(&mut self, range: TextRange, text: TokenText) {
        self.references.push((range, text));
    }

    fn visit_non_source_snippet(&mut self, root: &AnyJsRoot) {
        let preorder = root.syntax().preorder();

        for event in preorder {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(reference) = JsxReferenceIdentifier::cast_ref(&node) {
                        self.visit_jsx_reference_identifier(reference);
                    } else if let Some(reference) = JsReferenceIdentifier::cast_ref(&node) {
                        self.visit_reference_identifier(reference);
                    } else if let Some(member) = JsStaticMemberExpression::cast_ref(&node) {
                        self.visit_static_member_expression(member);
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }
    }

    fn visit_html_root(&mut self, root: &HtmlRoot) {
        for node in root.syntax().descendants() {
            if let Some(element) = HtmlElement::cast_ref(&node) {
                self.visit_html_element(&element);
            }

            if let Some(element) = HtmlSelfClosingElement::cast_ref(&node) {
                self.visit_html_self_closing_element(&element);
            }
        }
    }

    fn visit_html_element(&mut self, element: &HtmlElement) -> Option<()> {
        if element.is_script_tag() || element.is_style_tag() {
            return None;
        }

        let opening = element.opening_element().ok()?;
        let name = opening.name().ok()?;

        self.track_component_reference(&name);

        Some(())
    }

    fn visit_html_self_closing_element(&mut self, element: &HtmlSelfClosingElement) -> Option<()> {
        let name = element.name().ok()?;

        self.track_component_reference(&name);

        Some(())
    }

    fn track_component_reference(&mut self, name: &AnyHtmlTagName) {
        match name {
            AnyHtmlTagName::HtmlComponentName(component) => {
                if let Ok(token) = component.value_token() {
                    self.register_reference(token.text_trimmed_range(), token.token_text_trimmed());
                }
            }
            AnyHtmlTagName::HtmlMemberName(member) => {
                if let Ok(object) = member.object() {
                    self.track_component_object(&object);
                }
            }
            AnyHtmlTagName::HtmlTagName(_) => {}
        }
    }

    fn track_component_object(&mut self, object: &AnyHtmlComponentObjectName) {
        match object {
            AnyHtmlComponentObjectName::HtmlTagName(tag) => {
                if let Ok(token) = tag.value_token() {
                    self.register_reference(token.text_trimmed_range(), token.token_text_trimmed());
                }
            }
            AnyHtmlComponentObjectName::HtmlComponentName(component) => {
                if let Ok(token) = component.value_token() {
                    self.register_reference(token.text_trimmed_range(), token.token_text_trimmed());
                }
            }
            AnyHtmlComponentObjectName::HtmlMemberName(member) => {
                if let Ok(object) = member.object() {
                    self.track_component_object(&object);
                }
            }
        }
    }

    fn visit_jsx_reference_identifier(&mut self, reference: JsxReferenceIdentifier) -> Option<()> {
        let name_token = reference.value_token().ok()?;
        self.register_reference(
            name_token.text_trimmed_range(),
            name_token.token_text_trimmed(),
        );
        Some(())
    }

    fn visit_reference_identifier(&mut self, reference: JsReferenceIdentifier) -> Option<()> {
        let usage = AnyJsIdentifierUsage::from(reference.clone());
        if usage.is_only_type() {
            return None;
        }
        let name_token = reference.value_token().ok()?;
        self.register_reference(
            name_token.text_trimmed_range(),
            name_token.token_text_trimmed(),
        );
        Some(())
    }

    fn visit_static_member_expression(&mut self, member: JsStaticMemberExpression) -> Option<()> {
        let object = member.object().ok()?;
        if let Some(reference) = object.as_js_reference_identifier() {
            self.visit_reference_identifier(reference.clone())?;
        }
        Some(())
    }
}
