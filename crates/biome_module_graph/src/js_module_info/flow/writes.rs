//! Finds the property paths that an execution root may write.
//!
//! The flow graph has no write points, so a property path narrows only when
//! its execution root never writes it. Like TypeScript, the scan counts the
//! writes that run as part of the root's own flow and ignores the bodies of
//! other functions, which run only when called: a call never cancels
//! narrowing, in TypeScript or in Biome.

use super::subjects::{literal_key, property_key};
use super::{FlowSubject, FlowSubjectId, FlowSubjectTable, PropertyKey};
use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsName, JsComputedMemberAssignment, JsStaticMemberAssignment, JsSyntaxKind,
    JsSyntaxNode, JsUnaryExpression, JsUnaryOperator,
};
use biome_rowan::{AstNode, SyntaxResult, WalkEvent};
use rustc_hash::FxHashSet;

/// The property paths that the code of one execution root may write.
///
/// Every assignment to a member counts, whether it is `=`, a compound or
/// logical assignment, `++` or `--`, a destructuring target, or the target of
/// a `for...in` or `for...of` loop, and so does `delete`. Parentheses, `!`,
/// `as`, `satisfies`, and `<T>` around the objects of a written path are
/// ignored, so `(o as T).v = x` writes `o.v`. An element access writes the
/// property that its key names when the key is a literal that
/// [`literal_key`] accepts, so `o["v"] = x` writes `o.v`, and any property of
/// its object otherwise, as in `o[k] = x`.
///
/// The scan covers the whole root, including function and arrow expressions
/// that are called where they are defined, class static blocks, field
/// initializers, decorators, computed keys, and the parameters of nested
/// functions. It skips the bodies of other functions, methods, accessors, and
/// constructors. A write through another variable that holds the same object,
/// as in `a.v = x` after `const a = o`, does not write `o.v`. Wherever the
/// scan differs from TypeScript, it counts more writes: TypeScript ignores,
/// for example, writes in `async` and generator functions called where they
/// are defined, in field initializers and parameter defaults, and through
/// `as` and `satisfies`.
pub(super) struct WrittenPaths {
    /// Subjects that an assignment or `delete` writes directly.
    written: FxHashSet<FlowSubjectId>,
    /// Subjects with a property written through a key that is not a literal,
    /// as in `o[k] = x`, which can write any of their properties.
    wildcards: FxHashSet<FlowSubjectId>,
}

impl WrittenPaths {
    /// Collects the writes in the code of `root` to the paths stored in
    /// `subjects`.
    ///
    /// A write to a path that `subjects` does not store is ignored, because
    /// none of the stored subjects starts with it.
    pub(super) fn new(
        root: &AnyJsControlFlowRoot,
        model: &SemanticModel,
        subjects: &FlowSubjectTable,
    ) -> Self {
        let mut paths = Self {
            written: FxHashSet::default(),
            wildcards: FxHashSet::default(),
        };
        let root = root.syntax();
        let mut preorder = root.preorder();
        while let Some(event) = preorder.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if node != *root && runs_later(&node, root) {
                preorder.skip_subtree();
                continue;
            }
            if let Some(assignment) = JsStaticMemberAssignment::cast_ref(&node) {
                if let Ok(object) = assignment.object() {
                    paths.member(object, assignment.member(), model, subjects);
                }
            } else if let Some(assignment) = JsComputedMemberAssignment::cast_ref(&node) {
                if let Ok(object) = assignment.object() {
                    paths.computed_member(object, assignment.member(), model, subjects);
                }
            } else if let Some(unary) = JsUnaryExpression::cast_ref(&node)
                && unary.operator() == Ok(JsUnaryOperator::Delete)
                && let Ok(argument) = unary.argument()
            {
                paths.deleted(argument, model, subjects);
            }
        }
        paths
    }

    /// Returns whether a write in the root may change `subject`, given the
    /// `subjects` of the root, each at the index of its id.
    ///
    /// That is the case when the root writes `subject` or one of its
    /// prefixes, or writes a property of a proper prefix through a key that
    /// is not a literal. A write to an extension of `subject`, such as `o.v.w`
    /// for `o.v`, does not change the value of `subject`.
    pub(super) fn cancels(&self, subjects: &[FlowSubject], subject: FlowSubjectId) -> bool {
        if self.written.contains(&subject) {
            return true;
        }
        let mut current = subject;
        while let Some(FlowSubject::Member { object, .. }) = subjects.get(current as usize) {
            if self.written.contains(object) || self.wildcards.contains(object) {
                return true;
            }
            // A table stores objects before their members, so this only
            // guards against a malformed table.
            if *object >= current {
                return true;
            }
            current = *object;
        }
        false
    }

    /// Records the write of `name` on `object`.
    fn member(
        &mut self,
        object: AnyJsExpression,
        name: SyntaxResult<AnyJsName>,
        model: &SemanticModel,
        subjects: &FlowSubjectTable,
    ) {
        let Some(object) = self.object(object, model, subjects) else {
            return;
        };
        let key = match name {
            // A private name is never part of a subject's path.
            Ok(AnyJsName::JsPrivateName(_)) => return,
            Ok(name) => property_key(&name),
            Err(_) => None,
        };
        self.property(object, key, subjects);
    }

    /// Records the write of the property that `key` names on `object`, or of
    /// any property of `object` when `key` is not a literal that
    /// [`literal_key`] accepts.
    fn computed_member(
        &mut self,
        object: AnyJsExpression,
        key: SyntaxResult<AnyJsExpression>,
        model: &SemanticModel,
        subjects: &FlowSubjectTable,
    ) {
        if let Some(object) = self.object(object, model, subjects) {
            let key = key.ok().as_ref().and_then(literal_key);
            self.property(object, key, subjects);
        }
    }

    /// Records the write of the property `key` of the subject `object`, or of
    /// any of its properties when `key` is `None`.
    fn property(
        &mut self,
        object: FlowSubjectId,
        key: Option<PropertyKey>,
        subjects: &FlowSubjectTable,
    ) {
        match key {
            Some(key) => {
                if let Some(subject) = subjects.get(&FlowSubject::Member { object, key }) {
                    self.written.insert(subject);
                }
            }
            None => {
                self.wildcards.insert(object);
            }
        }
    }

    /// Records the property that `delete argument` removes, if any.
    fn deleted(
        &mut self,
        argument: AnyJsExpression,
        model: &SemanticModel,
        subjects: &FlowSubjectTable,
    ) {
        match omit_wrappers(argument) {
            Some(AnyJsExpression::JsStaticMemberExpression(member)) => {
                if let Ok(object) = member.object() {
                    self.member(object, member.member(), model, subjects);
                }
            }
            Some(AnyJsExpression::JsComputedMemberExpression(member)) => {
                if let Ok(object) = member.object() {
                    self.computed_member(object, member.member(), model, subjects);
                }
            }
            _ => {}
        }
    }

    /// Returns the subject that `expression`, the object of a written
    /// property, refers to, or `None` when no stored subject starts with it.
    ///
    /// A key along the object that is not a literal, as in `o[k].v`, can name
    /// any property, so it records a write of every property of the object
    /// before it and returns `None`.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Only variables and member accesses lead to a subject."
    )]
    fn object(
        &mut self,
        expression: AnyJsExpression,
        model: &SemanticModel,
        subjects: &FlowSubjectTable,
    ) -> Option<FlowSubjectId> {
        // The keys along the object from the outermost, with `None` for a
        // key that can name any property.
        let mut keys = Vec::new();
        let mut expression = expression;
        let reference = loop {
            match omit_wrappers(expression)? {
                AnyJsExpression::JsIdentifierExpression(identifier) => {
                    break identifier.name().ok()?;
                }
                AnyJsExpression::JsStaticMemberExpression(member) => {
                    match member.member() {
                        Ok(AnyJsName::JsPrivateName(_)) => return None,
                        Ok(name) => keys.push(property_key(&name)),
                        Err(_) => keys.push(None),
                    }
                    expression = member.object().ok()?;
                }
                AnyJsExpression::JsComputedMemberExpression(member) => {
                    keys.push(member.member().ok().as_ref().and_then(literal_key));
                    expression = member.object().ok()?;
                }
                _ => return None,
            }
        };
        let binding = model.binding(&reference)?;
        let mut subject = subjects.get(&FlowSubject::Binding(binding.range()))?;
        for key in keys.into_iter().rev() {
            match key {
                Some(key) => {
                    subject = subjects.get(&FlowSubject::Member {
                        object: subject,
                        key,
                    })?;
                }
                None => {
                    self.wildcards.insert(subject);
                    return None;
                }
            }
        }
        Some(subject)
    }
}

/// Returns whether `node`, which lies below `root`, runs only when something
/// calls it, after the code around it.
fn runs_later(node: &JsSyntaxNode, root: &JsSyntaxNode) -> bool {
    match node.kind() {
        JsSyntaxKind::JS_FUNCTION_DECLARATION
        | JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION => true,
        JsSyntaxKind::JS_FUNCTION_EXPRESSION | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
            !is_invoked(node)
        }
        // The body of a method, accessor, or constructor. Their decorators,
        // computed keys, and parameter decorators run with the class.
        JsSyntaxKind::JS_FUNCTION_BODY => node.parent().is_some_and(|parent| {
            parent != *root
                && !matches!(
                    parent.kind(),
                    JsSyntaxKind::JS_FUNCTION_EXPRESSION
                        | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                )
        }),
        _ => false,
    }
}

/// Returns whether the function or arrow expression `node` is called, or
/// constructed, where it is defined, as in `(() => {})()`.
fn is_invoked(node: &JsSyntaxNode) -> bool {
    let mut parent = node.parent();
    while let Some(node) = parent {
        match node.kind() {
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
            | JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_SATISFIES_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
            | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
            | JsSyntaxKind::TS_INSTANTIATION_EXPRESSION => parent = node.parent(),
            // The only expression directly below these nodes is the one they
            // call; arguments and template elements sit in lists.
            JsSyntaxKind::JS_CALL_EXPRESSION
            | JsSyntaxKind::JS_NEW_EXPRESSION
            | JsSyntaxKind::JS_TEMPLATE_EXPRESSION => return true,
            _ => return false,
        }
    }
    false
}

/// Removes parentheses, `!`, `as`, `satisfies`, and `<T>` from `expression`,
/// or returns `None` when one of them lacks its operand.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only wrappers that keep their operand's value are removed."
)]
fn omit_wrappers(mut expression: AnyJsExpression) -> Option<AnyJsExpression> {
    loop {
        expression = match expression {
            AnyJsExpression::JsParenthesizedExpression(wrapper) => wrapper.expression().ok()?,
            AnyJsExpression::TsNonNullAssertionExpression(wrapper) => wrapper.expression().ok()?,
            AnyJsExpression::TsAsExpression(wrapper) => wrapper.expression().ok()?,
            AnyJsExpression::TsSatisfiesExpression(wrapper) => wrapper.expression().ok()?,
            AnyJsExpression::TsTypeAssertionExpression(wrapper) => wrapper.expression().ok()?,
            expression => return Some(expression),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::super::subjects::subject_of;
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::JsExpressionStatement;
    use biome_languages::JsFileSource;

    const PATHS: [&str; 5] = ["o.v", "o.v.w", "o.w", "q.v", "o.v.w.x"];

    /// Returns the paths among [`PATHS`] that `body` cancels, as
    /// [`cancelled_among`] does.
    fn cancelled(body: &str) -> Vec<&'static str> {
        cancelled_among(&PATHS, body)
    }

    /// Returns the paths among `paths` that the function root
    /// `function f(o, q, k, xs, t) { ... }` with `body` cancels.
    fn cancelled_among(paths: &[&'static str], body: &str) -> Vec<&'static str> {
        let source = format!(
            "function f(o, q, k, xs, t) {{ {}; {body} }}",
            paths.join("; ")
        );
        let parsed = parse(&source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let mut subjects = FlowSubjectTable::default();
        let ids: Vec<_> = parsed
            .syntax()
            .descendants()
            .filter_map(JsExpressionStatement::cast)
            .take(paths.len())
            .map(|statement| {
                subject_of(&statement.expression().unwrap(), &model, &mut subjects).unwrap()
            })
            .collect();
        let root = parsed
            .syntax()
            .descendants()
            .filter_map(AnyJsControlFlowRoot::cast)
            .find(|root| root.syntax().kind() == JsSyntaxKind::JS_FUNCTION_DECLARATION)
            .unwrap();
        let written = WrittenPaths::new(&root, &model, &subjects);
        paths
            .iter()
            .copied()
            .zip(ids)
            .filter(|(_, id)| written.cancels(subjects.as_slice(), *id))
            .map(|(path, _)| path)
            .collect()
    }

    #[test]
    fn writes_cancel_the_path_and_its_extensions() {
        for body in [
            "o.v = null",
            "o.v += 1",
            "o.v ??= 1",
            "o.v++",
            "--o.v",
            "[o.v] = xs",
            "({ a: o.v } = xs)",
            "for (o.v of xs) {}",
            "for (o.v in xs) {}",
            "delete o.v",
            "delete (o?.v)",
            "(o).v = null",
            "(o as any).v = null",
            "(<any>o).v = null",
            "(o satisfies object).v = null",
            "o!.v = null",
            "o.v! = null",
            "(o.v as any) = null",
            "o.\\u0076 = null",
        ] {
            assert_eq!(cancelled(body), ["o.v", "o.v.w", "o.v.w.x"], "{body}");
        }
    }

    #[test]
    fn writes_to_extensions_keep_their_prefixes() {
        assert_eq!(cancelled("q.v = null"), ["q.v"]);
        assert_eq!(cancelled("o.v.w = null"), ["o.v.w", "o.v.w.x"]);
        assert_eq!(cancelled("o.v[k] = null"), ["o.v.w", "o.v.w.x"]);
        assert_eq!(cancelled("o['v'].w = null"), ["o.v.w", "o.v.w.x"]);
        assert_eq!(cancelled("o['v']['w'] = null"), ["o.v.w", "o.v.w.x"]);
        assert_eq!(cancelled("o['v'][k] = null"), ["o.v.w", "o.v.w.x"]);
    }

    #[test]
    fn literal_keys_cancel_the_path_they_name() {
        for body in [
            "o['v'] = null",
            "o[\"v\"] = null",
            "o['v'] += 1",
            "o['v']++",
            "[o['v']] = xs",
            "for (o['v'] of xs) {}",
            "delete o['v']",
            "(o as any)['v'] = null",
            "o['v'] ??= 1",
        ] {
            assert_eq!(cancelled(body), ["o.v", "o.v.w", "o.v.w.x"], "{body}");
        }
        assert_eq!(cancelled("o['x'] = null"), Vec::<&str>::new());
        assert_eq!(cancelled("o[0] = null"), Vec::<&str>::new());
    }

    #[test]
    fn integer_keys_cancel_the_element_they_name() {
        const ELEMENTS: [&str; 4] = ["t[0]", "t[1]", "t[0].v", "t['01']"];
        for body in ["t[0] = null", "t['0'] = null", "t[0]++", "delete t[0]"] {
            assert_eq!(
                cancelled_among(&ELEMENTS, body),
                ["t[0]", "t[0].v"],
                "{body}"
            );
        }
        assert_eq!(cancelled_among(&ELEMENTS, "t[1] = null"), ["t[1]"]);
        assert_eq!(cancelled_among(&ELEMENTS, "t['01'] = null"), ["t['01']"]);
        assert_eq!(cancelled_among(&ELEMENTS, "t[0].v = null"), ["t[0].v"]);
    }

    #[test]
    fn keys_that_are_not_literals_cancel_every_property_of_their_object() {
        for body in [
            "o[k] = null",
            "delete o[k]",
            "o[k].w = null",
            "(o as any)[k] = null",
            "o[k] += 1",
            "o[-0] = null",
            "o[0x0] = null",
            "o[1e0] = null",
            "o[9007199254740993] = null",
            "o[`v`] = null",
            "o['\\u0076'] = null",
            "o[('v')] = null",
        ] {
            assert_eq!(
                cancelled(body),
                ["o.v", "o.v.w", "o.w", "o.v.w.x"],
                "{body}"
            );
        }
        assert_eq!(
            cancelled_among(&["t[0]", "t[1]"], "t[-0] = null"),
            ["t[0]", "t[1]"]
        );
    }

    #[test]
    fn writes_that_run_with_the_root_cancel() {
        for body in [
            "(() => { o.v = null; })()",
            "(() => o.v = null)()",
            "(function () { o.v = null; })()",
            "(async () => { o.v = null; })()",
            "(function* () { o.v = null; })()",
            "new (function () { o.v = null; })()",
            "(() => { (() => { o.v = null; })(); })()",
            "class C { static { o.v = null; } }",
            "class C { static x = (o.v = null); }",
            "class C { x = (o.v = null); }",
            "class C { [(o.v = null, 'k')]() {} }",
            "class C { m(a = (o.v = null)) {} }",
            "if (q) { o.v = null; }",
        ] {
            assert_eq!(cancelled(body), ["o.v", "o.v.w", "o.v.w.x"], "{body}");
        }
    }

    #[test]
    fn writes_that_run_later_or_through_other_references_keep_narrowing() {
        for body in [
            "const g = () => { o.v = null; }",
            "const g = () => o.v = null",
            "function g() { o.v = null; }",
            "[1].forEach(() => { o.v = null; })",
            "(function () { o.v = null; }).call(this)",
            "(() => () => { o.v = null; })()",
            "class C { m() { o.v = null; } }",
            "class C { constructor() { o.v = null; } }",
            "class C { get x() { o.v = null; return 1; } }",
            "({ m() { o.v = null; } })",
            "({ set x(value) { o.v = value; } })",
            "const a = o; a.v = null",
            "this.v = null",
            "xs.v = null",
            "o.x = null",
            "o.w.v = null",
        ] {
            assert_eq!(cancelled(body), Vec::<&str>::new(), "{body}");
        }
    }

    #[test]
    fn a_write_to_the_variable_is_not_a_path_write() {
        // Collection excludes written variables, and with them their paths.
        assert_eq!(cancelled("o = q"), Vec::<&str>::new());
    }
}
