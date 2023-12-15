use crate::{
    inner_string_text, AnyJsBinding, AnyJsImportClause, AnyJsNamedImportSpecifier, JsImport,
    JsImportAssertion, JsModuleSource, JsSyntaxToken,
};
use biome_rowan::{AstNode, SyntaxResult, TokenText};

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
    /// let clause = make::js_import_default_clause(specifier, make::token(T![from]), source).build();
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
            Self::JsImportBareClause(_) => None,
            Self::JsImportDefaultClause(clause) => clause.type_token(),
            Self::JsImportNamedClause(clause) => clause.type_token(),
            Self::JsImportNamespaceClause(clause) => clause.type_token(),
            Self::JsImportCombinedClause(_) => None,
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
    /// let clause = make::js_import_default_clause(specifier, make::token(T![from]), source).build();
    ///
    /// assert_eq!(clause.source().unwrap().inner_string_text().unwrap().text(), "react");
    /// ```
    pub fn source(&self) -> SyntaxResult<JsModuleSource> {
        match self {
            Self::JsImportBareClause(clause) => clause.source(),
            Self::JsImportDefaultClause(clause) => clause.source(),
            Self::JsImportNamedClause(clause) => clause.source(),
            Self::JsImportNamespaceClause(clause) => clause.source(),
            Self::JsImportCombinedClause(clause) => clause.source(),
        }
    }

    /// Assertion of this import clause.
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let source = make::js_module_source(make::js_string_literal("react"));
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_default_import_specifier(binding.into());
    /// let clause = make::js_import_default_clause(specifier, make::token(T![from]), source).build();
    ///
    /// assert_eq!(clause.source().unwrap().inner_string_text().unwrap().text(), "react");
    /// ```
    pub fn assertion(&self) -> Option<JsImportAssertion> {
        match self {
            Self::JsImportBareClause(clause) => clause.assertion(),
            Self::JsImportDefaultClause(clause) => clause.assertion(),
            Self::JsImportNamedClause(clause) => clause.assertion(),
            Self::JsImportNamespaceClause(clause) => clause.assertion(),
            Self::JsImportCombinedClause(clause) => clause.assertion(),
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
            Self::JsNamedImportSpecifier(specifier) => specifier.name().ok()?.value().ok(),
            Self::JsShorthandNamedImportSpecifier(specifier) => specifier
                .local_name()
                .ok()?
                .as_js_identifier_binding()?
                .name_token()
                .ok(),
            Self::JsBogusNamedImportSpecifier(_) => None,
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

    pub fn with_type_token(self, type_token: Option<JsSyntaxToken>) -> AnyJsNamedImportSpecifier {
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
