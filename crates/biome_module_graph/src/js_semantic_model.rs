use biome_js_syntax::{
    AnyJsDeclaration, AnyJsImportClause, JsFunctionDeclaration, JsSyntaxNode,
    JsVariableDeclaration, JsVariableKind, TsTypeAliasDeclaration,
};
use biome_js_type_info::Type;
use biome_rowan::{AstNode, Text};

use crate::jsdoc_comment::JsdocComment;

/// Simplified version of the JS Semantic Model, that provides just enough
/// information to keep track of the types of all symbols in a module's global
/// scope.
///
/// The model is discarded as soon as the module's inferrence has completed.
#[derive(Clone, Debug)]
pub(crate) struct JsSemanticModel {
    pub declarations: Box<[JsDeclaration]>,
}

impl JsSemanticModel {
    /// Finds the value declaration with the given `name`.
    pub fn get_value(&self, name: &str) -> Option<&JsDeclaration> {
        self.declarations
            .iter()
            .find(|decl| decl.name == name && decl.kind.might_be_value())
    }

    /// Finds the type declaration with the given `name`.
    pub fn get_type(&self, name: &str) -> Option<&JsDeclaration> {
        self.declarations
            .iter()
            .find(|decl| decl.name == name && decl.kind.might_be_type())
    }
}

/// A single declaration of a variable or type.
#[derive(Clone, Debug)]
pub struct JsDeclaration {
    /// The name of the thing being declared.
    pub name: Text,

    /// The kind of thing being declared.
    pub kind: JsDeclarationKind,

    /// Optional JSDoc comment associated with the declaration.
    pub jsdoc_comment: Option<JsdocComment>,
}

#[derive(Clone, Debug)]
pub enum JsDeclarationKind {
    /// A `class` declaration.
    ///
    /// The type defined by the class.
    #[expect(unused)] // TODO
    Class(Type),

    /// A `function` or `var` declaration.
    ///
    /// The type (either declared or inferred) of the value.
    HoistedValue(Type),

    /// An `import` declaration.
    Import(Text),

    /// An `import type` declaration.
    ImportType(Text),

    /// An interface declaration.
    #[expect(unused)] // TODO
    Interface,

    /// A module declaration.
    #[expect(unused)] // TODO
    Module,

    /// A namespace declaration.
    #[expect(unused)] // TODO
    Namespace,

    /// A type declaration.
    Type(Type),

    /// A `let` or `const` declaration.
    ///
    /// The type (either declared or inferred) of the value.
    Value(Type),
}

impl JsDeclarationKind {
    /// Returns `true` for any declaration that _may_ be a type.
    ///
    /// The main reason why we can't be sure whether something is a value or a
    /// type is the `Import` variant, for which we don't know the kind of what
    /// we're importing.
    fn might_be_type(&self) -> bool {
        matches!(
            self,
            Self::Class(_)
                | Self::Import(_)
                | Self::ImportType(_)
                | Self::Namespace
                | Self::Interface
                | Self::Type(_)
        )
    }

    /// Returns `true` for any declaration that _may_ be a value.
    ///
    /// The main reason why we can't be sure whether something is a value or a
    /// type is the `Import` variant, for which we don't know the kind of what
    /// we're importing.
    fn might_be_value(&self) -> bool {
        matches!(
            self,
            Self::Class(_) | Self::HoistedValue(_) | Self::Import(_) | Self::Value(_)
        )
    }
}

/// Responsible for building the JsSemanticBuilder.
#[derive(Debug, Default)]
pub struct JsSemanticModelBuilder {
    declarations: Vec<JsDeclaration>,
}

impl JsSemanticModelBuilder {
    pub fn build(self) -> JsSemanticModel {
        JsSemanticModel {
            declarations: self.declarations.into(),
        }
    }

    pub fn push_node(&mut self, node: &JsSyntaxNode) {
        if let Some(import) = AnyJsImportClause::cast_ref(node) {
            // TODO: Handle CommonJS imports too.
            self.push_import(import);
        } else if let Some(decl) = AnyJsDeclaration::cast_ref(node) {
            self.push_declaration(decl);
        }
    }

    fn push_declaration(&mut self, decl: AnyJsDeclaration) -> Option<()> {
        match decl {
            AnyJsDeclaration::JsClassDeclaration(_decl) => {
                // TODO
                None
            }
            AnyJsDeclaration::JsFunctionDeclaration(decl) => self.push_function_declaration(decl),
            AnyJsDeclaration::JsVariableDeclaration(decl) => self.push_variable_declaration(decl),
            AnyJsDeclaration::TsDeclareFunctionDeclaration(_decl) => {
                // TODO
                None
            }
            AnyJsDeclaration::TsEnumDeclaration(_decl) => {
                // TODO
                None
            }
            AnyJsDeclaration::TsExternalModuleDeclaration(_decl) => {
                // TODO
                None
            }
            AnyJsDeclaration::TsGlobalDeclaration(_decl) => {
                // TODO
                None
            }
            AnyJsDeclaration::TsImportEqualsDeclaration(_decl) => {
                // TODO
                None
            }
            AnyJsDeclaration::TsInterfaceDeclaration(_decl) => {
                // TODO
                None
            }
            AnyJsDeclaration::TsModuleDeclaration(_decl) => {
                // TODO
                None
            }
            AnyJsDeclaration::TsTypeAliasDeclaration(decl) => {
                self.push_type_alias_declaration(decl)
            }
        }
    }

    fn push_function_declaration(&mut self, decl: JsFunctionDeclaration) -> Option<()> {
        let binding = decl.id().ok()?;
        let binding = binding.as_js_identifier_binding()?;
        let name_token = binding.name_token().ok()?;

        self.declarations.push(JsDeclaration {
            name: name_token.token_text_trimmed().into(),
            kind: JsDeclarationKind::HoistedValue(Type::from_js_function_declaration(&decl)),
            jsdoc_comment: JsdocComment::try_from(decl.syntax()).ok(),
        });

        Some(())
    }

    fn push_import(&mut self, node: AnyJsImportClause) -> Option<()> {
        match node {
            AnyJsImportClause::JsImportBareClause(_node) => {}
            AnyJsImportClause::JsImportCombinedClause(node) => {
                let source = node.source().ok()?;
                let source_token = source.as_js_module_source()?.value_token().ok()?;
                let default_specifier = node.default_specifier().ok()?;
                let local_name = default_specifier.local_name().ok()?;
                let default_identifier = local_name.as_js_identifier_binding()?;
                let default_name_token = default_identifier.name_token().ok()?;
                self.declarations.push(JsDeclaration {
                    name: default_name_token.token_text_trimmed().into(),
                    kind: JsDeclarationKind::Import(source_token.token_text().into()),
                    jsdoc_comment: None,
                });
                for specifier in node
                    .specifier()
                    .ok()?
                    // TODO: Handle `.as_js_namespace_import_specifier()`
                    .as_js_named_import_specifiers()?
                    .specifiers()
                {
                    let specifier = specifier.ok()?;
                    let local_name = specifier.local_name()?;
                    let identifier = local_name.as_js_identifier_binding()?;
                    let name_token = identifier.name_token().ok()?;
                    self.declarations.push(JsDeclaration {
                        name: name_token.token_text_trimmed().into(),
                        kind: if specifier.type_token().is_some() {
                            JsDeclarationKind::ImportType(source_token.token_text().into())
                        } else {
                            JsDeclarationKind::Import(source_token.token_text().into())
                        },
                        jsdoc_comment: None,
                    });
                }
            }
            AnyJsImportClause::JsImportDefaultClause(node) => {
                let source = node.source().ok()?;
                let source_token = source.as_js_module_source()?.value_token().ok()?;
                let local_name = node.default_specifier().ok()?.local_name().ok()?;
                let identifier = local_name.as_js_identifier_binding()?;
                let name_token = identifier.name_token().ok()?;
                self.declarations.push(JsDeclaration {
                    name: name_token.token_text_trimmed().into(),
                    kind: if node.type_token().is_some() {
                        JsDeclarationKind::ImportType(source_token.token_text().into())
                    } else {
                        JsDeclarationKind::Import(source_token.token_text().into())
                    },
                    jsdoc_comment: None,
                });
            }
            AnyJsImportClause::JsImportNamedClause(node) => {
                let source = node.source().ok()?;
                let source_token = source.as_js_module_source()?.value_token().ok()?;
                for specifier in node.named_specifiers().ok()?.specifiers() {
                    let specifier = specifier.ok()?;
                    let local_name = specifier.local_name()?;
                    let identifier = local_name.as_js_identifier_binding()?;
                    let name_token = identifier.name_token().ok()?;
                    self.declarations.push(JsDeclaration {
                        name: name_token.token_text_trimmed().into(),
                        kind: if node.type_token().is_some() || specifier.type_token().is_some() {
                            JsDeclarationKind::ImportType(source_token.token_text().into())
                        } else {
                            JsDeclarationKind::Import(source_token.token_text().into())
                        },
                        jsdoc_comment: None,
                    });
                }
            }
            AnyJsImportClause::JsImportNamespaceClause(_node) => {
                // TODO: Support namespace imports
            }
        }

        Some(())
    }

    fn push_type_alias_declaration(&mut self, decl: TsTypeAliasDeclaration) -> Option<()> {
        let binding = decl.binding_identifier().ok()?;
        let name_token = binding.as_ts_identifier_binding()?.name_token().ok()?;
        self.declarations.push(JsDeclaration {
            name: name_token.token_text_trimmed().into(),
            kind: JsDeclarationKind::Type(
                Type::from_ts_type_alias_declaration(&decl).unwrap_or_default(),
            ),
            jsdoc_comment: JsdocComment::try_from(decl.syntax()).ok(),
        });

        Some(())
    }

    fn push_variable_declaration(&mut self, decl: JsVariableDeclaration) -> Option<()> {
        let kind = decl.variable_kind().ok()?;
        for declarator in decl.declarators() {
            let declarator = declarator.ok()?;
            let binding = declarator.id().ok()?;
            // TODO: Handle object and array patterns
            let binding = binding.as_any_js_binding()?.as_js_identifier_binding()?;
            let name_token = binding.name_token().ok()?;

            self.declarations.push(JsDeclaration {
                name: name_token.token_text_trimmed().into(),
                kind: match kind {
                    JsVariableKind::Const | JsVariableKind::Let => JsDeclarationKind::Value(
                        Type::from_js_variable_declarator(&declarator).unwrap_or_default(),
                    ),
                    JsVariableKind::Var => JsDeclarationKind::HoistedValue(
                        Type::from_js_variable_declarator(&declarator).unwrap_or_default(),
                    ),
                    JsVariableKind::Using => return None,
                },
                jsdoc_comment: JsdocComment::try_from(decl.syntax()).ok(),
            });
        }

        Some(())
    }
}
