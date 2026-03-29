use biome_html_syntax::{HtmlRoot, SvelteAwaitBlock, SvelteEachBlock, SvelteSnippetBlock};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBindingPattern, AnyJsExpression, AnyJsModuleItem,
    AnyJsObjectBindingPatternMember, AnyJsObjectMember, AnyJsParameter, AnyJsRoot, AnyJsStatement,
    AnyTsIdentifierBinding, AnyTsType, JsCallExpression, JsExport, JsFileSource,
    JsFunctionDeclaration, JsImport, JsModuleItemList, JsParameters, JsVariableStatement,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TokenText, WalkEvent};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct EmbeddedExportedBindings {
    pub bindings: Vec<FxHashMap<TextRange, TokenText>>,
}

#[derive(Debug)]
pub(crate) struct EmbeddedBuilder {
    /// Bindings tracked inside JavaScript snippets.
    js_bindings: FxHashMap<TextRange, TokenText>,
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
            js_bindings: FxHashMap::default(),
        }
    }

    /// To call when visiting a source snippet, where bindings are defined.
    pub(crate) fn visit_js_source_snippet(&mut self, root: &AnyJsRoot) {
        let preorder = root.syntax().preorder();

        for event in preorder {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(module_item) = JsModuleItemList::cast_ref(&node) {
                        self.visit_module_item_list(module_item);
                    } else if let Some(expr) = JsCallExpression::cast_ref(&node) {
                        self.visit_define_props_call(&expr);
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }
    }

    /// To call when visiting an HTML root, where Svelte blocks can define bindings
    /// that are visible from nested template expressions.
    pub(crate) fn visit_html_root(&mut self, root: &HtmlRoot) {
        for node in root.syntax().descendants() {
            if let Some(each_block) = SvelteEachBlock::cast_ref(&node) {
                self.visit_svelte_each_block(each_block);
            } else if let Some(await_block) = SvelteAwaitBlock::cast_ref(&node) {
                self.visit_svelte_await_block(await_block);
            } else if let Some(snippet_block) = SvelteSnippetBlock::cast_ref(&node) {
                self.visit_svelte_snippet_block(snippet_block);
            }
        }
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
                self.js_bindings.insert(
                    imported_name.text_trimmed_range(),
                    imported_name.token_text_trimmed(),
                );
            }
        }

        // Handle default clause using accessors generated by the syntax crate.
        if let Some(import) = clause.as_js_import_default_clause() {
            let name = import.default_specifier().ok()?;
            let name = name.local_name().ok()?;
            let name = name.as_js_identifier_binding()?;
            let name = name.name_token().ok()?;
            self.js_bindings
                .insert(name.text_trimmed_range(), name.token_text_trimmed());
        }

        // Namespace imports: `import * as Foo from "bar"` should register `Foo`.
        if let Some(import) = clause.as_js_import_namespace_clause() {
            let specifier = import.namespace_specifier().ok()?;
            let name = specifier.local_name().ok()?;
            let name = name.as_js_identifier_binding()?;
            let name = name.name_token().ok()?;
            self.js_bindings
                .insert(name.text_trimmed_range(), name.token_text_trimmed());
        }

        Some(())
    }

    fn visit_svelte_each_block(&mut self, block: &SvelteEachBlock) {
        let Some(opening_block) = block.opening_block().ok() else {
            return;
        };
        let Some(item) = opening_block.item() else {
            return;
        };

        if let Some(each_item) = item.as_svelte_each_as_keyed_item() {
            let binding_text = each_item
                .name()
                .ok()
                .map(|name| name.syntax().text_trimmed());
            if let Some(binding_text) = binding_text {
                let binding_text = binding_text.to_string();
                self.track_binding_pattern_text(&binding_text);
            }

            if let Some(index) = each_item.index()
                && let Ok(index_name) = index.value()
                && let Ok(token) = index_name.ident_token()
            {
                self.js_bindings
                    .insert(token.text_trimmed_range(), token.token_text_trimmed());
            }
        } else if let Some(keyed_item) = item.as_svelte_each_keyed_item()
            && let Some(index) = keyed_item.index()
            && let Ok(index_name) = index.value()
            && let Ok(token) = index_name.ident_token()
        {
            self.js_bindings
                .insert(token.text_trimmed_range(), token.token_text_trimmed());
        }
    }

    fn visit_svelte_await_block(&mut self, block: &SvelteAwaitBlock) {
        let Some(opening_block) = block.opening_block().ok() else {
            return;
        };

        if let Some(then_clause) = opening_block.then_clause()
            && let Ok(name) = then_clause.name()
        {
            let binding_text = name.syntax().text_trimmed().to_string();
            self.track_binding_pattern_text(&binding_text);
        }

        if let Some(catch_clause) = opening_block.catch_clause()
            && let Ok(name) = catch_clause.name()
        {
            let binding_text = name.syntax().text_trimmed().to_string();
            self.track_binding_pattern_text(&binding_text);
        }
    }

    fn visit_svelte_snippet_block(&mut self, block: &SvelteSnippetBlock) {
        let Some(opening_block) = block.opening_block().ok() else {
            return;
        };
        let Some(expression) = opening_block.expression().ok() else {
            return;
        };
        let signature_text = expression.syntax().text_trimmed().to_string();
        self.track_svelte_snippet_signature(&signature_text);
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
                            self.js_bindings
                                .insert(token.text_trimmed_range(), token.token_text_trimmed());
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
                            self.js_bindings.insert(string_lit.range(), inner);
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
        self.js_bindings
            .insert(token.text_trimmed_range(), token.token_text_trimmed());
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
        self.js_bindings
            .insert(token.text_trimmed_range(), token.token_text_trimmed());
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
                    self.js_bindings
                        .insert(token.text_trimmed_range(), token.token_text_trimmed());
                }
                AnyJsBindingPattern::JsArrayBindingPattern(array_binding_pattern) => {
                    for element in array_binding_pattern.elements().iter().flatten() {
                        match element {
                            AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(
                                element,
                            ) => {
                                self.visit_any_js_binding_pattern(VecDeque::from([element
                                    .pattern()
                                    .ok()?]));
                            }
                            AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                                rest,
                            ) => {
                                self.visit_any_js_binding_pattern(VecDeque::from([rest
                                    .pattern()
                                    .ok()?]));
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
    fn visit_define_props_call(&mut self, call_expression: &biome_js_syntax::JsCallExpression) {
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
                    self.js_bindings
                        .insert(value.text_trimmed_range(), value.token_text_trimmed());
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
                        self.js_bindings
                            .insert(token.text_trimmed_range(), token.token_text_trimmed());
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
                        self.js_bindings.insert(string_lit.range(), inner);
                    }
                }
            }
            _ => {}
        }
    }

    fn track_binding_pattern_text(&mut self, binding_text: &str) {
        let source = format!("function __biome_placeholder__({binding_text}) {{}}");
        let parsed = parse(&source, JsFileSource::ts(), JsParserOptions::default());
        let Some(function) = parsed
            .tree()
            .syntax()
            .descendants()
            .find_map(JsFunctionDeclaration::cast)
        else {
            return;
        };
        let Some(parameters) = function.parameters().ok() else {
            return;
        };

        self.visit_js_parameters(&parameters);
    }

    fn track_svelte_snippet_signature(&mut self, signature_text: &str) {
        let source = format!("function {signature_text} {{}}");
        let parsed = parse(&source, JsFileSource::ts(), JsParserOptions::default());
        let Some(function) = parsed
            .tree()
            .syntax()
            .descendants()
            .find_map(JsFunctionDeclaration::cast)
        else {
            return;
        };

        self.register_js_binding(function.id());

        let Some(parameters) = function.parameters().ok() else {
            return;
        };

        self.visit_js_parameters(&parameters);
    }

    fn visit_js_parameters(&mut self, parameters: &JsParameters) {
        for parameter in parameters.items().iter().flatten() {
            match parameter {
                AnyJsParameter::AnyJsFormalParameter(formal_parameter) => {
                    let Some(formal_parameter) = formal_parameter.as_js_formal_parameter() else {
                        continue;
                    };
                    let Some(binding) = formal_parameter.binding().ok() else {
                        continue;
                    };
                    self.visit_any_js_binding_pattern(VecDeque::from([binding]));
                }
                AnyJsParameter::JsRestParameter(rest_parameter) => {
                    let Some(binding) = rest_parameter.binding().ok() else {
                        continue;
                    };
                    self.visit_any_js_binding_pattern(VecDeque::from([binding]));
                }
                AnyJsParameter::TsThisParameter(_) => {}
            }
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
                self.visit_any_js_binding_pattern(VecDeque::from([property.pattern().ok()?]));
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                let binding = rest.binding().ok()?;
                let binding = binding.as_js_identifier_binding()?;
                let token = binding.name_token().ok()?;
                self.js_bindings
                    .insert(token.text_trimmed_range(), token.token_text_trimmed());
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(property) => {
                let identifier = property.identifier().ok()?;
                let identifier = identifier.as_js_identifier_binding()?;
                let token = identifier.name_token().ok()?;
                self.js_bindings
                    .insert(token.text_trimmed_range(), token.token_text_trimmed());
            }
        }

        Some(())
    }

    fn visit_any_js_binding_pattern(&mut self, mut queue: VecDeque<AnyJsBindingPattern>) {
        while let Some(pattern) = queue.pop_front() {
            match pattern {
                AnyJsBindingPattern::AnyJsBinding(binding) => {
                    let Some(identifier) = binding.as_js_identifier_binding() else {
                        continue;
                    };
                    let Some(token) = identifier.name_token().ok() else {
                        continue;
                    };
                    self.js_bindings
                        .insert(token.text_trimmed_range(), token.token_text_trimmed());
                }
                AnyJsBindingPattern::JsArrayBindingPattern(binding_pattern) => {
                    for element in binding_pattern.elements().iter().flatten() {
                        match element {
                            AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(
                                element,
                            ) => {
                                let Some(pattern) = element.pattern().ok() else {
                                    continue;
                                };
                                queue.push_back(pattern);
                            }
                            AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                                rest_element,
                            ) => {
                                let Some(pattern) = rest_element.pattern().ok() else {
                                    continue;
                                };
                                queue.push_back(pattern);
                            }
                            AnyJsArrayBindingPatternElement::JsArrayHole(_) => {}
                        }
                    }
                }
                AnyJsBindingPattern::JsObjectBindingPattern(object) => {
                    for property in object.properties().iter().flatten() {
                        self.visit_object_binding_pattern_member(property);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::workspace::document::services::embedded_bindings::{
        EmbeddedBuilder, EmbeddedExportedBindings,
    };
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::{AnyJsRoot, JsFileSource};

    fn parse_js(source: &str) -> AnyJsRoot {
        let result = biome_js_parser::parse(source, JsFileSource::ts(), JsParserOptions::default());
        result.tree()
    }

    fn visit_js_root(service: &mut EmbeddedBuilder, root: &AnyJsRoot) {
        service.visit_js_source_snippet(root);
    }

    fn contains_binding(service: &EmbeddedExportedBindings, binding: &str) -> bool {
        for bindings in service.bindings.iter() {
            if bindings.values().any(|token| token.text() == binding) {
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
        visit_js_root(&mut builder, &parse_js(source));

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
        visit_js_root(&mut builder, &parse_js(source));
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
        visit_js_root(&mut builder, &parse_js(source));
        visit_js_root(&mut builder, &parse_js(source_2));
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
        visit_js_root(&mut builder, &parse_js(source));
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
        visit_js_root(&mut builder, &parse_js(source));
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
        visit_js_root(&mut builder, &parse_js(source));
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
        visit_js_root(&mut builder, &parse_js(source));
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
        visit_js_root(&mut builder, &parse_js(source));
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
        visit_js_root(&mut builder, &parse_js(source));
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
        visit_js_root(&mut builder, &parse_js(source));
        service.finish(builder);
        assert!(contains_binding(&service, "title"));
        assert!(contains_binding(&service, "likes"));
    }

    #[test]
    fn tracks_define_props_runtime_array() {
        // const props = defineProps(['foo'])
        let source = r#"
const props = defineProps(['foo'])
"#;
        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        visit_js_root(&mut builder, &parse_js(source));
        service.finish(builder);
        assert!(contains_binding(&service, "foo"));
    }

    #[test]
    fn tracks_svelte_snippet_bindings_from_html_root() {
        use biome_html_parser::{HtmlParserOptions, parse_html};

        let source = r#"
{#snippet user({ name, age }, ...rest)}
  <p>{name} is {age} years old</p>
{/snippet}

{@render user({ name: "John", age: 42 })}
"#;

        let parsed = parse_html(source, HtmlParserOptions::default().with_svelte());

        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        builder.visit_html_root(&parsed.tree());
        service.finish(builder);

        assert!(contains_binding(&service, "user"));
        assert!(contains_binding(&service, "name"));
        assert!(contains_binding(&service, "age"));
        assert!(contains_binding(&service, "rest"));
    }

    #[test]
    fn tracks_svelte_each_and_await_bindings_from_html_root() {
        use biome_html_parser::{HtmlParserOptions, parse_html};

        let source = r#"
{#each items as { value }, index}
  <span>{value}</span>
{/each}

{#await promise then result catch error}
  <span>{result}{error}</span>
{/await}
"#;

        let parsed = parse_html(source, HtmlParserOptions::default().with_svelte());

        let mut service = EmbeddedExportedBindings::default();
        let mut builder = service.builder();
        builder.visit_html_root(&parsed.tree());
        service.finish(builder);

        assert!(contains_binding(&service, "value"));
        assert!(contains_binding(&service, "index"));
        assert!(contains_binding(&service, "result"));
        assert!(contains_binding(&service, "error"));
    }
}
