use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBinding, AnyJsBindingPattern,
    AnyJsObjectBindingPatternMember, JsArrowFunctionExpression, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsFunctionExpression, JsLanguage, JsObjectBindingPattern,
    JsParameters, JsVariableDeclarator, JsxExpressionAttributeValue, JsxExpressionChild,
};
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListNodesIterator, TextRange};
use biome_rule_options::no_solid_destructured_props::NoSolidDestructuredPropsOptions;
use biome_string_case::Case;
use std::collections::VecDeque;
use std::iter::FusedIterator;

declare_lint_rule! {
    /// Disallow destructuring props inside JSX components in Solid projects.
    ///
    /// In Solid, props must be used with property accesses (props.foo) to preserve reactivity.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// let Component = ({}) => <div />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// let Component = ({ a: A }) => <div a={A} />;
    /// ```
    ///
    /// ```tsx,expect_diagnostic
    /// let Component = ({ prop1 }: Props) => <div p1={prop1} />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// let Component = (props) => <div />;
    /// ```
    ///
    /// ```jsx
    /// let Component = (props) => <div a={props.a} />;
    /// ```
    ///
    pub NoSolidDestructuredProps {
        version: "2.0.0",
        name: "noSolidDestructuredProps",
        language: "js",
        domains: &[RuleDomain::Solid],
        recommended: false,
        sources: &[RuleSource::EslintSolid("no-destructure").inspired()],
    }
}

pub enum Violation {
    EmptyBinding(TextRange),
    WithProps(TextRange),
}

impl Violation {
    fn range(&self) -> TextRange {
        match self {
            Self::EmptyBinding(range) => *range,
            Self::WithProps(range) => *range,
        }
    }

    fn message(&self) -> &str {
        match self {
            Self::EmptyBinding(_) => "You cannot destructure props.",
            Self::WithProps(_) => "This variable shouldn't be destructured.",
        }
    }
}

impl Rule for NoSolidDestructuredProps {
    type Query = Semantic<JsObjectBindingPattern>;
    type State = Violation;
    type Signals = Vec<Self::State>;
    type Options = NoSolidDestructuredPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding_pattern = ctx.query();
        let model = ctx.model();
        let Some(parameters) = binding_pattern
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(JsParameters::cast)
        else {
            return vec![];
        };

        // In solid, a component can't accept more than one property
        if parameters.items().len() > 1 {
            return vec![];
        }
        let mut bindings = vec![];

        if is_inside_jsx_component(&parameters).unwrap_or_default() {
            let properties = binding_pattern.properties();
            if properties.len() == 0 {
                bindings.push(Violation::EmptyBinding(binding_pattern.range()));
            } else {
                let iter = BindingPatterIterator::new(BindingPatternLikeList::Object(
                    binding_pattern.properties().iter(),
                ));
                for binding in iter {
                    if let Some(range) = is_binding_used_in_jsx(&binding, model) {
                        bindings.push(Violation::WithProps(range))
                    }
                }
            }
        }

        bindings
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diagnostic = if matches!(state, Violation::EmptyBinding(_)) {
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    { state.message()}
                },
            )
        } else {
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    { state.message()}
                },
            )
            .detail(
                node.range(),
                markup! {
                    "This is where the props were destructured."
                },
            )
        };
        Some(
            diagnostic
            .note(
                markup!{
                    "In Solid, props must be used with property accesses (props.foo) to preserve reactivity."
                }
            ).note(
                markup!{
                    "Remove the destructuring and use props.foo instead."
                })
        )
    }
}

fn is_inside_jsx_component(parameters: &JsParameters) -> Option<bool> {
    let parent = parameters.syntax().parent()?;

    if let Some(arrow_function_expression) = JsArrowFunctionExpression::cast(parent.clone()) {
        let variable_declarator = arrow_function_expression
            .syntax()
            .grand_parent()
            .and_then(JsVariableDeclarator::cast)?;

        return is_variable_declarator_pascal_case(&variable_declarator);
    }

    if let Some(function_declaration) = JsFunctionDeclaration::cast(parent.clone()) {
        let id = function_declaration.id().ok()?;
        let id = id.as_js_identifier_binding()?;
        let text = id.name_token().ok()?;

        return Some(is_pascal_case(text.text_trimmed()));
    }

    if let Some(function_declaration) = JsFunctionExportDefaultDeclaration::cast(parent.clone()) {
        let Some(id) = function_declaration.id() else {
            return Some(true);
        };
        let id = id.as_js_identifier_binding()?;
        let text = id.name_token().ok()?;

        return Some(is_pascal_case(text.text_trimmed()));
    }

    if let Some(function_expression) = JsFunctionExpression::cast(parent) {
        let variable_declarator = function_expression
            .syntax()
            .grand_parent()
            .and_then(JsVariableDeclarator::cast)?;

        return is_variable_declarator_pascal_case(&variable_declarator);
    }

    None
}

fn is_variable_declarator_pascal_case(variable_declarator: &JsVariableDeclarator) -> Option<bool> {
    let name = variable_declarator.id().ok()?;
    let name = name.as_any_js_binding()?.as_js_identifier_binding()?;
    let text = name.name_token().ok()?;

    Some(is_pascal_case(text.text_trimmed()))
}

fn is_pascal_case(name: &str) -> bool {
    Case::identify(name, false) == Case::Pascal
}

fn is_binding_used_in_jsx(binding: &AnyJsBinding, model: &SemanticModel) -> Option<TextRange> {
    if let Some(binding) = binding
        .as_js_identifier_binding()
        .map(|b| model.as_binding(b))
    {
        for reference in binding.all_reads() {
            if reference
                .syntax()
                .ancestors()
                .skip(1)
                .any(|ancestor| {
                    JsxExpressionAttributeValue::can_cast(ancestor.kind())
                        || JsxExpressionChild::can_cast(ancestor.kind())
                })
            {
                return Some(reference.syntax().text_trimmed_range());
            }
        }
    }

    None
}

enum BindingPatternLikeList {
    Array(AstSeparatedListNodesIterator<JsLanguage, AnyJsArrayBindingPatternElement>),
    Object(AstSeparatedListNodesIterator<JsLanguage, AnyJsObjectBindingPatternMember>),
}

struct BindingPatterIterator {
    queue: VecDeque<BindingPatternLikeList>,
    current_list: Option<BindingPatternLikeList>,
}

impl BindingPatterIterator {
    fn new(list: BindingPatternLikeList) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(list);
        Self {
            queue,
            current_list: None,
        }
    }
}

impl BindingPatterIterator {
    /// It returns the next [AnyJsBinding] from the current list.
    /// If in the current list there are nested binding patterns, they are queued, and `None` is returned
    fn next_binding(&mut self) -> Option<AnyJsBinding> {
        if let Some(current_list) = &mut self.current_list {
            match current_list {
                BindingPatternLikeList::Array(iter) => {
                    let item = iter.next()?.ok()?;
                    match item {
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(node) => {
                            let pattern = node.pattern().ok()?;
                            match pattern {
                                AnyJsBindingPattern::AnyJsBinding(binding) => Some(binding),
                                AnyJsBindingPattern::JsArrayBindingPattern(pattern) => {
                                    self.queue.push_back(BindingPatternLikeList::Array(
                                        pattern.elements().iter(),
                                    ));
                                    None
                                }
                                AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
                                    self.queue.push_back(BindingPatternLikeList::Object(
                                        pattern.properties().iter(),
                                    ));
                                    None
                                }
                            }
                        }
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(node) => {
                            let pattern = node.pattern().ok()?;
                            match pattern {
                                AnyJsBindingPattern::AnyJsBinding(binding) => Some(binding),
                                AnyJsBindingPattern::JsArrayBindingPattern(pattern) => {
                                    self.queue.push_back(BindingPatternLikeList::Array(
                                        pattern.elements().iter(),
                                    ));
                                    None
                                }
                                AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
                                    self.queue.push_back(BindingPatternLikeList::Object(
                                        pattern.properties().iter(),
                                    ));
                                    None
                                }
                            }
                        }
                        AnyJsArrayBindingPatternElement::JsArrayHole(_) => None,
                    }
                }
                BindingPatternLikeList::Object(iter) => {
                    let item = iter.next()?.ok()?;

                    match item {
                        AnyJsObjectBindingPatternMember::JsBogusBinding(_) |
                        AnyJsObjectBindingPatternMember::JsMetavariable(_) => None,
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(pattern) => {
                            let pattern = pattern.pattern().ok()?;
                            match pattern {
                                AnyJsBindingPattern::AnyJsBinding(binding) => {
                                    Some(binding)
                                }
                                AnyJsBindingPattern::JsArrayBindingPattern(pattern) => {
                                    self.queue.push_back(BindingPatternLikeList::Array(pattern.elements().iter()));
                                    None
                                }
                                AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
                                    self.queue.push_back(BindingPatternLikeList::Object(pattern.properties().iter()));
                                    None
                                }
                            }
                        }
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(node) => {
                            let binding = node.binding().ok()?;
                            Some(binding)
                        }
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(node) => {
                            let identifier = node.identifier().ok()?;
                            Some(identifier)

                        }
                    }
                }
            }
        } else {
            None
        }
    }
}

impl Iterator for BindingPatterIterator {
    type Item = AnyJsBinding;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() && self.current_list.is_none() {
            return None;
        };

        // Looks first for all the bindings available in the current list. Once the bindings
        // are finished, it checks if there are other binding patterns inside the queue, and the next item of the queue
        // is assigned as current list.
        // This will restart the loop until there are no more bindings and no more binding patterns inside the queue.
        loop {
            let next_binding = self.next_binding();
            if next_binding.is_some() {
                return next_binding;
            } else if let Some(current_list) = self.queue.pop_front() {
                self.current_list = Some(current_list);
            } else {
                self.current_list = None;
                break;
            }
        }

        None
    }
}

impl FusedIterator for BindingPatterIterator {}
