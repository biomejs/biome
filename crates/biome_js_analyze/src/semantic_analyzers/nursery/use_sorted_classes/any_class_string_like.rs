use biome_analyze::{
    AddVisitor, AnalyzerOptions, Phases, QueryMatch, Queryable, RuleKey, ServiceBag, Visitor,
    VisitorContext,
};
use biome_js_syntax::{
    JsCallArguments, JsCallExpression, JsLanguage, JsStringLiteralExpression,
    JsTemplateChunkElement, JsxAttribute, JsxString,
};
use biome_rowan::{
    declare_node_union, AstNode, Language, SyntaxNode, TextRange, TokenText, WalkEvent,
};

use super::UseSortedClassesOptions;

// utils
// -----

fn get_options_from_analyzer(analyzer_options: &AnalyzerOptions) -> UseSortedClassesOptions {
    match analyzer_options
        .configuration
        .rules
        .get_rule_options::<UseSortedClassesOptions>(&RuleKey::new("nursery", "useSortedClasses"))
    {
        Some(options) => options.clone(),
        None => UseSortedClassesOptions::default(),
    }
}

fn get_callee_name(call_expression: &JsCallExpression) -> Option<TokenText> {
    call_expression
        .callee()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?
        .name()
        .ok()
}

fn is_call_expression_of_target_function(
    call_expression: &JsCallExpression,
    target_functions: &[String],
) -> bool {
    match get_callee_name(call_expression) {
        Some(name) => target_functions.contains(&name.to_string()),
        None => false,
    }
}

fn get_attribute_name(attribute: &JsxAttribute) -> Option<String> {
    Some(attribute.name().ok()?.as_jsx_name()?.to_string())
}

fn is_target_attribute(attribute: &JsxAttribute, target_attributes: &[String]) -> bool {
    match get_attribute_name(attribute) {
        Some(name) => target_attributes.contains(&name.to_string()),
        None => false,
    }
}

// attributes visitor
// ------------------

#[derive(Default)]
struct StringLiteralInAttributeVisitor {
    in_target_attribute: bool,
}

// Finds class-like strings in JSX attributes, including class, className, and others defined in the options.
impl Visitor for StringLiteralInAttributeVisitor {
    type Language = JsLanguage;
    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        let options = get_options_from_analyzer(ctx.options);
        match event {
            WalkEvent::Enter(node) => {
                // When entering an attribute node, track if we are in a target attribute.
                if let Some(attribute) = JsxAttribute::cast_ref(node) {
                    self.in_target_attribute = is_target_attribute(&attribute, &options.attributes);
                }

                // When entering a JSX string node, and we are in a target attribute, emit.
                if let Some(jsx_string) = JsxString::cast_ref(node) {
                    if self.in_target_attribute {
                        ctx.match_query(AnyClassStringLike::JsxString(jsx_string));
                    }
                }

                // When entering a string literal node, and we are in a target attribute, emit.
                if let Some(string_literal) = JsStringLiteralExpression::cast_ref(node) {
                    if self.in_target_attribute {
                        ctx.match_query(AnyClassStringLike::JsStringLiteralExpression(
                            string_literal,
                        ));
                    }
                }
            }
            WalkEvent::Leave(node) => {
                // When leaving an attribute node, reset in_target_attribute.
                if JsxAttribute::cast_ref(node).is_some() {
                    self.in_target_attribute = false;
                }
            }
        }
    }
}

// functions (call expression) visitor
// -----------------------------------

#[derive(Default)]
struct StringLiteralInCallExpressionVisitor {
    in_target_function: bool,
    in_arguments: bool,
}

// Finds class-like strings inside function calls defined in the options, e.g. clsx(classes).
impl Visitor for StringLiteralInCallExpressionVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        let options = get_options_from_analyzer(ctx.options);
        if options.functions.is_empty() {
            return;
        }
        match event {
            WalkEvent::Enter(node) => {
                // When entering a call expression node, track if we are in a target function and reset
                // in_arguments.
                if let Some(call_expression) = JsCallExpression::cast_ref(node) {
                    self.in_target_function =
                        is_call_expression_of_target_function(&call_expression, &options.functions);
                    self.in_arguments = false;
                }

                // When entering a call arguments node, set in_arguments.
                if JsCallArguments::cast_ref(node).is_some() {
                    self.in_arguments = true;
                }

                // When entering a string literal node, and we are in a target function and in arguments, emit.
                if let Some(string_literal) = JsStringLiteralExpression::cast_ref(node) {
                    if self.in_target_function && self.in_arguments {
                        ctx.match_query(AnyClassStringLike::JsStringLiteralExpression(
                            string_literal,
                        ));
                    }
                }
            }
            WalkEvent::Leave(node) => {
                // When leaving a call arguments node, reset in_arguments.
                if JsCallArguments::cast_ref(node).is_some() {
                    self.in_arguments = false;
                }
            }
        }
    }
}

// functions (template chunk) visitor
// ----------------------------------

// Finds class-like template chunks in tagged template calls defined in the options, e.g. tw`classes`.
// TODO: template chunk visitor

// query
// -----

declare_node_union! {
    /// A string literal, JSX string, or template chunk representing a CSS class string.
    pub AnyClassStringLike = JsStringLiteralExpression | JsxString | JsTemplateChunkElement
}

impl AnyClassStringLike {
    /// Returns the value of the string literal, JSX string, or template chunk.
    pub fn value(&self) -> Option<String> {
        match self {
            AnyClassStringLike::JsStringLiteralExpression(string_literal) => {
                Some(string_literal.inner_string_text().ok()?.to_string())
            }
            AnyClassStringLike::JsxString(jsx_string) => {
                Some(jsx_string.inner_string_text().ok()?.to_string())
            }
            AnyClassStringLike::JsTemplateChunkElement(template_chunk) => {
                Some(template_chunk.to_string())
            }
        }
    }
}

impl QueryMatch for AnyClassStringLike {
    fn text_range(&self) -> TextRange {
        self.range()
    }
}

impl Queryable for AnyClassStringLike {
    type Input = Self;
    type Language = JsLanguage;
    type Output = AnyClassStringLike;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, || {
            StringLiteralInAttributeVisitor::default()
        });
        analyzer.add_visitor(Phases::Syntax, || {
            StringLiteralInCallExpressionVisitor::default()
        });
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.clone()
    }
}
