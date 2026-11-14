//! Maps the expressions that read or test a value to the subjects of an
//! execution root, and finds the destructuring patterns that a test of one of
//! their elements narrows.
//!
//! Reads and runtime tests go through [`subject_of`], so a test and a read of
//! the same subject get the same id even when their syntax differs, as in
//! `(o)?.v`, `o["v"]`, and `o.v`.

use super::{FlowSubject, FlowSubjectId, FlowSubjectTable, PropertyKey};
use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBindingPattern, AnyJsClass, AnyJsClassMember,
    AnyJsExpression, AnyJsGetter, AnyJsLiteralExpression, AnyJsName,
    AnyJsObjectBindingPatternMember, AnyJsObjectMemberName, AnyJsParameter, AnyJsSetter,
    JsArrayBindingPatternElement, JsArrowFunctionExpression, JsFormalParameter, JsFunctionBody,
    JsIdentifierBinding, JsInitializerClause, JsLanguage, JsMethodClassMember,
    JsMethodObjectMember, JsObjectBindingPatternProperty, JsObjectBindingPatternShorthandProperty,
    JsObjectExpression, JsParameters, JsPropertyClassMember, JsRestParameter, JsSyntaxKind,
    JsSyntaxNode, JsSyntaxToken, JsVariableDeclarator, binding_ext::AnyJsBindingDeclaration,
    inner_string_text, unescape_js_identifier,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxKindSet, Text, TextRange, TokenText,
    declare_node_union,
};
use std::borrow::Cow;

/// The value of JavaScript's `Number.MAX_SAFE_INTEGER`, 2^53 - 1: the largest
/// integer `n` such that a number represents both `n` and `n + 1` exactly.
const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

declare_node_union! {
    /// A property or an element of an object or array binding pattern that
    /// can bind a variable, as `kind`, `kind: k`, and `k` in
    /// `{ kind, kind: k }` and `[k]`, but not a rest element or a hole.
    pub(super) AnyPatternElement = JsObjectBindingPatternShorthandProperty
        | JsObjectBindingPatternProperty
        | JsArrayBindingPatternElement
}

declare_node_union! {
    /// A node that can hold a binding pattern directly: a variable declarator,
    /// a parameter, or a property or an element of an enclosing pattern.
    AnyPatternOwner = JsVariableDeclarator
        | JsFormalParameter
        | JsRestParameter
        | JsObjectBindingPatternProperty
        | JsArrayBindingPatternElement
}

declare_node_union! {
    /// A class or object member whose body, or initializer for a field, runs
    /// with the `this` of its class or object.
    AnyThisMember = JsMethodClassMember
        | JsPropertyClassMember
        | JsMethodObjectMember
        | AnyJsGetter
        | AnyJsSetter
}

declare_node_union! {
    /// A class or an object literal, whose members share its `this`.
    AnyMemberOwner = AnyJsClass | JsObjectExpression
}

/// The children of a member that run with the member's `this`: its body, its
/// parameters, the parameter of a setter, and the initializer of a field.
const MEMBER_BODY_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionBody::KIND_SET
    .union(JsParameters::KIND_SET)
    .union(JsFormalParameter::KIND_SET)
    .union(JsInitializerClause::KIND_SET);

/// The nodes that end the walk of [`this_can_narrow`] before it finds the
/// member whose `this` the root has: execution roots, which have a `this` of
/// their own, other class members, and classes, reached from their decorators
/// or `extends` clause. The walk checks arrow functions and the members of
/// [`AnyThisMember`] first.
const THIS_BOUNDARY_KINDS: SyntaxKindSet<JsLanguage> = AnyJsControlFlowRoot::KIND_SET
    .union(AnyJsClassMember::KIND_SET)
    .union(AnyJsClass::KIND_SET);

/// Returns the id of the subject that `expression` reads, adding the subject
/// and every prefix of its path to `subjects` on first use.
///
/// A subject is an identifier that refers to a declaration, `this` when the
/// table stores it, or a member access whose object is a subject. A static
/// member access must name a property or a private name, as in `o.v`, `a.b.c`,
/// or `this.#v`, and an element access must have a key that spells the
/// property's name, as in `o["v"]` or `t[0]`. [`literal_key`] lists the
/// accepted keys. Parentheses around the expression and its objects are
/// skipped, and an optional access reads the same subject as a plain one, as
/// in TypeScript: `(o)?.v` is `o.v`.
///
/// Returns `None` for any other expression, including `o[k]`, `o!.v`,
/// `f().v`, `super.v`, and a global, and when the table has used every
/// [`FlowSubjectId`].
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only identifiers, `this`, and member accesses can be subjects."
)]
pub(super) fn subject_of(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    subjects: &mut FlowSubjectTable,
) -> Option<FlowSubjectId> {
    let mut keys = Vec::new();
    let mut expression = expression.clone();
    let mut subject = loop {
        match expression {
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                expression = parenthesized.expression().ok()?;
            }
            AnyJsExpression::JsStaticMemberExpression(member) => {
                keys.push(property_key(&member.member().ok()?)?);
                expression = member.object().ok()?;
            }
            AnyJsExpression::JsComputedMemberExpression(member) => {
                keys.push(literal_key(&member.member().ok()?)?);
                expression = member.object().ok()?;
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                break subjects.reference(model, &identifier.name().ok()?)?;
            }
            AnyJsExpression::JsThisExpression(this) => break subjects.this(&this)?,
            _ => return None,
        }
    };
    for key in keys.into_iter().rev() {
        subject = subjects.intern(FlowSubject::Member {
            object: subject,
            key,
        })?;
    }
    Some(subject)
}

/// Returns the key of the property or private name that `name` names. A
/// private name such as `#v` gives a [`PropertyKey::Private`], which differs
/// from the key of `["#v"]`.
///
/// Returns `None` for a name whose escape is malformed, because its decoded
/// text is unknown.
pub(super) fn property_key(name: &AnyJsName) -> Option<PropertyKey> {
    let (token, key): (_, fn(Text) -> PropertyKey) = match name {
        AnyJsName::JsName(name) => (name.value_token(), PropertyKey::Name),
        AnyJsName::JsPrivateName(name) => (name.value_token(), PropertyKey::Private),
        AnyJsName::JsMetavariable(_) => return None,
    };
    identifier_text(&token.ok()?).map(key)
}

/// Returns the key of the property that the identifier `token` names, with
/// its escapes decoded, as the shorthand property `{ \u006bind }` names
/// `kind`.
///
/// Returns `None` for a name whose escape is malformed.
pub(super) fn identifier_key(token: &JsSyntaxToken) -> Option<PropertyKey> {
    identifier_text(token).map(PropertyKey::Name)
}

/// Returns the text of the identifier `token` with its escapes decoded, or
/// `None` when an escape is malformed.
fn identifier_text(token: &JsSyntaxToken) -> Option<Text> {
    let text = match unescape_js_identifier(token.text_trimmed()) {
        Cow::Borrowed(_) => Text::from(token.token_text_trimmed()),
        Cow::Owned(decoded) => Text::from(decoded),
    };
    (!text.contains('\\')).then_some(text)
}

/// Returns the key of the property that `name` names in an object binding
/// pattern, as both `{ kind: k }` and `{ "kind": k }` name `kind`.
///
/// An identifier has its escapes decoded. A string literal, a number literal,
/// or the expression of a computed name gives a key only when its spelling is
/// the property's name: a string literal without escapes, or a decimal integer
/// literal from `0` to 2^53 - 1 without leading zeros or separators.
/// [`literal_key`] applies the same rule to the key of an element access.
///
/// Returns `None` for any other name, such as `{ "\u006bind": k }`,
/// `{ 0x0: k }`, or ``{ [`kind`]: k }``, and for an identifier whose escape is
/// malformed.
pub(super) fn member_name_key(name: &AnyJsObjectMemberName) -> Option<PropertyKey> {
    match name {
        AnyJsObjectMemberName::JsLiteralMemberName(name) => {
            let token = name.value().ok()?;
            match token.kind() {
                JsSyntaxKind::IDENT => identifier_key(&token),
                JsSyntaxKind::JS_STRING_LITERAL => string_key(inner_string_text(&token)),
                JsSyntaxKind::JS_NUMBER_LITERAL => decimal_integer_key(&token),
                _ => None,
            }
        }
        AnyJsObjectMemberName::JsComputedMemberName(name) => literal_key(&name.expression().ok()?),
        AnyJsObjectMemberName::JsMetavariable(_) => None,
    }
}

/// Returns the key of the property that the element access key `key` names
/// when its spelling is the property's name: a string literal without escapes,
/// such as `"v"`, or a decimal integer literal from `0` to 2^53 - 1 without
/// leading zeros or separators, such as `0`.
///
/// JavaScript converts a number key to a string, and such an integer converts
/// to its own digits, so `t[0]` reads the same property as `t["0"]`. A string
/// key reads the property it spells, so `o["v"]` reads the same property as
/// `o.v`.
///
/// Returns `None` for every other key, including a number literal that
/// converts to other text, such as `0x0`, `1e0`, `1.0`, or `1_0`, and an
/// expression that is not a literal, such as `k`, `-0`, or a template literal.
/// An integer literal above 2^53 - 1 can also convert to other text, as
/// `9007199254740993` converts to `"9007199254740992"`.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only string and number literals can name a property by their spelling."
)]
pub(super) fn literal_key(key: &AnyJsExpression) -> Option<PropertyKey> {
    match key {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string),
        ) => string_key(string.inner_string_text().ok()?),
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number),
        ) => decimal_integer_key(&number.value_token().ok()?),
        _ => None,
    }
}

/// Returns the key of the property that `text` names, where `text` is the
/// content of a string literal without its quotes. Returns `None` when `text`
/// has an escape, because escapes are not decoded.
fn string_key(text: TokenText) -> Option<PropertyKey> {
    (!text.contains('\\')).then(|| PropertyKey::Name(text.into()))
}

/// Returns the key that the number literal `token` names when it is a
/// decimal integer from `0` to 2^53 - 1 without leading zeros or separators.
/// Such a literal converts to a string equal to its spelling, so the spelling
/// is the property's name. Returns `None` for any other number literal.
fn decimal_integer_key(token: &JsSyntaxToken) -> Option<PropertyKey> {
    let text = token.text_trimmed();
    let is_decimal_integer =
        text.bytes().all(|byte| byte.is_ascii_digit()) && (text == "0" || !text.starts_with('0'));
    (is_decimal_integer
        && text
            .parse::<u64>()
            .is_ok_and(|value| value <= MAX_SAFE_INTEGER))
    .then(|| PropertyKey::Name(Text::from(token.token_text_trimmed())))
}

/// Returns the destructuring pattern that a runtime test of `binding` narrows,
/// with the elements of that pattern whose reads the narrowing reaches, or
/// `None` when no such pattern holds `binding`.
///
/// This follows how TypeScript narrows destructured variables. The pattern's
/// declaration, found through the enclosing patterns, is a `const`
/// declarator, as in `const { kind, v } = s` or `for (const [k, x] of xs)`,
/// or a parameter, as in `({ kind, v }: S) => {}` or `(...[k, x]: T) => {}`,
/// and none of the declaration's variables is written. No enclosing pattern
/// supplies a default, as `{ a: { kind, v } = d }` does, because the declared
/// types of the elements ignore it.
///
/// The returned elements are the direct elements of the pattern that are
/// neither rest elements nor have a default, such as `kind` and `v` in
/// `{ kind, v }` or `k` and `x` in `[k, , x]`. Each is the range of its
/// variable with the key of the property it reads. An object element reads
/// the property that its key names, and an array element reads its decimal
/// index, counting holes. Returns `None` unless `binding` is one of them and
/// there are at least two, so that a test of one can narrow another.
/// TypeScript counts every element, but with a single such element the
/// pattern narrows nothing that the test of that element doesn't.
pub(super) fn narrowing_pattern(
    binding: &Binding,
    model: &SemanticModel,
) -> Option<(AnyJsBindingPattern, Vec<(TextRange, PropertyKey)>)> {
    let element = AnyPatternElement::from_declaration(binding.tree().declaration()?)?;
    // An element's parent is its list, inside the pattern.
    let list = element.syntax().parent()?;
    let pattern = AnyJsBindingPattern::cast(list.parent()?)?;
    let elements: Vec<_> = list
        .children()
        .enumerate()
        .filter_map(|(index, element)| pattern_element(&AnyPatternElement::cast(element)?, index))
        .collect();
    (elements.len() >= 2
        && elements.iter().any(|(range, _)| *range == binding.range())
        && has_constant_declaration(&pattern, model))
    .then_some((pattern, elements))
}

impl AnyPatternElement {
    /// Returns the pattern element that `declaration` is, or `None` when it
    /// is another declaration.
    fn from_declaration(declaration: AnyJsBindingDeclaration) -> Option<Self> {
        match declaration {
            AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(property) => {
                Some(property.into())
            }
            AnyJsBindingDeclaration::JsObjectBindingPatternProperty(property) => {
                Some(property.into())
            }
            AnyJsBindingDeclaration::JsArrayBindingPatternElement(element) => Some(element.into()),
            _ => None,
        }
    }
}

/// Returns the variable that `element`, at `index` in its pattern, binds with
/// the key of the property it reads, as [`narrowing_pattern`] describes, or
/// `None` for an element with a default, a nested pattern, and a key that
/// names no property.
fn pattern_element(element: &AnyPatternElement, index: usize) -> Option<(TextRange, PropertyKey)> {
    match element {
        AnyPatternElement::JsObjectBindingPatternShorthandProperty(property) => {
            if property.init().is_some() {
                return None;
            }
            let binding = property.identifier().ok()?;
            let binding = binding.as_js_identifier_binding()?;
            Some((
                binding.range(),
                identifier_key(&binding.name_token().ok()?)?,
            ))
        }
        AnyPatternElement::JsObjectBindingPatternProperty(property) => {
            if property.init().is_some() {
                return None;
            }
            let AnyJsBindingPattern::AnyJsBinding(binding) = property.pattern().ok()? else {
                return None;
            };
            let binding = binding.as_js_identifier_binding()?;
            Some((binding.range(), member_name_key(&property.member().ok()?)?))
        }
        AnyPatternElement::JsArrayBindingPatternElement(element) => {
            if element.init().is_some() {
                return None;
            }
            let AnyJsBindingPattern::AnyJsBinding(binding) = element.pattern().ok()? else {
                return None;
            };
            let binding = binding.as_js_identifier_binding()?;
            Some((binding.range(), index_key(index)?))
        }
    }
}

/// Spells the array indices that [`index_key`] names without building a string.
const INDEX_NAMES: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
];

/// Returns the key that names the element at `index` of an array, as `t[0]`
/// and `t["0"]` do. Returns `None` from index 16 on, so narrowing ignores
/// array pattern elements at such indices.
pub(super) fn index_key(index: usize) -> Option<PropertyKey> {
    INDEX_NAMES
        .get(index)
        .map(|name| PropertyKey::Name(Text::from(*name)))
}

/// Returns the key that names `element` in the array that its pattern
/// destructures, from its index in the pattern, counting holes, as
/// [`index_key`] does: `k` reads `t[1]` in `[, k]`.
pub(super) fn array_element_key(element: &JsArrayBindingPatternElement) -> Option<PropertyKey> {
    // Elements from index 16 on have no key, so the walk stops before them.
    let index = element
        .syntax()
        .parent()?
        .children()
        .take(INDEX_NAMES.len())
        .position(|sibling| sibling == *element.syntax())?;
    index_key(index)
}

/// Returns whether the declaration of the destructuring `pattern` lets a test
/// of one element narrow the others, as [`narrowing_pattern`] requires.
///
/// The declaration must be a `const` declarator or a parameter whose
/// variables are never written, so the values that the pattern binds can't
/// change after the destructuring. No enclosing pattern may supply a default,
/// because the declared types of the elements ignore it.
fn has_constant_declaration(pattern: &AnyJsBindingPattern, model: &SemanticModel) -> bool {
    let mut pattern = pattern.syntax().clone();
    let root = loop {
        let Some(owner) = pattern.parent().and_then(AnyPatternOwner::cast) else {
            return false;
        };
        let has_default = match &owner {
            AnyPatternOwner::JsVariableDeclarator(declarator) => {
                let is_const = declarator
                    .declaration()
                    .is_some_and(|declaration| declaration.is_const());
                if !is_const {
                    return false;
                }
                break pattern;
            }
            AnyPatternOwner::JsFormalParameter(_) | AnyPatternOwner::JsRestParameter(_) => {
                break pattern;
            }
            // A nested pattern is the value of a property or an element of the
            // enclosing pattern, whose parent is its list.
            AnyPatternOwner::JsObjectBindingPatternProperty(property) => property.init().is_some(),
            AnyPatternOwner::JsArrayBindingPatternElement(element) => element.init().is_some(),
        };
        if has_default {
            return false;
        }
        let Some(enclosing) = owner.syntax().grand_parent() else {
            return false;
        };
        pattern = enclosing;
    };
    // As in TypeScript, a write to any variable of a parameter, including one
    // in a nested pattern, stops the narrowing. A `const` variable is never
    // written.
    let Some(root) = AnyJsBindingPattern::cast(root) else {
        return false;
    };
    let mut written = false;
    visit_pattern_bindings(&root, &mut |binding| {
        written |= model.as_binding(binding).all_writes().next().is_some();
    });
    !written
}

/// Calls `visit` with each variable that `pattern` binds, including those of
/// nested patterns but not those declared inside its defaults.
fn visit_pattern_bindings(
    pattern: &AnyJsBindingPattern,
    visit: &mut impl FnMut(&JsIdentifierBinding),
) {
    match pattern {
        AnyJsBindingPattern::AnyJsBinding(binding) => {
            if let Some(binding) = binding.as_js_identifier_binding() {
                visit(binding);
            }
        }
        AnyJsBindingPattern::JsArrayBindingPattern(pattern) => {
            for element in pattern.elements().iter().flatten() {
                let element = match element {
                    AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(element) => {
                        element.pattern()
                    }
                    AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(element) => {
                        element.pattern()
                    }
                    AnyJsArrayBindingPatternElement::JsArrayHole(_) => continue,
                };
                if let Ok(element) = element {
                    visit_pattern_bindings(&element, visit);
                }
            }
        }
        AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
            for property in pattern.properties().iter().flatten() {
                match property {
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                        if let Ok(element) = property.pattern() {
                            visit_pattern_bindings(&element, visit);
                        }
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                        property,
                    ) => {
                        if let Some(binding) = property
                            .identifier()
                            .ok()
                            .and_then(|binding| binding.as_js_identifier_binding().cloned())
                        {
                            visit(&binding);
                        }
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                        if let Some(binding) = rest
                            .binding()
                            .ok()
                            .and_then(|binding| binding.as_js_identifier_binding().cloned())
                        {
                            visit(&binding);
                        }
                    }
                    AnyJsObjectBindingPatternMember::JsBogusBinding(_)
                    | AnyJsObjectBindingPatternMember::JsMetavariable(_) => {}
                }
            }
        }
    }
}

/// Returns whether paths on the `this` of `root`, such as `this.v`, can narrow
/// there. They can when Biome types that `this` as TypeScript does and
/// TypeScript narrows its properties like those of a variable.
///
/// That is the case in a class method or accessor that is not `static`, in an
/// object literal method or accessor, and in an arrow function whose `this`
/// comes from one of them or from the initializer of a class field that is not
/// `static`. The member's class or object must also have a name that Biome can
/// find: the name of a class declaration or class expression, or a variable
/// that the class expression or object literal initializes, as in
/// `const o = { ... }`. [`types_this_of_members`] has the exact lookup. In a
/// class, `this` is an instance of the class, and in an object literal, it is
/// the object.
///
/// It is not the case:
///
/// - In a constructor, where TypeScript reads a field without an initializer
///   at its declared type after a test that leaves `undefined` possible, as in
///   `if (this.x !== null) this.x` (TS2565). An arrow function called where it
///   is defined runs in its constructor's flow, so no arrow function whose
///   `this` comes from a constructor narrows it either.
/// - In a `static` member or static block, where `this` is the class itself,
///   which Biome types as an instance of the class.
/// - In a member with a `this` parameter, whose type Biome ignores.
/// - In a member of a class or object literal without such a name, such as an
///   object literal passed to a call. Biome types that `this` from an
///   enclosing class or object literal, or not at all.
/// - In a function declaration or expression, and in a module or namespace.
///   Biome types the `this` of a function only when the function is the value
///   of a class field or an object literal property, as in
///   `{ f: function () {} }`.
/// - In an arrow function in the decorators, computed key, or `extends` clause
///   of a class or a member. That arrow function sees the `this` of the code
///   around the class or object, but Biome types it as an instance of the
///   class or as the object.
pub(super) fn this_can_narrow(root: &AnyJsControlFlowRoot) -> bool {
    // The child of `node` that the walk comes from, or `None` at the root.
    let mut child: Option<JsSyntaxNode> = None;
    for node in root.syntax().ancestors() {
        // Parameters, the body, and a field initializer run with the member's
        // `this`, unlike its decorators and computed key.
        let in_body = child
            .as_ref()
            .is_none_or(|child| MEMBER_BODY_KINDS.matches(child.kind()));
        if let Some(member) = AnyThisMember::cast_ref(&node) {
            return in_body
                && !is_static(&member)
                && !has_this_parameter(&member)
                // A member's parent is its list, inside the class or object.
                && node
                    .grand_parent()
                    .and_then(AnyMemberOwner::cast)
                    .is_some_and(|owner| types_this_of_members(&owner));
        }
        // An arrow function has the `this` of the code around it.
        if !JsArrowFunctionExpression::can_cast(node.kind())
            && THIS_BOUNDARY_KINDS.matches(node.kind())
        {
            return false;
        }
        child = Some(node);
    }
    false
}

/// Returns whether Biome types the `this` of the members of `owner`, a class
/// or an object literal, as an instance of that class or as that object.
///
/// Biome finds that type through a name: the name of a class declaration, or
/// the variable that a class expression or object literal initializes, as in
/// `const o = { ... }`, or else the name of a class expression. It looks only
/// at the nearest variable declaration around `owner`. Without such a name, it
/// types `this` from an enclosing class or object literal, or not at all: in
/// `class K { m() { use({ n() { this } }) } }`, it types `this` as `K`.
fn types_this_of_members(owner: &AnyMemberOwner) -> bool {
    let declarator = owner
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(JsVariableDeclarator::cast);
    let initializer = declarator
        .as_ref()
        .and_then(|declarator| declarator.initializer()?.expression().ok());
    let initializes_declarator = || {
        initializer
            .as_ref()
            .is_some_and(|initializer| initializer.syntax() == owner.syntax())
            && declarator
                .as_ref()
                .and_then(|declarator| declarator.id().ok())
                .is_some_and(|id| {
                    id.as_any_js_binding()
                        .is_some_and(|binding| binding.as_js_identifier_binding().is_some())
                })
    };
    match owner {
        AnyMemberOwner::AnyJsClass(AnyJsClass::JsClassExpression(class)) => {
            if matches!(initializer, Some(AnyJsExpression::JsClassExpression(_))) {
                initializes_declarator()
            } else {
                class.id().is_some()
            }
        }
        AnyMemberOwner::AnyJsClass(AnyJsClass::JsClassExportDefaultDeclaration(class)) => {
            class.id().is_some()
        }
        AnyMemberOwner::AnyJsClass(AnyJsClass::JsClassDeclaration(_)) => true,
        AnyMemberOwner::JsObjectExpression(_) => initializes_declarator(),
    }
}

/// Returns whether the class or object `member` is declared `static`.
fn is_static(member: &AnyThisMember) -> bool {
    match member {
        AnyThisMember::JsMethodClassMember(method) => method
            .modifiers()
            .iter()
            .any(|modifier| modifier.as_js_static_modifier().is_some()),
        AnyThisMember::JsPropertyClassMember(property) => property
            .modifiers()
            .iter()
            .any(|modifier| modifier.as_js_static_modifier().is_some()),
        AnyThisMember::JsMethodObjectMember(_) => false,
        AnyThisMember::AnyJsGetter(getter) => getter.is_static(),
        AnyThisMember::AnyJsSetter(setter) => setter.is_static(),
    }
}

/// Returns whether the class or object `member` is a method that declares a
/// `this` parameter, as in `m(this: Other) {}`.
fn has_this_parameter(member: &AnyThisMember) -> bool {
    let parameters = match member {
        AnyThisMember::JsMethodClassMember(method) => method.parameters(),
        AnyThisMember::JsMethodObjectMember(method) => method.parameters(),
        AnyThisMember::JsPropertyClassMember(_)
        | AnyThisMember::AnyJsGetter(_)
        | AnyThisMember::AnyJsSetter(_) => return false,
    };
    parameters
        .ok()
        .and_then(|parameters| parameters.items().iter().next())
        .is_some_and(|first| matches!(first, Ok(AnyJsParameter::TsThisParameter(_))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::{JsExpressionStatement, JsStaticMemberExpression, JsThisExpression};
    use biome_languages::JsFileSource;
    use biome_rowan::AstNode;

    /// Returns the subject of each expression statement in the body of the
    /// method `m(o, a)` of a class with a private field `#p`, written as a
    /// dotted path, or `None` for an expression that is not a subject.
    fn subjects(body: &str) -> Vec<Option<String>> {
        subjects_in(&format!("class K {{ #p; m(o, a) {{ {body} }} }}"), |root| {
            root.syntax()
                .descendants()
                .filter_map(JsExpressionStatement::cast)
                .map(|statement| statement.expression().unwrap())
                .collect()
        })
    }

    /// Returns the subject of each expression that `expressions` selects in
    /// the first execution root of `source` below the module, written as a
    /// dotted path, or `None` for an expression that is not a subject. The
    /// root's table stores `this` when [`this_can_narrow`] holds.
    fn subjects_in(
        source: &str,
        expressions: impl Fn(&AnyJsControlFlowRoot) -> Vec<AnyJsExpression>,
    ) -> Vec<Option<String>> {
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let root = parsed
            .syntax()
            .descendants()
            .filter_map(AnyJsControlFlowRoot::cast)
            .nth(1)
            .unwrap();
        let mut table = FlowSubjectTable::new(&root, this_can_narrow(&root));
        let ids: Vec<_> = expressions(&root)
            .iter()
            .map(|expression| subject_of(expression, &model, &mut table))
            .collect();
        ids.into_iter()
            .map(|id| Some(path(source, table.as_slice(), id?)))
            .collect()
    }

    fn path(source: &str, subjects: &[FlowSubject], id: FlowSubjectId) -> String {
        match &subjects[id as usize] {
            FlowSubject::Binding(range) | FlowSubject::Pattern(range) => source[*range].to_owned(),
            FlowSubject::This => "this".to_owned(),
            // A name that starts with `#` is printed as a string key, so it
            // differs from a private name.
            FlowSubject::Member {
                object,
                key: PropertyKey::Name(name),
            } if name.starts_with('#') => {
                format!(r#"{}["{name}"]"#, path(source, subjects, *object))
            }
            FlowSubject::Member {
                object,
                key: PropertyKey::Name(name),
            } => format!("{}.{name}", path(source, subjects, *object)),
            FlowSubject::Member {
                object,
                key: PropertyKey::Private(name),
            } => format!("{}.#{name}", path(source, subjects, *object)),
        }
    }

    #[test]
    fn index_keys_name_array_elements_up_to_index_15() {
        assert!(matches!(index_key(0), Some(PropertyKey::Name(name)) if name.text() == "0"));
        assert!(matches!(index_key(15), Some(PropertyKey::Name(name)) if name.text() == "15"));
        assert!(index_key(16).is_none());
    }

    #[test]
    fn variables_and_named_properties_of_subjects_are_subjects() {
        assert_eq!(
            subjects("o; o.v; a.b.c; o?.v; (o).v; ((o.v)); o.\\u0076; a?.b?.c;"),
            [
                Some("o".to_owned()),
                Some("o.v".to_owned()),
                Some("a.b.c".to_owned()),
                Some("o.v".to_owned()),
                Some("o.v".to_owned()),
                Some("o.v".to_owned()),
                Some("o.v".to_owned()),
                Some("a.b.c".to_owned()),
            ]
        );
    }

    #[test]
    fn other_expressions_are_not_subjects() {
        assert_eq!(
            subjects(
                "o[k]; o!.v; (o as any).v; f().v; o.f().v; missing.v; o[k].v; super.v; this[k];"
            ),
            vec![None; 9]
        );
    }

    #[test]
    fn literal_keys_name_the_property_they_spell() {
        assert_eq!(
            subjects(
                r#"o["v"]; o['v']; o?.["v"]; (o)["v"].w; o.v["w"]; o[0]; o["0"]; o["01"]; o[""]; o[9007199254740991];"#
            ),
            [
                Some("o.v".to_owned()),
                Some("o.v".to_owned()),
                Some("o.v".to_owned()),
                Some("o.v.w".to_owned()),
                Some("o.v.w".to_owned()),
                Some("o.0".to_owned()),
                Some("o.0".to_owned()),
                Some("o.01".to_owned()),
                Some("o.".to_owned()),
                Some("o.9007199254740991".to_owned()),
            ]
        );
    }

    #[test]
    fn keys_spelled_differently_from_their_property_are_not_subjects() {
        assert_eq!(
            subjects(
                r#"o[-0]; o[0x0]; o[0b0]; o[0o0]; o[1e0]; o[1.0]; o[.5]; o[1_0]; o[1n]; o[9007199254740992]; o[12345678901234567890]; o["\u0076"]; o['\x76']; o[`v`]; o[("v")]; o[true]; o[null];"#
            ),
            vec![None; 17]
        );
    }

    #[test]
    fn paths_on_this_and_private_names_are_subjects() {
        assert_eq!(
            subjects(
                r##"this.v; (this).v; this?.v; this["v"]; this.#p; this.#\u0070; this["#p"]; o.#p; this.#p.v;"##
            ),
            [
                Some("this.v".to_owned()),
                Some("this.v".to_owned()),
                Some("this.v".to_owned()),
                Some("this.v".to_owned()),
                Some("this.#p".to_owned()),
                Some("this.#p".to_owned()),
                // A string key never names a private member.
                Some(r##"this["#p"]"##.to_owned()),
                Some("o.#p".to_owned()),
                Some("this.#p.v".to_owned()),
            ]
        );
    }

    #[test]
    fn this_is_a_subject_only_where_it_can_narrow() {
        let this_paths = |source: &str| {
            subjects_in(source, |root| {
                root.syntax()
                    .descendants()
                    .filter_map(JsStaticMemberExpression::cast)
                    .map(AnyJsExpression::from)
                    .collect()
            })
        };
        assert_eq!(
            this_paths("class K { m() { this.v; } }"),
            [Some("this.v".to_owned())]
        );
        for source in [
            "class K { constructor() { this.v; } }",
            "class K { static m() { this.v; } }",
            "function f() { this.v; }",
        ] {
            assert_eq!(this_paths(source), [None], "{source}");
        }
        // The decorators and `extends` clause of a class and the computed key
        // of an object literal method see the root's `this`, which Biome types
        // there as an instance of the class or as the object.
        assert_eq!(
            this_paths(
                "class K { m() { this.v; class D extends (this.w) {} (@dec(this.x) class {}); const o = { [this.y]() {}, [this.z]: 1 }; } }"
            ),
            [
                Some("this.v".to_owned()),
                None,
                None,
                None,
                Some("this.z".to_owned())
            ]
        );
    }

    #[test]
    fn this_can_narrow_in_instance_members_and_object_methods() {
        // Each source has one `this`, read in the root that is checked.
        for (source, expected) in [
            ("class K { m() { this; } }", true),
            ("class K { get g() { return this; } }", true),
            ("class K { set s(v) { this; } }", true),
            ("class K { m() { const f = () => this; } }", true),
            ("class K { m() { const f = () => () => this; } }", true),
            ("class K { m(f = () => this) {} }", true),
            ("class K { m() { const o = { f: () => this }; } }", true),
            ("class K { f = () => this; }", true),
            ("const o = { m() { this; } };", true),
            ("const o = { get g() { return this; } };", true),
            ("const o = { m() { const f = () => this; } };", true),
            ("const C = class { m() { this; } };", true),
            ("use(class Named { m() { this; } });", true),
            ("const C = use(class Named { m() { this; } });", true),
            ("export default class K { m() { this; } }", true),
            ("use({ m() { this; } });", false),
            ("const o = use({ m() { this; } });", false),
            ("const o = { inner: { m() { this; } } };", false),
            ("const { a } = { m() { this; } };", false),
            ("let o; o = { m() { this; } };", false),
            ("use(class { m() { this; } });", false),
            ("const C = use(class { m() { this; } });", false),
            (
                "const C = class { n() { use(class { m() { this; } }); } };",
                false,
            ),
            ("export default class { m() { this; } }", false),
            ("class K { constructor() { this; } }", false),
            ("class K { constructor() { const f = () => this; } }", false),
            ("class K { constructor() { (() => this)(); } }", false),
            ("class K { static m() { this; } }", false),
            ("class K { static get g() { return this; } }", false),
            ("class K { static { this; } }", false),
            ("class K { static f = () => this; }", false),
            ("class K { m(this: K) { this; } }", false),
            ("class K { m(this: K) { const f = () => this; } }", false),
            ("const o = { m(this: object) { this; } };", false),
            ("class K { m() { function g() { this; } } }", false),
            ("function f() { this; }", false),
            ("const f = function () { this; };", false),
            ("const o = { f: function () { this; } };", false),
            ("const f = () => this;", false),
            ("const o = { f: () => this };", false),
            ("this;", false),
            (
                "class K { m() { class D extends (() => this, Object) {} } }",
                false,
            ),
            ("class K { [(() => this)()]() {} }", false),
            ("class K { @dec(() => this) m() {} }", false),
            ("const o = { [(() => this)()]() {} };", false),
        ] {
            let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
            assert!(!parsed.has_errors(), "{source}");
            let this = parsed
                .syntax()
                .descendants()
                .find_map(JsThisExpression::cast)
                .unwrap();
            let root = this
                .syntax()
                .ancestors()
                .find_map(AnyJsControlFlowRoot::cast)
                .unwrap();
            assert_eq!(this_can_narrow(&root), expected, "{source}");
        }
    }

    #[test]
    fn a_path_and_its_prefixes_share_ids() {
        let source = "function f(o) { o.v.w; o.v; o; }";
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let mut table = FlowSubjectTable::default();
        let ids: Vec<_> = parsed
            .syntax()
            .descendants()
            .filter_map(JsExpressionStatement::cast)
            .map(|statement| subject_of(&statement.expression().unwrap(), &model, &mut table))
            .collect();
        // `o.v.w` adds `o` and `o.v` first, so later reads find them.
        assert_eq!(ids, [Some(2), Some(1), Some(0)]);
        assert_eq!(table.as_slice().len(), 3);
    }
}
