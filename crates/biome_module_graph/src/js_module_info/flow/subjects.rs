//! Maps the expressions that read or test a value to the subjects of an
//! execution root.
//!
//! Reads and runtime tests go through [`subject_of`], so a test and a read of
//! the same subject get the same id even when their syntax differs, as in
//! `(o)?.v`, `o["v"]`, and `o.v`.

use super::{FlowSubject, FlowSubjectId, FlowSubjectTable, PropertyKey};
use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsExpression, AnyJsGetter, AnyJsLiteralExpression, AnyJsName,
    AnyJsParameter, AnyJsSetter, JsClassDeclaration, JsClassExportDefaultDeclaration,
    JsClassExpression, JsMethodClassMember, JsMethodObjectMember, JsObjectExpression, JsParameters,
    JsPropertyClassMember, JsSyntaxKind, JsSyntaxNode, JsVariableDeclarator,
    unescape_js_identifier,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxResult, Text};
use std::borrow::Cow;

/// The largest integer that a JavaScript number holds exactly, 2^53 - 1.
const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

/// Returns the id of the subject that `expression` reads, adding the subject
/// and every prefix of its path to `subjects` on first use.
///
/// A subject is an identifier that refers to a declaration, `this` when the
/// table stores it, or a member access whose object is a subject: a static
/// member access with a named property or a private name, such as `o.v`,
/// `a.b.c`, or `this.#v`, or an element access whose key [`literal_key`]
/// accepts, such as `o["v"]` or `t[0]`. Parentheses around the expression and
/// its objects are skipped, and an optional access reads the same subject as a
/// plain one, as in TypeScript: `(o)?.v` is `o.v`.
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

/// Returns the key of the property or private name that `name` names, so
/// `#v` gives a [`PropertyKey::Private`] that differs from the key of `["#v"]`.
///
/// Returns `None` for a name whose escape is malformed, because its decoded
/// text is unknown.
pub(super) fn property_key(name: &AnyJsName) -> Option<PropertyKey> {
    let (token, key): (_, fn(Text) -> PropertyKey) = match name {
        AnyJsName::JsName(name) => (name.value_token(), PropertyKey::Name),
        AnyJsName::JsPrivateName(name) => (name.value_token(), PropertyKey::Private),
        AnyJsName::JsMetavariable(_) => return None,
    };
    let token = token.ok()?;
    let text = match unescape_js_identifier(token.text_trimmed()) {
        Cow::Borrowed(_) => Text::from(token.token_text_trimmed()),
        Cow::Owned(decoded) => Text::from(decoded),
    };
    (!text.contains('\\')).then(|| key(text))
}

/// Returns the key of the property that the element access key `key` names
/// when its spelling is the property's name: a string literal without escapes,
/// such as `"v"`, or a decimal integer literal from `0` to 2^53 - 1 without
/// leading zeros or separators, such as `0`.
///
/// JavaScript converts a number key to a string, and such an integer converts
/// to its own digits, so `t[0]` reads the same property as `t["0"]`, and
/// `o["v"]` the same as `o.v`.
///
/// Returns `None` for every other key. That includes a number literal that
/// converts to other text, such as `0x0`, `1e0`, `1.0`, `1_0`, and
/// `9007199254740993`, which converts to `"9007199254740992"`, and an
/// expression that is not a literal, such as `k`, `-0`, or a template literal.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only string and number literals can name a property by their spelling."
)]
pub(super) fn literal_key(key: &AnyJsExpression) -> Option<PropertyKey> {
    match key {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string),
        ) => {
            let text = string.inner_string_text().ok()?;
            (!text.contains('\\')).then(|| PropertyKey::Name(text.into()))
        }
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number),
        ) => {
            let token = number.value_token().ok()?;
            let text = token.text_trimmed();
            let is_decimal_integer = text.bytes().all(|byte| byte.is_ascii_digit())
                && (text == "0" || !text.starts_with('0'));
            (is_decimal_integer
                && text
                    .parse::<u64>()
                    .is_ok_and(|value| value <= MAX_SAFE_INTEGER))
            .then(|| PropertyKey::Name(Text::from(token.token_text_trimmed())))
        }
        _ => None,
    }
}

/// Returns whether paths on the `this` of `root`, such as `this.v`, can narrow
/// there: whether Biome types that `this` as TypeScript does and TypeScript
/// narrows its properties as those of a variable.
///
/// That is the case in a class method or accessor that is not `static`, in an
/// object literal method or accessor, and in an arrow function whose `this`
/// comes from one of them or from the initializer of a class field that is not
/// `static`, provided that Biome finds the member's class or object, as
/// [`types_this_of_members`] describes. In a class, `this` is an instance of
/// the class, and in an object literal, it is the object.
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
/// - In a member of a class or object literal that Biome does not find, such
///   as an object literal passed to a call, where Biome types `this` from an
///   enclosing class or object literal, or not at all.
/// - In a function declaration or expression, and in a module or namespace.
///   Biome types the `this` of a function only when the function is the value
///   of a class field or an object literal property, as in
///   `{ f: function () {} }`.
/// - In an arrow function in the decorators, computed key, or `extends` clause
///   of a class or a member, which see the `this` of the code around the class
///   or object while Biome types it as an instance of the class or as the
///   object.
pub(super) fn this_can_narrow(root: &AnyJsControlFlowRoot) -> bool {
    // The child of `node` that the walk comes from, or `None` at the root.
    let mut child: Option<JsSyntaxNode> = None;
    for node in root.syntax().ancestors() {
        // Parameters and the body run with the member's `this`, unlike its
        // decorators and computed key.
        let in_body = child.as_ref().is_none_or(|child| {
            matches!(
                child.kind(),
                JsSyntaxKind::JS_FUNCTION_BODY
                    | JsSyntaxKind::JS_PARAMETERS
                    | JsSyntaxKind::JS_FORMAL_PARAMETER
                    | JsSyntaxKind::JS_INITIALIZER_CLAUSE
            )
        });
        match node.kind() {
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {}
            JsSyntaxKind::JS_METHOD_CLASS_MEMBER
            | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER
            | JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
            | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER => {
                return in_body
                    && !is_static(&node)
                    && !has_this_parameter(&node)
                    // A member's parent is its list, inside the class or object.
                    && node
                        .grand_parent()
                        .is_some_and(|owner| types_this_of_members(&owner));
            }
            kind if AnyJsControlFlowRoot::can_cast(kind)
                || AnyJsClassMember::can_cast(kind)
                || matches!(
                    kind,
                    JsSyntaxKind::JS_CLASS_DECLARATION
                        | JsSyntaxKind::JS_CLASS_EXPRESSION
                        | JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION
                ) =>
            {
                return false;
            }
            _ => {}
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
fn types_this_of_members(owner: &JsSyntaxNode) -> bool {
    let declarator = owner
        .ancestors()
        .skip(1)
        .find_map(JsVariableDeclarator::cast);
    let initializer = declarator
        .as_ref()
        .and_then(|declarator| declarator.initializer()?.expression().ok());
    let initializes_declarator = || {
        initializer
            .as_ref()
            .is_some_and(|initializer| initializer.syntax() == owner)
            && declarator
                .as_ref()
                .and_then(|declarator| declarator.id().ok())
                .is_some_and(|id| {
                    id.as_any_js_binding()
                        .is_some_and(|binding| binding.as_js_identifier_binding().is_some())
                })
    };
    if let Some(class) = JsClassExpression::cast_ref(owner) {
        if matches!(initializer, Some(AnyJsExpression::JsClassExpression(_))) {
            initializes_declarator()
        } else {
            class.id().is_some()
        }
    } else if let Some(class) = JsClassExportDefaultDeclaration::cast_ref(owner) {
        class.id().is_some()
    } else {
        JsClassDeclaration::can_cast(owner.kind())
            || (JsObjectExpression::can_cast(owner.kind()) && initializes_declarator())
    }
}

/// Returns whether the class or object `member` is declared `static`.
fn is_static(member: &JsSyntaxNode) -> bool {
    if let Some(method) = JsMethodClassMember::cast_ref(member) {
        method
            .modifiers()
            .iter()
            .any(|modifier| modifier.as_js_static_modifier().is_some())
    } else if let Some(property) = JsPropertyClassMember::cast_ref(member) {
        property
            .modifiers()
            .iter()
            .any(|modifier| modifier.as_js_static_modifier().is_some())
    } else if let Some(getter) = AnyJsGetter::cast_ref(member) {
        getter.is_static()
    } else {
        AnyJsSetter::cast_ref(member).is_some_and(|setter| setter.is_static())
    }
}

/// Returns whether the class or object method `member` declares a `this`
/// parameter, as in `m(this: Other) {}`.
fn has_this_parameter(member: &JsSyntaxNode) -> bool {
    let parameters: Option<SyntaxResult<JsParameters>> =
        if let Some(method) = JsMethodClassMember::cast_ref(member) {
            Some(method.parameters())
        } else {
            JsMethodObjectMember::cast_ref(member).map(|method| method.parameters())
        };
    parameters
        .and_then(Result::ok)
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
            FlowSubject::Binding(range) => source[*range].to_owned(),
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
