use crate::{JsBatchMutation, declare_transformation};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::category;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsExportClause, AnyJsExpression, AnyJsImportClause, AnyTsType,
    JsClassDeclaration, JsClassExportDefaultDeclaration, JsExport, JsExportNamedFromSpecifier,
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
    pub AnyTsStrippableSyntax =
        // TS-only syntax that is erased entirely.
        TsInterfaceDeclaration
        | TsTypeAliasDeclaration
        | TsDeclareStatement
        | TsDeclareFunctionDeclaration
        | TsDeclareFunctionExportDefaultDeclaration
        | TsTypeAnnotation
        | TsReturnTypeAnnotation
        | TsDefiniteVariableAnnotation
        | TsOptionalPropertyAnnotation
        | TsDefinitePropertyAnnotation
        | TsTypeParameters
        | TsTypeArguments
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
        | TsThisParameter
        // Expression wrappers: only the type syntax around the inner expression is erased.
        | TsAsExpression
        | TsSatisfiesExpression
        | TsNonNullAssertionExpression
        | TsTypeAssertionExpression
        | TsInstantiationExpression
        | TsAsAssignment
        | TsSatisfiesAssignment
        | TsNonNullAssertionAssignment
        | TsTypeAssertionAssignment
        // TS-only syntax that generates runtime code; it can't be erased.
        | TsEnumDeclaration
        | TsModuleDeclaration
        | TsGlobalDeclaration
        | TsExternalModuleDeclaration
        | TsPropertyParameter
        | TsExportAssignmentClause
        | TsExportAsNamespaceClause
        | TsImportEqualsDeclaration
        // JS nodes hosting TS-only tokens or clauses.
        | JsImport
        | JsExport
        | JsShorthandNamedImportSpecifier
        | JsNamedImportSpecifier
        | JsExportNamedShorthandSpecifier
        | JsExportNamedSpecifier
        | JsExportNamedFromSpecifier
        | JsClassDeclaration
        | JsClassExportDefaultDeclaration
        | JsFormalParameter
        | JsMethodClassMember
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
        let syntax = node.syntax();

        // Constructs inside an erased ancestor (a type, a type-only declaration, ...) are
        // already erased with it and must not produce their own overlapping signal.
        if syntax
            .ancestors()
            .skip(1)
            .any(|ancestor| erases_descendants(&ancestor))
        {
            return None;
        }

        match node {
            // Erased entirely. If the construct is the clause of an `export`, the `export`
            // keyword can't be left dangling, so the whole export is erased with it.
            AnyTsStrippableSyntax::TsInterfaceDeclaration(_)
            | AnyTsStrippableSyntax::TsTypeAliasDeclaration(_)
            | AnyTsStrippableSyntax::TsDeclareStatement(_)
            | AnyTsStrippableSyntax::TsDeclareFunctionDeclaration(_)
            | AnyTsStrippableSyntax::TsDeclareFunctionExportDefaultDeclaration(_) => {
                Some(StripTypesState::Erase(vec![export_erase_range(syntax)]))
            }

            // Erased entirely, never a direct export clause.
            AnyTsStrippableSyntax::TsTypeAnnotation(_)
            | AnyTsStrippableSyntax::TsReturnTypeAnnotation(_)
            | AnyTsStrippableSyntax::TsDefiniteVariableAnnotation(_)
            | AnyTsStrippableSyntax::TsOptionalPropertyAnnotation(_)
            | AnyTsStrippableSyntax::TsDefinitePropertyAnnotation(_)
            | AnyTsStrippableSyntax::TsTypeParameters(_)
            | AnyTsStrippableSyntax::TsImplementsClause(_)
            | AnyTsStrippableSyntax::TsAccessibilityModifier(_)
            | AnyTsStrippableSyntax::TsReadonlyModifier(_)
            | AnyTsStrippableSyntax::TsOverrideModifier(_)
            | AnyTsStrippableSyntax::TsAbstractModifier(_)
            | AnyTsStrippableSyntax::TsDeclareModifier(_)
            | AnyTsStrippableSyntax::TsPropertySignatureClassMember(_)
            | AnyTsStrippableSyntax::TsInitializedPropertySignatureClassMember(_)
            | AnyTsStrippableSyntax::TsMethodSignatureClassMember(_)
            | AnyTsStrippableSyntax::TsGetterSignatureClassMember(_)
            | AnyTsStrippableSyntax::TsSetterSignatureClassMember(_)
            | AnyTsStrippableSyntax::TsConstructorSignatureClassMember(_)
            | AnyTsStrippableSyntax::TsIndexSignatureClassMember(_) => {
                Some(StripTypesState::Erase(vec![syntax.text_trimmed_range()]))
            }

            // The type arguments of an instantiation expression are erased by the wrapper.
            AnyTsStrippableSyntax::TsTypeArguments(_) => {
                if syntax.parent().is_some_and(|parent| {
                    parent.kind() == JsSyntaxKind::TS_INSTANTIATION_EXPRESSION
                }) {
                    None
                } else {
                    Some(StripTypesState::Erase(vec![syntax.text_trimmed_range()]))
                }
            }

            // A `this` parameter also takes its trailing comma with it, as erasing it alone
            // may leave the parameter list with a leading comma.
            AnyTsStrippableSyntax::TsThisParameter(_) => {
                Some(StripTypesState::Erase(vec![range_with_trailing_separator(
                    syntax,
                )]))
            }

            // Expression wrappers: keep the inner expression, erase the type syntax around it.
            // Nested wrappers produce their own signals.
            AnyTsStrippableSyntax::TsAsExpression(_)
            | AnyTsStrippableSyntax::TsSatisfiesExpression(_)
            | AnyTsStrippableSyntax::TsNonNullAssertionExpression(_)
            | AnyTsStrippableSyntax::TsTypeAssertionExpression(_)
            | AnyTsStrippableSyntax::TsInstantiationExpression(_)
            | AnyTsStrippableSyntax::TsAsAssignment(_)
            | AnyTsStrippableSyntax::TsSatisfiesAssignment(_)
            | AnyTsStrippableSyntax::TsNonNullAssertionAssignment(_)
            | AnyTsStrippableSyntax::TsTypeAssertionAssignment(_) => {
                Some(StripTypesState::Erase(wrapper_ranges(syntax)))
            }

            AnyTsStrippableSyntax::TsEnumDeclaration(_) => Some(StripTypesState::Unsupported {
                syntax: "enum declarations",
            }),
            AnyTsStrippableSyntax::TsModuleDeclaration(_) => Some(StripTypesState::Unsupported {
                syntax: "namespace and module declarations",
            }),
            AnyTsStrippableSyntax::TsGlobalDeclaration(_)
            | AnyTsStrippableSyntax::TsExternalModuleDeclaration(_) => {
                Some(StripTypesState::Unsupported {
                    syntax: "ambient module declarations",
                })
            }
            AnyTsStrippableSyntax::TsPropertyParameter(_) => Some(StripTypesState::Unsupported {
                syntax: "parameter properties",
            }),
            AnyTsStrippableSyntax::TsExportAssignmentClause(_) => {
                Some(StripTypesState::Unsupported {
                    syntax: "export = assignments",
                })
            }
            AnyTsStrippableSyntax::TsExportAsNamespaceClause(_) => {
                Some(StripTypesState::Unsupported {
                    syntax: "export as namespace declarations",
                })
            }

            // `import type A = ...` is type-only; without `type` it generates a runtime require.
            AnyTsStrippableSyntax::TsImportEqualsDeclaration(declaration) => {
                if declaration.type_token().is_some() {
                    Some(StripTypesState::Erase(vec![export_erase_range(syntax)]))
                } else {
                    Some(StripTypesState::Unsupported {
                        syntax: "import = declarations",
                    })
                }
            }

            // `import type ... from "..."` is erased entirely.
            AnyTsStrippableSyntax::JsImport(import) => import
                .import_clause()
                .is_ok_and(|clause| is_type_only_import_clause(&clause))
                .then(|| StripTypesState::Erase(vec![syntax.text_trimmed_range()])),

            // `export type { ... }` and `export declare ...` are erased entirely. Exports of a
            // type-only declaration are handled by the declaration itself.
            AnyTsStrippableSyntax::JsExport(export) => export
                .export_clause()
                .is_ok_and(|clause| is_type_only_export_clause(&clause))
                .then(|| StripTypesState::Erase(vec![syntax.text_trimmed_range()])),

            // `import { type A } from "..."` erases the specifier together with its trailing
            // comma, keeping the (side effect of the) import itself, like `verbatimModuleSyntax`
            // does.
            AnyTsStrippableSyntax::JsShorthandNamedImportSpecifier(_)
            | AnyTsStrippableSyntax::JsNamedImportSpecifier(_)
            | AnyTsStrippableSyntax::JsExportNamedShorthandSpecifier(_)
            | AnyTsStrippableSyntax::JsExportNamedSpecifier(_)
            | AnyTsStrippableSyntax::JsExportNamedFromSpecifier(_) => {
                find_direct_token(syntax, T![type])
                    .map(|_| StripTypesState::Erase(vec![range_with_trailing_separator(syntax)]))
            }

            // JS nodes hosting TS-only tokens.
            AnyTsStrippableSyntax::JsClassDeclaration(_)
            | AnyTsStrippableSyntax::JsClassExportDefaultDeclaration(_) => {
                let abstract_token = find_direct_token(syntax, T![abstract])?;

                // Erasing `abstract` from the class alone would leave its abstract members
                // behind, which no longer parse, so they are erased together.
                let mut ranges = vec![abstract_token.text_trimmed_range()];
                ranges.extend(abstract_members(syntax).map(|member| member.text_trimmed_range()));

                Some(StripTypesState::Erase(ranges))
            }
            AnyTsStrippableSyntax::JsFormalParameter(_)
            | AnyTsStrippableSyntax::JsMethodClassMember(_) => find_direct_token(syntax, T![?])
                .map(|token| StripTypesState::Erase(vec![token.text_trimmed_range()])),
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

/// Returns the range to erase for a declaration, extended to the enclosing `export` when the
/// declaration is its clause, as erasing the declaration alone would leave a dangling `export`.
fn export_erase_range(node: &JsSyntaxNode) -> TextRange {
    let target = match node.parent() {
        Some(parent) if parent.kind() == JsSyntaxKind::JS_EXPORT => parent,
        Some(parent) if parent.kind() == JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE => {
            parent.parent().unwrap_or(parent)
        }
        _ => node.clone(),
    };

    target.text_trimmed_range()
}

/// Returns the ranges around the inner expression of a wrapper like `expr as T`, `expr!`,
/// `<T>expr`, or `expr<T>`.
fn wrapper_ranges(node: &JsSyntaxNode) -> Vec<TextRange> {
    let Some(inner) = node.children().find(|child| {
        AnyJsExpression::can_cast(child.kind()) || AnyJsAssignment::can_cast(child.kind())
    }) else {
        return vec![node.text_trimmed_range()];
    };

    let outer = node.text_trimmed_range();
    let inner_range = inner.text_trimmed_range();
    let mut ranges = Vec::new();
    if outer.start() < inner_range.start() {
        ranges.push(TextRange::new(outer.start(), inner_range.start()));
    }
    if inner_range.end() < outer.end() {
        ranges.push(TextRange::new(inner_range.end(), outer.end()));
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
fn abstract_members(class: &JsSyntaxNode) -> impl Iterator<Item = JsSyntaxNode> {
    class
        .children()
        .find(|child| child.kind() == JsSyntaxKind::JS_CLASS_MEMBER_LIST)
        .into_iter()
        .flat_map(|members| members.children())
        .filter(|member| {
            member
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
