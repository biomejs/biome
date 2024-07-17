use super::*;
use biome_js_syntax::{
    JsArrowFunctionExpression, JsConstructorClassMember, JsFunctionDeclaration,
    JsFunctionExpression, JsGetterClassMember, JsGetterObjectMember, JsMethodClassMember,
    JsMethodObjectMember, JsSetterClassMember, JsSetterObjectMember,
};
use biome_rowan::{AstNode, SyntaxNodeCast};
use std::rc::Rc;

/// Marker trait that groups all "AstNode" that have closure
pub trait HasClosureAstNode {
    fn node_text_range(&self) -> TextRange;
}

macro_rules! SyntaxTextRangeHasClosureAstNode {
    ($($kind:tt => $node:tt,)*) => {
        $(
            impl HasClosureAstNode for $node {
                #[inline(always)]
                fn node_text_range(&self) -> TextRange {
                    self.syntax().text_trimmed_range()
                }
            }
        )*

        /// All nodes that have an associated closure
        /// and can be used by the [SemanticModel].
        pub enum AnyHasClosureNode {
            $(
                $node($node),
            )*
        }

        impl AnyHasClosureNode {
            pub fn from_node(node: &JsSyntaxNode) -> Option<AnyHasClosureNode> {
                match node.kind() {
                    $(
                    JsSyntaxKind::$kind => node
                        .clone()
                        .cast::<$node>()
                        .map(AnyHasClosureNode::$node),
                    )*
                    _ => None,
                }
            }
        }

        impl HasClosureAstNode for AnyHasClosureNode {
            #[inline(always)]
            fn node_text_range(&self) -> TextRange {
                match self {
                    $(
                        AnyHasClosureNode::$node(node) => node.syntax().text_trimmed_range(),
                    )*
                }
            }
        }
    };
}

SyntaxTextRangeHasClosureAstNode! {
    JS_FUNCTION_DECLARATION => JsFunctionDeclaration,
    JS_FUNCTION_EXPRESSION => JsFunctionExpression,
    JS_ARROW_FUNCTION_EXPRESSION => JsArrowFunctionExpression,
    JS_CONSTRUCTOR_CLASS_MEMBER => JsConstructorClassMember,
    JS_METHOD_CLASS_MEMBER => JsMethodClassMember,
    JS_GETTER_CLASS_MEMBER => JsGetterClassMember,
    JS_SETTER_CLASS_MEMBER => JsSetterClassMember,
    JS_METHOD_OBJECT_MEMBER => JsMethodObjectMember,
    JS_GETTER_OBJECT_MEMBER => JsGetterObjectMember,
    JS_SETTER_OBJECT_MEMBER => JsSetterObjectMember,
}

#[derive(Clone)]
pub enum CaptureType {
    ByReference,
    Type,
}

/// Provides all information regarding a specific closure capture.
#[derive(Clone)]
pub struct Capture {
    data: Rc<SemanticModelData>,
    ty: CaptureType,
    node: JsSyntaxNode,
    binding_id: BindingId,
}

impl Capture {
    /// Returns if the capture is by reference or just the type of the variable.
    pub fn ty(&self) -> &CaptureType {
        &self.ty
    }

    /// Returns the reference node of the capture
    pub fn node(&self) -> &JsSyntaxNode {
        &self.node
    }

    /// Returns the binding of this capture
    pub fn binding(&self) -> Binding {
        Binding {
            data: self.data.clone(),
            id: self.binding_id,
        }
    }

    /// Returns the trimmed text range of the declaration of this capture.
    /// This is equivalent, but faster, to:
    ///
    /// ```rs, ignore
    /// self.binding().syntax().text_trimmed_range()
    /// ```
    pub fn declaration_range(&self) -> TextRange {
        self.data.binding(self.binding_id).range
    }
}

pub struct AllCapturesIter {
    data: Rc<SemanticModelData>,
    closure_range: TextRange,
    scopes: Vec<ScopeId>,
    references: Vec<ReferenceId>,
}

impl Iterator for AllCapturesIter {
    type Item = Capture;

    fn next(&mut self) -> Option<Self::Item> {
        'references: loop {
            while let Some(reference) = self.references.pop() {
                let binding_id = reference.binding_id();
                let binding = &self.data.binding(binding_id);
                if !self.closure_range.contains(binding.range.start()) {
                    let reference = &binding.references[reference.index()];
                    return Some(Capture {
                        data: self.data.clone(),
                        node: self.data.binding_node_by_start[&reference.range_start].clone(), // TODO change node to store the range
                        ty: CaptureType::ByReference,
                        binding_id,
                    });
                }
            }

            'scopes: while let Some(scope_id) = self.scopes.pop() {
                let scope = &self.data.scopes[scope_id.index()];

                if scope.is_closure {
                    continue 'scopes;
                }
                self.references.clear();
                self.references
                    .extend(scope.read_references.iter().copied());
                self.references
                    .extend(scope.write_references.iter().copied());
                self.scopes.extend(scope.children.iter());
                continue 'references;
            }

            return None;
        }
    }
}

impl FusedIterator for AllCapturesIter {}

/// Iterate all immediate children closures of a specific closure
pub struct ChildrenIter {
    data: Rc<SemanticModelData>,
    scopes: Vec<ScopeId>,
}

impl Iterator for ChildrenIter {
    type Item = Closure;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(scope_id) = self.scopes.pop() {
            let scope = &self.data.scopes[scope_id.index()];
            if scope.is_closure {
                return Some(Closure {
                    data: self.data.clone(),
                    scope_id,
                });
            } else {
                self.scopes.extend(scope.children.iter());
            }
        }

        None
    }
}

impl FusedIterator for ChildrenIter {}

/// Iterate all descendents closures of a specific closure
pub struct DescendentsIter {
    data: Rc<SemanticModelData>,
    scopes: Vec<ScopeId>,
}

impl Iterator for DescendentsIter {
    type Item = Closure;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(scope_id) = self.scopes.pop() {
            let scope = &self.data.scopes[scope_id.index()];
            self.scopes.extend(scope.children.iter());
            if scope.is_closure {
                return Some(Closure {
                    data: self.data.clone(),
                    scope_id,
                });
            }
        }

        None
    }
}

impl FusedIterator for DescendentsIter {}

/// Provides all information regarding a specific closure.
#[derive(Clone)]
pub struct Closure {
    data: Rc<SemanticModelData>,
    scope_id: ScopeId,
}

impl Closure {
    pub(super) fn from_node(data: Rc<SemanticModelData>, node: &impl HasClosureAstNode) -> Closure {
        let closure_range = node.node_text_range();
        let scope_id = data.scope(closure_range);

        Closure { data, scope_id }
    }

    pub(super) fn from_scope(data: Rc<SemanticModelData>, scope_id: ScopeId) -> Option<Closure> {
        let node = &data.scope_node_by_range[&data.scopes[scope_id.index()].range];
        match node.kind() {
            JsSyntaxKind::JS_FUNCTION_DECLARATION
            | JsSyntaxKind::JS_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => Some(Closure { data, scope_id }),
            _ => None,
        }
    }

    /// Range of this [Closure]
    pub fn closure_range(&self) -> TextRange {
        self.data.scopes[self.scope_id.index()].range
    }

    /// Return all [Reference] this closure captures, not taking into
    /// consideration any capture of children closures
    ///
    /// ```rust,ignore
    /// let inner_function = "let a, b;
    /// function f(c) {
    ///     console.log(a);
    ///     function g() {
    ///         console.log(b, c);
    ///     }
    /// }";
    /// assert!(model.closure(function_f).all_captures(), &["a"]);
    /// ```
    pub fn all_captures(&self) -> impl Iterator<Item = Capture> {
        let scope = &self.data.scopes[self.scope_id.index()];

        let scopes = scope.children.clone();

        let mut references = scope.read_references.clone();
        references.extend(scope.write_references.iter().copied());

        AllCapturesIter {
            data: self.data.clone(),
            closure_range: self.closure_range(),
            scopes,
            references,
        }
    }

    /// Return all immediate children closures of this closure.
    ///
    /// ```rust,ignore
    /// let inner_function = "let a, b;
    /// function f(c) {
    ///     console.log(a);
    ///     function g() {
    ///         function h() {
    ///         }
    ///         console.log(b, c);
    ///     }
    /// }";
    /// assert!(model.closure(function_f).children(), &["g"]);
    /// ```
    pub fn children(&self) -> impl Iterator<Item = Closure> {
        let scope = &self.data.scopes[self.scope_id.index()];
        ChildrenIter {
            data: self.data.clone(),
            scopes: scope.children.clone(),
        }
    }

    /// Returns all descendents of this closure in breadth-first order. Starting with the current
    /// [Closure].
    ///
    /// ```rust,ignore
    /// let inner_function = "let a, b;
    /// function f(c) {
    ///     console.log(a);
    ///     function g() {
    ///         function h() {
    ///         }
    ///         console.log(b, c);
    ///     }
    /// }";
    /// assert!(model.closure(function_f).descendents(), &["f", "g", "h"]);
    /// ```
    pub fn descendents(&self) -> impl Iterator<Item = Closure> {
        let scopes = vec![self.scope_id];
        DescendentsIter {
            data: self.data.clone(),
            scopes,
        }
    }
}

pub trait ClosureExtensions {
    fn closure(&self, model: &SemanticModel) -> Closure
    where
        Self: HasClosureAstNode + Sized,
    {
        model.closure(self)
    }
}

impl<T: HasClosureAstNode> ClosureExtensions for T {}

#[cfg(test)]
mod test {
    use super::*;
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::{JsArrowFunctionExpression, JsFileSource, JsSyntaxKind};
    use biome_rowan::SyntaxNodeCast;

    fn assert_closure(code: &str, name: &str, captures: &[&str]) {
        let r = biome_js_parser::parse(code, JsFileSource::tsx(), JsParserOptions::default());
        let model = semantic_model(&r.tree(), SemanticModelOptions::default());

        let closure = if name != "ARROWFUNCTION" {
            let node = r
                .syntax()
                .descendants()
                .filter(|x| x.text_trimmed() == name)
                .last()
                .unwrap();
            let node = node
                .parent()
                .and_then(|node| AnyHasClosureNode::from_node(&node))
                .unwrap();
            model.closure(&node)
        } else {
            let node = r
                .syntax()
                .descendants()
                .filter(|x| x.kind() == JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
                .last()
                .unwrap()
                .cast::<JsArrowFunctionExpression>()
                .unwrap();
            model.closure(&node)
        };

        let expected_captures: BTreeSet<String> =
            captures.iter().map(|x| (*x).to_string()).collect();

        let all_captures: BTreeSet<String> = closure
            .all_captures()
            .map(|x| x.node().text_trimmed().to_string())
            .collect();

        let intersection = expected_captures.intersection(&all_captures);
        let intersection_count = intersection.count();

        assert_eq!(intersection_count, expected_captures.len());
        assert_eq!(intersection_count, all_captures.len());
    }

    fn get_closure_children(code: &str, name: &str) -> Vec<Closure> {
        let r = biome_js_parser::parse(code, JsFileSource::tsx(), JsParserOptions::default());
        let model = semantic_model(&r.tree(), SemanticModelOptions::default());

        let closure = if name != "ARROWFUNCTION" {
            let node = r
                .syntax()
                .descendants()
                .filter(|x| x.text_trimmed() == name)
                .last()
                .unwrap();
            let node = node
                .parent()
                .and_then(|node| AnyHasClosureNode::from_node(&node))
                .unwrap();
            model.closure(&node)
        } else {
            let node = r
                .syntax()
                .descendants()
                .filter(|x| x.kind() == JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
                .last()
                .unwrap()
                .cast::<JsArrowFunctionExpression>()
                .unwrap();
            model.closure(&node)
        };

        closure.children().collect()
    }

    #[test]
    pub fn ok_semantic_model_closure() {
        assert_closure("function f() {}", "f", &[]);

        let two_captures = "let a, b; function f(c) {console.log(a, b, c)}";
        assert_closure(two_captures, "f", &["a", "b"]);
        assert_eq!(get_closure_children(two_captures, "f").len(), 0);

        let inner_function = "let a, b;
        function f(c) {
            console.log(a);
            function g() {
                console.log(b, c);
            }
        }";
        assert_closure(inner_function, "f", &["a"]);
        assert_closure(inner_function, "g", &["b", "c"]);
        assert_eq!(get_closure_children(inner_function, "f").len(), 1);
        assert_eq!(get_closure_children(inner_function, "g").len(), 0);

        let arrow_function = "let a, b;
        function f(c) {
            console.log(a);
            c.map(x => x + b + c);
        }";
        assert_closure(arrow_function, "f", &["a"]);
        assert_closure(arrow_function, "ARROWFUNCTION", &["b", "c"]);
        assert_eq!(get_closure_children(arrow_function, "f").len(), 1);
        assert_eq!(
            get_closure_children(arrow_function, "ARROWFUNCTION").len(),
            0
        );

        let writes = "let a;
        function f(c) {
            a = 1
        }";
        assert_closure(writes, "f", &["a"]);

        let class_callables = "let a;
        class A {
            constructor() { console.log(a); }
            f() { console.log(a); }

            get getValue() { console.log(a); }
            set setValue(v) { console.log(a); }
        }";
        assert_closure(class_callables, "constructor", &["a"]);
        assert_closure(class_callables, "f", &["a"]);
        assert_closure(class_callables, "getValue", &["a"]);
        assert_closure(class_callables, "setValue", &["a"]);

        let object_callables = "let a;
        let a = {
            f() { console.log(a); }

            get getValue() { console.log(a); }
            set setValue(v) { console.log(a); }
        }";
        assert_closure(object_callables, "f", &["a"]);
        assert_closure(object_callables, "getValue", &["a"]);
        assert_closure(object_callables, "setValue", &["a"]);
    }
}
