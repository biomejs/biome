use crate::embed::types::{EmbedBlockKind, SvelteBlockKind};
use biome_html_syntax::{HtmlFileSource, HtmlVariant};
use biome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayBindingPatternElement, AnyJsArrayElement,
    AnyJsAssignmentPattern, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression,
    AnyJsModuleItem, AnyJsObjectAssignmentPatternMember, AnyJsObjectBindingPatternMember,
    AnyJsObjectMember, AnyJsRoot, AnyJsStatement, AnyTsIdentifierBinding, AnyTsType,
    JsAssignmentExpression, JsCallExpression, JsExport, JsImport, JsModuleItemList,
    JsVariableStatement,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TokenText, WalkEvent};
use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct EmbeddedExportedBindings {
    pub bindings: Vec<Vec<(TextRange, TokenText)>>,
}

#[derive(Debug)]
pub(crate) struct EmbeddedBuilder {
    /// Bindings tracked inside JavaScript snippets.
    js_bindings: Vec<(TextRange, TokenText)>,
}

impl EmbeddedExportedBindings {
    pub(crate) fn builder(&self) -> EmbeddedBuilder {
        EmbeddedBuilder::new()
    }

    pub(crate) fn finish(&mut self, builder: EmbeddedBuilder) {
        self.bindings.push(builder.js_bindings);
    }
}
impl EmbeddedBuilder {
    fn new() -> Self {
        Self {
            js_bindings: Vec::default(),
        }
    }

    pub(crate) fn register_binding(&mut self, range: TextRange, text: TokenText) {
        self.js_bindings.push((range, text));
    }

    /// To call when visiting a source snippet, where bindings are defined.
    pub(crate) fn visit_js_source_snippet(
        &mut self,
        root: &AnyJsRoot,
        host_file_source: &HtmlFileSource,
        embed_block_kind: Option<&EmbedBlockKind>,
    ) {
        let preorder = root.syntax().preorder();

        for event in preorder {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(module_item) = JsModuleItemList::cast_ref(&node) {
                        self.visit_module_item_list(module_item);
                    } else if let Some(expr) = JsCallExpression::cast_ref(&node) {
                        // call expressions might have different semantics based on the host language
                        match host_file_source.variant() {
                            HtmlVariant::Standard(_) => {}
                            HtmlVariant::Astro => {}
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
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }
    }

    /// Registers the left-hand side of a `{@const name = value}` assignment
    /// as a binding. Only runs when the enclosing embed is a Svelte const
    /// block; every other Svelte embed leaves assignments alone.
    fn visit_svelte_const_assignment(
        &mut self,
        assign: &JsAssignmentExpression,
        embed_block_kind: Option<&EmbedBlockKind>,
    ) -> Option<()> {
        let EmbedBlockKind::Svelte(SvelteBlockKind::Const) = embed_block_kind? else {
            return None;
        };
        let left = assign.left().ok()?;
        let ident = left.as_any_js_assignment()?.as_js_identifier_assignment()?;
        let token = ident.name_token().ok()?;
        self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
        Some(())
    }

    /// Visits expression statements that are defined, usually, in `snippet` and `render` functions.
    pub(crate) fn visit_svelte_block_call_expressions(
        &mut self,
        call_expression: &JsCallExpression,
        embed_block_kind: Option<&EmbedBlockKind>,
    ) -> Option<()> {
        let embed_block_kind = embed_block_kind?;
        match embed_block_kind {
            EmbedBlockKind::Svelte(svelte) => match svelte {
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
            EmbedBlockKind::Neutral => return None,
        }

        None
    }

    /// Walks a snippet parameter expression and registers every identifier
    /// that appears at a binding position: plain references, shorthand and
    /// literal-keyed properties of an object pattern, array elements, and
    /// nested destructuring.
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

    /// Walks the left-hand side of a default-initialized snippet parameter
    /// and registers every identifier introduced by it. Covers plain
    /// identifier defaults (`figure(image = fallback)`) as well as object
    /// and array destructure defaults (`figure({ src } = fallback)`,
    /// `figure([{ id } = fallback])`). Nested patterns are walked
    /// iteratively through a work queue — no recursion.
    fn visit_svelte_assignment_pattern(&mut self, pattern: AnyJsAssignmentPattern) -> Option<()> {
        let mut queue: VecDeque<AnyJsAssignmentPattern> = VecDeque::new();
        queue.push_back(pattern);

        while let Some(current) = queue.pop_front() {
            self.process_svelte_assignment_pattern_step(current, &mut queue);
        }

        Some(())
    }

    /// Processes a single `AnyJsAssignmentPattern` entry from the work queue
    /// used by [`Self::visit_svelte_assignment_pattern`]. A failure here only
    /// skips the current entry; the caller continues with the next one.
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

        // Handle default clause using accessors generated by the syntax crate.
        if let Some(import) = clause.as_js_import_default_clause() {
            let name = import.default_specifier().ok()?;
            let name = name.local_name().ok()?;
            let name = name.as_js_identifier_binding()?;
            let name = name.name_token().ok()?;
            self.register_binding(name.text_trimmed_range(), name.token_text_trimmed());
        }

        // Namespace imports: `import * as Foo from "bar"` should register `Foo`.
        if let Some(import) = clause.as_js_import_namespace_clause() {
            let specifier = import.namespace_specifier().ok()?;
            let name = specifier.local_name().ok()?;
            let name = name.as_js_identifier_binding()?;
            let name = name.name_token().ok()?;
            self.register_binding(name.text_trimmed_range(), name.token_text_trimmed());
        }

        Some(())
    }

    /// Handles `export default { props: { ... } }` patterns from Vue Options API.
    /// Extracts prop names from the `props` object and registers them as bindings.
    fn visit_js_export(&mut self, export: JsExport) -> Option<()> {
        // You may be confused as to why we don't use our existing helpers for dealing with vue components from crates/biome_js_analyze/src/frameworks/vue/vue_component.rs
        // The reason is that we don't have access to the semantic model, nor the snippet's file source here, which is required for those helpers. However, using those helpers
        // would be preferable since they could help reduce the duplicated logic for this.

        let clause = export.export_clause().ok()?;

        // Only handle `export default { ... }` patterns
        let default_clause = clause.as_js_export_default_expression_clause()?;
        let expression = default_clause.expression().ok()?;

        // Must be an object expression
        let object_expr = expression.as_js_object_expression()?;

        // Find the "props" property
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
                // `props: { loading: Boolean, ... }` — extract keys
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
                // `props: ['loading', 'disabled']` — extract string literal values
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
                            // Use the string literal's range as a unique key;
                            // only the value (prop name) matters for binding lookups.
                            self.register_binding(string_lit.range(), inner);
                        }
                    }
                }
                _ => {}
            }
        }

        Some(())
    }

    /// Registers the name binding from a `SyntaxResult<AnyJsBinding>`.
    /// Used for `JsFunctionDeclaration::id()`, `JsClassDeclaration::id()`,
    /// `TsEnumDeclaration::id()`, and `TsDeclareFunctionDeclaration::id()`.
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
    /// Registers the name binding from a `SyntaxResult<AnyTsIdentifierBinding>`.
    /// Used for `TsInterfaceDeclaration::id()` and
    /// `TsTypeAliasDeclaration::binding_identifier()`.
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
            // If the initializer is a defineProps(...) call, extract prop names from it.
            if let Some(initializer) = declarator.initializer()
                && let Ok(AnyJsExpression::JsCallExpression(call)) = initializer.expression()
            {
                self.visit_define_props_call(&call);
            }

            let id = declarator.id().ok()?;

            match id {
                AnyJsBindingPattern::AnyJsBinding(binding) => {
                    let identifier = binding.as_js_identifier_binding()?;
                    let token = identifier.name_token().ok()?;
                    self.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
                }
                AnyJsBindingPattern::JsArrayBindingPattern(array_binding_pattern) => {
                    for element in array_binding_pattern.elements().iter().flatten() {
                        match element {
                            AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(
                                element,
                            ) => {
                                self.visit_any_js_binding_pattern(VecDeque::from([element
                                    .pattern()
                                    .ok()?]))?;
                            }
                            AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                                rest,
                            ) => {
                                self.visit_any_js_binding_pattern(VecDeque::from([rest
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
        }

        Some(())
    }

    fn visit_js_expression_statement(&mut self, statement: biome_js_syntax::JsExpressionStatement) {
        let Ok(AnyJsExpression::JsCallExpression(call_expression)) = statement.expression() else {
            return;
        };
        self.visit_define_props_call(&call_expression);
    }

    /// Extracts prop name bindings from a `defineProps(...)` call expression.
    ///
    /// Handles three forms used in Vue `<script setup>`:
    /// - Type-argument: `defineProps<{ title: String }>()`
    /// - Runtime object: `defineProps({ title: String })`
    /// - Runtime array:  `defineProps(['title'])`
    fn visit_define_props_call(&mut self, call_expression: &JsCallExpression) {
        let Ok(callee) = call_expression.callee() else {
            return;
        };

        let callee_text = callee.syntax().text_trimmed();
        // defineProps is a macro used in Vue SFCs to define component props.
        // TODO: only bother with this check in Vue files. Currently, this check applies to all html-ish files.
        if callee_text != "defineProps" {
            return;
        }

        // Type-argument form: defineProps<{ title: String }>()
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

        // Runtime argument forms: defineProps({ ... }) or defineProps([...])
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
            // defineProps({ title: String, likes: Number })
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
            // defineProps(['title', 'likes'])
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
                self.visit_any_js_binding_pattern(VecDeque::from([property.pattern().ok()?]))?;
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

    fn visit_any_js_binding_pattern(
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

#[cfg(test)]
mod tests {
    use crate::embed::types::{EmbedBlockKind, SvelteBlockKind};
    use crate::workspace::document::services::embedded_bindings::{
        EmbeddedBuilder, EmbeddedExportedBindings,
    };
    use biome_html_syntax::HtmlFileSource;
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::{AnyJsRoot, JsFileSource};

    fn parse_js(source: &str) -> AnyJsRoot {
        let result = biome_js_parser::parse(source, JsFileSource::ts(), JsParserOptions::default());
        result.tree()
    }

    fn visit_js_root(
        service: &mut EmbeddedBuilder,
        root: &AnyJsRoot,
        html_file_source: HtmlFileSource,
    ) {
        service.visit_js_source_snippet(root, &html_file_source, Some(&EmbedBlockKind::default()));
    }

    fn visit_snippet_header(service: &mut EmbeddedBuilder, source: &str) {
        service.visit_js_source_snippet(
            &parse_js(source),
            &HtmlFileSource::svelte(),
            Some(&EmbedBlockKind::Svelte(SvelteBlockKind::Snippet)),
        );
    }

    fn visit_render_block(service: &mut EmbeddedBuilder, source: &str) {
        service.visit_js_source_snippet(
            &parse_js(source),
            &HtmlFileSource::svelte(),
            Some(&EmbedBlockKind::Svelte(SvelteBlockKind::Render)),
        );
    }

    fn contains_binding(service: &EmbeddedExportedBindings, binding: &str) -> bool {
        for bindings in service.bindings.iter() {
            if bindings.iter().any(|(_, token)| token.text() == binding) {
                return true;
            }
        }
        false
    }

    #[test]
    fn tracks_import_and_let_js_bindings() {
        let source = r#"import { Component } from "somewhere";
import Component2 from "component.astro"

let variable = "salut";
 "#;

        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());

        service.finish(builder);

        assert!(contains_binding(&service, "Component"));
        assert!(contains_binding(&service, "Component2"));
        assert!(contains_binding(&service, "variable"));
    }

    #[test]
    fn tracks_import_and_binding_patterns() {
        let source = r#"import { Component } from "somewhere";
import Component2 from "component.astro"

let {variable, foo: bar} = {};
let [arr, ...rest] = [];

 "#;
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
        service.finish(builder);

        assert!(contains_binding(&service, "Component"));
        assert!(contains_binding(&service, "Component2"));
        assert!(contains_binding(&service, "variable"));
        assert!(contains_binding(&service, "bar"));
        assert!(contains_binding(&service, "arr"));
        assert!(contains_binding(&service, "rest"));
    }

    #[test]
    fn tracks_multiple_snippets() {
        let source = r#"import { Component } from "somewhere";
import Component2 from "component.astro"

let {variable, foo: bar} = {};
let [arr, ...rest] = [];

 "#;

        let source_2 = r#"import { Alas } from "somewhere";
import Alas2 from "component.astro"

let lorem = "";
 "#;

        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
        visit_js_root(&mut builder, &parse_js(source_2), HtmlFileSource::vue());
        service.finish(builder);

        assert!(contains_binding(&service, "Component"));
        assert!(contains_binding(&service, "Component2"));
        assert!(contains_binding(&service, "variable"));
        assert!(contains_binding(&service, "bar"));
        assert!(contains_binding(&service, "arr"));
        assert!(contains_binding(&service, "rest"));
        assert!(contains_binding(&service, "Alas"));
        assert!(contains_binding(&service, "Alas2"));
        assert!(contains_binding(&service, "lorem"));
    }

    #[test]
    fn tracks_function_declarations() {
        let source = r#"
function buildLink(base: string, path: string): string { return base + path; }
async function fetchData() {}
function* generator() {}
"#;
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
        service.finish(builder);
        assert!(contains_binding(&service, "buildLink"));
        assert!(contains_binding(&service, "fetchData"));
        assert!(contains_binding(&service, "generator"));
    }

    #[test]
    fn tracks_class_declarations() {
        let source = r#"
class MyService {}
abstract class BaseHandler {}
"#;
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
        service.finish(builder);
        assert!(contains_binding(&service, "MyService"));
        assert!(contains_binding(&service, "BaseHandler"));
    }

    #[test]
    fn tracks_typescript_declarations() {
        let source = r#"
type UserId = string;
interface UserProfile { name: string }
enum Direction { Up, Down }
"#;
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
        service.finish(builder);
        assert!(contains_binding(&service, "UserId"));
        assert!(contains_binding(&service, "UserProfile"));
        assert!(contains_binding(&service, "Direction"));
    }

    #[test]
    fn tracks_namespace_imports() {
        let source = r#"import * as Vue from "vue";"#;
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
        service.finish(builder);
        assert!(contains_binding(&service, "Vue"));
    }

    #[test]
    fn tracks_vue_options_api_props_object() {
        let source = r#"
export default {
  props: {
    loading: Boolean,
    disabled: Boolean,
  },
}
"#;
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
        service.finish(builder);
        assert!(contains_binding(&service, "loading"));
        assert!(contains_binding(&service, "disabled"));
    }

    #[test]
    fn tracks_vue_options_api_props_array() {
        let source = r#"
export default {
  props: ['loading', 'disabled'],
}
"#;
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
        service.finish(builder);
        assert!(contains_binding(&service, "loading"));
        assert!(contains_binding(&service, "disabled"));
    }

    #[test]
    fn tracks_define_props_runtime_object() {
        // defineProps({ title: String, likes: Number })
        let source = r#"
defineProps({
  title: String,
  likes: Number,
})
"#;
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
        service.finish(builder);
        assert!(contains_binding(&service, "title"));
        assert!(contains_binding(&service, "likes"));
    }

    #[test]
    fn tracks_svelte_snippet_plain_identifier_params() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "figure(image)");
        service.finish(builder);
        assert!(contains_binding(&service, "figure"));
        assert!(contains_binding(&service, "image"));
    }

    #[test]
    fn tracks_svelte_snippet_object_destructured_params() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "figure({ src, caption })");
        service.finish(builder);
        assert!(contains_binding(&service, "figure"));
        assert!(contains_binding(&service, "src"));
        assert!(contains_binding(&service, "caption"));
    }

    #[test]
    fn tracks_svelte_snippet_array_destructured_params() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "figure([first, second])");
        service.finish(builder);
        assert!(contains_binding(&service, "first"));
        assert!(contains_binding(&service, "second"));
    }

    #[test]
    fn tracks_svelte_snippet_rest_params() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "figure(...rest)");
        service.finish(builder);
        assert!(contains_binding(&service, "rest"));
    }

    #[test]
    fn tracks_svelte_snippet_nested_destructured_params() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "figure({ a: [b, c], ...d })");
        service.finish(builder);
        assert!(contains_binding(&service, "b"));
        assert!(contains_binding(&service, "c"));
        assert!(contains_binding(&service, "d"));
    }

    #[test]
    fn tracks_svelte_snippet_default_value_params() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "figure(image = fallback)");
        service.finish(builder);
        assert!(contains_binding(&service, "figure"));
        assert!(contains_binding(&service, "image"));
    }

    #[test]
    fn tracks_svelte_snippet_object_destructure_default() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "figure({ src, caption } = fallback)");
        service.finish(builder);
        assert!(contains_binding(&service, "figure"));
        assert!(contains_binding(&service, "src"));
        assert!(contains_binding(&service, "caption"));
    }

    #[test]
    fn tracks_svelte_snippet_array_destructure_default() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "figure([first, second] = fallback)");
        service.finish(builder);
        assert!(contains_binding(&service, "figure"));
        assert!(contains_binding(&service, "first"));
        assert!(contains_binding(&service, "second"));
    }

    #[test]
    fn tracks_svelte_snippet_nested_object_destructure_default() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "figure({ item: { src } = fallback })");
        service.finish(builder);
        assert!(contains_binding(&service, "src"));
    }

    #[test]
    fn tracks_svelte_snippet_nested_array_destructure_default() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "figure([{ id } = fallback])");
        service.finish(builder);
        assert!(contains_binding(&service, "id"));
    }

    #[test]
    fn tracks_multiple_svelte_snippet_headers_with_destructured_defaults() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "withPlainDefault(image = fallback)");
        visit_snippet_header(
            &mut builder,
            "withObjectDefault({ src, caption } = fallback)",
        );
        visit_snippet_header(
            &mut builder,
            "withArrayDefault([first, second] = emptyList)",
        );
        service.finish(builder);
        assert!(contains_binding(&service, "withPlainDefault"));
        assert!(contains_binding(&service, "withObjectDefault"));
        assert!(contains_binding(&service, "withArrayDefault"));
    }

    #[test]
    fn tracks_svelte_snippet_object_rest_default() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_snippet_header(&mut builder, "figure({ src, ...rest } = fallback)");
        service.finish(builder);
        assert!(contains_binding(&service, "src"));
        assert!(contains_binding(&service, "rest"));
    }

    #[test]
    fn tracks_svelte_render_block_callee_only() {
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_render_block(&mut builder, "figure(img)");
        service.finish(builder);
        assert!(contains_binding(&service, "figure"));
        assert!(!contains_binding(&service, "img"));
    }

    #[test]
    fn tracks_define_props_runtime_array() {
        // const props = defineProps(['foo'])
        let source = r#"
const props = defineProps(['foo'])
"#;
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
        service.finish(builder);
        assert!(contains_binding(&service, "foo"));
    }
}
