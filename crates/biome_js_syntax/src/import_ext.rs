use crate::{
    AnyJsBinding, AnyJsCombinedSpecifier, AnyJsImportClause, AnyJsModuleSource,
    AnyJsNamedImportSpecifier, JsCallExpression, JsDefaultImportSpecifier, JsImport,
    JsImportAssertion, JsImportCallExpression, JsModuleSource, JsNamedImportSpecifier,
    JsNamedImportSpecifiers, JsNamespaceImportSpecifier, JsShorthandNamedImportSpecifier,
    JsSyntaxKind, JsSyntaxToken, inner_string_text,
};
use biome_rowan::{
    AstNode, SyntaxError, SyntaxNodeOptionExt, SyntaxResult, Text, TextRange, TokenText,
    declare_node_union,
};

impl JsImport {
    /// It checks if the source of an import against the string `source_to_check`
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let source = make::js_module_source(make::js_string_literal("react"));
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_default_import_specifier(binding.into());
    /// let clause = make::js_import_default_clause(specifier, make::token(T![from]), source.into()).build();
    /// let import = make::js_import(make::token(T![import]), clause.into()).build();
    ///
    /// assert_eq!(import.source_text().unwrap().text(), "react");
    /// ```
    pub fn source_text(&self) -> SyntaxResult<TokenText> {
        self.import_clause()?.source()?.inner_string_text()
    }
}

impl AnyJsImportClause {
    /// Type token of the import clause.
    ///
    /// ```ts
    /// import { type X }
    ///          ^^^^
    /// ```
    pub fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsImportDefaultClause(clause) => clause.type_token(),
            Self::JsImportNamedClause(clause) => clause.type_token(),
            Self::JsImportNamespaceClause(clause) => clause.type_token(),
            Self::JsImportBareClause(_) | Self::JsImportCombinedClause(_) => None,
        }
    }

    /// Source of this import clause.
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let source = make::js_module_source(make::js_string_literal("react"));
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_default_import_specifier(binding.into());
    /// let clause = make::js_import_default_clause(specifier, make::token(T![from]), source.into()).build();
    ///
    /// assert_eq!(clause.source().unwrap().as_js_module_source().unwrap().inner_string_text().unwrap().text(), "react");
    /// ```
    pub fn source(&self) -> SyntaxResult<JsModuleSource> {
        let source = match self {
            Self::JsImportBareClause(clause) => clause.source(),
            Self::JsImportDefaultClause(clause) => clause.source(),
            Self::JsImportNamedClause(clause) => clause.source(),
            Self::JsImportNamespaceClause(clause) => clause.source(),
            Self::JsImportCombinedClause(clause) => clause.source(),
        };

        source.and_then(|source| match source {
            AnyJsModuleSource::JsModuleSource(source) => Ok(source),
            AnyJsModuleSource::JsMetavariable(_) => Err(SyntaxError::UnexpectedMetavariable),
        })
    }

    pub fn named_specifiers(&self) -> Option<JsNamedImportSpecifiers> {
        match self {
            Self::JsImportBareClause(_) => None,
            Self::JsImportCombinedClause(clause) => {
                if let Ok(AnyJsCombinedSpecifier::JsNamedImportSpecifiers(named_specifiers)) =
                    clause.specifier()
                {
                    Some(named_specifiers)
                } else {
                    None
                }
            }
            Self::JsImportDefaultClause(_) => None,
            Self::JsImportNamedClause(clause) => clause.named_specifiers().ok(),
            Self::JsImportNamespaceClause(_) => None,
        }
    }

    /// Attribute of this import clause.
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let source = make::js_module_source(make::js_string_literal("react"));
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_default_import_specifier(binding.into());
    /// let clause = make::js_import_default_clause(specifier, make::token(T![from]), source.into()).build();
    ///
    /// assert_eq!(clause.source().unwrap().as_js_module_source().unwrap().inner_string_text().unwrap().text(), "react");
    /// ```
    pub fn attribute(&self) -> Option<JsImportAssertion> {
        match self {
            Self::JsImportBareClause(clause) => clause.assertion(),
            Self::JsImportDefaultClause(clause) => clause.assertion(),
            Self::JsImportNamedClause(clause) => clause.assertion(),
            Self::JsImportNamespaceClause(clause) => clause.assertion(),
            Self::JsImportCombinedClause(clause) => clause.assertion(),
        }
    }

    /// Creates a vector with items of type `T` that is constructed by calling
    /// the given `filter_map` function for every symbol that is imported in
    /// this clause.
    ///
    /// `filter_map` receives two arguments: The name of the imported symbol
    /// (not the local name it is imported as), and the trimmed text range of
    /// the imported symbol.
    ///
    /// ## Known Caveat
    ///
    /// This method only filters over imported default and named symbols.
    /// Namespace imports are not considered, because they cannot be
    /// individually evaluated.
    pub fn filter_map_all_imported_symbols<F, T>(&self, filter_map: F) -> Vec<T>
    where
        F: Fn(Text, TextRange) -> Option<T>,
    {
        let process_default_specifier = |specifier: SyntaxResult<JsDefaultImportSpecifier>| {
            specifier
                .ok()
                .and_then(|specifier| specifier.local_name().ok())
                .and_then(|local_name| {
                    local_name
                        .as_js_identifier_binding()
                        .and_then(|binding| binding.name_token().ok())
                })
                .and_then(|local_name| {
                    filter_map(Text::new_static("default"), local_name.text_trimmed_range())
                })
        };

        let process_named_specifiers = |specifiers: &JsNamedImportSpecifiers| {
            specifiers
                .specifiers()
                .into_iter()
                .flatten()
                .filter_map(|specifier| {
                    let imported_name = specifier.imported_name()?;
                    filter_map(
                        imported_name.token_text_trimmed().into(),
                        imported_name.text_trimmed_range(),
                    )
                })
        };

        match self {
            Self::JsImportCombinedClause(node) => {
                let mut vec = Vec::from_iter(process_default_specifier(node.default_specifier()));
                if let Some(specifiers) = node.specifier().ok()
                    && let Some(specifiers) = specifiers.as_js_named_import_specifiers()
                {
                    vec.extend(process_named_specifiers(specifiers));
                }
                vec
            }
            Self::JsImportDefaultClause(node) => {
                process_default_specifier(node.default_specifier())
                    .into_iter()
                    .collect()
            }
            Self::JsImportNamedClause(node) => node
                .named_specifiers()
                .ok()
                .iter()
                .flat_map(process_named_specifiers)
                .collect(),
            Self::JsImportBareClause(_) | Self::JsImportNamespaceClause(_) => Vec::new(),
        }
    }

    /// Returns an import clause with `named_specifiers` as named specifiers
    /// or the import clause itself if it doesn't accept any named specifiers.
    pub fn with_named_specifiers(self, named_specifiers: JsNamedImportSpecifiers) -> Self {
        match self {
            Self::JsImportBareClause(_) => self,
            Self::JsImportCombinedClause(clause) => if matches!(
                clause.specifier(),
                Ok(AnyJsCombinedSpecifier::JsNamedImportSpecifiers(_))
            ) {
                clause.with_specifier(named_specifiers.into())
            } else {
                clause
            }
            .into(),
            Self::JsImportDefaultClause(_) => self,
            Self::JsImportNamedClause(clause) => {
                clause.with_named_specifiers(named_specifiers).into()
            }
            Self::JsImportNamespaceClause(_) => self,
        }
    }

    /// Returns an import clause with `attribute` as import attribute.
    pub fn with_attribute(self, attribute: Option<JsImportAssertion>) -> Self {
        match self {
            Self::JsImportBareClause(clause) => clause.with_assertion(attribute).into(),
            Self::JsImportCombinedClause(clause) => clause.with_assertion(attribute).into(),
            Self::JsImportDefaultClause(clause) => clause.with_assertion(attribute).into(),
            Self::JsImportNamedClause(clause) => clause.with_assertion(attribute).into(),
            Self::JsImportNamespaceClause(clause) => clause.with_assertion(attribute).into(),
        }
    }
}

impl AnyJsNamedImportSpecifier {
    /// Type token of the import specifier.
    ///
    /// ```ts
    /// import { type X }
    ///          ^^^^
    /// ```
    pub fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsBogusNamedImportSpecifier(_) => None,
            Self::JsNamedImportSpecifier(specifier) => specifier.type_token(),
            Self::JsShorthandNamedImportSpecifier(specifier) => specifier.type_token(),
        }
    }

    /// Returns the import clause that includes this specifier.
    pub fn import_clause(&self) -> Option<AnyJsImportClause> {
        AnyJsImportClause::cast(self.syntax().ancestors().nth(3)?)
    }

    /// Returns `true` if this specifier or its import clause has **only** a type modifier.
    pub fn imports_only_types(&self) -> bool {
        self.type_token().is_some() || self.import_clause().and_then(|x| x.type_token()).is_some()
    }

    /// Imported name of this import specifier
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::AnyJsNamedImportSpecifier;
    ///
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_shorthand_named_import_specifier(binding.into()).build();
    /// let specifier = AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier);
    ///
    /// assert_eq!(specifier.imported_name().unwrap().text_trimmed(), "React");
    /// ```
    pub fn imported_name(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsBogusNamedImportSpecifier(_) => None,
            Self::JsNamedImportSpecifier(specifier) => {
                specifier.name().and_then(|name| name.value()).ok()
            }
            Self::JsShorthandNamedImportSpecifier(specifier) => {
                let imported_name = specifier.local_name().ok()?;
                let imported_name = imported_name.as_js_identifier_binding()?;
                imported_name.name_token().ok()
            }
        }
    }

    /// Local name of this import specifier
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::AnyJsNamedImportSpecifier;
    ///
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_shorthand_named_import_specifier(binding.into()).build();
    /// let specifier = AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier);
    ///
    /// let name_token = specifier.local_name().unwrap().as_js_identifier_binding().unwrap().name_token();
    /// assert_eq!(name_token.unwrap().text_trimmed(), "React");
    /// ```
    pub fn local_name(&self) -> Option<AnyJsBinding> {
        match self {
            Self::JsBogusNamedImportSpecifier(_) => None,
            Self::JsNamedImportSpecifier(specifier) => specifier.local_name().ok(),
            Self::JsShorthandNamedImportSpecifier(specifier) => specifier.local_name().ok(),
        }
    }

    pub fn with_type_token(self, type_token: Option<JsSyntaxToken>) -> Self {
        match self {
            Self::JsBogusNamedImportSpecifier(_) => self,
            Self::JsNamedImportSpecifier(specifier) => specifier.with_type_token(type_token).into(),
            Self::JsShorthandNamedImportSpecifier(specifier) => {
                specifier.with_type_token(type_token).into()
            }
        }
    }
}

impl JsModuleSource {
    /// Get the inner text of a string not including the quotes
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    /// let source_token = make::js_string_literal("react")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]);
    /// let source = make::js_module_source(source_token);
    ///
    /// assert_eq!(source.inner_string_text().unwrap().text(), "react");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

declare_node_union! {
    /// This node union is meant to match the following syntax:
    /// ```js
    ///    import "lodash";
    /// //        ^^^^^^^^
    ///    require("lodash")
    /// // ^^^^^^^^^^^^^^^^^
    ///    import("lodash")
    /// // ^^^^^^^^^^^^^^^^
    /// ```
    pub AnyJsImportLike = JsModuleSource | JsCallExpression |  JsImportCallExpression
}

impl AnyJsImportLike {
    /// Returns the inner text of specifier:
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::AnyJsImportLike;
    ///
    /// let source_name = make::js_module_source(make::js_string_literal("foo"));
    /// let any_import_specifier = AnyJsImportLike::JsModuleSource(source_name);
    /// assert_eq!(any_import_specifier.inner_string_text().unwrap().text(), "foo")
    /// ```
    pub fn inner_string_text(&self) -> Option<TokenText> {
        match self {
            Self::JsModuleSource(source) => source.inner_string_text().ok(),
            Self::JsCallExpression(expression) => {
                let callee = expression.callee().ok()?;
                let name = callee.as_js_reference_identifier()?.value_token().ok()?;
                if name.text_trimmed() == "require" {
                    let [Some(argument)] = expression.arguments().ok()?.get_arguments_by_index([0])
                    else {
                        return None;
                    };
                    argument
                        .as_any_js_expression()?
                        .as_any_js_literal_expression()?
                        .as_js_string_literal_expression()?
                        .inner_string_text()
                        .ok()
                } else {
                    None
                }
            }
            Self::JsImportCallExpression(import_call) => {
                let [Some(argument)] = import_call.arguments().ok()?.get_arguments_by_index([0])
                else {
                    return None;
                };
                argument
                    .as_any_js_expression()?
                    .as_any_js_literal_expression()?
                    .as_js_string_literal_expression()?
                    .inner_string_text()
                    .ok()
            }
        }
    }

    /// Returns the whole token text of the specifier, with quotes included:
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::AnyJsImportLike;
    ///
    /// let source_name = make::js_module_source(make::js_string_literal("foo"));
    /// let any_import_specifier = AnyJsImportLike::JsModuleSource(source_name);
    /// assert_eq!(any_import_specifier.module_name_token().unwrap().text(), "\"foo\"")
    /// ```
    pub fn module_name_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsModuleSource(source) => source.value_token().ok(),
            Self::JsCallExpression(expression) => {
                let callee = expression.callee().ok()?;
                let name = callee.as_js_reference_identifier()?.value_token().ok()?;
                if name.text_trimmed() == "require" {
                    let [Some(argument)] = expression.arguments().ok()?.get_arguments_by_index([0])
                    else {
                        return None;
                    };
                    argument
                        .as_any_js_expression()?
                        .as_any_js_literal_expression()?
                        .as_js_string_literal_expression()?
                        .value_token()
                        .ok()
                } else {
                    None
                }
            }
            Self::JsImportCallExpression(import_call) => {
                let [Some(argument)] = import_call.arguments().ok()?.get_arguments_by_index([0])
                else {
                    return None;
                };
                argument
                    .as_any_js_expression()?
                    .as_any_js_literal_expression()?
                    .as_js_string_literal_expression()?
                    .value_token()
                    .ok()
            }
        }
    }

    /// Check whether the js import specifier like is in a ts module declaration:
    ///
    /// ```ts
    /// declare module "abc" {}
    /// ```
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::{AnyJsImportLike, JsSyntaxKind, JsSyntaxToken};
    ///
    /// let module_token = JsSyntaxToken::new_detached(JsSyntaxKind::MODULE_KW, "module", [], []);
    /// let module_source = make::js_module_source(make::js_string_literal("foo"));
    /// let module_declaration = make::ts_external_module_declaration(module_token, module_source.into()).build();
    /// let any_import_specifier = AnyJsImportLike::JsModuleSource(module_declaration.source().unwrap().as_js_module_source().unwrap().clone());
    /// assert!(any_import_specifier.is_in_ts_module_declaration());
    ///
    /// let module_source = make::js_module_source(make::js_string_literal("bar"));
    /// let any_import_specifier = AnyJsImportLike::JsModuleSource(module_source.into());
    /// assert!(!any_import_specifier.is_in_ts_module_declaration());
    /// ```
    pub fn is_in_ts_module_declaration(&self) -> bool {
        // It first has to be a JsModuleSource
        matches!(self, Self::JsModuleSource(_))
            && matches!(
                self.syntax().parent().kind(),
                Some(JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION)
            )
    }

    /// Returns whether this is a static import.
    ///
    /// Static imports are those where no variables are allowed within the
    /// module specifier. Compare this to  `import()` and `require()`
    /// expressions, which are considered dynamic imports.
    pub fn is_static_import(&self) -> bool {
        matches!(self, Self::JsModuleSource(_))
    }
}

declare_node_union! {
    pub AnyJsImportSpecifier = JsNamedImportSpecifier
        | JsShorthandNamedImportSpecifier
        | JsNamespaceImportSpecifier
        | JsDefaultImportSpecifier
}

impl AnyJsImportSpecifier {
    /// Imported name of this import specifier.
    pub fn local_name(&self) -> SyntaxResult<AnyJsBinding> {
        match self {
            Self::JsNamedImportSpecifier(specifier) => specifier.local_name(),
            Self::JsShorthandNamedImportSpecifier(specifier) => specifier.local_name(),
            Self::JsNamespaceImportSpecifier(specifier) => specifier.local_name(),
            Self::JsDefaultImportSpecifier(specifier) => specifier.local_name(),
        }
    }
}
