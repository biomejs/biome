use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsExpression, AnyJsFunction, AnyJsRoot, JsCallExpression,
    JsExpressionStatement, JsLanguage, JsModuleItemList, JsStatementList, JsSyntaxKind,
    JsSyntaxNode, T,
};
use biome_rowan::TriviaPieceKind;
use biome_rowan::{
    AstNode, BatchMutation, BatchMutationExt, Direction, SyntaxElement, SyntaxTriviaPiece,
};
use rustc_hash::FxHashSet;

struct HeaderTrivia {
    declaration: Vec<(TriviaPieceKind, String)>,
    first_item: Vec<SyntaxTriviaPiece<JsLanguage>>,
}

pub(crate) fn extract_module_constant(
    root: &AnyJsRoot,
    model: &SemanticModel,
    target: &JsSyntaxNode,
    value: AnyJsExpression,
    candidate_name: &str,
) -> Option<(BatchMutation<JsLanguage>, String)> {
    let list = root_list(root)?;
    let top_level_item = target
        .ancestors()
        .find(|ancestor| ancestor.parent().is_some_and(|parent| parent == list))?;
    if target
        .ancestors()
        .any(|ancestor| ancestor.kind() == JsSyntaxKind::JS_WITH_STATEMENT)
    {
        // A `with` scope can dynamically shadow any generated identifier.
        return None;
    }
    if direct_eval_affects_target(root, model, target) {
        // Direct eval in a sloppy script can introduce a binding into an enclosing function.
        return None;
    }

    let occupied_reference_names = occupied_reference_names(model);
    let name = collision_free_name(model, target, candidate_name, &occupied_reference_names);
    let replacement =
        make::js_identifier_expression(make::js_reference_identifier(make::ident(&name)))
            .into_syntax();
    let replacement = preserve_target_trivia(target, replacement)?;
    let transformed_item = replace_target(&top_level_item, target, replacement)?;
    let insertion_slot = insertion_slot(&list, top_level_item.index())?;
    let line_ending = source_line_ending(&list);
    let value = trim_expression_trivia(value)?;
    let header_trivia = if insertion_slot == 0 {
        Some(split_header_trivia(&list)?)
    } else {
        None
    };
    let declaration = make_declaration(
        &name,
        value,
        header_trivia
            .as_ref()
            .map_or(&[][..], |trivia| trivia.declaration.as_slice()),
        header_trivia
            .as_ref()
            .map_or(&[][..], |trivia| trivia.first_item.as_slice()),
        insertion_slot == 0,
        &line_ending,
    );

    let replaced_list = list.clone().splice_slots(
        top_level_item.index()..=top_level_item.index(),
        [Some(SyntaxElement::Node(transformed_item))],
    );
    let new_list = if insertion_slot == 0 {
        // Detach the original first item's remaining whitespace before inserting at slot zero.
        // Leading trivia on the first list item is rendered before every list child by the CST.
        let first_item = replaced_list.first_child()?;
        let first_item = first_item.with_leading_trivia_pieces([])?;
        let list_without_first_trivia =
            replaced_list.splice_slots(0..=0, [Some(SyntaxElement::Node(first_item))]);
        let list_with_declaration =
            list_without_first_trivia.splice_slots(0..0, [Some(SyntaxElement::Node(declaration))]);
        list_with_declaration
    } else {
        replaced_list.splice_slots(
            insertion_slot..insertion_slot,
            [Some(SyntaxElement::Node(declaration))],
        )
    };

    let mut mutation = root.clone().begin();
    // The replacement list already contains the deliberate header/item trivia split. The
    // default replacement would copy the old list's leading trivia and duplicate the header.
    mutation.replace_element_discard_trivia(list.into(), new_list.into());

    Some((mutation, name))
}

fn root_list(root: &AnyJsRoot) -> Option<JsSyntaxNode> {
    // Expression snippets and declaration-only roots cannot receive a runtime binding.
    let list = match root {
        AnyJsRoot::JsModule(module) => module.items().into_syntax(),
        AnyJsRoot::JsScript(script) => script.statements().into_syntax(),
        _ => return None,
    };

    (JsModuleItemList::can_cast(list.kind()) || JsStatementList::can_cast(list.kind()))
        .then_some(list)
}

fn collision_free_name(
    model: &SemanticModel,
    target: &JsSyntaxNode,
    candidate_name: &str,
    occupied_reference_names: &FxHashSet<String>,
) -> String {
    if name_is_free_in_target_scopes(model, target, candidate_name, occupied_reference_names) {
        return candidate_name.to_string();
    }

    let mut suffix = 2;
    loop {
        let name = format!("{candidate_name}_{suffix}");
        if name_is_free_in_target_scopes(model, target, &name, occupied_reference_names) {
            return name;
        }
        suffix += 1;
    }
}

fn name_is_free_in_target_scopes(
    model: &SemanticModel,
    target: &JsSyntaxNode,
    name: &str,
    occupied_reference_names: &FxHashSet<String>,
) -> bool {
    model
        .scope(target)
        .ancestors()
        .all(|scope| scope.get_binding(name).is_none())
        && !occupied_reference_names.contains(name)
}

fn occupied_reference_names(model: &SemanticModel) -> FxHashSet<String> {
    let mut names = FxHashSet::default();
    for reference in model.all_unresolved_references() {
        if let Some(token) = reference.syntax().first_token() {
            names.insert(token.text_trimmed().to_string());
        }
    }
    for reference in model.all_global_references() {
        if let Some(token) = reference.syntax().first_token() {
            names.insert(token.text_trimmed().to_string());
        }
    }
    names
}

fn direct_eval_affects_target(
    root: &AnyJsRoot,
    model: &SemanticModel,
    target: &JsSyntaxNode,
) -> bool {
    if !matches!(root, AnyJsRoot::JsScript(_)) {
        return false;
    }

    let target_functions = target
        .ancestors()
        .filter(|ancestor| AnyJsFunction::can_cast(ancestor.kind()))
        .collect::<Vec<_>>();

    root.syntax()
        .descendants()
        .filter_map(JsCallExpression::cast)
        .any(|call| {
            if !is_direct_eval(model, &call) {
                return false;
            }

            let eval_function = call
                .syntax()
                .ancestors()
                .find(|ancestor| AnyJsFunction::can_cast(ancestor.kind()));
            eval_function.is_none_or(|eval_function| {
                target_functions
                    .iter()
                    .any(|target_function| target_function == &eval_function)
            })
        })
}

fn is_direct_eval(model: &SemanticModel, call: &JsCallExpression) -> bool {
    let Some(reference) = call
        .callee()
        .ok()
        .map(|callee| callee.omit_parentheses())
        .and_then(|callee| callee.as_js_reference_identifier())
    else {
        return false;
    };

    reference
        .value_token()
        .is_ok_and(|token| token.text_trimmed() == "eval")
        && model.binding(&reference).is_none()
}

fn preserve_target_trivia(
    target: &JsSyntaxNode,
    replacement: JsSyntaxNode,
) -> Option<JsSyntaxNode> {
    let replacement = if let Some(trivia) = target.first_leading_trivia() {
        replacement.with_leading_trivia_pieces(trivia.pieces())?
    } else {
        replacement
    };

    if let Some(trivia) = target.last_trailing_trivia() {
        replacement.with_trailing_trivia_pieces(trivia.pieces())
    } else {
        Some(replacement)
    }
}

fn trim_expression_trivia(value: AnyJsExpression) -> Option<AnyJsExpression> {
    let value = value
        .into_syntax()
        .with_leading_trivia_pieces([])?
        .with_trailing_trivia_pieces([])?;

    AnyJsExpression::cast(value)
}

fn split_header_trivia(list: &JsSyntaxNode) -> Option<HeaderTrivia> {
    let first_item = list.first_child()?;
    let pieces = first_item
        .first_leading_trivia()
        .map(|trivia| trivia.pieces().collect::<Vec<_>>())
        .unwrap_or_default();
    let split_index = if pieces.iter().any(|piece| piece.kind().is_comment()) {
        find_first_blank_line_index(&pieces)?
    } else {
        pieces.len()
    };
    let first_item = pieces[split_index..].to_vec();
    if first_item.iter().any(|piece| piece.kind().is_comment()) {
        // Comments after the header separator belong to the original item and cannot safely be
        // relocated through a slot-zero insertion.
        return None;
    }

    Some(HeaderTrivia {
        declaration: pieces[..split_index]
            .iter()
            .map(|piece| (piece.kind(), piece.text().to_string()))
            .collect(),
        first_item,
    })
}

fn find_first_blank_line_index(pieces: &[SyntaxTriviaPiece<JsLanguage>]) -> Option<usize> {
    for (index, piece) in pieces.iter().enumerate() {
        if !piece.is_newline() {
            continue;
        }

        let mut next_index = index + 1;
        while pieces
            .get(next_index)
            .is_some_and(SyntaxTriviaPiece::is_whitespace)
        {
            next_index += 1;
        }
        if pieces
            .get(next_index)
            .is_some_and(SyntaxTriviaPiece::is_newline)
        {
            return Some(next_index);
        }
    }
    None
}

fn replace_target(
    top_level_item: &JsSyntaxNode,
    target: &JsSyntaxNode,
    replacement: JsSyntaxNode,
) -> Option<JsSyntaxNode> {
    let mut child = target.clone();
    let mut replacement = replacement;

    loop {
        let parent = child.parent()?;
        let is_top_level_item = &parent == top_level_item;
        // Keep the original parent for the next ancestor step; replacement returns a new tree.
        let updated_parent = parent
            .clone()
            .replace_child(child.into(), replacement.into())?;
        if is_top_level_item {
            return Some(updated_parent);
        }
        child = parent;
        replacement = updated_parent;
    }
}

fn insertion_slot(list: &JsSyntaxNode, target_index: usize) -> Option<usize> {
    let mut slot = 0;
    let mut in_directive_prologue = true;

    for child in list.children() {
        if is_import_like(&child) {
            slot = child.index() + 1;
        } else if in_directive_prologue && is_directive_statement(&child) {
            slot = child.index() + 1;
        } else {
            // A non-leading statement ends the prologue, so later string expressions are ordinary code.
            in_directive_prologue = false;
        }
    }

    // Inserting after the target could move initialization below code that uses it.
    (slot <= target_index).then_some(slot)
}

fn is_import_like(node: &JsSyntaxNode) -> bool {
    matches!(
        node.kind(),
        JsSyntaxKind::JS_IMPORT | JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION
    ) || (node.kind() == JsSyntaxKind::JS_EXPORT
        && node
            .descendants()
            .any(|descendant| descendant.kind() == JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION))
}

fn is_directive_statement(node: &JsSyntaxNode) -> bool {
    let Some(statement) = JsExpressionStatement::cast_ref(node) else {
        return false;
    };

    statement.expression().ok().is_some_and(|expression| {
        expression
            .as_any_js_literal_expression()
            .is_some_and(|literal| literal.as_js_string_literal_expression().is_some())
    })
}

fn make_declaration(
    name: &str,
    value: AnyJsExpression,
    declaration_leading_trivia: &[(TriviaPieceKind, String)],
    declaration_trailing_trivia: &[SyntaxTriviaPiece<JsLanguage>],
    add_trailing_line_ending: bool,
    line_ending: &str,
) -> JsSyntaxNode {
    let mut leading_trivia = declaration_leading_trivia.to_vec();
    if !matches!(leading_trivia.last(), Some((TriviaPieceKind::Newline, _))) {
        leading_trivia.push((TriviaPieceKind::Newline, line_ending.to_string()));
    }

    let whitespace = [(TriviaPieceKind::Whitespace, " ")];
    let const_token = make::token(T![const])
        .with_leading_trivia(
            leading_trivia
                .iter()
                .map(|(kind, text)| (*kind, text.as_str())),
        )
        .with_trailing_trivia(whitespace);
    let name_token = make::ident(name).with_trailing_trivia(whitespace);
    let initializer =
        make::js_initializer_clause(make::token(T![=]).with_trailing_trivia(whitespace), value);
    // Identifier bindings are nested in the binding-pattern union in the CST schema.
    let binding = AnyJsBinding::JsIdentifierBinding(make::js_identifier_binding(name_token));
    let binding_pattern = AnyJsBindingPattern::AnyJsBinding(binding);
    let declarator = make::js_variable_declarator(binding_pattern)
        .with_initializer(initializer)
        .build();
    let declaration = make::js_variable_declaration(
        const_token,
        make::js_variable_declarator_list([declarator], []),
    )
    .build();

    let mut statement = make::js_variable_statement(declaration);
    let mut semicolon = make::token(T![;]);
    if add_trailing_line_ending {
        let mut trailing_trivia = vec![(TriviaPieceKind::Newline, line_ending.to_string())];
        trailing_trivia.extend(
            declaration_trailing_trivia
                .iter()
                .map(|piece| (piece.kind(), piece.text().to_string())),
        );
        semicolon = semicolon.with_trailing_trivia(
            trailing_trivia
                .iter()
                .map(|(kind, text)| (*kind, text.as_str())),
        );
    }
    statement = statement.with_semicolon_token(semicolon);
    statement.build().into_syntax()
}

fn source_line_ending(list: &JsSyntaxNode) -> &'static str {
    // Detached factory tokens do not inherit the source file's line-separator convention.
    for token in list.descendants_tokens(Direction::Next) {
        for text in [
            token.leading_trivia().text(),
            token.text(),
            token.trailing_trivia().text(),
        ] {
            if let Some(line_ending) = find_line_ending(text) {
                return line_ending;
            }
        }
    }

    "\n"
}

fn find_line_ending(text: &str) -> Option<&'static str> {
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        let line_ending = match character {
            '\r' if characters.peek() == Some(&'\n') => "\r\n",
            '\r' => "\r",
            '\n' => "\n",
            '\u{2028}' => "\u{2028}",
            '\u{2029}' => "\u{2029}",
            _ => continue,
        };
        return Some(line_ending);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::extract_module_constant;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::{
        AnyJsExpression, JsFunctionDeclaration, JsModuleItemList, JsReferenceIdentifier,
        JsRegexLiteralExpression,
    };
    use biome_languages::JsFileSource;
    use biome_rowan::{AstNode, AstNodeList, AstSeparatedList};

    #[test]
    fn extracts_expression_after_imports_and_directives() {
        let parsed = parse(
            r#"import value from "module";
"use strict";
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, name) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();
        let transformed = parse(
            &output,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        assert_eq!(name, "REGEX");
        assert!(output.contains("const REGEX = /x/;"));
        assert!(output.contains("return REGEX.test(input);"));
        assert!(
            output.find("import value from \"module\";").unwrap()
                < output.find("const REGEX").unwrap()
        );
        assert!(output.find("\"use strict\";").unwrap() < output.find("const REGEX").unwrap());

        let module_items = transformed
            .syntax()
            .descendants()
            .find_map(JsModuleItemList::cast)
            .expect("expected a module item list");
        let generated_declaration = module_items
            .iter()
            .find_map(|item| {
                let statement = item
                    .as_any_js_statement()?
                    .as_js_variable_statement()?
                    .clone();
                let declarator = statement
                    .declaration()
                    .ok()?
                    .declarators()
                    .iter()
                    .next()?
                    .ok()?;
                let id = declarator.id().ok()?;
                let binding = id.as_any_js_binding()?.as_js_identifier_binding()?;
                (binding
                    .name_token()
                    .ok()
                    .is_some_and(|token| token.text_trimmed() == "REGEX"))
                .then_some(statement)
            })
            .expect("expected REGEX at module scope");
        assert!(
            generated_declaration
                .syntax()
                .parent()
                .is_some_and(|parent| JsModuleItemList::can_cast(parent.kind()))
        );
        assert!(
            generated_declaration
                .syntax()
                .descendants()
                .any(|node| JsRegexLiteralExpression::can_cast(node.kind()))
        );

        let function = transformed
            .syntax()
            .descendants()
            .find_map(JsFunctionDeclaration::cast)
            .expect("expected read function");
        assert!(
            function
                .syntax()
                .descendants()
                .filter_map(JsReferenceIdentifier::cast)
                .any(|reference| {
                    reference
                        .value_token()
                        .ok()
                        .is_some_and(|token| token.text_trimmed() == "REGEX")
                })
        );
        assert!(
            !function
                .syntax()
                .descendants()
                .any(|node| JsRegexLiteralExpression::can_cast(node.kind()))
        );
    }

    #[test]
    fn extracts_script_expression_with_collision_suffix() {
        let parsed = parse(
            r#"const REGEX = 0;
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, name) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a script-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(name, "REGEX_2");
        assert!(output.contains("const REGEX_2 = /x/;"));
        assert!(output.contains("return REGEX_2.test(input);"));
    }

    #[test]
    fn transfers_header_trivia_when_target_is_not_first_item() {
        let source = "// header\n \t\nconst before = 0;\nfunction read(input) {\n    return /x/.test(input);\n}\n";
        let parsed = parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(output.matches("// header").count(), 1);
        assert!(output.starts_with("// header\n \t\nconst REGEX = /x/;\n\nconst before = 0;"));
    }

    #[test]
    fn rejects_attached_first_item_documentation() {
        let parsed = parse(
            r#"/** Documentation for before. */
const before = 0;
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn rejects_later_comment_blocks_after_header_separator() {
        let parsed = parse(
            r#"/** File header. */

/** Documentation for before. */
const before = 0;
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn avoids_shadowing_target_scope_bindings() {
        let parsed = parse(
            r#"function read(REGEX) {
    return /x/.test(REGEX);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, name) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(name, "REGEX_2");
        assert!(output.contains("const REGEX_2 = /x/;"));
        assert!(output.contains("return REGEX_2.test(REGEX);"));
    }

    #[test]
    fn avoids_capturing_unresolved_reference_elsewhere() {
        let parsed = parse(
            r#"function read(input) {
    return /x/.test(input);
}
console.log(REGEX);
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, name) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(name, "REGEX_2");
        assert!(output.contains("const REGEX_2 = /x/;"));
        assert!(output.contains("console.log(REGEX);"));
    }

    #[test]
    fn avoids_capturing_configured_global_reference_elsewhere() {
        let parsed = parse(
            r#"function read(input) {
    return /x/.test(input);
}
console.log(REGEX);
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let mut options = SemanticModelOptions::default();
        options.globals.insert("REGEX".to_string());
        let model = semantic_model(&parsed.tree(), options);
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, name) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(name, "REGEX_2");
        assert!(output.contains("const REGEX_2 = /x/;"));
        assert!(output.contains("console.log(REGEX);"));
    }

    #[test]
    fn rejects_target_under_dynamic_with() {
        let parsed = parse(
            r#"function read(input) {
    with (scope) {
        return /x/.test(input);
    }
}
"#,
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn rejects_target_affected_by_direct_eval_in_sloppy_script() {
        let parsed = parse(
            r#"function read(input) {
    eval("var REGEX = /other/;");
    return /x/.test(input);
}
"#,
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn rejects_target_before_later_import() {
        let parsed = parse(
            r#"function read(input) {
    return /x/.test(input);
}
import value from "module";
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        assert!(
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .is_none()
        );
    }

    #[test]
    fn places_constant_after_typescript_import_equals() {
        let parsed = parse(
            r#"import value = require("module");
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a TypeScript module-level extraction");
        let output = mutation.commit().to_string();

        assert!(
            output.find("import value = require(\"module\");").unwrap()
                < output.find("const REGEX").unwrap()
        );
        assert!(output.find("const REGEX").unwrap() < output.find("function read").unwrap());
    }

    #[test]
    fn places_constant_after_exported_typescript_import_equals() {
        let parsed = parse(
            r#"export import value = require("module");
function read(input) {
    return /x/.test(input);
}
"#,
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a TypeScript module-level extraction");
        let output = mutation.commit().to_string();

        assert!(
            output
                .find("export import value = require(\"module\");")
                .unwrap()
                < output.find("const REGEX").unwrap()
        );
        assert!(output.find("const REGEX").unwrap() < output.find("function read").unwrap());
    }

    #[test]
    fn preserves_target_comments_without_duplicating_them() {
        let parsed = parse(
            r#"function read(input) {
    return /* before */ /x/ /* after */.test(input);
}
"#,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a module-level extraction");
        let output = mutation.commit().to_string();

        assert_eq!(output.matches("/* before */").count(), 1);
        assert_eq!(output.matches("/* after */").count(), 1);
        assert!(output.contains("const REGEX = /x/;"));
        assert!(output.contains("return /* before */ REGEX /* after */.test(input);"));
    }

    #[test]
    fn places_script_constant_after_directives_with_crlf() {
        let parsed = parse(
            "\"use strict\";\r\nfunction read(input) {\r\n    return /x/.test(input);\r\n}\r\n",
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a script-level extraction");
        let output = mutation.commit().to_string();

        assert!(output.contains("\"use strict\";\r\nconst REGEX = /x/;\r\nfunction"));
        assert!(output.contains("return REGEX.test(input);"));
    }

    #[test]
    fn preserves_unicode_line_separators() {
        for separator in ["\u{2028}", "\u{2029}"] {
            let source = format!(
                "function read(input) {{{separator}    return /x/.test(input);{separator}}}{separator}"
            );
            let parsed = parse(
                &source,
                JsFileSource::js_script(),
                JsParserOptions::default(),
            );
            let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
            let target = parsed
                .syntax()
                .descendants()
                .find_map(JsRegexLiteralExpression::cast)
                .expect("expected a regex literal");
            let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

            let (mutation, _) =
                extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                    .expect("expected a script-level extraction");
            let output = mutation.commit().to_string();

            assert!(output.contains(&format!("const REGEX = /x/;{separator}function")));
        }
    }

    #[test]
    fn preserves_crlf_inside_multiline_comment_tokens() {
        let source = "const before = /* first line\r\nsecond line */ 0;\nfunction read(input) {\n    return /x/.test(input);\n}\n";
        let parsed = parse(
            source,
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let target = parsed
            .syntax()
            .descendants()
            .find_map(JsRegexLiteralExpression::cast)
            .expect("expected a regex literal");
        let value = AnyJsExpression::cast(target.syntax().clone()).expect("regex expression");

        let (mutation, _) =
            extract_module_constant(&parsed.tree(), &model, target.syntax(), value, "REGEX")
                .expect("expected a script-level extraction");
        let output = mutation.commit().to_string();

        assert!(output.contains("const REGEX = /x/;\r\nconst before"));
    }
}
