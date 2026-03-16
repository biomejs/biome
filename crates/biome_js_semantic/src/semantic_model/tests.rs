#[cfg(test)]
mod test {
    use crate::{
        BindingExtensions, CanBeImportedExported, SemanticFlavor, SemanticModelOptions,
        SemanticScopeExtensions, semantic_model,
    };
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::{
        JsFileSource, JsIdentifierAssignment, JsIdentifierBinding, JsReferenceIdentifier,
        JsSyntaxKind, TsIdentifierBinding,
    };
    use biome_rowan::{AstNode, SyntaxNodeCast};

    fn svelte_options() -> SemanticModelOptions {
        SemanticModelOptions {
            flavor: SemanticFlavor::Svelte,
            ..SemanticModelOptions::default()
        }
    }

    #[test]
    pub fn ok_semantic_model() {
        let r = biome_js_parser::parse(
            "function f(){let a = arguments[0]; let b = a + 1; b = 2; console.log(b)}",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&r.tree(), SemanticModelOptions::default());

        let arguments_reference = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsReferenceIdentifier>())
            .find(|x| x.to_trimmed_string() == "arguments")
            .unwrap();

        let b_from_b_equals_2 = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsIdentifierAssignment>())
            .find(|x| x.to_trimmed_string() == "b")
            .unwrap();

        // Scope hierarchy  navigation

        let block_scope = arguments_reference.scope(&model);
        let func_scope = block_scope.parent().unwrap();
        let global_scope = func_scope.parent().unwrap();

        assert!(global_scope.parent().is_none());
        assert_eq!(global_scope, model.global_scope());
        assert_eq!(block_scope.ancestors().count(), 3);

        // Scope equality

        assert_eq!(block_scope, block_scope);
        assert_eq!(func_scope, func_scope);
        assert_eq!(global_scope, global_scope);

        assert_ne!(block_scope, func_scope);
        assert_ne!(block_scope, global_scope);

        // Bindings

        // block scope must have two bindings: a and b
        let bindings = block_scope.bindings().collect::<Vec<_>>();
        match bindings.as_slice() {
            [a, b] => {
                assert_eq!("a", a.syntax().text_trimmed());
                assert_eq!("b", b.syntax().text_trimmed());
            }
            _ => {
                panic!("wrong number of bindings");
            }
        }

        // function scope must have zero bindings
        // "f" was actually hoisted to the global scope
        let mut bindings = func_scope.bindings();
        assert!(bindings.next().is_none());
        assert!(global_scope.get_binding("f").is_some());

        // Binding by name

        let binding = block_scope.get_binding("arguments");
        assert!(binding.is_none());

        let binding = block_scope.get_binding("a").unwrap();
        assert_eq!("a", binding.syntax().text_trimmed());

        // Declaration (from Read reference)

        let arguments_declaration = arguments_reference.binding(&model);
        assert!(arguments_declaration.is_none());

        let a_from_a_plus_1 = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsReferenceIdentifier>())
            .find(|x| x.to_trimmed_string() == "a")
            .unwrap();

        let a_declaration = a_from_a_plus_1.binding(&model).unwrap();
        assert_eq!("a", a_declaration.syntax().text_trimmed());

        // Declarations (from Write reference)

        let b_declaration = b_from_b_equals_2.binding(&model).unwrap();
        assert_eq!("b", b_declaration.syntax().text_trimmed());

        // All references

        assert_eq!(1, a_declaration.all_references().count());
        assert_eq!(1, a_declaration.all_reads().count());
        assert!(a_declaration.all_reads().all(|r| r.is_read()));
        assert!(a_declaration.all_writes().all(|r| r.is_write()));

        assert_eq!(2, b_declaration.all_references().count());
        assert_eq!(1, b_declaration.all_reads().count());
        assert_eq!(1, b_declaration.all_writes().count());
        assert!(b_declaration.all_reads().all(|r| r.is_read()));
        assert!(b_declaration.all_writes().all(|r| r.is_write()));
    }

    #[test]
    pub fn ok_semantic_model_function_scope() {
        let r = biome_js_parser::parse(
            "function f() {} function g() {}",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&r.tree(), SemanticModelOptions::default());

        let function_f = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsIdentifierBinding>())
            .find(|x| x.to_trimmed_string() == "f")
            .unwrap();

        let function_g = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsIdentifierBinding>())
            .find(|x| x.to_trimmed_string() == "g")
            .unwrap();

        // "f" and "g" tokens are not in the same scope, because
        // the keyword "function" starts a new scope
        // but they are both hoisted to the same scope
        assert_ne!(function_f.scope(&model), function_g.scope(&model));
        assert_eq!(
            function_f.scope_hoisted_to(&model),
            function_g.scope_hoisted_to(&model)
        );

        // they are hoisted to the global scope
        let global_scope = model.global_scope();
        assert_eq!(function_f.scope_hoisted_to(&model).unwrap(), global_scope);
        assert_eq!(function_g.scope_hoisted_to(&model).unwrap(), global_scope);

        // And we can find their binding inside the global scope
        assert!(global_scope.get_binding("g").is_some());
        assert!(global_scope.get_binding("f").is_some());
    }

    /// Finds the last time a token named "name" is used and see if its node is marked as exported
    fn assert_is_exported(is_exported: bool, name: &str, code: &str) {
        let r = biome_js_parser::parse(code, JsFileSource::tsx(), JsParserOptions::default());
        let model = semantic_model(&r.tree(), SemanticModelOptions::default());

        let node = r
            .syntax()
            .descendants()
            .filter(|x| x.text_trimmed() == name)
            .last()
            .unwrap();

        match node.kind() {
            JsSyntaxKind::JS_IDENTIFIER_BINDING => {
                let binding = JsIdentifierBinding::cast(node).unwrap();
                // These do the same thing, but with different APIs
                assert!(is_exported == model.is_exported(&binding), "at \"{code}\"");
                assert!(is_exported == binding.is_exported(&model), "at \"{code}\"");
            }
            JsSyntaxKind::TS_IDENTIFIER_BINDING => {
                let binding = TsIdentifierBinding::cast(node).unwrap();
                // These do the same thing, but with different APIs
                assert!(is_exported == model.is_exported(&binding), "at \"{code}\"");
                assert!(is_exported == binding.is_exported(&model), "at \"{code}\"");
            }
            JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
                // Do nothing.
            }
            x => {
                panic!("This node cannot be exported! {x:?}");
            }
        };
    }

    #[test]
    pub fn ok_semantic_model_is_exported() {
        // Variables
        assert_is_exported(false, "A", "const A = 1");
        assert_is_exported(true, "A", "export const A = 1");
        assert_is_exported(true, "A", "const A = 1; export default A");
        assert_is_exported(true, "A", "const A = 1; export {A}");
        assert_is_exported(false, "A", "const A = 1; export {type A}");
        assert_is_exported(false, "A", "const A = 1; export type {A}");

        // Functions
        assert_is_exported(false, "f", "function f() {}");
        assert_is_exported(true, "f", "export function f() {}");
        assert_is_exported(true, "f", "export default function f() {}");
        assert_is_exported(true, "f", "function f() {} export default f");
        assert_is_exported(true, "f", "function f() {} export {f}");
        assert_is_exported(false, "f", "function f() {} export {type f}");
        assert_is_exported(false, "f", "function f() {} export type {f}");
        assert_is_exported(true, "f", "function f() {} export {f as g}");

        // Classes
        assert_is_exported(false, "A", "class A{}");
        assert_is_exported(true, "A", "export class A{}");
        assert_is_exported(true, "A", "export default class A{}");
        assert_is_exported(true, "A", "class A{} export default A");
        assert_is_exported(true, "A", "class A{} export {A}");
        assert_is_exported(true, "A", "class A{} export {type A}");
        assert_is_exported(true, "A", "class A{} export {A as B}");
        assert_is_exported(true, "A", "class A{} export {type A as B}");

        // Interfaces
        assert_is_exported(false, "A", "interface A{}");
        assert_is_exported(true, "A", "export interface A{}");
        assert_is_exported(true, "A", "export default interface A{}");
        assert_is_exported(true, "A", "interface A{} export default A");
        assert_is_exported(true, "A", "interface A{} export {A}");
        assert_is_exported(true, "A", "interface A{} export {type A}");
        assert_is_exported(true, "A", "interface A{} export type {A}");
        assert_is_exported(true, "A", "interface A{} export {A as B}");
        assert_is_exported(true, "A", "interface A{} export {type A as B}");

        // Type Aliases
        assert_is_exported(false, "A", "type A = number;");
        assert_is_exported(true, "A", "export type A = number;");
        assert_is_exported(true, "A", "type A = number; export default A");
        assert_is_exported(true, "A", "type A = number; export {A}");
        assert_is_exported(true, "A", "type A = number; export {type A}");
        assert_is_exported(true, "A", "type A = number; export type {A}");
        assert_is_exported(true, "A", "type A = number; export {A as B}");
        assert_is_exported(true, "A", "type A = number; export {type A as B}");

        // Enums
        assert_is_exported(false, "A", "enum A {};");
        assert_is_exported(true, "A", "export enum A {};");
        assert_is_exported(true, "A", "enum A {}; export default A");
        assert_is_exported(true, "A", "enum A {}; export {A}");
        assert_is_exported(true, "A", "enum A {}; export {type A}");
        assert_is_exported(true, "A", "enum A {}; export type {A}");
        assert_is_exported(true, "A", "enum A {}; export {A as B}");
        assert_is_exported(true, "A", "enum A {}; export {type A as B}");
    }

    #[test]
    pub fn ok_semantic_model_globals() {
        let r = biome_js_parser::parse(
            "console.log()",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let mut options = SemanticModelOptions::default();
        options.globals.insert("console".into());

        let model = semantic_model(&r.tree(), options);

        let globals: Vec<_> = model.all_global_references().collect();

        assert_eq!(globals.len(), 1);
        assert!(globals[0].is_read());
        assert_eq!(globals[0].syntax().text_trimmed(), "console");
    }

    #[test]
    pub fn ok_semantic_model_ts_construct_signature_member() {
        let r = biome_js_parser::parse(
            "export interface TypedEventConstructor<T, I> { new(): Event & TypedEvent; }",
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let _model = semantic_model(&r.tree(), SemanticModelOptions::default());
    }

    #[test]
    pub fn ok_semantic_model_namespace_import_type_unqualified_reference_is_unresolved_and_tracked_for_usage()
     {
        let r = biome_js_parser::parse(
            "import type * as Namespace from \"mod\"; type T = Namespace;",
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&r.tree(), SemanticModelOptions::default());

        let namespace_binding = model
            .global_scope()
            .get_binding("Namespace")
            .expect("expected Namespace binding");
        // TODO(semantic-invalid-namespace-type-ref): This currently tracks a read to keep
        // `noUnusedImports` behavior while still emitting unresolved diagnostics.
        assert_eq!(namespace_binding.all_references().count(), 1);

        let mut unresolved_references = model.all_unresolved_references();
        let unresolved_reference = unresolved_references
            .next()
            .expect("expected one unresolved reference");
        assert_eq!(
            unresolved_reference.tree().syntax().text_trimmed(),
            "Namespace"
        );
        assert!(unresolved_references.next().is_none());
    }

    #[test]
    pub fn ok_semantic_model_svelte_store_dereference() {
        let r = biome_js_parser::parse(
            "const store = 1; $store;",
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&r.tree(), svelte_options());

        assert_eq!(model.all_unresolved_references().count(), 0);
    }

    #[test]
    pub fn ok_semantic_model_svelte_rune_is_not_normalized_as_store() {
        let r = biome_js_parser::parse(
            "const state = 1; $state;",
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&r.tree(), svelte_options());

        let mut unresolved_references = model.all_unresolved_references();
        let unresolved_reference = unresolved_references
            .next()
            .expect("expected one unresolved reference");
        assert_eq!(
            unresolved_reference.tree().syntax().text_trimmed(),
            "$state"
        );
        assert!(unresolved_references.next().is_none());
    }

    #[test]
    pub fn ok_semantic_model_svelte_dollar_identifier_binding_uses_exact_name() {
        let r = biome_js_parser::parse(
            "const $store = 1; $store;",
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&r.tree(), svelte_options());

        assert_eq!(model.all_unresolved_references().count(), 0);
    }

    #[test]
    pub fn ok_semantic_model_svelte_store_dereference_can_resolve_outer_value_binding() {
        let r = biome_js_parser::parse(
            "const store = 1; { type store = number; $store; }",
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&r.tree(), svelte_options());

        assert_eq!(model.all_unresolved_references().count(), 0);
    }

    #[test]
    pub fn ok_semantic_model_svelte_store_dereference_resolves_configured_global() {
        let r = biome_js_parser::parse("$store;", JsFileSource::ts(), JsParserOptions::default());
        let mut options = svelte_options();
        options.globals.insert("store".into());

        let model = semantic_model(&r.tree(), options);

        assert_eq!(model.all_unresolved_references().count(), 0);
        assert_eq!(model.all_global_references().count(), 1);
    }

    #[test]
    pub fn ok_semantic_model_svelte_double_dollar_reference_is_not_normalized() {
        let r = biome_js_parser::parse("$$state;", JsFileSource::ts(), JsParserOptions::default());
        let mut options = svelte_options();
        options.globals.insert("$state".into());

        let model = semantic_model(&r.tree(), options);

        assert_eq!(model.all_unresolved_references().count(), 1);
        assert_eq!(model.all_global_references().count(), 0);
    }

    #[test]
    pub fn ok_semantic_model_svelte_store_assignment_is_not_tracked_as_binding_write() {
        let r = biome_js_parser::parse(
            "const store = 1; $store = 2;",
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&r.tree(), svelte_options());

        let store_binding = model
            .global_scope()
            .get_binding("store")
            .expect("expected store binding");
        assert_eq!(store_binding.all_writes().count(), 0);
        assert_eq!(store_binding.all_reads().count(), 1);

        let assignment = r
            .syntax()
            .descendants()
            .find_map(JsIdentifierAssignment::cast)
            .expect("expected an assignment");
        assert!(model.binding(&assignment).is_none());
    }
}
