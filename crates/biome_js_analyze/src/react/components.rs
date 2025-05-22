use biome_js_syntax::export_ext::{AnyJsExported, ExportedItem};
use biome_js_syntax::{
    AnyJsBinding, AnyJsCallArgument, AnyJsExpression, JsArrowFunctionExpression,
    JsAssignmentExpression, JsClassDeclaration, JsClassExportDefaultDeclaration,
    JsExportDefaultExpressionClause, JsExtendsClause, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsFunctionExpression, JsLanguage, JsSyntaxToken,
    JsVariableDeclarator,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNode, TextRange, declare_node_union};

/// React Function components may have no more than one parameter (props).
/// Parameter count in case of wrapped components (in memo or forwardRef) is not checked.
const REACT_COMPONENT_PARAMS_LIMIT: usize = 1;

/// Represents information about a React component.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ReactComponentInfo {
    /// Range of the component defintion start token.
    /// This can be used for diagnostics in the absence of the names.
    pub(crate) start_range: TextRange,
    /// Name of the component.
    pub(crate) name: Option<JsSyntaxToken>,
    /// Name of the function/class expression. It is not exposed outside of that function/class,
    /// but can be used to name the component for the user in the absence of a name.
    pub(crate) name_hint: Option<JsSyntaxToken>,
    /// Kind of the component.
    pub(crate) kind: ReactComponentKind,
}

impl ReactComponentInfo {
    /// Returns the declaration highlight range.
    /// If name or name_hint is not present, it will return the start range.
    pub(crate) fn declaration_highlight_range(&self) -> TextRange {
        if let Some(name) = &self.name {
            name.text_trimmed_range()
        } else if let Some(name_hint) = &self.name_hint {
            name_hint.text_trimmed_range()
        } else {
            self.start_range
        }
    }

    fn from_any_js_function_declaration(
        function_declaration: &AnyJsFunctionDeclaration,
    ) -> Option<Self> {
        let id = function_declaration.id()?;
        let identifier = id.as_js_identifier_binding()?;
        let name = identifier.name_token().ok()?;

        if function_declaration.param_count()? > REACT_COMPONENT_PARAMS_LIMIT {
            return None;
        }

        if is_react_component_name(name.text_trimmed()) {
            Some(Self {
                name: Some(name),
                name_hint: None,
                start_range: function_declaration.start_range(),
                kind: ReactComponentKind::Function(ReactFunctionComponentInfo {
                    wrappers: Box::new([]),
                }),
            })
        } else {
            None
        }
    }

    fn from_any_js_class_declaration(class_declaration: &AnyJsClassDeclaration) -> Option<Self> {
        let super_class =
            ReactSuperClass::from_extends_clause(&class_declaration.extends_clause()?)?;
        let name = class_declaration
            .id()?
            .as_js_identifier_binding()?
            .name_token()
            .ok();

        Some(Self {
            start_range: class_declaration.start_range(),
            name,
            name_hint: None,
            kind: ReactComponentKind::Class(ReactClassComponentInfo { super_class }),
        })
    }

    /// Creates a `ReactComponentInfo` from a declaration.
    /// Declarations can be:
    /// - Function declaration
    /// - Class declaration
    /// - Variable declarator
    /// - Assignment expression
    pub(crate) fn from_declaration(syntax: &SyntaxNode<JsLanguage>) -> Option<Self> {
        if let Some(function_declaration) = AnyJsFunctionDeclaration::cast_ref(syntax) {
            return Self::from_any_js_function_declaration(&function_declaration);
        }
        if let Some(class_declaration) = AnyJsClassDeclaration::cast_ref(syntax) {
            return Self::from_any_js_class_declaration(&class_declaration);
        }
        if let Some(variable_declarator) = JsVariableDeclarator::cast_ref(syntax) {
            let name = variable_declarator
                .id()
                .ok()?
                .as_any_js_binding()?
                .as_js_identifier_binding()?
                .name_token()
                .ok()?;

            let mut result = Self::from_expression(
                variable_declarator
                    .initializer()?
                    .expression()
                    .ok()?
                    .syntax(),
            )?;
            if !is_react_component_name(name.text_trimmed()) {
                return None;
            }
            result.name = Some(name);
            return Some(result);
        }
        if let Some(assignment) = JsAssignmentExpression::cast_ref(syntax) {
            let name = assignment
                .left()
                .ok()?
                .as_any_js_assignment()?
                .as_js_identifier_assignment()?
                .name_token()
                .ok()?;

            if !is_react_component_name(name.text_trimmed()) {
                return None;
            }

            let mut result = Self::from_expression(assignment.right().ok()?.syntax())?;
            result.name = Some(name);
            return Some(result);
        }
        if let Some(export_default) = JsExportDefaultExpressionClause::cast_ref(syntax) {
            if let Some(result) = Self::from_expression(export_default.expression().ok()?.syntax())
            {
                if let ReactComponentKind::Function(function_info) = &result.kind {
                    // If the function is not wrapped in memo or forwardRef and has no name,
                    // we can't be sure that it's a React component.
                    if function_info.wrappers.is_empty() {
                        return None;
                    }
                }
                return Some(result);
            }
        }
        None
    }

    /// Creates a `ReactComponentInfo` from an expression.
    /// It is not guaranteed that the expression is a React component,
    /// but if any reqiuirements are not met, it will return `None`.
    /// Never returns a name, can only return a name hint.
    pub(crate) fn from_expression(syntax: &SyntaxNode<JsLanguage>) -> Option<Self> {
        let any_expression = AnyJsExpression::cast_ref(syntax)?;
        // Classes can be expressions too. Example:
        //     const MyComponent = class extends React.Component {};
        if let AnyJsExpression::JsClassExpression(class_expression) = &any_expression {
            let extends_clause = class_expression.extends_clause()?;
            let super_class = ReactSuperClass::from_extends_clause(&extends_clause)?;
            return Some(Self {
                name: None,
                name_hint: class_expression.id().and_then(|id| {
                    id.as_js_identifier_binding()
                        .and_then(|id_binding| id_binding.name_token().ok())
                }),
                start_range: class_expression.class_token().map_or_else(
                    |_| class_expression.range(),
                    |class_token| class_token.text_range(),
                ),
                kind: ReactComponentKind::Class(ReactClassComponentInfo { super_class }),
            });
        }

        // Trying to handle as a React function component.

        let mut expression = any_expression;
        let mut wrappers = Vec::<ReactFunctionComponentWrapper>::new();

        // Check if the expression is wrapped in memo or forwardRef.
        while let AnyJsExpression::JsCallExpression(call) = &expression {
            let args = call.arguments().ok()?.args();
            // Both memo and forwardRef take one argument.
            if args.len() != 1 {
                return None;
            }

            let callee_name = call.callee().ok()?.get_callee_member_name()?;
            let callee_member_name = callee_name.text_trimmed();
            if callee_member_name == "memo" {
                wrappers.push(ReactFunctionComponentWrapper::Memo);
            } else if callee_member_name == "forwardRef" {
                wrappers.push(ReactFunctionComponentWrapper::ForwardRef);
            } else {
                return None;
            }

            // There can be multiple wrappers, e.g. memo(forwardRef(...)).
            if let Some(Ok(AnyJsCallArgument::AnyJsExpression(call_expression))) = args.first() {
                expression = call_expression
            }
        }

        let mut name_hint = None;

        let function_expression = AnyJsFunctionExpression::cast_ref(expression.syntax());

        if wrappers.is_empty() && function_expression.is_none() {
            // We have no evidence that this is React component.
            return None;
        }

        // If the expressionn is not wrapped in memo or forwardRef,
        // check if it's a function expression.
        if wrappers.is_empty() {
            let function_expression = function_expression?;
            if function_expression.param_count()? > REACT_COMPONENT_PARAMS_LIMIT {
                return None;
            }
            name_hint = function_expression.id().and_then(|id| {
                id.as_js_identifier_binding()
                    .and_then(|id_binding| id_binding.name_token().ok())
            });
        };

        if let Some(identifier) = expression.as_js_identifier_expression() {
            name_hint = identifier
                .name()
                .ok()
                .and_then(|name| name.value_token().ok());
        }

        Some(Self {
            name: None,
            name_hint,
            start_range: expression.syntax().first_token()?.text_range(),
            kind: ReactComponentKind::Function(ReactFunctionComponentInfo {
                wrappers: wrappers.into_boxed_slice(),
            }),
        })
    }

    /// Creates a `ReactComponentInfo` from an exported item.
    pub(crate) fn from_exported_item(item: &ExportedItem) -> Option<Self> {
        let exported = item.exported.as_ref()?;
        match exported {
            AnyJsExported::AnyJsExpression(expression) => {
                let mut result = Self::from_expression(expression.syntax())?;
                if let Some(ident) = item.identifier.as_ref() {
                    if let Some(name) = ident.name_token() {
                        if !is_react_component_name(name.text_trimmed()) {
                            return None;
                        }
                        result.name = Some(name);
                    }
                }
                Some(result)
            }
            AnyJsExported::JsFunctionDeclaration(decl) => Self::from_any_js_function_declaration(
                &AnyJsFunctionDeclaration::JsFunctionDeclaration(decl.clone()),
            ),
            AnyJsExported::JsFunctionExportDefaultDeclaration(decl) => {
                Self::from_any_js_function_declaration(
                    &AnyJsFunctionDeclaration::JsFunctionExportDefaultDeclaration(decl.clone()),
                )
            }
            AnyJsExported::JsClassExportDefaultDeclaration(decl) => {
                Self::from_any_js_class_declaration(
                    &AnyJsClassDeclaration::JsClassExportDefaultDeclaration(decl.clone()),
                )
            }
            AnyJsExported::JsClassDeclaration(decl) => Self::from_any_js_class_declaration(
                &AnyJsClassDeclaration::JsClassDeclaration(decl.clone()),
            ),
            _ => None,
        }
    }
}

declare_node_union! {
    AnyJsFunctionDeclaration = JsFunctionExportDefaultDeclaration | JsFunctionDeclaration
}

impl AnyJsClassDeclaration {
    fn id(&self) -> Option<AnyJsBinding> {
        match self {
            Self::JsClassExportDefaultDeclaration(decl) => decl.id(),
            Self::JsClassDeclaration(decl) => decl.id().ok(),
        }
    }

    fn extends_clause(&self) -> Option<JsExtendsClause> {
        match self {
            Self::JsClassExportDefaultDeclaration(decl) => decl.extends_clause(),
            Self::JsClassDeclaration(decl) => decl.extends_clause(),
        }
    }

    fn start_range(&self) -> TextRange {
        (match self {
            Self::JsClassExportDefaultDeclaration(decl) => decl.class_token(),
            Self::JsClassDeclaration(decl) => decl.class_token(),
        })
        .map(|token| token.text_range())
        .unwrap_or(self.range())
    }
}

declare_node_union! {
    AnyJsFunctionExpression = JsFunctionExpression | JsArrowFunctionExpression
}

/// Represents the kind of React component.
/// It can be either a function component or a class component.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ReactComponentKind {
    Function(ReactFunctionComponentInfo),
    Class(ReactClassComponentInfo),
}

impl AnyJsFunctionExpression {
    fn id(&self) -> Option<AnyJsBinding> {
        match self {
            Self::JsFunctionExpression(expr) => expr.id(),
            Self::JsArrowFunctionExpression(_) => None,
        }
    }

    fn param_count(&self) -> Option<usize> {
        match self {
            Self::JsFunctionExpression(expr) => Some(expr.parameters().ok()?.items().len()),
            Self::JsArrowFunctionExpression(expr) => {
                Some(expr.parameters().ok()?.as_js_parameters()?.items().len())
            }
        }
    }
}

declare_node_union! {
    AnyJsClassDeclaration = JsClassExportDefaultDeclaration | JsClassDeclaration
}

impl AnyJsFunctionDeclaration {
    fn id(&self) -> Option<AnyJsBinding> {
        match self {
            Self::JsFunctionExportDefaultDeclaration(decl) => decl.id(),
            Self::JsFunctionDeclaration(decl) => decl.id().ok(),
        }
    }

    fn param_count(&self) -> Option<usize> {
        let parameters = match self {
            Self::JsFunctionExportDefaultDeclaration(decl) => decl.parameters(),
            Self::JsFunctionDeclaration(decl) => decl.parameters(),
        };
        parameters.ok().map(|params| params.items().len())
    }

    fn start_range(&self) -> TextRange {
        (match self {
            Self::JsFunctionDeclaration(decl) => decl.function_token(),
            Self::JsFunctionExportDefaultDeclaration(decl) => decl.function_token(),
        })
        .map_or_else(|_| self.range(), |token| token.text_range())
    }
}

/// Checks whether the given function name belongs to a React component, based
/// on the official convention for React component naming: React component names
/// must start with a capital letter.
pub(crate) fn is_react_component_name(name: &str) -> bool {
    name.chars().next().is_some_and(char::is_uppercase)
}

declare_node_union! {
    pub AnyPotentialReactComponentDeclaration = JsClassExportDefaultDeclaration | JsFunctionExportDefaultDeclaration | JsFunctionDeclaration | JsVariableDeclarator | JsAssignmentExpression | JsExportDefaultExpressionClause
}

/// Represents a React function component.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ReactFunctionComponentInfo {
    /// List of wrappers that was used to wrap the component.
    pub(crate) wrappers: Box<[ReactFunctionComponentWrapper]>,
}

/// Represents a React class component.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ReactClassComponentInfo {
    /// Base class of the component.
    pub(crate) super_class: ReactSuperClass,
}

/// Represents the wrappers that can be used to wrap a React function component.
/// One component can be wrapped in multiple wrappers.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ReactFunctionComponentWrapper {
    /// Wrapped using `React.memo()`.
    Memo,
    /// Wrapped using `React.forwardRef()`.
    ForwardRef,
}

/// Represents the super class of a React class component.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ReactSuperClass {
    Component,
    PureComponent,
}

impl ReactSuperClass {
    /// Checks if the given `extends_clause` is a React component class.
    /// This check isn't precise since it doesn't check that those components were
    /// actually imported from React/React-like library.
    /// This might be improved in the future if required.
    fn from_extends_clause(extends_clause: &JsExtendsClause) -> Option<Self> {
        let super_class = extends_clause.super_class().ok()?;
        let super_class_member = super_class.get_callee_member_name()?;
        let class_name = super_class_member.text_trimmed();
        if class_name == "Component" {
            Some(Self::Component)
        } else if class_name == "PureComponent" {
            Some(Self::PureComponent)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use biome_js_parser::{JsParserOptions, Parse, parse};
    use biome_js_syntax::{AnyJsRoot, JsFileSource};

    fn parse_jsx(code: &str) -> Parse<AnyJsRoot> {
        let source = parse(code, JsFileSource::jsx(), JsParserOptions::default());

        if source.has_errors() {
            panic!("syntax error")
        }

        source
    }

    mod from_expression {
        use super::*;
        use biome_js_syntax::JsInitializerClause;
        use biome_rowan::AstNode;

        #[test]
        fn it_should_handle_function_expressions() {
            let source = parse_jsx(
                r#"
                    function test() {
                        const func1 = function() {
                            return <div>Hello, world!</div>;
                        };

                        const func2 = function ComponentName() {
                            return <div>Hello, world!</div>;
                        };

                        const arrow1 = () => {
                            return <div>Hello, world!</div>;
                        };

                        const arrow2 = (props) => {
                            return <div>Hello, world!</div>;
                        };

                        const tooManyParams = (props, ref) => {
                            return <div>Hello, world!</div>;
                        };
                    }
                "#,
            );

            let function_expressions = source
                .syntax()
                .descendants()
                .filter_map(AnyJsExpression::cast)
                .filter(|expr| {
                    matches!(
                        expr,
                        AnyJsExpression::JsFunctionExpression(_)
                            | AnyJsExpression::JsArrowFunctionExpression(_)
                    )
                })
                .collect::<Vec<_>>();

            assert_eq!(function_expressions.len(), 5);

            // Test each expression
            let info1 = ReactComponentInfo::from_expression(function_expressions[0].syntax());
            match info1 {
                Some(info) => {
                    assert_eq!(info.name, None);
                    assert_eq!(info.name_hint, None);
                    if let ReactComponentKind::Function(func_info) = info.kind {
                        assert_eq!(func_info.wrappers.len(), 0);
                    } else {
                        panic!("Expected function component");
                    }
                }
                None => panic!("Expected function component"),
            }

            let info2 = ReactComponentInfo::from_expression(function_expressions[1].syntax());
            let name_hint = function_expressions[1]
                .as_js_function_expression()
                .unwrap()
                .id()
                .unwrap()
                .as_js_identifier_binding()
                .unwrap()
                .name_token()
                .ok();
            match info2 {
                Some(info) => {
                    assert_eq!(info.name, None);
                    assert_eq!(info.name_hint, name_hint);
                    if let ReactComponentKind::Function(func_info) = info.kind {
                        assert_eq!(func_info.wrappers.len(), 0);
                    } else {
                        panic!("Expected function component");
                    }
                }
                None => panic!("Expected function component"),
            }

            let info3 = ReactComponentInfo::from_expression(function_expressions[2].syntax());
            match info3 {
                Some(info) => {
                    assert_eq!(info.name, None);
                    assert_eq!(info.name_hint, None);
                    if let ReactComponentKind::Function(func_info) = info.kind {
                        assert_eq!(func_info.wrappers.len(), 0);
                    } else {
                        panic!("Expected function component");
                    }
                }
                None => panic!("Expected function component"),
            }

            let info4 = ReactComponentInfo::from_expression(function_expressions[3].syntax());
            match info4 {
                Some(info) => {
                    assert_eq!(info.name, None);
                    assert_eq!(info.name_hint, None);
                    if let ReactComponentKind::Function(func_info) = info.kind {
                        assert_eq!(func_info.wrappers.len(), 0);
                    } else {
                        panic!("Expected function component");
                    }
                }
                None => panic!("Expected function component"),
            }

            // Too many parameters, should return None
            let info5 = ReactComponentInfo::from_expression(function_expressions[4].syntax());
            assert!(info5.is_none());
        }

        #[test]
        fn it_should_handle_wrapped_function_expressions() {
            let source = parse_jsx(
                r#"
                    function test() {
                        const memo1 = memo(function() {
                            return <div>Hello, world!</div>;
                        });

                        const memo2 = React.memo(() => {
                            return <div>Hello, world!</div>;
                        });

                        const forwardRef1 = forwardRef((props, ref) => {
                            return <div>Hello, world!</div>;
                        });

                        const forwardRef2 = React.forwardRef((props, ref) => {
                            return <div>Hello, world!</div>;
                        });

                        const double = memo(forwardRef((props, ref) => {
                            return <div>Hello, world!</div>;
                        }));

                        const nested = React.memo(React.forwardRef((props, ref) => {
                            return <div>Hello, world!</div>;
                        }));

                        // Invalid wrappers
                        const invalid1 = someOtherWrapper(() => {
                            return <div>Hello, world!</div>;
                        });

                        const invalid2 = memo(() => {}, extraArg);
                    }
                "#,
            );

            let call_expressions = source
                .syntax()
                .descendants()
                .filter_map(|syntax| {
                    if let Some(expr) = AnyJsExpression::cast_ref(&syntax) {
                        if expr.parent::<JsInitializerClause>().is_some() {
                            return Some(expr);
                        }
                    }
                    None
                })
                .collect::<Vec<_>>();

            // Test memo wrapper
            let info1 = ReactComponentInfo::from_expression(call_expressions[0].syntax());
            assert!(info1.is_some());
            let info1 = info1.unwrap();
            assert_eq!(info1.name, None);
            if let ReactComponentKind::Function(func_info) = info1.kind {
                assert_eq!(func_info.wrappers.len(), 1);
                assert_eq!(func_info.wrappers[0], ReactFunctionComponentWrapper::Memo);
            } else {
                panic!("Expected function component");
            }

            // Test React.memo wrapper
            let info2 = ReactComponentInfo::from_expression(call_expressions[1].syntax());
            assert!(info2.is_some());
            let info2 = info2.unwrap();
            if let ReactComponentKind::Function(func_info) = info2.kind {
                assert_eq!(func_info.wrappers.len(), 1);
                assert_eq!(func_info.wrappers[0], ReactFunctionComponentWrapper::Memo);
            } else {
                panic!("Expected function component");
            }

            // Test forwardRef wrapper
            let info3 = ReactComponentInfo::from_expression(call_expressions[2].syntax());
            assert!(info3.is_some());
            let info3 = info3.unwrap();
            if let ReactComponentKind::Function(func_info) = info3.kind {
                assert_eq!(func_info.wrappers.len(), 1);
                assert_eq!(
                    func_info.wrappers[0],
                    ReactFunctionComponentWrapper::ForwardRef
                );
            } else {
                panic!("Expected function component");
            }

            // Test React.forwardRef wrapper
            let info4 = ReactComponentInfo::from_expression(call_expressions[3].syntax());
            assert!(info4.is_some());
            let info4 = info4.unwrap();
            if let ReactComponentKind::Function(func_info) = info4.kind {
                assert_eq!(func_info.wrappers.len(), 1);
                assert_eq!(
                    func_info.wrappers[0],
                    ReactFunctionComponentWrapper::ForwardRef
                );
            } else {
                panic!("Expected function component");
            }

            // Test memo(forwardRef()) double wrapper
            let info5 = ReactComponentInfo::from_expression(call_expressions[4].syntax());
            assert!(info5.is_some());
            let info5 = info5.unwrap();
            if let ReactComponentKind::Function(func_info) = info5.kind {
                assert_eq!(func_info.wrappers.len(), 2);
                assert_eq!(func_info.wrappers[0], ReactFunctionComponentWrapper::Memo);
                assert_eq!(
                    func_info.wrappers[1],
                    ReactFunctionComponentWrapper::ForwardRef
                );
            } else {
                panic!("Expected function component");
            }

            // Test React.memo(React.forwardRef()) double wrapper
            let info6 = ReactComponentInfo::from_expression(call_expressions[5].syntax());
            assert!(info6.is_some());
            let info6 = info6.unwrap();
            if let ReactComponentKind::Function(func_info) = info6.kind {
                assert_eq!(func_info.wrappers.len(), 2);
                assert_eq!(func_info.wrappers[0], ReactFunctionComponentWrapper::Memo);
                assert_eq!(
                    func_info.wrappers[1],
                    ReactFunctionComponentWrapper::ForwardRef
                );
            } else {
                panic!("Expected function component");
            }

            // Test invalid wrapper
            let info7 = ReactComponentInfo::from_expression(call_expressions[6].syntax());
            assert!(info7.is_none());

            // Test invalid memo (too many args)
            let info8 = ReactComponentInfo::from_expression(call_expressions[7].syntax());
            assert!(info8.is_none());
        }

        #[test]
        fn it_should_handle_class_expressions() {
            let source = parse_jsx(
                r#"
                    function test() {
                        const class1 = class extends React.Component {
                            render() {
                                return <div>Hello, world!</div>;
                            }
                        };

                        const class2 = class NamedComponent extends Component {
                            render() {
                                return <div>Hello, world!</div>;
                            }
                        };

                        const class3 = class extends React.PureComponent {
                            render() {
                                return <div>Hello, world!</div>;
                            }
                        };

                        const class4 = class extends PureComponent {
                            render() {
                                return <div>Hello, world!</div>;
                            }
                        };

                        // Invalid - not extending a React component
                        const invalid = class NonComponent {
                            render() {
                                return <div>Hello, world!</div>;
                            }
                        };
                    }
                "#,
            );

            let class_expressions = source
                .syntax()
                .descendants()
                .filter_map(|node| {
                    if let Some(expr) = AnyJsExpression::cast(node.clone()) {
                        if matches!(expr, AnyJsExpression::JsClassExpression(_)) {
                            return Some(expr);
                        }
                    }
                    None
                })
                .collect::<Vec<_>>();

            assert_eq!(class_expressions.len(), 5);

            // Test React.Component
            let info1 = ReactComponentInfo::from_expression(class_expressions[0].syntax());
            assert!(info1.is_some());
            let info1 = info1.unwrap();
            assert_eq!(info1.name, None);
            assert_eq!(info1.name_hint, None);
            if let ReactComponentKind::Class(class_info) = info1.kind {
                assert_eq!(class_info.super_class, ReactSuperClass::Component);
            } else {
                panic!("Expected class component");
            }

            // Test Component with name hint
            let info2 = ReactComponentInfo::from_expression(class_expressions[1].syntax());
            let name_hint = class_expressions[1]
                .as_js_class_expression()
                .unwrap()
                .id()
                .and_then(|id| {
                    id.as_js_identifier_binding()
                        .and_then(|id_binding| id_binding.name_token().ok())
                });
            assert!(info2.is_some());
            let info2 = info2.unwrap();
            assert_eq!(info2.name, None);
            assert_eq!(info2.name_hint, name_hint);
            if let ReactComponentKind::Class(class_info) = info2.kind {
                assert_eq!(class_info.super_class, ReactSuperClass::Component);
            } else {
                panic!("Expected class component");
            }

            // Test React.PureComponent
            let info3 = ReactComponentInfo::from_expression(class_expressions[2].syntax());
            assert!(info3.is_some());
            let info3 = info3.unwrap();
            if let ReactComponentKind::Class(class_info) = info3.kind {
                assert_eq!(class_info.super_class, ReactSuperClass::PureComponent);
            } else {
                panic!("Expected class component");
            }

            // Test PureComponent
            let info4 = ReactComponentInfo::from_expression(class_expressions[3].syntax());
            assert!(info4.is_some());
            let info4 = info4.unwrap();
            if let ReactComponentKind::Class(class_info) = info4.kind {
                assert_eq!(class_info.super_class, ReactSuperClass::PureComponent);
            } else {
                panic!("Expected class component");
            }

            // Test invalid class (not extending React component)
            let info5 = ReactComponentInfo::from_expression(class_expressions[4].syntax());
            assert!(info5.is_none());
        }
    }

    mod from_exported_item {
        use super::*;
        use biome_js_syntax::JsExport;
        use biome_rowan::AstNode;

        #[test]
        fn it_should_handle_exported_expressions() {
            let source = parse_jsx(
                r#"
                    export const MyComponent1 = function() {
                        return <div>Hello, world!</div>;
                    };

                    export const MyComponent2 = () => {
                        return <div>Hello, world!</div>;
                    };

                    export const MyComponent3 = memo(() => {
                        return <div>Hello, world!</div>;
                    });

                    export const MyComponent4 = class extends React.Component {
                        render() {
                            return <div>Hello, world!</div>;
                        }
                    };

                    // Invalid - lowercase name
                    export const myComponent5 = () => {
                        return <div>Hello, world!</div>;
                    };

                    // Another syntax
                    export { MyComponent1 as RenamedComponent };
                "#,
            );

            let exported_items = source
                .syntax()
                .descendants()
                .filter_map(JsExport::cast)
                .flat_map(|export| JsExport::get_exported_items(&export))
                .collect::<Vec<_>>();

            assert_eq!(exported_items.len(), 6);

            // Test each valid exported item
            for (index, item) in exported_items.iter().enumerate().take(4) {
                let component_info = ReactComponentInfo::from_exported_item(item);
                assert!(component_info.is_some(), "Failed on item {}", index);

                let component_info = component_info.unwrap();
                assert!(component_info.name.is_some());

                // Verify component kind based on index
                match index {
                    0 | 1 => {
                        // Function components
                        if let ReactComponentKind::Function(func_info) = &component_info.kind {
                            assert_eq!(func_info.wrappers.len(), 0);
                        } else {
                            panic!("Expected function component for index {}", index);
                        }
                    }
                    2 => {
                        // Memo wrapped function component
                        if let ReactComponentKind::Function(func_info) = &component_info.kind {
                            assert_eq!(func_info.wrappers.len(), 1);
                            assert_eq!(func_info.wrappers[0], ReactFunctionComponentWrapper::Memo);
                        } else {
                            panic!("Expected function component with Memo wrapper");
                        }
                    }
                    3 => {
                        // Class component
                        if let ReactComponentKind::Class(class_info) = &component_info.kind {
                            assert_eq!(class_info.super_class, ReactSuperClass::Component);
                        } else {
                            panic!("Expected class component");
                        }
                    }
                    _ => unreachable!(),
                }
            }

            // Test invalid exported component (lowercase name)
            assert_eq!(
                ReactComponentInfo::from_exported_item(&exported_items[4]),
                None
            );

            // Test unknown export
            assert_eq!(
                ReactComponentInfo::from_exported_item(&exported_items[5]),
                None
            );
        }

        #[test]
        fn it_should_handle_exported_declarations() {
            let source = parse_jsx(
                r#"
                    export function MyComponent1() {
                        return <div>Hello, world!</div>;
                    }

                    export function MyComponent2(props) {
                        return <div>Hello, {props.name}</div>;
                    }

                    // Invalid - lowercase name
                    export function myComponent5() {
                        return <div>Hello, world!</div>;
                    }

                    // Invalid - too many parameters
                    export function MyComponent6(props, context) {
                        return <div>Hello, world!</div>;
                    }

                    export default function MyComponent3() {
                        return <div>Hello, world!</div>;
                    }

                    export class MyComponent4 extends React.Component {
                        render() {
                            return <div>Hello, world!</div>;
                        }
                    }

                    // Invalid - not extending a React component
                    export class NonComponent7 {
                        render() {
                            return <div>Hello, world!</div>;
                        }
                    }
                "#,
            );

            // Find all exported declarations and create ExportedItem instances
            let exported_items = source
                .syntax()
                .descendants()
                .filter_map(JsExport::cast)
                .flat_map(|export| JsExport::get_exported_items(&export))
                .collect::<Vec<_>>();

            let exported_functions = exported_items
                .iter()
                .filter(|item| {
                    matches!(item.exported, Some(AnyJsExported::JsFunctionDeclaration(_)))
                })
                .collect::<Vec<_>>();

            // Test exported function declarations
            // MyComponent1, MyComponent2, myComponent5, MyComponent6
            assert_eq!(exported_functions.len(), 4);

            let component1 = ReactComponentInfo::from_exported_item(exported_functions[0]);
            assert!(component1.is_some());
            if let ReactComponentKind::Function(func_info) = component1.unwrap().kind {
                assert_eq!(func_info.wrappers.len(), 0);
            } else {
                panic!("Expected function component");
            }

            let component2 = ReactComponentInfo::from_exported_item(exported_functions[1]);
            assert!(component2.is_some());
            if let ReactComponentKind::Function(func_info) = component2.unwrap().kind {
                assert_eq!(func_info.wrappers.len(), 0);
            } else {
                panic!("Expected function component");
            }

            // Invalid - lowercase name
            assert_eq!(
                ReactComponentInfo::from_exported_item(exported_functions[2]),
                None
            );

            // Invalid - too many params
            assert_eq!(
                ReactComponentInfo::from_exported_item(exported_functions[3]),
                None
            );

            let exported_default_functions = exported_items
                .iter()
                .filter(|item| {
                    matches!(
                        item.exported,
                        Some(AnyJsExported::JsFunctionExportDefaultDeclaration(_))
                    )
                })
                .collect::<Vec<_>>();

            // Test exported default function declarations
            assert_eq!(exported_default_functions.len(), 1);

            let component4 = ReactComponentInfo::from_exported_item(exported_default_functions[0]);
            assert!(component4.is_some());
            if let ReactComponentKind::Function(func_info) = component4.unwrap().kind {
                assert_eq!(func_info.wrappers.len(), 0);
            } else {
                panic!("Expected function component");
            }

            let exported_classes = exported_items
                .iter()
                .filter(|item| matches!(item.exported, Some(AnyJsExported::JsClassDeclaration(_))))
                .collect::<Vec<_>>();

            // Test exported class declarations
            assert_eq!(exported_classes.len(), 2); // MyComponent4, NonComponent7

            let component5 = ReactComponentInfo::from_exported_item(exported_classes[0]);
            assert!(component5.is_some());
            if let ReactComponentKind::Class(class_info) = component5.unwrap().kind {
                assert_eq!(class_info.super_class, ReactSuperClass::Component);
            } else {
                panic!("Expected class component");
            }

            // Invalid - not extending a React component
            assert_eq!(
                ReactComponentInfo::from_exported_item(exported_classes[1]),
                None
            );
        }

        #[test]
        fn it_should_handle_default_class_export() {
            let source = parse_jsx(
                r#"
                    export default class MyComponent extends React.Component {
                        render() {
                            return <div>Hello, world!</div>;
                        }
                    }
                "#,
            );
            let exported_items = source
                .syntax()
                .descendants()
                .filter_map(JsExport::cast)
                .flat_map(|export| JsExport::get_exported_items(&export))
                .collect::<Vec<_>>();

            let exported_default_classes = exported_items
                .iter()
                .filter(|item| {
                    matches!(
                        item.exported,
                        Some(AnyJsExported::JsClassExportDefaultDeclaration(_))
                    )
                })
                .collect::<Vec<_>>();

            assert_eq!(exported_default_classes.len(), 1);

            let component = ReactComponentInfo::from_exported_item(exported_default_classes[0]);
            assert!(component.is_some());

            if let ReactComponentKind::Class(class_info) = component.unwrap().kind {
                assert_eq!(class_info.super_class, ReactSuperClass::Component);
            } else {
                panic!("Expected class component");
            }
        }
    }

    mod from_declaration {
        use super::*;
        use biome_rowan::AstNode;

        #[test]
        fn it_should_handle_valid_function_declarations() {
            let source = parse_jsx(
                r#"
                    function MyComponent1() {
                        return <div>Hello, world!</div>;
                    }

                    function MyComponent2(param) {
                        return <div>Hello, world!</div>;
                    }

                    export function MyComponent3(param) {
                        return <div>Hello, world!</div>;
                    }

                    export default function MyComponent4(param) {
                        return <div>Hello, world!</div>;
                    };
                "#,
            );

            let funcs = source
                .syntax()
                .descendants()
                .filter_map(AnyJsFunctionDeclaration::cast)
                .collect::<Vec<_>>();

            assert_eq!(funcs.len(), 4);

            for func in funcs {
                let component_info = ReactComponentInfo::from_declaration(func.syntax());
                assert_eq!(
                    component_info,
                    Some(ReactComponentInfo {
                        name: func
                            .id()
                            .unwrap()
                            .as_js_identifier_binding()
                            .unwrap()
                            .name_token()
                            .ok(),
                        name_hint: None,
                        start_range: func.start_range(),
                        kind: ReactComponentKind::Function(ReactFunctionComponentInfo {
                            wrappers: Box::new([]),
                        }),
                    })
                )
            }
        }

        #[test]
        fn it_should_ignore_invalid_function_declarations() {
            let source = parse_jsx(
                r#"
                    function myComponent1() {
                        return <div>Hello, world!</div>;
                    }

                    function myComponent2(param) {
                        return <div>Hello, world!</div>;
                    }

                    export function myComponent3(param) {
                        return <div>Hello, world!</div>;
                    }

                    function MyComponent4(param1, param2) {
                        return <div>Hello, world!</div>;
                    }

                    export default function myComponent5() {
                        return <div>Hello, world!</div>;
                    };
                "#,
            );

            let funcs = source
                .syntax()
                .descendants()
                .filter_map(AnyJsFunctionDeclaration::cast)
                .collect::<Vec<_>>();

            assert_eq!(funcs.len(), 5);

            for func in funcs {
                let component_info = ReactComponentInfo::from_declaration(func.syntax());
                assert_eq!(component_info, None)
            }
        }

        #[test]
        fn it_should_handle_valid_class_declarations() {
            let source = parse_jsx(
                r#"
                    class MyComponent1 extends React.Component {
                        render() {
                            return <div>Hello, world!</div>;
                        }
                    }

                    class MyComponent2 extends Component {
                        render() {
                            return <div>Hello, world!</div>;
                        }
                    }

                    class MyComponent3 extends React.PureComponent {
                        render() {
                            return <div>Hello, world!</div>;
                        }
                    }

                    class MyComponent4 extends PureComponent {
                        render() {
                            return <div>Hello, world!</div>;
                        }
                    }

                    export class MyComponent5 extends React.Component {
                        render() {
                            return <div>Hello, world!</div>;
                        }
                    }

                    export default class MyComponent6 extends React.Component {
                        render() {
                            return <div>Hello, world!</div>;
                        }
                    }
                "#,
            );

            let class_declarations = source
                .syntax()
                .descendants()
                .filter_map(AnyJsClassDeclaration::cast)
                .collect::<Vec<_>>();

            assert_eq!(class_declarations.len(), 6);

            for (index, class_declaration) in class_declarations.iter().enumerate() {
                let component_info =
                    ReactComponentInfo::from_declaration(class_declaration.syntax());
                assert_eq!(
                    component_info,
                    Some(ReactComponentInfo {
                        name: class_declaration
                            .id()
                            .unwrap()
                            .as_js_identifier_binding()
                            .unwrap()
                            .name_token()
                            .ok(),
                        name_hint: None,
                        start_range: class_declaration.start_range(),
                        kind: ReactComponentKind::Class(ReactClassComponentInfo {
                            super_class: match index {
                                2 | 3 => ReactSuperClass::PureComponent,
                                _ => ReactSuperClass::Component,
                            }
                        })
                    })
                )
            }
        }

        #[test]
        fn it_should_ignore_invalid_class_declarations() {
            let source = parse_jsx(
                r#"
                    class NonComponent1 {}
                    export class NonComponent2 {}
                "#,
            );

            let class_declarations = source
                .syntax()
                .descendants()
                .filter_map(AnyJsClassDeclaration::cast)
                .collect::<Vec<_>>();

            assert_eq!(class_declarations.len(), 2);

            for class_declaration in class_declarations.iter() {
                let component_info =
                    ReactComponentInfo::from_declaration(class_declaration.syntax());
                assert_eq!(component_info, None)
            }
        }

        #[test]
        fn it_should_handle_valid_variable_declarations() {
            let source = parse_jsx(
                r#"
                    const MyComponent1 = function() {
                        return <div>Hello, world!</div>;
                    };

                    const MyComponent2 = function Component(param) {
                        return <div>Hello, world!</div>;
                    };

                    const MyComponent3 = () => {
                        return <div>Hello, world!</div>;
                    };

                    export const MyComponent4 = function() {
                        return <div>Hello, world!</div>;
                    };

                    const MyComponent5 = memo((param) => {
                        return <div>Hello, world!</div>;
                    });

                    const MyComponent6 = React.memo((param) => {
                        return <div>Hello, world!</div>;
                    });

                    const MyComponent7 = forwardRef((param) => {
                        return <div>Hello, world!</div>;
                    });

                    const MyComponent8 = React.forwardRef((param) => {
                        return <div>Hello, world!</div>;
                    });

                    const MyComponent9 = memo(forwardRef((param) => {
                        return <div>Hello, world!</div>;
                    }));

                    export const MyComponent10 = React.memo(React.forwardRef((param) => {
                        return <div>Hello, world!</div>;
                    }));
                "#,
            );

            let variable_declarators = source
                .syntax()
                .descendants()
                .filter_map(JsVariableDeclarator::cast)
                .collect::<Vec<_>>();

            assert_eq!(variable_declarators.len(), 10);

            for (index, decl) in variable_declarators.iter().enumerate() {
                let Some(component_info) = ReactComponentInfo::from_declaration(decl.syntax())
                else {
                    panic!("Failed to get component info.");
                };
                assert_eq!(
                    component_info,
                    ReactComponentInfo {
                        name: decl
                            .id()
                            .unwrap()
                            .as_any_js_binding()
                            .unwrap()
                            .as_js_identifier_binding()
                            .unwrap()
                            .name_token()
                            .ok(),
                        name_hint: if index == 1 {
                            decl.initializer()
                                .unwrap()
                                .expression()
                                .unwrap()
                                .as_js_function_expression()
                                .unwrap()
                                .id()
                                .unwrap()
                                .as_js_identifier_binding()
                                .unwrap()
                                .name_token()
                                .ok()
                        } else {
                            None
                        },
                        start_range: component_info.start_range,
                        kind: ReactComponentKind::Function(ReactFunctionComponentInfo {
                            wrappers: match index {
                                4 | 5 => Box::new([ReactFunctionComponentWrapper::Memo]),
                                6 | 7 => Box::new([ReactFunctionComponentWrapper::ForwardRef]),
                                8 | 9 => Box::new([
                                    ReactFunctionComponentWrapper::Memo,
                                    ReactFunctionComponentWrapper::ForwardRef,
                                ]),
                                _ => Box::new([]),
                            },
                        }),
                    }
                )
            }
        }

        #[test]
        fn it_should_ignore_invalid_variable_declarations() {
            let source = parse_jsx(
                r#"
                    const myComponent1 = function() {
                        return <div>Hello, world!</div>;
                    };

                    const myComponent2 = function(param) {
                        return <div>Hello, world!</div>;
                    };

                    const myComponent3 = () => {
                        return <div>Hello, world!</div>;
                    };

                    const myComponent4 = (param1, param2) => {
                        return <div>Hello, world!</div>;
                    };

                    export const myComponent5 = function() {
                        return <div>Hello, world!</div>;
                    };

                    const myComponent6 = memo((param) => {
                        return <div>Hello, world!</div>;
                    });

                    const myComponent7 = React.memo((param) => {
                        return <div>Hello, world!</div>;
                    });

                    const my_component_8 = forwardRef((param) => {
                        return <div>Hello, world!</div>;
                    });

                    const myComponent9 = React.forwardRef((param) => {
                        return <div>Hello, world!</div>;
                    });

                    const myComponent10 = memo(forwardRef((param) => {
                        return <div>Hello, world!</div>;
                    }));

                    export const myComponent11 = React.memo(React.forwardRef((param) => {
                        return <div>Hello, world!</div>;
                    }));

                    const MyComponent12 = class NonComponent {};
                "#,
            );

            let variable_declarators = source
                .syntax()
                .descendants()
                .filter_map(JsVariableDeclarator::cast)
                .collect::<Vec<_>>();

            assert_eq!(variable_declarators.len(), 12);

            for decl in variable_declarators.iter() {
                let component_info = ReactComponentInfo::from_declaration(decl.syntax());
                assert_eq!(component_info, None)
            }
        }

        #[test]
        fn it_should_handle_valid_assignments() {
            let source = parse_jsx(
                r#"
                    let MyComponent;

                    MyComponent = function() {
                        return <div>Hello, world!</div>;
                    };
                "#,
            );

            let assignments = source
                .syntax()
                .descendants()
                .filter_map(JsAssignmentExpression::cast)
                .collect::<Vec<_>>();

            assert_eq!(assignments.len(), 1);

            for assignment in assignments {
                let Some(component_info) =
                    ReactComponentInfo::from_declaration(assignment.syntax())
                else {
                    panic!("Failed to get component info.");
                };
                assert_eq!(
                    component_info,
                    ReactComponentInfo {
                        name: assignment
                            .left()
                            .unwrap()
                            .as_any_js_assignment()
                            .unwrap()
                            .as_js_identifier_assignment()
                            .unwrap()
                            .name_token()
                            .ok(),
                        name_hint: None,
                        start_range: component_info.start_range,
                        kind: ReactComponentKind::Function(ReactFunctionComponentInfo {
                            wrappers: Box::new([])
                        }),
                    }
                )
            }
        }

        #[test]
        fn it_should_ignore_invalid_assignments() {
            let source = parse_jsx(
                r#"
                    let NonComponent;

                    NonComponent = 123;
                "#,
            );

            let assignments = source
                .syntax()
                .descendants()
                .filter_map(JsAssignmentExpression::cast)
                .collect::<Vec<_>>();

            assert_eq!(assignments.len(), 1);

            for assignment in assignments {
                let component_info = ReactComponentInfo::from_declaration(assignment.syntax());
                assert_eq!(component_info, None)
            }
        }
    }
}
