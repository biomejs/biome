//! Finds the writes of an execution root that its flow graph doesn't model.
//!
//! The builder adds a write point for each write in the root's own flow. A
//! write that runs with the root but outside that flow, as in a function
//! expression called where it is defined, a class static block, or a field
//! initializer, has no point, so a subject that such code writes doesn't
//! narrow anywhere in the root. Like TypeScript, the scan ignores the bodies
//! of other functions, which run only when called: a call never cancels
//! narrowing, in TypeScript or in Biome.

use super::subjects::{literal_key, property_key};
use super::{FlowSubject, FlowSubjectId, FlowSubjectTable, PropertyKey};
use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsName, AnyTsCastExpression, JsArrowFunctionExpression,
    JsComputedMemberAssignment, JsFunctionBody, JsFunctionExpression, JsIdentifierAssignment,
    JsLanguage, JsNewOrCallExpression, JsParenthesizedExpression, JsStaticMemberAssignment,
    JsSyntaxNode, JsTemplateExpression, JsUnaryExpression, JsUnaryOperator,
    TsInstantiationExpression, TsNonNullAssertionExpression, assign_ext::AnyJsMemberAssignment,
};
use biome_rowan::{
    AstNode, SyntaxKindSet, SyntaxResult, TextRange, TextSize, WalkEvent, declare_node_union,
};
use rustc_hash::FxHashSet;

declare_node_union! {
    /// Code that the write scan skips when it runs only when something calls
    /// it: a function, or the body of a method, accessor, or constructor.
    AnyDeferredCode = AnyJsFunction | JsFunctionBody
}

/// The nodes that the write scan reads: writes, `delete`, and the code it may
/// skip.
const WRITE_SCAN_KINDS: SyntaxKindSet<JsLanguage> = AnyDeferredCode::KIND_SET
    .union(AnyJsMemberAssignment::KIND_SET)
    .union(JsIdentifierAssignment::KIND_SET)
    .union(JsUnaryExpression::KIND_SET);

/// The writes that have a write point in the graph of an execution root, as
/// the builder records them.
#[derive(Default)]
pub(super) struct WrittenTargets {
    /// The start of each variable that an assignment target with a write
    /// point names, which is where the semantic model's reference to the
    /// variable starts.
    pub(super) variables: FxHashSet<TextSize>,
    /// The range of each member assignment target and `delete` expression
    /// with a write point.
    pub(super) members: FxHashSet<TextRange>,
}

/// Returns whether `node`, which lies below `root`, runs only when something
/// calls the code around it, as in the body of a function declaration or of
/// a callback, so that a write in it never runs as part of the root's flow.
pub(super) fn runs_only_when_called(node: &JsSyntaxNode, root: &AnyJsControlFlowRoot) -> bool {
    node.ancestors()
        .take_while(|ancestor| ancestor != root.syntax())
        .filter_map(AnyDeferredCode::cast)
        .any(|code| runs_later(&code, root))
}

/// Function and arrow expressions, whose bodies run when the function does.
const FUNCTION_EXPRESSION_KINDS: SyntaxKindSet<JsLanguage> =
    JsFunctionExpression::KIND_SET.union(JsArrowFunctionExpression::KIND_SET);

/// Expressions between a function expression and the call that invokes it,
/// which keep the value of their operand.
const TYPE_WRAPPER_KINDS: SyntaxKindSet<JsLanguage> = JsParenthesizedExpression::KIND_SET
    .union(AnyTsCastExpression::KIND_SET)
    .union(TsNonNullAssertionExpression::KIND_SET)
    .union(TsInstantiationExpression::KIND_SET);

/// Expressions that call or construct the expression directly below them.
const CALL_KINDS: SyntaxKindSet<JsLanguage> =
    JsNewOrCallExpression::KIND_SET.union(JsTemplateExpression::KIND_SET);

/// The subjects that the code of one execution root may write.
///
/// Every assignment to a variable or a member counts, whether it is `=`, a
/// compound or logical assignment, `++` or `--`, a destructuring target, or
/// the target of a `for...in` or `for...of` loop, and so does `delete`.
/// Parentheses, `!`, `as`, `satisfies`, and `<T>` around the objects of a
/// written path are ignored, so `(o as T).v = x` writes `o.v`. An element
/// access writes the property that its key names when the key is a literal
/// that [`literal_key`] accepts, so `o["v"] = x` writes `o.v`, and any
/// property of its object otherwise, as in `o[k] = x`.
///
/// The scan covers the whole root, including function and arrow expressions
/// that are called where they are defined, class static blocks, field
/// initializers, decorators, computed keys, and the parameters of methods,
/// accessors, and constructors. It skips the bodies of other functions,
/// methods, accessors, and constructors. A write through another variable that
/// holds the same object, as in `a.v = x` after `const a = o`, does not write
/// `o.v`. The scan treats every `this` and `super` as the root's `this`. So
/// `super.v = x` writes `this.v`, because it sets `v` on `this` or calls a
/// setter of a parent class with `this` as its receiver. Wherever the scan
/// differs from TypeScript, it counts more writes: TypeScript ignores, for
/// example, writes in `async` and generator functions called where they are
/// defined, in field initializers and the parameter defaults of methods,
/// through `as` and `satisfies`, and through `super`. Like the scan, TypeScript 7.0.2 treats
/// `this.v = x` in a static block as a write of the root's `this.v`, although
/// the static block's `this` is the class.
///
/// The scan tells apart the writes that have a write point in the graph,
/// which the builder recorded, from the others, which run outside the
/// root's flow or through a target that the builder doesn't map to a
/// subject, as `(o as T).v = x`.
pub(super) struct WrittenPaths {
    /// Subjects that an assignment or `delete` writes directly.
    written: FxHashSet<FlowSubjectId>,
    /// Subjects with a property written through a key that does not spell the
    /// property's name, as in `o[k] = x`. Such a write counts as a write of
    /// every property of the subject.
    wildcards: FxHashSet<FlowSubjectId>,
    /// The subjects of `written` that a write without a write point writes.
    unmodeled: FxHashSet<FlowSubjectId>,
    /// The subjects of `wildcards` that a write without a write point
    /// writes.
    unmodeled_wildcards: FxHashSet<FlowSubjectId>,
    /// The subjects that the write being scanned writes, each with whether
    /// it writes any of their properties instead.
    found: Vec<(FlowSubjectId, bool)>,
}

impl WrittenPaths {
    /// Collects the writes in the code of `root` to the subjects stored in
    /// `subjects`, telling apart those that `modeled` records as having a
    /// write point.
    ///
    /// A write to a path that `subjects` does not store is ignored, because
    /// none of the stored subjects starts with it.
    pub(super) fn new(
        root: &AnyJsControlFlowRoot,
        model: &SemanticModel,
        subjects: &FlowSubjectTable,
        modeled: &WrittenTargets,
    ) -> Self {
        let mut paths = Self {
            written: FxHashSet::default(),
            wildcards: FxHashSet::default(),
            unmodeled: FxHashSet::default(),
            unmodeled_wildcards: FxHashSet::default(),
            found: Vec::new(),
        };
        let mut events = root.syntax().preorder_of_kinds(WRITE_SCAN_KINDS);
        while let Some(event) = events.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if node != *root.syntax()
                && let Some(code) = AnyDeferredCode::cast_ref(&node)
                && runs_later(&code, root)
            {
                events.skip_subtree();
                continue;
            }
            let is_modeled = if let Some(assignment) = JsStaticMemberAssignment::cast_ref(&node) {
                if let Ok(object) = assignment.object() {
                    paths.member(object, assignment.member(), model, subjects);
                }
                modeled.members.contains(&assignment.range())
            } else if let Some(assignment) = JsComputedMemberAssignment::cast_ref(&node) {
                if let Ok(object) = assignment.object() {
                    paths.computed_member(object, assignment.member(), model, subjects);
                }
                modeled.members.contains(&assignment.range())
            } else if let Some(assignment) = JsIdentifierAssignment::cast_ref(&node) {
                if let Some(subject) = model
                    .binding(&assignment)
                    .and_then(|binding| subjects.get(&FlowSubject::Binding(binding.range())))
                {
                    paths.found.push((subject, false));
                }
                modeled
                    .variables
                    .contains(&assignment.syntax().text_trimmed_range().start())
            } else if let Some(unary) = JsUnaryExpression::cast_ref(&node)
                && unary.operator() == Ok(JsUnaryOperator::Delete)
                && let Ok(argument) = unary.argument()
            {
                paths.deleted(argument, model, subjects);
                modeled.members.contains(&unary.range())
            } else {
                true
            };
            for (subject, wildcard) in std::mem::take(&mut paths.found) {
                let (all, unmodeled) = if wildcard {
                    (&mut paths.wildcards, &mut paths.unmodeled_wildcards)
                } else {
                    (&mut paths.written, &mut paths.unmodeled)
                };
                all.insert(subject);
                if !is_modeled {
                    unmodeled.insert(subject);
                }
            }
        }
        paths
    }

    /// Returns whether a write in the root may change `subject`, given the
    /// `subjects` of the root, each at the index of its id. With
    /// `modeled` false, only the writes that have no write point count,
    /// which narrowing can't follow along the flow.
    ///
    /// That is the case when the root writes `subject` or one of its
    /// prefixes, or writes a property of a proper prefix through a key that
    /// is not a literal. A write to an extension of `subject`, such as `o.v.w`
    /// for `o.v`, does not change the value of `subject`.
    pub(super) fn cancels(
        &self,
        subjects: &[FlowSubject],
        subject: FlowSubjectId,
        modeled: bool,
    ) -> bool {
        let (written, wildcards) = if modeled {
            (&self.written, &self.wildcards)
        } else {
            (&self.unmodeled, &self.unmodeled_wildcards)
        };
        if written.contains(&subject) {
            return true;
        }
        let mut current = subject;
        while let Some(FlowSubject::Member { object, .. }) = subjects.get(current as usize) {
            if written.contains(object) || wildcards.contains(object) {
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
        let key = name.ok().as_ref().and_then(property_key);
        self.property(object, key, subjects);
    }

    /// Records the write of the property that `key` names on `object`, or of
    /// any property of `object` when `key` does not spell the property's name.
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
                    self.found.push((subject, false));
                }
            }
            None => self.found.push((object, true)),
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
    /// `this` and `super` refer to the root's `this`. A key along the object
    /// that does not spell the property's name, as in `o[k].v`, can name any
    /// property, so it records a write of every property of the object before
    /// it and returns `None`.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Only variables, `this`, `super`, and member accesses lead to a subject."
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
        let mut subject = loop {
            match omit_wrappers(expression)? {
                AnyJsExpression::JsIdentifierExpression(identifier) => {
                    let binding = model.binding(&identifier.name().ok()?)?;
                    break subjects.get(&FlowSubject::Binding(binding.range()))?;
                }
                AnyJsExpression::JsThisExpression(_) | AnyJsExpression::JsSuperExpression(_) => {
                    break subjects.get(&FlowSubject::This)?;
                }
                AnyJsExpression::JsStaticMemberExpression(member) => {
                    keys.push(member.member().ok().as_ref().and_then(property_key));
                    expression = member.object().ok()?;
                }
                AnyJsExpression::JsComputedMemberExpression(member) => {
                    keys.push(member.member().ok().as_ref().and_then(literal_key));
                    expression = member.object().ok()?;
                }
                _ => return None,
            }
        };
        for key in keys.into_iter().rev() {
            match key {
                Some(key) => {
                    subject = subjects.get(&FlowSubject::Member {
                        object: subject,
                        key,
                    })?;
                }
                None => {
                    self.found.push((subject, true));
                    return None;
                }
            }
        }
        Some(subject)
    }
}

/// Returns whether `code`, which lies below `root`, runs only when something
/// calls it, after the code around it.
fn runs_later(code: &AnyDeferredCode, root: &AnyJsControlFlowRoot) -> bool {
    match code {
        AnyDeferredCode::AnyJsFunction(
            AnyJsFunction::JsFunctionDeclaration(_)
            | AnyJsFunction::JsFunctionExportDefaultDeclaration(_),
        ) => true,
        AnyDeferredCode::AnyJsFunction(
            function @ (AnyJsFunction::JsFunctionExpression(_)
            | AnyJsFunction::JsArrowFunctionExpression(_)),
        ) => !is_invoked(function),
        // The body of a method, accessor, or constructor. Their decorators,
        // computed keys, and parameter decorators run with the class.
        AnyDeferredCode::JsFunctionBody(body) => body.syntax().parent().is_some_and(|parent| {
            parent != *root.syntax() && !FUNCTION_EXPRESSION_KINDS.matches(parent.kind())
        }),
    }
}

/// Returns whether `function`, a function or arrow expression, is called, or
/// constructed, where it is defined, as in `(() => {})()`.
pub(super) fn is_invoked(function: &AnyJsFunction) -> bool {
    let mut parent = function.syntax().parent();
    while let Some(node) = parent {
        let kind = node.kind();
        if !TYPE_WRAPPER_KINDS.matches(kind) {
            // The only expression directly below these nodes is the one they
            // call; arguments and template elements sit in lists.
            return CALL_KINDS.matches(kind);
        }
        parent = node.parent();
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
    use super::super::builder::{BuiltRoot, build_root};
    use super::super::guards::GuardFunctions;
    use super::super::subjects::subject_of;
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::{JsExpressionStatement, JsSyntaxKind};
    use biome_languages::JsFileSource;
    use rustc_hash::FxHashMap;

    const PATHS: [&str; 5] = ["o.v", "o.v.w", "o.w", "q.v", "o.v.w.x"];

    /// Returns the paths among [`PATHS`] that any write of `body` cancels, as
    /// [`cancelled_among`] does.
    fn cancelled(body: &str) -> Vec<&'static str> {
        cancelled_among(&PATHS, body)
    }

    /// Returns the paths among `paths` that any write of the method root
    /// `m(o, q, k, xs, t) { ... }` with `body` cancels, as a property guard
    /// held in a `const` needs. The method's class declares a private field
    /// `#p`, and paths on `this` can narrow in it.
    fn cancelled_among(paths: &[&'static str], body: &str) -> Vec<&'static str> {
        scanned(paths, body, true)
    }

    /// Returns the paths among `paths` that a write of `body` without a write
    /// point cancels, as [`cancelled_among`] describes the root.
    fn unmodeled_among(paths: &[&'static str], body: &str) -> Vec<&'static str> {
        scanned(paths, body, false)
    }

    fn scanned(paths: &[&'static str], body: &str, modeled: bool) -> Vec<&'static str> {
        let source = format!(
            "class K extends Object {{ #p; m(o, q, k, xs, t) {{ {}; {body} }} }}",
            paths.join("; ")
        );
        let parsed = parse(&source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let root = parsed
            .syntax()
            .descendants()
            .filter_map(AnyJsControlFlowRoot::cast)
            .find(|root| root.syntax().kind() == JsSyntaxKind::JS_METHOD_CLASS_MEMBER)
            .unwrap();
        let raw_binding_types = FxHashMap::default();
        let BuiltRoot {
            mut subjects,
            written,
            ..
        } = build_root(
            &root,
            &model,
            &mut GuardFunctions::new(&[], &raw_binding_types),
            true,
        )
        .expect("root must have flow");
        let ids: Vec<_> = parsed
            .syntax()
            .descendants()
            .filter_map(JsExpressionStatement::cast)
            .take(paths.len())
            .map(|statement| {
                subject_of(&statement.expression().unwrap(), &model, &mut subjects).unwrap()
            })
            .collect();
        let written = WrittenPaths::new(&root, &model, &subjects, &written);
        paths
            .iter()
            .copied()
            .zip(ids)
            .filter(|(_, id)| written.cancels(subjects.as_slice(), *id, modeled))
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
    fn writes_that_run_with_the_root_outside_its_flow_cancel() {
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
            // The builder maps no subject to an object behind `as` or `!`.
            "(o as any).v = null",
            "o!.v = null",
        ] {
            assert_eq!(
                unmodeled_among(&PATHS, body),
                ["o.v", "o.v.w", "o.v.w.x"],
                "{body}"
            );
            assert_eq!(cancelled(body), ["o.v", "o.v.w", "o.v.w.x"], "{body}");
        }
    }

    #[test]
    fn writes_in_the_root_flow_are_write_points() {
        for body in [
            "if (q) { o.v = null; }",
            "o.v = null",
            "o['v'] ??= 1",
            "o.v++",
            "[o.v] = xs",
            "for (o.v of xs) {}",
            "delete o.v",
            "(o.v as any) = null",
            "o.v! = null",
        ] {
            assert_eq!(unmodeled_among(&PATHS, body), Vec::<&str>::new(), "{body}");
            assert_eq!(cancelled(body), ["o.v", "o.v.w", "o.v.w.x"], "{body}");
        }
        assert_eq!(unmodeled_among(&PATHS, "o[k] = null"), Vec::<&str>::new());
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

    const THIS_PATHS: [&str; 5] = ["this.v", "this.v.w", "this.#p", "this.w", "o.v"];

    #[test]
    fn writes_through_this_or_super_cancel_paths_on_this() {
        for body in [
            "this.v = null",
            "(this).v = null",
            "this['v'] = null",
            "this.v ??= null",
            "this.v++",
            "[this.v] = xs",
            "delete this.v",
            "super.v = null",
            "super['v'] = null",
            "(() => { this.v = null; })()",
            // TypeScript also counts a function called where it is defined,
            // although its `this` is another value.
            "(function () { this.v = null; })()",
            "class C { static { this.v = null; } }",
        ] {
            assert_eq!(
                cancelled_among(&THIS_PATHS, body),
                ["this.v", "this.v.w"],
                "{body}"
            );
        }
        assert_eq!(
            cancelled_among(&THIS_PATHS, "this.v.w = null"),
            ["this.v.w"]
        );
        assert_eq!(cancelled_among(&THIS_PATHS, "this.#p = null"), ["this.#p"]);
        // A string key names a property, never a private name.
        assert_eq!(
            cancelled_among(&THIS_PATHS, "this['#p'] = null"),
            Vec::<&str>::new()
        );
        for body in ["this[k] = null", "super[k] = null", "delete this[k]"] {
            assert_eq!(
                cancelled_among(&THIS_PATHS, body),
                ["this.v", "this.v.w", "this.#p", "this.w"],
                "{body}"
            );
        }
    }

    #[test]
    fn writes_to_this_in_callbacks_and_other_members_keep_narrowing() {
        for body in [
            "[1].forEach(() => { this.v = null; })",
            "const g = () => { this.v = null; }",
            "(function () { this.v = null; }).call(this)",
            "class C { m() { this.v = null; } }",
            "const self = this; self.v = null",
            "o.w = null",
        ] {
            assert_eq!(
                cancelled_among(&THIS_PATHS, body),
                Vec::<&str>::new(),
                "{body}"
            );
        }
    }

    #[test]
    fn a_write_to_the_variable_writes_its_paths() {
        assert_eq!(cancelled("o = q"), ["o.v", "o.v.w", "o.w", "o.v.w.x"]);
        // The graph has a write point for it.
        assert_eq!(unmodeled_among(&PATHS, "o = q"), Vec::<&str>::new());
        // A write in a function called where it is defined has none.
        assert_eq!(
            unmodeled_among(&PATHS, "(() => { o = q; })()"),
            ["o.v", "o.v.w", "o.w", "o.v.w.x"]
        );
    }
}
