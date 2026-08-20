use crate::{JsBatchMutation, declare_transformation};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::category;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsExportClause, AnyJsImportClause, AnyTsType, JsClassDeclaration,
    JsClassExportDefaultDeclaration, JsExport, JsExportNamedFromSpecifier,
    JsExportNamedShorthandSpecifier, JsExportNamedSpecifier, JsFormalParameter, JsImport,
    JsLanguage, JsMethodClassMember, JsNamedImportSpecifier, JsShorthandNamedImportSpecifier,
    JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, T, TsAbstractModifier, TsAccessibilityModifier,
    TsAsAssignment, TsAsExpression, TsConstructorSignatureClassMember,
    TsDeclareFunctionDeclaration, TsDeclareFunctionExportDefaultDeclaration, TsDeclareModifier,
    TsDeclareStatement, TsDefinitePropertyAnnotation, TsDefiniteVariableAnnotation,
    TsEnumDeclaration, TsExportAsNamespaceClause, TsExportAssignmentClause,
    TsExternalModuleDeclaration, TsGetterSignatureClassMember, TsGlobalDeclaration,
    TsImplementsClause, TsImportEqualsDeclaration, TsIndexSignatureClassMember,
    TsInitializedPropertySignatureClassMember, TsInstantiationExpression, TsInterfaceDeclaration,
    TsMethodSignatureClassMember, TsModuleDeclaration, TsNonNullAssertionAssignment,
    TsNonNullAssertionExpression, TsOptionalPropertyAnnotation, TsOverrideModifier,
    TsPropertyParameter, TsPropertySignatureClassMember, TsReadonlyModifier,
    TsReturnTypeAnnotation, TsSatisfiesAssignment, TsSatisfiesExpression,
    TsSetterSignatureClassMember, TsThisParameter, TsTypeAliasDeclaration, TsTypeAnnotation,
    TsTypeArguments, TsTypeAssertionAssignment, TsTypeAssertionExpression, TsTypeParameters,
};
use biome_rowan::{
    AstNode, BatchMutationExt, NodeOrToken, SyntaxTriviaPiece, TextRange, TriviaPiece,
    declare_node_union,
};

declare_transformation! {
    /// Strips TypeScript-only syntax, replacing it with whitespace of the same length.
    ///
    /// Line breaks are preserved, so the positions of the remaining JavaScript code are
    /// identical to the TypeScript source and the output can be executed without a source map.
    /// Comments inside erased ranges are erased with them.
    ///
    /// Only *erasable* syntax is supported, like the type stripping of Node.js: constructs that
    /// generate runtime code (`enum`, `namespace`, parameter properties, `export =`, ...) are
    /// reported as diagnostics and left untouched.
    pub(crate) StripTypes {
        version: "next",
        name: "stripTypes",
        language: "ts",
    }
}

declare_node_union! {
    /// TS-only declarations that are erased together with the `export` in front of them.
    pub AnyTsErasableDeclaration =
        TsInterfaceDeclaration
        | TsTypeAliasDeclaration
        | TsDeclareStatement
        | TsDeclareFunctionDeclaration
        | TsDeclareFunctionExportDefaultDeclaration
        | TsImportEqualsDeclaration
}

declare_node_union! {
    /// TS-only syntax that is erased in place, together with its descendants.
    pub AnyTsErasableSyntax =
        TsTypeAnnotation
        | TsReturnTypeAnnotation
        | TsDefiniteVariableAnnotation
        | TsOptionalPropertyAnnotation
        | TsDefinitePropertyAnnotation
        | TsTypeParameters
        | TsImplementsClause
        | TsAccessibilityModifier
        | TsReadonlyModifier
        | TsOverrideModifier
        | TsAbstractModifier
        | TsDeclareModifier
        | TsPropertySignatureClassMember
        | TsInitializedPropertySignatureClassMember
        | TsMethodSignatureClassMember
        | TsGetterSignatureClassMember
        | TsSetterSignatureClassMember
        | TsConstructorSignatureClassMember
        | TsIndexSignatureClassMember
}

declare_node_union! {
    /// TS-only syntax wrapping an expression or an assignment, e.g. `expr as T` or `expr!`.
    /// Only the type syntax around the inner expression is erased.
    pub AnyTsTypeWrapper =
        TsAsExpression
        | TsSatisfiesExpression
        | TsNonNullAssertionExpression
        | TsTypeAssertionExpression
        | TsInstantiationExpression
        | TsAsAssignment
        | TsSatisfiesAssignment
        | TsNonNullAssertionAssignment
        | TsTypeAssertionAssignment
}

declare_node_union! {
    /// TS-only syntax that generates runtime code and thus can't be erased.
    pub AnyTsRuntimeSyntax =
        TsEnumDeclaration
        | TsModuleDeclaration
        | TsGlobalDeclaration
        | TsExternalModuleDeclaration
        | TsPropertyParameter
        | TsExportAssignmentClause
        | TsExportAsNamespaceClause
}

declare_node_union! {
    /// Import and export specifiers that may be marked with `type`.
    pub AnyJsMaybeTypeOnlySpecifier =
        JsShorthandNamedImportSpecifier
        | JsNamedImportSpecifier
        | JsExportNamedShorthandSpecifier
        | JsExportNamedSpecifier
        | JsExportNamedFromSpecifier
}

declare_node_union! {
    /// Classes that may carry the `abstract` modifier.
    pub AnyJsMaybeAbstractClass = JsClassDeclaration | JsClassExportDefaultDeclaration
}

declare_node_union! {
    /// Nodes that may carry the `?` marker of an optional parameter or method.
    pub AnyJsMaybeOptional = JsFormalParameter | JsMethodClassMember
}

declare_node_union! {
    pub AnyTsStrippableSyntax =
        AnyTsErasableDeclaration
        | AnyTsErasableSyntax
        | AnyTsTypeWrapper
        | AnyTsRuntimeSyntax
        | AnyJsMaybeTypeOnlySpecifier
        | AnyJsMaybeAbstractClass
        | AnyJsMaybeOptional
        | TsTypeArguments
        | TsThisParameter
        | JsImport
        | JsExport
}

impl AnyTsRuntimeSyntax {
    /// Returns how the syntax is named in the diagnostic.
    fn syntax_name(&self) -> &'static str {
        match self {
            Self::TsEnumDeclaration(_) => "enum declarations",
            Self::TsModuleDeclaration(_) => "namespace and module declarations",
            Self::TsGlobalDeclaration(_) | Self::TsExternalModuleDeclaration(_) => {
                "ambient module declarations"
            }
            Self::TsPropertyParameter(_) => "parameter properties",
            Self::TsExportAssignmentClause(_) => "export = assignments",
            Self::TsExportAsNamespaceClause(_) => "export as namespace declarations",
        }
    }
}

#[derive(Debug)]
pub enum StripTypesState {
    /// The ranges of the construct to be replaced with whitespace.
    Erase(Vec<TextRange>),
    /// The construct generates runtime code and thus can't be erased.
    Unsupported { syntax: &'static str },
}

impl Rule for StripTypes {
    type Query = Ast<AnyTsStrippableSyntax>;
    type State = StripTypesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Constructs inside an erased ancestor (a type, a type-only declaration, ...) are
        // already erased with it and must not produce their own overlapping signal.
        if node
            .syntax()
            .ancestors()
            .skip(1)
            .any(|ancestor| erases_descendants(&ancestor))
        {
            return None;
        }

        match node {
            // If the declaration is the clause of an `export`, the `export` keyword can't be
            // left dangling, so the whole export is erased with it.
            AnyTsStrippableSyntax::AnyTsErasableDeclaration(declaration) => {
                // `import A = require("a")` generates a runtime require, unlike `import type`.
                if let AnyTsErasableDeclaration::TsImportEqualsDeclaration(import) = declaration
                    && import.type_token().is_none()
                {
                    return Some(StripTypesState::Unsupported {
                        syntax: "import = declarations",
                    });
                }

                Some(StripTypesState::Erase(vec![export_erase_range(
                    declaration,
                )]))
            }

            AnyTsStrippableSyntax::AnyTsErasableSyntax(erasable) => {
                Some(StripTypesState::Erase(vec![erasable.range()]))
            }

            // The type arguments of an instantiation expression are erased by the wrapper.
            AnyTsStrippableSyntax::TsTypeArguments(arguments) => arguments
                .syntax()
                .parent()
                .is_none_or(|parent| parent.kind() != JsSyntaxKind::TS_INSTANTIATION_EXPRESSION)
                .then(|| StripTypesState::Erase(vec![arguments.range()])),

            // A `this` parameter also takes its trailing comma with it, as erasing it alone
            // may leave the parameter list with a leading comma.
            AnyTsStrippableSyntax::TsThisParameter(parameter) => Some(StripTypesState::Erase(
                vec![range_with_trailing_separator(parameter.syntax())],
            )),

            // Keep the inner expression, erase the type syntax around it. Nested wrappers
            // produce their own signals.
            AnyTsStrippableSyntax::AnyTsTypeWrapper(wrapper) => {
                Some(StripTypesState::Erase(wrapper_ranges(wrapper)))
            }

            AnyTsStrippableSyntax::AnyTsRuntimeSyntax(runtime) => {
                Some(StripTypesState::Unsupported {
                    syntax: runtime.syntax_name(),
                })
            }

            // `import type ... from "..."` is erased entirely.
            AnyTsStrippableSyntax::JsImport(import) => import
                .import_clause()
                .is_ok_and(|clause| is_type_only_import_clause(&clause))
                .then(|| StripTypesState::Erase(vec![import.range()])),

            // `export type { ... }` and `export declare ...` are erased entirely. Exports of a
            // type-only declaration are handled by the declaration itself.
            AnyTsStrippableSyntax::JsExport(export) => export
                .export_clause()
                .is_ok_and(|clause| is_type_only_export_clause(&clause))
                .then(|| StripTypesState::Erase(vec![export.range()])),

            // `import { type A } from "..."` erases the specifier together with its trailing
            // comma, keeping the (side effect of the) import itself, like `verbatimModuleSyntax`
            // does.
            AnyTsStrippableSyntax::AnyJsMaybeTypeOnlySpecifier(specifier) => {
                find_direct_token(specifier.syntax(), T![type]).map(|_| {
                    StripTypesState::Erase(vec![range_with_trailing_separator(specifier.syntax())])
                })
            }

            AnyTsStrippableSyntax::AnyJsMaybeAbstractClass(class) => {
                let abstract_token = find_direct_token(class.syntax(), T![abstract])?;

                // Erasing `abstract` from the class alone would leave its abstract members
                // behind, which no longer parse, so they are erased together.
                let mut ranges = vec![abstract_token.text_trimmed_range()];
                ranges.extend(abstract_members(class).map(|member| member.range()));

                Some(StripTypesState::Erase(ranges))
            }

            AnyTsStrippableSyntax::AnyJsMaybeOptional(node) => {
                find_direct_token(node.syntax(), T![?])
                    .map(|token| StripTypesState::Erase(vec![token.text_trimmed_range()]))
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let StripTypesState::Unsupported { syntax } = state else {
            return None;
        };

        Some(
            RuleDiagnostic::new(
                category!("transformations/stripTypes"),
                ctx.query().range(),
                markup! {
                    <Emphasis>{syntax}</Emphasis>" cannot be stripped because they generate runtime code."
                },
            )
            .note(markup! {
                "Only type syntax that can be erased is supported. Rewrite this using JavaScript syntax."
            }),
        )
    }

    fn transform(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsBatchMutation> {
        let StripTypesState::Erase(ranges) = state else {
            return None;
        };

        let root = ctx.root();
        let mut mutation = root.clone().begin();

        // The erased ranges are replaced at the token level: every token (or trivia piece)
        // inside an erased range keeps its kind but has its text blanked, so the tree keeps
        // its shape and only its text representation changes.
        for token in tokens_in_ranges(root.syntax(), ranges) {
            mutation.replace_token_discard_trivia(token.clone(), blank_token(&token, ranges));
        }

        Some(mutation)
    }
}

/// Returns `true` if erasing `node` also erases all of its descendants, i.e. the descendants
/// must not produce their own signal.
fn erases_descendants(node: &JsSyntaxNode) -> bool {
    match node.kind() {
        JsSyntaxKind::TS_INTERFACE_DECLARATION
        | JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION
        | JsSyntaxKind::TS_DECLARE_STATEMENT
        | JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION
        | JsSyntaxKind::TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION
        | JsSyntaxKind::TS_TYPE_ANNOTATION
        | JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION
        | JsSyntaxKind::TS_DEFINITE_VARIABLE_ANNOTATION
        | JsSyntaxKind::TS_OPTIONAL_PROPERTY_ANNOTATION
        | JsSyntaxKind::TS_DEFINITE_PROPERTY_ANNOTATION
        | JsSyntaxKind::TS_TYPE_PARAMETERS
        | JsSyntaxKind::TS_TYPE_ARGUMENTS
        | JsSyntaxKind::TS_IMPLEMENTS_CLAUSE
        | JsSyntaxKind::TS_PROPERTY_SIGNATURE_CLASS_MEMBER
        | JsSyntaxKind::TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER
        | JsSyntaxKind::TS_METHOD_SIGNATURE_CLASS_MEMBER
        | JsSyntaxKind::TS_GETTER_SIGNATURE_CLASS_MEMBER
        | JsSyntaxKind::TS_SETTER_SIGNATURE_CLASS_MEMBER
        | JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER
        | JsSyntaxKind::TS_INDEX_SIGNATURE_CLASS_MEMBER
        | JsSyntaxKind::TS_THIS_PARAMETER => true,

        // Everything inside a type is erased by whoever owns the type.
        kind if AnyTsType::can_cast(kind) => true,

        JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION => find_direct_token(node, T![type]).is_some(),
        JsSyntaxKind::JS_IMPORT => JsImport::cast_ref(node)
            .and_then(|import| import.import_clause().ok())
            .is_some_and(|clause| is_type_only_import_clause(&clause)),
        JsSyntaxKind::JS_EXPORT => JsExport::cast_ref(node)
            .and_then(|export| export.export_clause().ok())
            .is_some_and(|clause| is_type_only_export_clause(&clause)),

        _ => false,
    }
}

fn is_type_only_import_clause(clause: &AnyJsImportClause) -> bool {
    match clause {
        AnyJsImportClause::JsImportNamedClause(clause) => clause.type_token().is_some(),
        AnyJsImportClause::JsImportDefaultClause(clause) => clause.type_token().is_some(),
        AnyJsImportClause::JsImportNamespaceClause(clause) => clause.type_token().is_some(),
        _ => false,
    }
}

fn is_type_only_export_clause(clause: &AnyJsExportClause) -> bool {
    match clause {
        AnyJsExportClause::JsExportNamedClause(clause) => clause.type_token().is_some(),
        AnyJsExportClause::JsExportFromClause(clause) => clause.type_token().is_some(),
        AnyJsExportClause::JsExportNamedFromClause(clause) => clause.type_token().is_some(),
        AnyJsExportClause::TsExportDeclareClause(_) => true,
        _ => false,
    }
}

/// Returns the range to erase for `declaration`, extended to the enclosing `export` when the
/// declaration is its clause, as erasing the declaration alone would leave a dangling `export`.
fn export_erase_range(declaration: &AnyTsErasableDeclaration) -> TextRange {
    let export = match declaration.syntax().parent() {
        Some(parent) if parent.kind() == JsSyntaxKind::JS_EXPORT => parent,
        Some(parent) if parent.kind() == JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE => {
            parent.parent().unwrap_or(parent)
        }
        _ => return declaration.range(),
    };

    export.text_trimmed_range()
}

/// Returns the ranges of the type syntax of `wrapper`, i.e. everything around the expression
/// (or assignment) it wraps.
fn wrapper_ranges(wrapper: &AnyTsTypeWrapper) -> Vec<TextRange> {
    let inner = match wrapper {
        AnyTsTypeWrapper::TsAsExpression(node) => node.expression().map(|node| node.range()),
        AnyTsTypeWrapper::TsSatisfiesExpression(node) => node.expression().map(|node| node.range()),
        AnyTsTypeWrapper::TsNonNullAssertionExpression(node) => {
            node.expression().map(|node| node.range())
        }
        AnyTsTypeWrapper::TsTypeAssertionExpression(node) => {
            node.expression().map(|node| node.range())
        }
        AnyTsTypeWrapper::TsInstantiationExpression(node) => {
            node.expression().map(|node| node.range())
        }
        AnyTsTypeWrapper::TsAsAssignment(node) => node.assignment().map(|node| node.range()),
        AnyTsTypeWrapper::TsSatisfiesAssignment(node) => node.assignment().map(|node| node.range()),
        AnyTsTypeWrapper::TsNonNullAssertionAssignment(node) => {
            node.assignment().map(|node| node.range())
        }
        AnyTsTypeWrapper::TsTypeAssertionAssignment(node) => {
            node.assignment().map(|node| node.range())
        }
    };

    let outer = wrapper.range();
    let Ok(inner) = inner else {
        return vec![outer];
    };

    let mut ranges = Vec::new();
    if outer.start() < inner.start() {
        ranges.push(TextRange::new(outer.start(), inner.start()));
    }
    if inner.end() < outer.end() {
        ranges.push(TextRange::new(inner.end(), outer.end()));
    }

    ranges
}

/// Returns the range of a node together with the separator that follows it in a separated
/// list, as erasing only the node may leave the list with a leading comma. A left-over
/// *trailing* comma is valid and needs no such handling.
fn range_with_trailing_separator(node: &JsSyntaxNode) -> TextRange {
    let range = node.text_trimmed_range();

    node.next_sibling_or_token()
        .and_then(|element| element.into_token())
        .filter(|token| token.kind() == T![,])
        .map_or(range, |token| range.cover(token.text_trimmed_range()))
}

/// Returns the class members carrying an `abstract` modifier.
///
/// Only the modifier list a member owns directly is matched. `abstract` is reachable from the
/// two lists below and nowhere else in the grammar, so a `TS_ABSTRACT_MODIFIER` found deeper in
/// a member belongs to a nested class, which erases its own abstract members.
fn abstract_members(class: &AnyJsMaybeAbstractClass) -> impl Iterator<Item = AnyJsClassMember> {
    let members = match class {
        AnyJsMaybeAbstractClass::JsClassDeclaration(class) => class.members(),
        AnyJsMaybeAbstractClass::JsClassExportDefaultDeclaration(class) => class.members(),
    };

    members.into_iter().filter(|member| {
        member
            .syntax()
            .children()
            .filter(|child| {
                matches!(
                    child.kind(),
                    JsSyntaxKind::TS_PROPERTY_SIGNATURE_MODIFIER_LIST
                        | JsSyntaxKind::TS_METHOD_SIGNATURE_MODIFIER_LIST
                )
            })
            .any(|modifiers| {
                modifiers
                    .children()
                    .any(|modifier| modifier.kind() == JsSyntaxKind::TS_ABSTRACT_MODIFIER)
            })
    })
}

fn find_direct_token(node: &JsSyntaxNode, kind: JsSyntaxKind) -> Option<JsSyntaxToken> {
    node.children_with_tokens()
        .filter_map(|element| element.into_token())
        .find(|token| token.kind() == kind)
}

/// Collects every token whose text (or trivia) intersects one of the erased ranges.
fn tokens_in_ranges(root: &JsSyntaxNode, ranges: &[TextRange]) -> Vec<JsSyntaxToken> {
    let mut tokens: Vec<JsSyntaxToken> = Vec::new();

    for range in ranges {
        let mut token = match root.covering_element(*range) {
            NodeOrToken::Token(token) => Some(token),
            NodeOrToken::Node(node) => node.first_token(),
        };

        while let Some(current) = token {
            let current_range = current.text_range();
            if current_range.start() >= range.end() {
                break;
            }
            if current_range.end() > range.start() && !tokens.contains(&current) {
                tokens.push(current.clone());
            }
            token = current.next_token();
        }
    }

    tokens
}

/// Rebuilds `token` with every part inside one of the erased ranges replaced by whitespace.
///
/// The erased ranges always align with token and trivia piece boundaries, so each part (the
/// token text, or one of its trivia pieces) is either kept verbatim or blanked entirely.
fn blank_token(token: &JsSyntaxToken, ranges: &[TextRange]) -> JsSyntaxToken {
    let mut text = String::with_capacity(usize::from(token.text_range().len()));
    let mut leading = Vec::new();
    let mut trailing = Vec::new();

    for piece in token.leading_trivia().pieces() {
        append_trivia_piece(&piece, ranges, &mut text, &mut leading);
    }

    let trimmed = token.text_trimmed_range();
    if ranges.iter().any(|range| range.contains_range(trimmed)) {
        append_blanked_text(token.text_trimmed(), &mut text);
    } else {
        text.push_str(token.text_trimmed());
    }

    for piece in token.trailing_trivia().pieces() {
        append_trivia_piece(&piece, ranges, &mut text, &mut trailing);
    }

    JsSyntaxToken::new_detached(token.kind(), &text, leading, trailing)
}

fn append_trivia_piece(
    piece: &SyntaxTriviaPiece<JsLanguage>,
    ranges: &[TextRange],
    text: &mut String,
    pieces: &mut Vec<TriviaPiece>,
) {
    if ranges
        .iter()
        .any(|range| range.contains_range(piece.text_range()))
    {
        append_blanked_trivia(piece.text(), text, pieces);
    } else {
        pieces.push(TriviaPiece::new(piece.kind(), piece.text_len()));
        text.push_str(piece.text());
    }
}

/// Appends `source` blanked as whitespace trivia of the same byte length, keeping the line
/// breaks as dedicated newline pieces so the line numbers don't shift.
fn append_blanked_trivia(source: &str, text: &mut String, pieces: &mut Vec<TriviaPiece>) {
    let mut whitespace_len = 0u32;
    let mut chars = source.chars().peekable();

    while let Some(char) = chars.next() {
        if matches!(char, '\n' | '\r') {
            if whitespace_len > 0 {
                pieces.push(TriviaPiece::whitespace(whitespace_len));
                whitespace_len = 0;
            }

            text.push(char);
            let mut len = 1u32;
            if char == '\r' && chars.peek() == Some(&'\n') {
                chars.next();
                text.push('\n');
                len = 2;
            }
            pieces.push(TriviaPiece::newline(len));
        } else {
            // One space per byte, so byte offsets are preserved even for multi-byte characters.
            for _ in 0..char.len_utf8() {
                text.push(' ');
            }
            whitespace_len += char.len_utf8() as u32;
        }
    }

    if whitespace_len > 0 {
        pieces.push(TriviaPiece::whitespace(whitespace_len));
    }
}

/// Appends `source` with every character other than a line break replaced by whitespace.
fn append_blanked_text(source: &str, text: &mut String) {
    for char in source.chars() {
        if matches!(char, '\n' | '\r') {
            text.push(char);
        } else {
            for _ in 0..char.len_utf8() {
                text.push(' ');
            }
        }
    }
}
