//! The value pass: computes what is known about the value of each template
//! expression and of each `const`, and which variables hold a store.
//!
//! Identifiers are followed to the initializer of a `const` declared once,
//! across the scripts and the template. A rune call is not looked into, so
//! nothing is known of `x` in `const x = $state('a')`.

use super::builder::ModelBuilder;
use super::model::*;
use super::script::{Declared, Tree};
use super::snippets::root_expression;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, JsBinaryOperator,
    JsIdentifierBinding, JsSyntaxKind, JsSyntaxNode, JsVariableDeclarator,
};
use biome_rowan::{AstNode, TextRange};
use rustc_hash::{FxHashMap, FxHashSet};

/// How many identifiers are followed through `const` initializers.
const MAX_DEPTH: u8 = 32;

/// How many strings a set of possible values may hold.
const MAX_STRINGS: usize = 16;

/// Computes the facts of every template expression and every `const`.
pub(crate) fn collect_facts(builder: &mut ModelBuilder, trees: &[Tree], declared: &[Declared]) {
    let references: FxHashMap<_, _> = builder
        .data
        .references
        .iter()
        .filter(|reference| reference.via.is_none())
        .filter_map(|reference| match reference.resolution {
            Resolution::Symbol(symbol) => Some((reference.range, symbol)),
            _ => None,
        })
        .collect();
    let globals: FxHashSet<_> = builder
        .data
        .references
        .iter()
        .filter(|reference| reference.resolution == Resolution::Global)
        .map(|reference| reference.range)
        .collect();
    let mut folder = Folder {
        builder,
        trees,
        declared,
        references,
        globals,
        memo: FxHashMap::default(),
        visiting: FxHashSet::default(),
    };

    let mut symbol_facts = Vec::new();
    for entry in declared {
        let facts = folder.symbol_facts(entry.symbol, 0);
        if !facts.is_empty() {
            symbol_facts.push((entry.symbol, facts));
        }
    }
    let mut snippet_facts = Vec::new();
    for (index, tree) in trees.iter().enumerate() {
        let Some(snippet) = tree.snippet else {
            continue;
        };
        if let Some(expression) = root_expression(tree.root) {
            let facts = folder.fold(index, &expression, 0);
            snippet_facts.push((snippet, facts));
        }
    }
    for (symbol, facts) in symbol_facts {
        builder.symbol_mut(symbol).facts = facts;
    }
    for (snippet, facts) in snippet_facts {
        builder.data.snippets[snippet.index()].facts = facts;
    }
}

struct Folder<'a, 'b> {
    builder: &'a ModelBuilder,
    trees: &'a [Tree<'b>],
    declared: &'a [Declared],
    /// The symbol each reference resolves to, by the host range of the
    /// reference.
    references: FxHashMap<TextRange, SymbolId>,
    /// The host ranges of the references nothing in the component declares.
    globals: FxHashSet<TextRange>,
    memo: FxHashMap<SymbolId, ValueFacts>,
    visiting: FxHashSet<SymbolId>,
}

impl Folder<'_, '_> {
    fn symbol_facts(&mut self, symbol: SymbolId, depth: u8) -> ValueFacts {
        if let Some(facts) = self.memo.get(&symbol) {
            return facts.clone();
        }
        if depth > MAX_DEPTH || !self.visiting.insert(symbol) {
            return ValueFacts::default();
        }
        let facts = self
            .initializer(symbol)
            .map_or_else(ValueFacts::default, |(tree, init)| {
                self.fold(tree, &init, depth + 1)
            });
        self.visiting.remove(&symbol);
        self.memo.insert(symbol, facts.clone());
        facts
    }

    /// Returns the tree and the initializer of a `const` that declares one
    /// variable with a plain name.
    fn initializer(&self, symbol: SymbolId) -> Option<(usize, AnyJsExpression)> {
        let data = &self.builder.data.symbols[symbol.index()];
        if data.declaration_kind != DeclarationKind::Const {
            return None;
        }
        let declared = self.declared.iter().find(|entry| entry.symbol == symbol)?;
        let init = const_initializer(&declared.binding)?;
        Some((declared.tree, init))
    }

    fn fold(&mut self, tree: usize, expression: &AnyJsExpression, depth: u8) -> ValueFacts {
        if depth > MAX_DEPTH {
            return ValueFacts::default();
        }
        let expression = expression.clone().omit_parentheses();
        match &expression {
            AnyJsExpression::AnyJsLiteralExpression(literal) => literal_facts(literal),
            AnyJsExpression::JsTemplateExpression(template) => {
                if template.tag().is_some() {
                    return ValueFacts::default();
                }
                let mut facts = string_facts(String::new());
                for element in template.elements() {
                    let part = match element {
                        AnyJsTemplateElement::JsTemplateChunkElement(chunk) => chunk
                            .template_chunk_token()
                            .ok()
                            .and_then(|token| unescape(token.text_trimmed()))
                            .map_or_else(ValueFacts::default, string_facts),
                        AnyJsTemplateElement::JsTemplateElement(element) => {
                            match element.expression() {
                                Ok(inner) => to_string(self.fold(tree, &inner, depth + 1)),
                                Err(_) => ValueFacts::default(),
                            }
                        }
                    };
                    facts = concat(facts, part);
                }
                facts
            }
            AnyJsExpression::JsBinaryExpression(binary) => {
                if !matches!(binary.operator(), Ok(JsBinaryOperator::Plus)) {
                    return ValueFacts::default();
                }
                let (Ok(left), Ok(right)) = (binary.left(), binary.right()) else {
                    return ValueFacts::default();
                };
                let left = self.fold(tree, &left, depth + 1);
                let right = self.fold(tree, &right, depth + 1);
                // `+` concatenates when either side is a string.
                let is_string = |facts: &ValueFacts| {
                    matches!(facts.value, Some(StaticValue::String(_)))
                        || facts.strings.is_some()
                        || facts.prefix.is_some()
                        || facts.suffix.is_some()
                };
                if is_string(&left) || is_string(&right) {
                    concat(to_string(left), to_string(right))
                } else {
                    ValueFacts::default()
                }
            }
            AnyJsExpression::JsConditionalExpression(conditional) => {
                let (Ok(consequent), Ok(alternate)) =
                    (conditional.consequent(), conditional.alternate())
                else {
                    return ValueFacts::default();
                };
                let consequent = self.fold(tree, &consequent, depth + 1);
                let alternate = self.fold(tree, &alternate, depth + 1);
                either(consequent, alternate)
            }
            AnyJsExpression::JsLogicalExpression(logical) => {
                let (Ok(left), Ok(right)) = (logical.left(), logical.right()) else {
                    return ValueFacts::default();
                };
                let left = self.fold(tree, &left, depth + 1);
                let right = self.fold(tree, &right, depth + 1);
                let mut facts = either(left, right);
                // The result is one of the operands, but which one depends on
                // their values, so only the possible strings are known.
                facts.value = None;
                facts.prefix = None;
                facts.suffix = None;
                facts
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                let Ok(reference) = identifier.name() else {
                    return ValueFacts::default();
                };
                self.identifier(tree, reference.syntax(), depth)
            }
            AnyJsExpression::TsAsExpression(_)
            | AnyJsExpression::TsSatisfiesExpression(_)
            | AnyJsExpression::TsNonNullAssertionExpression(_) => {
                match expression.clone().omit_type_wrappers() {
                    Some(inner) if inner != expression => self.fold(tree, &inner, depth + 1),
                    _ => ValueFacts::default(),
                }
            }
            _ => ValueFacts::default(),
        }
    }

    fn identifier(&mut self, tree: usize, reference: &JsSyntaxNode, depth: u8) -> ValueFacts {
        let host = self.trees[tree].host(reference.text_trimmed_range());
        if let Some(symbol) = self.references.get(&host).copied() {
            return self.symbol_facts(symbol, depth + 1);
        }
        if self.globals.contains(&host) && reference.text_trimmed() == "undefined" {
            return ValueFacts {
                value: Some(StaticValue::Undefined),
                ..ValueFacts::default()
            };
        }
        // A `const` declared inside a function of the same tree.
        let js = self.trees[tree].js;
        let Some(binding) = biome_js_syntax::JsReferenceIdentifier::cast_ref(reference)
            .and_then(|reference| js.binding(&reference))
        else {
            return ValueFacts::default();
        };
        match const_initializer(&binding.syntax()) {
            Some(init) => self.fold(tree, &init, depth + 1),
            None => ValueFacts::default(),
        }
    }
}

/// Returns the initializer of the `const` that declares `binding`, when the
/// declarator declares `binding` alone and the initializer is not a rune
/// call.
fn const_initializer(binding: &JsSyntaxNode) -> Option<AnyJsExpression> {
    if !JsIdentifierBinding::can_cast(binding.kind()) {
        return None;
    }
    let declarator = JsVariableDeclarator::cast(binding.parent()?)?;
    let is_const = declarator
        .syntax()
        .ancestors()
        .find(|ancestor| ancestor.kind() == JsSyntaxKind::JS_VARIABLE_DECLARATION)
        .and_then(|declaration| {
            biome_js_syntax::JsVariableDeclaration::cast(declaration)?
                .kind()
                .ok()
        })
        .map_or(
            // A `{@const}` tag keeps its `const` keyword in the host document.
            declarator
                .syntax()
                .parent()
                .is_some_and(|parent| parent.kind() == JsSyntaxKind::JS_SVELTE_LEGACY_CONST_ROOT),
            |token| token.kind() == JsSyntaxKind::CONST_KW,
        );
    if !is_const {
        return None;
    }
    declarator.initializer()?.expression().ok()
}

fn literal_facts(literal: &AnyJsLiteralExpression) -> ValueFacts {
    let value = match literal {
        AnyJsLiteralExpression::JsStringLiteralExpression(string) => {
            return string
                .inner_string_text()
                .ok()
                .and_then(|text| unescape(text.text()))
                .map_or_else(ValueFacts::default, string_facts);
        }
        AnyJsLiteralExpression::JsNumberLiteralExpression(number) => number
            .value_token()
            .ok()
            .map(|token| StaticValue::Number(token.text_trimmed().into())),
        AnyJsLiteralExpression::JsBooleanLiteralExpression(boolean) => boolean
            .value_token()
            .ok()
            .map(|token| StaticValue::Bool(token.text_trimmed() == "true")),
        AnyJsLiteralExpression::JsNullLiteralExpression(_) => Some(StaticValue::Null),
        _ => None,
    };
    ValueFacts {
        value,
        ..ValueFacts::default()
    }
}

fn string_facts(text: String) -> ValueFacts {
    let text: Box<str> = text.into();
    ValueFacts {
        value: Some(StaticValue::String(text.clone())),
        strings: Some(Box::new([text.clone()])),
        prefix: Some(text.clone()),
        suffix: Some(text),
    }
}

/// Converts the facts of a value to the facts of its conversion to a
/// string, as done by a template literal.
fn to_string(facts: ValueFacts) -> ValueFacts {
    match &facts.value {
        Some(StaticValue::String(_)) => facts,
        // The source text is the string only for a plain decimal integer:
        // `1.0`, `0x10` and `1_000` convert to other strings.
        Some(StaticValue::Number(text)) if is_canonical_integer(text) => {
            string_facts(text.to_string())
        }
        Some(StaticValue::Number(_)) => ValueFacts::default(),
        Some(StaticValue::Bool(value)) => string_facts(value.to_string()),
        Some(StaticValue::Null) => string_facts("null".into()),
        Some(StaticValue::Undefined) => string_facts("undefined".into()),
        None => ValueFacts {
            value: None,
            ..facts
        },
    }
}

fn is_canonical_integer(text: &str) -> bool {
    text == "0"
        || (!text.starts_with('0')
            && text.len() <= 15
            && text.bytes().all(|byte| byte.is_ascii_digit()))
}

fn exact_string(facts: &ValueFacts) -> Option<&str> {
    match &facts.value {
        Some(StaticValue::String(text)) => Some(text),
        _ => None,
    }
}

/// The facts of the concatenation of two strings.
fn concat(left: ValueFacts, right: ValueFacts) -> ValueFacts {
    let value = match (exact_string(&left), exact_string(&right)) {
        (Some(left), Some(right)) => Some(StaticValue::String(format!("{left}{right}").into())),
        _ => None,
    };
    let strings = match (&left.strings, &right.strings) {
        (Some(left), Some(right)) if left.len() * right.len() <= MAX_STRINGS => Some(
            left.iter()
                .flat_map(|left| {
                    right
                        .iter()
                        .map(move |right| format!("{left}{right}").into())
                })
                .collect(),
        ),
        _ => None,
    };
    let prefix: Option<Box<str>> = match exact_string(&left) {
        Some(text) => Some(format!("{text}{}", right.prefix.as_deref().unwrap_or("")).into()),
        None => left.prefix.clone(),
    };
    let suffix: Option<Box<str>> = match exact_string(&right) {
        Some(text) => Some(format!("{}{text}", left.suffix.as_deref().unwrap_or("")).into()),
        None => right.suffix.clone(),
    };
    ValueFacts {
        value,
        strings,
        prefix: prefix.filter(|prefix| !prefix.is_empty()),
        suffix: suffix.filter(|suffix| !suffix.is_empty()),
    }
}

/// The facts of a value that is one of two values.
fn either(left: ValueFacts, right: ValueFacts) -> ValueFacts {
    let value = (left.value.is_some() && left.value == right.value)
        .then(|| left.value.clone())
        .flatten();
    let strings = match (left.strings, right.strings) {
        (Some(left), Some(right)) => {
            let mut all = left.into_vec();
            for string in right {
                if !all.contains(&string) {
                    all.push(string);
                }
            }
            (all.len() <= MAX_STRINGS).then(|| all.into_boxed_slice())
        }
        _ => None,
    };
    let prefix = match (&left.prefix, &right.prefix) {
        (Some(left), Some(right)) => Some(common_prefix(left, right)),
        _ => None,
    };
    let suffix = match (&left.suffix, &right.suffix) {
        (Some(left), Some(right)) => Some(common_suffix(left, right)),
        _ => None,
    };
    ValueFacts {
        value,
        strings,
        prefix: prefix.filter(|prefix| !prefix.is_empty()).map(Box::from),
        suffix: suffix.filter(|suffix| !suffix.is_empty()).map(Box::from),
    }
}

fn common_prefix<'a>(left: &'a str, right: &str) -> &'a str {
    let end = left
        .char_indices()
        .zip(right.chars())
        .find(|((_, left), right)| left != right)
        .map_or_else(|| left.len().min(right.len()), |((index, _), _)| index);
    &left[..end]
}

fn common_suffix<'a>(left: &'a str, right: &str) -> &'a str {
    let mut length = 0;
    for (left_char, right_char) in left.chars().rev().zip(right.chars().rev()) {
        if left_char != right_char {
            break;
        }
        length += left_char.len_utf8();
    }
    &left[left.len() - length..]
}

/// Resolves the escapes of a string written in JavaScript. Returns `None`
/// when an escape stands for another character, such as `\n` or `\x41`,
/// which the model does not decode.
fn unescape(text: &str) -> Option<String> {
    let Some(first_escape) = text.find('\\') else {
        return Some(text.to_string());
    };
    let mut value = String::with_capacity(text.len());
    value.push_str(&text[..first_escape]);
    let mut chars = text[first_escape..].chars();
    while let Some(char) = chars.next() {
        if char != '\\' {
            value.push(char);
            continue;
        }
        match chars.next() {
            Some(escaped)
                if !escaped.is_ascii_alphanumeric() && escaped != '\n' && escaped != '\r' =>
            {
                value.push(escaped);
            }
            _ => return None,
        }
    }
    Some(value)
}

/// Records the variables initialized with a store, as `count` in
/// `const count = writable(0)` with `writable` imported from `svelte/store`.
pub(crate) fn collect_store_creators(
    builder: &mut ModelBuilder,
    trees: &[Tree],
    declared: &[Declared],
) {
    let mut found = Vec::new();
    for entry in declared {
        let tree = trees[entry.tree];
        let Some(declarator) = entry.binding.parent().and_then(JsVariableDeclarator::cast) else {
            continue;
        };
        let Some(AnyJsExpression::JsCallExpression(call)) = declarator
            .initializer()
            .and_then(|init| init.expression().ok())
            .map(AnyJsExpression::omit_parentheses)
        else {
            continue;
        };
        let Some(callee) = call.callee().ok().map(AnyJsExpression::omit_parentheses) else {
            continue;
        };
        let (base, member) = match &callee {
            AnyJsExpression::JsIdentifierExpression(identifier) => (identifier.clone(), None),
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let Some(AnyJsExpression::JsIdentifierExpression(object)) =
                    member.object().ok().map(AnyJsExpression::omit_parentheses)
                else {
                    continue;
                };
                let name = member
                    .member()
                    .ok()
                    .and_then(|name| name.value_token().ok())
                    .map(|token| token.text_trimmed().to_string());
                (object, name)
            }
            _ => continue,
        };
        let Some(reference) = base.name().ok() else {
            continue;
        };
        let range = tree.host(reference.syntax().text_trimmed_range());
        let Some(symbol) = builder
            .data
            .references
            .iter()
            .find(|reference| reference.range == range && reference.via.is_none())
            .and_then(|reference| match reference.resolution {
                Resolution::Symbol(symbol) => Some(symbol),
                _ => None,
            })
        else {
            continue;
        };
        let Some(import) = builder.data.symbols[symbol.index()].import.as_ref() else {
            continue;
        };
        if import.source.text() != "svelte/store" {
            continue;
        }
        let name = match (&import.imported, &member) {
            (Imported::Named(name), None) => name.text().to_string(),
            (Imported::Namespace, Some(member)) => member.clone(),
            _ => continue,
        };
        if let Some(creator) = StoreCreator::from_name(&name) {
            found.push((entry.symbol, creator));
        }
    }
    for (symbol, creator) in found {
        builder.symbol_mut(symbol).store = Some(creator);
    }
}
