use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBindingPattern, AnyJsImportClause, AnyJsModuleItem,
    AnyJsObjectBindingPatternMember, AnyJsRoot, JsImport, JsModuleItemList, JsVariableStatement,
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
                    if let Some(module_item) = JsModuleItemList::cast(node) {
                        self.visit_module_item_list(module_item);
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }
    }

    fn visit_module_item_list(&mut self, list: JsModuleItemList) {
        for item in list {
            match item {
                AnyJsModuleItem::AnyJsStatement(statement) => {
                    if let Some(variable_statement) = statement.as_js_variable_statement() {
                        self.visit_js_variable_statement(variable_statement.clone());
                    }
                }
                AnyJsModuleItem::JsExport(_) => {}
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

        if let AnyJsImportClause::JsImportDefaultClause(import) = clause {
            let name = import.default_specifier().ok()?;
            let name = name.local_name().ok()?;
            let name = name.as_js_identifier_binding()?;
            let name = name.name_token().ok()?;
            self.js_bindings
                .insert(name.text_trimmed_range(), name.token_text_trimmed());
        }

        Some(())
    }

    fn visit_js_variable_statement(&mut self, statement: JsVariableStatement) -> Option<()> {
        let declaration = statement.declaration().ok()?;
        for declarator in declaration.declarators().iter().flatten() {
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

    fn visit_any_js_binding_pattern(
        &mut self,
        mut queue: VecDeque<AnyJsBindingPattern>,
    ) -> Option<()> {
        while let Some(pattern) = queue.pop_front() {
            match pattern {
                AnyJsBindingPattern::AnyJsBinding(binding) => {
                    let identifier = binding.as_js_identifier_binding()?;
                    let token = identifier.name_token().ok()?;
                    self.js_bindings
                        .insert(token.text_trimmed_range(), token.token_text_trimmed());
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
}
