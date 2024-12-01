use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

use crate::{
    AnyJsBindingPattern, AnyJsDeclarationClause, AnyJsExportClause, AnyJsExportDefaultDeclaration,
    AnyJsExportNamedSpecifier, AnyJsExpression, AnyTsIdentifierBinding, AnyTsType, JsExport,
    JsExportNamedClause, JsIdentifierExpression, JsLiteralExportName, JsReferenceIdentifier,
    JsSyntaxToken, TsEnumDeclaration,
};

declare_node_union! {
    pub AnyIdentifier = AnyJsBindingPattern | AnyTsIdentifierBinding | JsIdentifierExpression | JsLiteralExportName | JsReferenceIdentifier
}

impl AnyIdentifier {
    pub fn name_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::AnyJsBindingPattern(id) => id
                .as_any_js_binding()?
                .as_js_identifier_binding()?
                .name_token(),
            Self::AnyTsIdentifierBinding(id) => id.as_ts_identifier_binding()?.name_token(),
            Self::JsIdentifierExpression(id) => id.name().ok()?.value_token(),
            Self::JsLiteralExportName(id) => id.value(),
            Self::JsReferenceIdentifier(id) => id.value_token(),
        }
        .ok()
    }
}

declare_node_union! {
    pub AnyJsExported = AnyJsExpression | AnyJsExportClause | AnyIdentifier | AnyTsType | TsEnumDeclaration
}

#[derive(Clone, Debug)]
pub struct ExportedItem {
    // The identifier of the exported object
    pub identifier: Option<AnyIdentifier>,
    // The exported object
    pub exported: Option<AnyJsExported>,
    // Whether it is default exported or not
    pub is_default: bool,
}

impl JsExport {
    /// Returns a list of the exported items.
    /// ## Example
    /// When a named export is made, it returns a list of them.
    /// ```js
    /// export {foo, bar as baz};
    /// ```
    /// will return
    /// ```js
    /// [
    ///     ExportedItem { identifier: Some(AnyIdentifier::JsLiteralExportName("foo")), exported: None, is_default: false },
    ///     ExportedItem { identifier: Some(AnyIdentifier::JsLiteralExportName("baz")), exported: None, is_default: false },
    /// ]
    /// ```
    ///
    ///
    /// When multiple variables are exported, it returns the list of those variables.
    ///
    /// ```js
    /// export const x = 100, y = 200;
    /// ```
    /// will return
    /// ```js
    /// [
    ///     ExportedItem { identifier: Some(AnyIdentifier::AnyJsBindingPattern("x")), exported: Some(AnyJsExported::AnyJsExpression(100)), is_default: false },
    ///     ExportedItem { identifier: Some(AnyIdentifier::AnyJsBindingPattern("y")), exported: Some(AnyJsExported::AnyJsExpression(200)), is_default: false },
    /// ]
    /// ```
    /// When a function is exported, it returns the function name. It also checks whether it is a default export.
    /// ```js
    /// export default function foo() {};
    /// ```
    /// will return
    /// ```js
    /// [
    ///     ExportedItem { identifier: Some(AnyIdentifier::AnyJsBindingPattern("foo")), exported: None, is_default: true },
    /// ]
    /// ```
    pub fn get_exported_items(&self) -> Vec<ExportedItem> {
        self.export_clause()
            .ok()
            .and_then(|export_clause| match export_clause {
                // export const x = 100;
                AnyJsExportClause::AnyJsDeclarationClause(declaration_clause) => {
                    match declaration_clause {
                        // export function foo() {}
                        AnyJsDeclarationClause::JsFunctionDeclaration(
                            function_declaration_clause,
                        ) => function_declaration_clause.id().ok().map(|function_id| {
                            vec![ExportedItem {
                                identifier: Some(AnyIdentifier::AnyJsBindingPattern(
                                    AnyJsBindingPattern::AnyJsBinding(function_id),
                                )),
                                exported: None,
                                is_default: false,
                            }]
                        }),
                        // export const x = 100;
                        AnyJsDeclarationClause::JsVariableDeclarationClause(
                            variable_declaration_clause,
                        ) => variable_declaration_clause.declaration().ok().map(
                            |variable_declaration| {
                                variable_declaration
                                    .declarators()
                                    .into_iter()
                                    .filter_map(|declarator| {
                                        let declarator = declarator.ok()?;
                                        let identifier = declarator.id().ok()?;
                                        let initializer = declarator
                                            .initializer()
                                            .and_then(|init| init.expression().ok());
                                        Some(ExportedItem {
                                            identifier: Some(AnyIdentifier::AnyJsBindingPattern(
                                                identifier,
                                            )),
                                            exported: initializer
                                                .map(AnyJsExported::AnyJsExpression),
                                            is_default: false,
                                        })
                                    })
                                    .collect()
                            },
                        ),
                        // export enum X {}
                        AnyJsDeclarationClause::TsEnumDeclaration(ts_enum_declaration) => {
                            ts_enum_declaration.id().ok().map(|enum_id| {
                                vec![ExportedItem {
                                    identifier: Some(AnyIdentifier::AnyJsBindingPattern(
                                        AnyJsBindingPattern::AnyJsBinding(enum_id),
                                    )),
                                    exported: Some(AnyJsExported::TsEnumDeclaration(
                                        ts_enum_declaration,
                                    )),
                                    is_default: false,
                                }]
                            })
                        }
                        // export type X = number;
                        AnyJsDeclarationClause::TsTypeAliasDeclaration(
                            ts_type_alias_declaration,
                        ) => ts_type_alias_declaration.binding_identifier().ok().map(
                            |type_alias_id| {
                                vec![ExportedItem {
                                    identifier: Some(AnyIdentifier::AnyTsIdentifierBinding(
                                        type_alias_id,
                                    )),
                                    exported: ts_type_alias_declaration
                                        .ty()
                                        .ok()
                                        .map(AnyJsExported::AnyTsType),
                                    is_default: false,
                                }]
                            },
                        ),
                        _ => None,
                    }
                }
                AnyJsExportClause::JsExportDefaultDeclarationClause(default_declaration_clause) => {
                    default_declaration_clause
                        .declaration()
                        .ok()
                        .and_then(|default_declation| match default_declation {
                            // export default function x() {}
                            AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(
                                function_declaration,
                            ) => function_declaration.id(),
                            // export default class x {}
                            AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(
                                class_declaration,
                            ) => class_declaration.id(),
                            _ => None,
                        })
                        .map(|any_js_binding| {
                            vec![ExportedItem {
                                identifier: Some(AnyIdentifier::AnyJsBindingPattern(
                                    AnyJsBindingPattern::AnyJsBinding(any_js_binding),
                                )),
                                exported: None,
                                is_default: true,
                            }]
                        })
                }
                // export default x;
                AnyJsExportClause::JsExportDefaultExpressionClause(clause) => {
                    clause.expression().ok().map(|expression| match expression {
                        AnyJsExpression::JsIdentifierExpression(identifier) => {
                            vec![ExportedItem {
                                identifier: Some(AnyIdentifier::JsIdentifierExpression(identifier)),
                                exported: None,
                                is_default: true,
                            }]
                        }
                        _ => vec![ExportedItem {
                            identifier: None,
                            exported: Some(AnyJsExported::AnyJsExpression(expression)),
                            is_default: true,
                        }],
                    })
                }
                // export { x, y, z };
                AnyJsExportClause::JsExportNamedClause(named_clause) => Some(
                    named_clause
                        .specifiers()
                        .into_iter()
                        .filter_map(|r| r.ok())
                        .filter_map(|export_specifier| match export_specifier {
                            AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(
                                shorthand,
                            ) => shorthand.name().ok().map(|name| ExportedItem {
                                identifier: Some(AnyIdentifier::JsReferenceIdentifier(name)),
                                exported: None,
                                is_default: false,
                            }),
                            AnyJsExportNamedSpecifier::JsExportNamedSpecifier(specifier) => {
                                specifier.exported_name().ok().map(|exported_name| {
                                    if exported_name.to_trimmed_string() == "default" {
                                        return ExportedItem {
                                            identifier: specifier.local_name().ok().map(
                                                |local_name| {
                                                    AnyIdentifier::JsReferenceIdentifier(local_name)
                                                },
                                            ),
                                            exported: None,
                                            is_default: true,
                                        };
                                    }
                                    ExportedItem {
                                        identifier: Some(AnyIdentifier::JsLiteralExportName(
                                            exported_name,
                                        )),
                                        exported: None,
                                        is_default: false,
                                    }
                                })
                            }
                        })
                        .collect(),
                ),
                _ => None,
            })
            .unwrap_or_default()
    }
}

impl AnyJsExportNamedSpecifier {
    /// Type token of the export specifier.
    ///
    /// ```ts
    /// export { type X }
    ///          ^^^^
    /// ```
    pub fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsExportNamedShorthandSpecifier(specifier) => specifier.type_token(),
            Self::JsExportNamedSpecifier(specifier) => specifier.type_token(),
        }
    }

    /// Returns the export clause that includes this specifier.
    pub fn export_named_clause(&self) -> Option<JsExportNamedClause> {
        JsExportNamedClause::cast(self.syntax().grand_parent()?)
    }

    /// Returns `true` if this specifier or its export clause has **only** a type modifier.
    pub fn exports_only_types(&self) -> bool {
        self.type_token().is_some()
            || self
                .export_named_clause()
                .and_then(|x| x.type_token())
                .is_some()
    }

    /// Returns the local name of the export.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{AnyJsExportNamedSpecifier, T};
    /// use biome_js_factory::make;
    ///
    /// let specifier = make::js_export_named_shorthand_specifier(
    ///     make::js_reference_identifier(make::ident("a"))
    /// ).with_type_token(make::token(T![type])).build();
    /// let export = AnyJsExportNamedSpecifier::from(specifier.clone());
    ///
    /// assert_eq!(export.local_name(), specifier.name());
    ///
    /// let specifier = make::js_export_named_specifier(
    ///     make::js_reference_identifier(make::ident("a")),
    ///     make::token(T![as]),
    ///     make::js_literal_export_name(make::ident("b")),
    /// ).build();
    /// let export = AnyJsExportNamedSpecifier::from(specifier.clone());
    ///
    /// assert_eq!(export.local_name(), specifier.local_name());
    /// ```
    pub fn local_name(&self) -> SyntaxResult<JsReferenceIdentifier> {
        match self {
            Self::JsExportNamedShorthandSpecifier(specifier) => specifier.name(),
            Self::JsExportNamedSpecifier(specifier) => specifier.local_name(),
        }
    }

    pub fn with_type_token(self, type_token: Option<JsSyntaxToken>) -> Self {
        match self {
            Self::JsExportNamedShorthandSpecifier(specifier) => {
                specifier.with_type_token(type_token).into()
            }
            Self::JsExportNamedSpecifier(specifier) => specifier.with_type_token(type_token).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use biome_js_factory::syntax::{JsExport, JsSyntaxKind::*};
    use biome_js_factory::JsSyntaxTreeBuilder;
    use biome_rowan::AstNode;

    #[test]
    fn test_get_exported_items() {
        let mut tree_builder = JsSyntaxTreeBuilder::new();
        //export {foo, bar as baz}
        tree_builder.start_node(JS_EXPORT);
        tree_builder.token(EXPORT_KW, "export");
        tree_builder.start_node(JS_EXPORT_NAMED_CLAUSE);
        tree_builder.token(L_CURLY, "{");
        tree_builder.start_node(JS_EXPORT_NAMED_SPECIFIER_LIST);
        // foo
        tree_builder.start_node(JS_EXPORT_NAMED_SHORTHAND_SPECIFIER);
        tree_builder.start_node(JS_REFERENCE_IDENTIFIER);
        tree_builder.token(IDENT, "foo");
        tree_builder.finish_node(); // JS_REFERENCE_IDENTIFIER
        tree_builder.finish_node(); // JS_EXPORT_NAMED_SHORTHAND_SPECIFIER
        tree_builder.token(COMMA, ",");
        // bar as baz
        tree_builder.start_node(JS_EXPORT_NAMED_SPECIFIER);
        tree_builder.start_node(JS_REFERENCE_IDENTIFIER);
        tree_builder.token(IDENT, "bar");
        tree_builder.finish_node(); // JS_REFERENCE_IDENTIFIER
        tree_builder.token(AS_KW, "as");
        tree_builder.start_node(JS_LITERAL_EXPORT_NAME);
        tree_builder.token(IDENT, "baz");
        tree_builder.finish_node(); // JS_LITERAL_EXPORT_NAME
        tree_builder.finish_node(); // JS_EXPORT_NAMED_SPECIFIER

        tree_builder.finish_node(); // JS_EXPORT_NAMED_SPECIFIER_LIST
        tree_builder.token(R_CURLY, "}");
        tree_builder.finish_node(); // JS_EXPORT_NAMED_CLAUSE
        tree_builder.finish_node(); // JS_EXPORT

        let node = tree_builder.finish();

        let export = JsExport::cast(node).unwrap();
        let exported_items = export.get_exported_items();
        assert_eq!(exported_items.len(), 2);
        assert_eq!(
            exported_items[0].identifier.as_ref().unwrap().to_string(),
            "foo"
        );
        assert_eq!(
            exported_items[1].identifier.as_ref().unwrap().to_string(),
            "baz"
        );
        assert!(exported_items[0].exported.is_none());
        assert!(exported_items[1].exported.is_none());
        assert!(!exported_items[0].is_default);
        assert!(!exported_items[1].is_default);
    }

    #[test]
    fn test_get_exported_items_default() {
        let mut tree_builder = JsSyntaxTreeBuilder::new();
        // export default foo;
        tree_builder.start_node(JS_EXPORT);
        tree_builder.token(EXPORT_KW, "export");
        tree_builder.start_node(JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE);
        tree_builder.token(DEFAULT_KW, "default");
        tree_builder.start_node(JS_IDENTIFIER_EXPRESSION);
        tree_builder.start_node(JS_REFERENCE_IDENTIFIER);
        tree_builder.token(IDENT, "foo");
        tree_builder.finish_node(); // JS_REFERENCE_IDENTIFIER
        tree_builder.finish_node(); // JS_IDENTIFIER_EXPRESSION
        tree_builder.finish_node(); // JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
        tree_builder.finish_node(); // JS_EXPORT

        let node = tree_builder.finish();
        let export = JsExport::cast(node).unwrap();
        let exported_items = export.get_exported_items();

        assert_eq!(exported_items.len(), 1);
        assert_eq!(
            exported_items[0].identifier.as_ref().unwrap().to_string(),
            "foo"
        );
        assert!(exported_items[0].exported.is_none());
        assert!(exported_items[0].is_default);
    }

    #[test]
    fn test_get_exported_items_variable_declaration() {
        let mut tree_builder = JsSyntaxTreeBuilder::new();
        // export const x = 100, y = 200;
        tree_builder.start_node(JS_EXPORT);
        tree_builder.token(EXPORT_KW, "export");
        tree_builder.start_node(JS_VARIABLE_DECLARATION_CLAUSE);
        tree_builder.start_node(JS_VARIABLE_DECLARATION);
        tree_builder.token(CONST_KW, "const");
        tree_builder.start_node(JS_VARIABLE_DECLARATOR_LIST);
        tree_builder.start_node(JS_VARIABLE_DECLARATOR);
        tree_builder.start_node(JS_IDENTIFIER_BINDING);
        tree_builder.token(IDENT, "x");
        tree_builder.finish_node(); // JS_IDENTIFIER_BINDING
        tree_builder.start_node(JS_INITIALIZER_CLAUSE);
        tree_builder.token(EQ, "=");
        tree_builder.start_node(JS_NUMBER_LITERAL_EXPRESSION);
        tree_builder.token(JS_NUMBER_LITERAL, "100");
        tree_builder.finish_node(); // JS_NUMBER_LITERAL_EXPRESSION
        tree_builder.finish_node(); // JS_INITIALIZER_CLAUSE
        tree_builder.finish_node(); // JS_VARIABLE_DECLARATOR
        tree_builder.token(COMMA, ",");
        tree_builder.start_node(JS_VARIABLE_DECLARATOR);
        tree_builder.start_node(JS_IDENTIFIER_BINDING);
        tree_builder.token(IDENT, "y");
        tree_builder.finish_node(); // JS_IDENTIFIER_BINDING
        tree_builder.start_node(JS_INITIALIZER_CLAUSE);
        tree_builder.token(EQ, "=");
        tree_builder.start_node(JS_NUMBER_LITERAL_EXPRESSION);
        tree_builder.token(JS_NUMBER_LITERAL, "200");
        tree_builder.finish_node(); // JS_NUMBER_LITERAL_EXPRESSION
        tree_builder.finish_node(); // JS_INITIALIZER_CLAUSE
        tree_builder.finish_node(); // JS_VARIABLE_DECLARATOR
        tree_builder.finish_node(); // JS_VARIABLE_DECLARATION
        tree_builder.finish_node(); // JS_VARIABLE_DECLARATION_LIST
        tree_builder.finish_node(); // JS_VARIABLE_DECLARATION_CLAUSE
        tree_builder.finish_node(); // JS_EXPORT

        let node = tree_builder.finish();
        let export = JsExport::cast(node).unwrap();
        let exported_items = export.get_exported_items();

        assert_eq!(exported_items.len(), 2);
        assert_eq!(
            exported_items[0].identifier.as_ref().unwrap().to_string(),
            "x"
        );
        assert_eq!(
            exported_items[1].identifier.as_ref().unwrap().to_string(),
            "y"
        );
        assert_eq!(
            exported_items[0].exported.clone().unwrap().to_string(),
            "100"
        );
        assert_eq!(
            exported_items[1].exported.clone().unwrap().to_string(),
            "200"
        );
        assert!(!exported_items[0].is_default);
        assert!(!exported_items[1].is_default);
    }

    #[test]
    fn test_get_exported_items_function_declaration() {
        let mut tree_builder = JsSyntaxTreeBuilder::new();
        // export function foo() {}
        tree_builder.start_node(JS_EXPORT);
        tree_builder.token(EXPORT_KW, "export");

        tree_builder.start_node(JS_FUNCTION_DECLARATION);
        tree_builder.token(FUNCTION_KW, "function");
        tree_builder.start_node(JS_IDENTIFIER_BINDING);
        tree_builder.token(IDENT, "foo");
        tree_builder.finish_node(); // JS_IDENTIFIER_BINDING
        tree_builder.start_node(JS_PARAMETERS);
        tree_builder.token(L_PAREN, "(");
        tree_builder.token(R_PAREN, ")");
        tree_builder.finish_node(); // JS_PARAMETERS
        tree_builder.start_node(JS_FUNCTION_BODY);
        tree_builder.token(L_CURLY, "{");
        tree_builder.token(R_CURLY, "}");
        tree_builder.finish_node(); // JS_FUNCTION_BODY
        tree_builder.finish_node(); // JS_FUNCTION_DECLARATION
        tree_builder.finish_node(); // JS_EXPORT

        let node = tree_builder.finish();
        let export = JsExport::cast(node).unwrap();
        let exported_items = export.get_exported_items();

        assert_eq!(exported_items.len(), 1);
        assert_eq!(
            exported_items[0].identifier.as_ref().unwrap().to_string(),
            "foo"
        );
        assert!(exported_items[0].exported.is_none());
        assert!(!exported_items[0].is_default);
    }
}
