use std::vec;

use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, AddVisitor, AnalyzerOptions, FixKind,
    Phases, QueryMatch, Queryable, Rule, RuleDiagnostic, RuleKey, ServiceBag, Visitor,
    VisitorContext,
};
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};
use biome_diagnostics::Applicability;
use biome_js_factory::make::{js_string_literal, js_string_literal_expression, jsx_string};
use biome_js_syntax::{
    JsCallArguments, JsCallExpression, JsLanguage, JsStringLiteralExpression,
    JsTemplateChunkElement, JsxAttribute, JsxString,
};
use biome_rowan::{AstNode, BatchMutationExt, Language, SyntaxNode, TextRange, WalkEvent};
use rustc_hash::FxHashMap;

use crate::JsRuleAction;

// TODO: variants
// TODO: extensibility
// TODO: preset generation script
// TODO: automatic config sync
// TODO: remove duplicated classes

const CLASS_ATTRIBUTES: [&str; 2] = ["class", "className"];

// rule metadata
// -------------

declare_rule! {
    /// Enforce the sorting of CSS classes.
    ///
    /// TODO: description
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="px-2 foo px-4 bar" />;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// // TODO: examples
    /// ```
    ///
    pub(crate) UseSortedClasses {
        version: "next",
        name: "useSortedClasses",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

// utils
// -----

fn get_callee_name(call_expression: &JsCallExpression) -> Option<String> {
    Some(
        call_expression
            .callee()
            .ok()?
            .as_js_identifier_expression()?
            .name()
            .ok()?
            .name()
            .ok()?
            .to_string(),
    )
}

fn get_attribute_name(attribute: &JsxAttribute) -> Option<String> {
    Some(attribute.name().ok()?.as_jsx_name()?.to_string())
}

fn is_call_expression_of_valid_function(
    call_expression: &JsCallExpression,
    functions: &[String],
) -> bool {
    match get_callee_name(call_expression) {
        Some(name) => functions.contains(&name.to_string()),
        None => false,
    }
}

fn is_valid_attribute(attribute: &JsxAttribute, attributes: &[String]) -> bool {
    match get_attribute_name(attribute) {
        Some(name) => attributes.contains(&name.to_string()),
        None => false,
    }
}

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

// attributes visitor
// ------------------

#[derive(Default)]
struct StringLiteralInAttributeVisitor {
    in_valid_attribute: bool,
}

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
                // When entering an attribute node, track if we are in a valid attribute.
                if let Some(attribute) = JsxAttribute::cast_ref(node) {
                    self.in_valid_attribute = is_valid_attribute(&attribute, &options.attributes);
                }

                // When entering a JSX string node, and we are in a valid attribute, emit.
                if let Some(jsx_string) = JsxString::cast_ref(node) {
                    if self.in_valid_attribute {
                        ctx.match_query(ClassStringLike::JsxString(jsx_string));
                    }
                }

                // When entering a string literal node, and we are in a valid attribute, emit.
                if let Some(string_literal) = JsStringLiteralExpression::cast_ref(node) {
                    if self.in_valid_attribute {
                        ctx.match_query(ClassStringLike::StringLiteral(string_literal));
                    }
                }
            }
            WalkEvent::Leave(node) => {
                // When leaving an attribute node, reset in_valid_attribute.
                if JsxAttribute::cast_ref(node).is_some() {
                    self.in_valid_attribute = false;
                }
            }
        }
    }
}

// functions (call expression) visitor
// -----------------------------------

#[derive(Default)]
struct StringLiteralInCallExpressionVisitor {
    in_valid_function: bool,
    in_arguments: bool,
}

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
                // When entering a call expression node, track if we are in a valid function and reset
                // in_arguments.
                if let Some(call_expression) = JsCallExpression::cast_ref(node) {
                    self.in_valid_function =
                        is_call_expression_of_valid_function(&call_expression, &options.functions);
                    self.in_arguments = false;
                }

                // When entering a call arguments node, set in_arguments.
                if JsCallArguments::cast_ref(node).is_some() {
                    self.in_arguments = true;
                }

                // When entering a string literal node, and we are in a valid function and in arguments, emit.
                if let Some(string_literal) = JsStringLiteralExpression::cast_ref(node) {
                    if self.in_valid_function && self.in_arguments {
                        ctx.match_query(ClassStringLike::StringLiteral(string_literal));
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

// TODO: template chunk visitor

// custom query
// ------------

#[derive(Clone)]
pub(crate) enum ClassStringLike {
    StringLiteral(JsStringLiteralExpression),
    JsxString(JsxString),
    TemplateChunk(JsTemplateChunkElement),
}

impl ClassStringLike {
    fn range(&self) -> TextRange {
        match self {
            ClassStringLike::StringLiteral(string_literal) => string_literal.range(),
            ClassStringLike::JsxString(jsx_string) => jsx_string.range(),
            ClassStringLike::TemplateChunk(template_chunk) => template_chunk.range(),
        }
    }

    fn value(&self) -> Option<String> {
        match self {
            ClassStringLike::StringLiteral(string_literal) => {
                Some(string_literal.inner_string_text().ok()?.to_string())
            }
            ClassStringLike::JsxString(jsx_string) => {
                Some(jsx_string.inner_string_text().ok()?.to_string())
            }
            ClassStringLike::TemplateChunk(template_chunk) => Some(template_chunk.to_string()),
        }
    }
}

impl QueryMatch for ClassStringLike {
    fn text_range(&self) -> TextRange {
        self.range()
    }
}

impl Queryable for ClassStringLike {
    type Input = Self;
    type Language = JsLanguage;
    type Output = ClassStringLike;
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

// class sorting
// -------------

fn get_utilities_match(spec: &String, class_name: &str) -> Option<bool> {
    if spec.ends_with('$') && class_name == &spec[..spec.len() - 1] {
        return Some(true);
    }
    if class_name.starts_with(spec) && class_name != spec.as_str() {
        return Some(false);
    }
    None
}

fn find_utilities_index(utilities: &[String], class_name: &str) -> Option<usize> {
    let mut matched = false;
    let mut match_index: usize = 0;
    let mut last_size: usize = 0;
    for (i, spec) in utilities.iter().enumerate() {
        match get_utilities_match(spec, class_name) {
            Some(true) => return Some(i),
            Some(false) => {
                let spec_size = spec.chars().count();
                if spec_size > last_size {
                    match_index = i;
                    last_size = spec_size;
                    matched = true;
                }
            }
            _ => {}
        }
    }
    if matched {
        Some(match_index)
    } else {
        None
    }
}

// TODO: detect arbitrary css (e.g. [background:red]), put at the end
fn sort_class_name(class_name: &str, utilities: &Vec<String>) -> String {
    let classes = class_name.split_whitespace().collect::<Vec<&str>>();
    let mut unordered_classes: Vec<&str> = Vec::new();
    let mut utilities_map: FxHashMap<usize, Vec<&str>> = FxHashMap::default();
    for class in classes {
        match find_utilities_index(utilities, class) {
            Some(index) => {
                utilities_map.entry(index).or_default().push(class);
            }
            None => {
                unordered_classes.push(class);
            }
        }
    }
    let mut sorted_classes: Vec<&str> = unordered_classes;
    for i in 0..utilities.len() {
        if let Some(classes) = utilities_map.get(&i) {
            let mut abc_classes = classes.clone();
            abc_classes.sort_unstable();
            sorted_classes.extend(abc_classes);
        }
    }
    sorted_classes.join(" ")
}

// rule
// ----

impl Rule for UseSortedClasses {
    type Query = ClassStringLike;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = UseSortedClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let value = ctx.query().value()?;
        let options = &ctx.options();
        let sorted_value = sort_class_name(value.as_str(), &options.utilities);
        if value != sorted_value {
            Some(sorted_value)
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(rule_category!(), ctx.query().range(), "TODO: title").note(
                markup! {
                    "TODO: description."
                },
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        match ctx.query() {
            ClassStringLike::StringLiteral(string_literal) => {
                let replacement = js_string_literal_expression(js_string_literal(state));
                mutation.replace_node(string_literal.clone(), replacement);
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! {
                        "TODO: message."
                    }
                    .to_owned(),
                    mutation,
                })
            }
            ClassStringLike::JsxString(jsx_string_node) => {
                let replacement = jsx_string(js_string_literal(state));
                mutation.replace_node(jsx_string_node.clone(), replacement);
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! {
                        "TODO: message."
                    }
                    .to_owned(),
                    mutation,
                })
            }
            _ => None,
        }
    }
}

// options
// -------

struct UtilityLayer {
    layer: String,
    classes: Vec<String>,
}

impl Deserializable for UtilityLayer {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(UtilityLayerVisitor, name, diagnostics)
    }
}

struct UtilityLayerVisitor;
impl DeserializationVisitor for UtilityLayerVisitor {
    type Output = UtilityLayer;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        let mut layer: Option<String> = None;
        let mut classes: Option<Vec<String>> = None;
        const ALLOWED_OPTIONS: &[&str] = &["layer", "classes"];

        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "layer" => {
                    if let Some(layer_option) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        layer = Some(layer_option);
                    }
                }
                "classes" => {
                    if let Some(classes_option) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        classes = Some(classes_option);
                    }
                }
                unknown_key => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_OPTIONS,
                )),
            }
        }

        let missing_layer = layer.is_none();
        let missing_classes = classes.is_none();

        if missing_layer || missing_classes {
            let mut missing_keys: Vec<&str> = Vec::new();
            if missing_layer {
                missing_keys.push("layer");
            }
            if missing_classes {
                missing_keys.push("classes");
            }
            let missing_keys = missing_keys.join(", ");
            // TODO: how to actually handle this?
            diagnostics.push(DeserializationDiagnostic::new(format!(
                "Missing {}.",
                missing_keys
            )));

            None
        } else {
            Some(UtilityLayer {
                layer: layer.expect("TODO: error message (this should never happen)"),
                classes: classes.expect("TODO: error message (this should never happen)"),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct UseSortedClassesOptions {
    pub attributes: Vec<String>,
    pub functions: Vec<String>,
    pub utilities: Vec<String>,
}

impl Default for UseSortedClassesOptions {
    fn default() -> Self {
        UseSortedClassesOptions {
            attributes: CLASS_ATTRIBUTES.iter().map(|&s| s.to_string()).collect(),
            functions: Vec::new(),
            utilities: Vec::new(),
        }
    }
}

const ALLOWED_OPTIONS: &[&str] = &["attributes", "functions", "preset", "utilities"];

impl Deserializable for UseSortedClassesOptions {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(UseSortedClassesOptionsVisitor, name, diagnostics)
    }
}

struct UseSortedClassesOptionsVisitor;
impl DeserializationVisitor for UseSortedClassesOptionsVisitor {
    type Output = UseSortedClassesOptions;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        let mut result = UseSortedClassesOptions::default();
        let mut preset: UseSortedClassesPreset = UseSortedClassesPreset::TailwindCSS;
        let mut utilities_option: Option<Vec<UtilityLayer>> = None;

        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "attributes" => {
                    if let Some(attributes_option) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        let attributes_option: Vec<String> = attributes_option; // TODO: is there a better way to do this?
                        result.attributes.extend(attributes_option);
                    }
                }
                "functions" => {
                    if let Some(functions) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        result.functions = functions;
                    }
                }
                "preset" => {
                    if let Some(preset_option) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        let preset_option: String = preset_option; // TODO: is there a better way to do this?
                        let preset_option = preset_option.as_str();
                        match preset_option {
                            "tailwind-css" => {
                                preset = UseSortedClassesPreset::TailwindCSS;
                            }
                            "no-preset" => {
                                preset = UseSortedClassesPreset::None;
                            }
                            _ => {
                                diagnostics.push(DeserializationDiagnostic::new_unknown_value(
                                    preset_option,
                                    value.range(),
                                    ALLOWED_PRESETS,
                                ));
                            }
                        }
                    }
                }
                "utilities" => {
                    if let Some(utilities_opt) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        utilities_option = Some(utilities_opt);
                    }
                }
                unknown_key => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_OPTIONS,
                )),
            }
        }

        let resolved_utilities = match utilities_option {
            Some(utilities) => utilities,
            None => get_utilities_preset(&preset),
        };
        result.utilities = resolved_utilities
            .iter()
            .flat_map(|layer| {
                // TODO: extend layer here
                layer.classes.clone()
            })
            .collect();

        Some(result)
    }
}

// presets
// -------

#[derive(Debug, Default, Clone)]
pub enum UseSortedClassesPreset {
    None,
    #[default]
    TailwindCSS,
}

const ALLOWED_PRESETS: &[&str] = &["no-preset", "tailwind-css"];

// TODO: move to separate file?
fn get_utilities_preset(preset: &UseSortedClassesPreset) -> Vec<UtilityLayer> {
    match preset {
        UseSortedClassesPreset::None => {
            vec![]
        }
        UseSortedClassesPreset::TailwindCSS => {
            // TAILWIND-PRESET-START
            vec![
                UtilityLayer {
                    layer: String::from("components"),
                    classes: vec![String::from("container$")],
                },
                UtilityLayer {
                    layer: String::from("utilities"),
                    classes: vec![
                        String::from("sr-only$"),
                        String::from("not-sr-only$"),
                        String::from("pointer-events-none$"),
                        String::from("pointer-events-auto$"),
                        String::from("visible$"),
                        String::from("invisible$"),
                        String::from("collapse$"),
                        String::from("static$"),
                        String::from("fixed$"),
                        String::from("absolute$"),
                        String::from("relative$"),
                        String::from("sticky$"),
                        String::from("inset-"),
                        String::from("inset-x-"),
                        String::from("inset-y-"),
                        String::from("start-"),
                        String::from("end-"),
                        String::from("top-"),
                        String::from("right-"),
                        String::from("bottom-"),
                        String::from("left-"),
                        String::from("isolate$"),
                        String::from("isolation-auto$"),
                        String::from("z-"),
                        String::from("order-"),
                        String::from("col-"),
                        String::from("col-start-"),
                        String::from("col-end-"),
                        String::from("row-"),
                        String::from("row-start-"),
                        String::from("row-end-"),
                        String::from("float-start$"),
                        String::from("float-end$"),
                        String::from("float-right$"),
                        String::from("float-left$"),
                        String::from("float-none$"),
                        String::from("clear-start$"),
                        String::from("clear-end$"),
                        String::from("clear-left$"),
                        String::from("clear-right$"),
                        String::from("clear-both$"),
                        String::from("clear-none$"),
                        String::from("m-"),
                        String::from("mx-"),
                        String::from("my-"),
                        String::from("ms-"),
                        String::from("me-"),
                        String::from("mt-"),
                        String::from("mr-"),
                        String::from("mb-"),
                        String::from("ml-"),
                        String::from("box-border$"),
                        String::from("box-content$"),
                        String::from("line-clamp-"),
                        String::from("line-clamp-none$"),
                        String::from("block$"),
                        String::from("inline-block$"),
                        String::from("inline$"),
                        String::from("flex$"),
                        String::from("inline-flex$"),
                        String::from("table$"),
                        String::from("inline-table$"),
                        String::from("table-caption$"),
                        String::from("table-cell$"),
                        String::from("table-column$"),
                        String::from("table-column-group$"),
                        String::from("table-footer-group$"),
                        String::from("table-header-group$"),
                        String::from("table-row-group$"),
                        String::from("table-row$"),
                        String::from("flow-root$"),
                        String::from("grid$"),
                        String::from("inline-grid$"),
                        String::from("contents$"),
                        String::from("list-item$"),
                        String::from("hidden$"),
                        String::from("aspect-"),
                        String::from("size-"),
                        String::from("h-"),
                        String::from("max-h-"),
                        String::from("min-h-"),
                        String::from("w-"),
                        String::from("min-w-"),
                        String::from("max-w-"),
                        String::from("flex-shrink$"),
                        String::from("flex-shrink-"),
                        String::from("shrink$"),
                        String::from("shrink-"),
                        String::from("flex-grow$"),
                        String::from("flex-grow-"),
                        String::from("grow$"),
                        String::from("grow-"),
                        String::from("basis-"),
                        String::from("table-auto$"),
                        String::from("table-fixed$"),
                        String::from("caption-top$"),
                        String::from("caption-bottom$"),
                        String::from("border-collapse$"),
                        String::from("border-separate$"),
                        String::from("border-spacing-"),
                        String::from("border-spacing-x-"),
                        String::from("border-spacing-y-"),
                        String::from("origin-"),
                        String::from("translate-x-"),
                        String::from("translate-y-"),
                        String::from("rotate-"),
                        String::from("skew-x-"),
                        String::from("skew-y-"),
                        String::from("scale-"),
                        String::from("scale-x-"),
                        String::from("scale-y-"),
                        String::from("transform$"),
                        String::from("transform-cpu$"),
                        String::from("transform-gpu$"),
                        String::from("transform-none$"),
                        String::from("animate-"),
                        String::from("cursor-"),
                        String::from("touch-auto$"),
                        String::from("touch-none$"),
                        String::from("touch-pan-x$"),
                        String::from("touch-pan-left$"),
                        String::from("touch-pan-right$"),
                        String::from("touch-pan-y$"),
                        String::from("touch-pan-up$"),
                        String::from("touch-pan-down$"),
                        String::from("touch-pinch-zoom$"),
                        String::from("touch-manipulation$"),
                        String::from("select-none$"),
                        String::from("select-text$"),
                        String::from("select-all$"),
                        String::from("select-auto$"),
                        String::from("resize-none$"),
                        String::from("resize-y$"),
                        String::from("resize-x$"),
                        String::from("resize$"),
                        String::from("snap-none$"),
                        String::from("snap-x$"),
                        String::from("snap-y$"),
                        String::from("snap-both$"),
                        String::from("snap-mandatory$"),
                        String::from("snap-proximity$"),
                        String::from("snap-start$"),
                        String::from("snap-end$"),
                        String::from("snap-center$"),
                        String::from("snap-align-none$"),
                        String::from("snap-normal$"),
                        String::from("snap-always$"),
                        String::from("scroll-m-"),
                        String::from("scroll-mx-"),
                        String::from("scroll-my-"),
                        String::from("scroll-ms-"),
                        String::from("scroll-me-"),
                        String::from("scroll-mt-"),
                        String::from("scroll-mr-"),
                        String::from("scroll-mb-"),
                        String::from("scroll-ml-"),
                        String::from("scroll-p-"),
                        String::from("scroll-px-"),
                        String::from("scroll-py-"),
                        String::from("scroll-ps-"),
                        String::from("scroll-pe-"),
                        String::from("scroll-pt-"),
                        String::from("scroll-pr-"),
                        String::from("scroll-pb-"),
                        String::from("scroll-pl-"),
                        String::from("list-inside$"),
                        String::from("list-outside$"),
                        String::from("list-"),
                        String::from("list-image-"),
                        String::from("appearance-none$"),
                        String::from("appearance-auto$"),
                        String::from("columns-"),
                        String::from("break-before-auto$"),
                        String::from("break-before-avoid$"),
                        String::from("break-before-all$"),
                        String::from("break-before-avoid-page$"),
                        String::from("break-before-page$"),
                        String::from("break-before-left$"),
                        String::from("break-before-right$"),
                        String::from("break-before-column$"),
                        String::from("break-inside-auto$"),
                        String::from("break-inside-avoid$"),
                        String::from("break-inside-avoid-page$"),
                        String::from("break-inside-avoid-column$"),
                        String::from("break-after-auto$"),
                        String::from("break-after-avoid$"),
                        String::from("break-after-all$"),
                        String::from("break-after-avoid-page$"),
                        String::from("break-after-page$"),
                        String::from("break-after-left$"),
                        String::from("break-after-right$"),
                        String::from("break-after-column$"),
                        String::from("auto-cols-"),
                        String::from("grid-flow-row$"),
                        String::from("grid-flow-col$"),
                        String::from("grid-flow-dense$"),
                        String::from("grid-flow-row-dense$"),
                        String::from("grid-flow-col-dense$"),
                        String::from("auto-rows-"),
                        String::from("grid-cols-"),
                        String::from("grid-rows-"),
                        String::from("flex-row$"),
                        String::from("flex-row-reverse$"),
                        String::from("flex-col$"),
                        String::from("flex-col-reverse$"),
                        String::from("flex-wrap$"),
                        String::from("flex-wrap-reverse$"),
                        String::from("flex-nowrap$"),
                        String::from("place-content-center$"),
                        String::from("place-content-start$"),
                        String::from("place-content-end$"),
                        String::from("place-content-between$"),
                        String::from("place-content-around$"),
                        String::from("place-content-evenly$"),
                        String::from("place-content-baseline$"),
                        String::from("place-content-stretch$"),
                        String::from("place-items-start$"),
                        String::from("place-items-end$"),
                        String::from("place-items-center$"),
                        String::from("place-items-baseline$"),
                        String::from("place-items-stretch$"),
                        String::from("content-normal$"),
                        String::from("content-center$"),
                        String::from("content-start$"),
                        String::from("content-end$"),
                        String::from("content-between$"),
                        String::from("content-around$"),
                        String::from("content-evenly$"),
                        String::from("content-baseline$"),
                        String::from("content-stretch$"),
                        String::from("items-start$"),
                        String::from("items-end$"),
                        String::from("items-center$"),
                        String::from("items-baseline$"),
                        String::from("items-stretch$"),
                        String::from("justify-normal$"),
                        String::from("justify-start$"),
                        String::from("justify-end$"),
                        String::from("justify-center$"),
                        String::from("justify-between$"),
                        String::from("justify-around$"),
                        String::from("justify-evenly$"),
                        String::from("justify-stretch$"),
                        String::from("justify-items-start$"),
                        String::from("justify-items-end$"),
                        String::from("justify-items-center$"),
                        String::from("justify-items-stretch$"),
                        String::from("gap-"),
                        String::from("gap-x-"),
                        String::from("gap-y-"),
                        String::from("space-x-"),
                        String::from("space-y-"),
                        String::from("space-y-reverse$"),
                        String::from("space-x-reverse$"),
                        String::from("divide-x$"),
                        String::from("divide-x-"),
                        String::from("divide-y$"),
                        String::from("divide-y-"),
                        String::from("divide-y-reverse$"),
                        String::from("divide-x-reverse$"),
                        String::from("divide-solid$"),
                        String::from("divide-dashed$"),
                        String::from("divide-dotted$"),
                        String::from("divide-double$"),
                        String::from("divide-none$"),
                        String::from("divide-"),
                        String::from("divide-opacity-"),
                        String::from("place-self-auto$"),
                        String::from("place-self-start$"),
                        String::from("place-self-end$"),
                        String::from("place-self-center$"),
                        String::from("place-self-stretch$"),
                        String::from("self-auto$"),
                        String::from("self-start$"),
                        String::from("self-end$"),
                        String::from("self-center$"),
                        String::from("self-stretch$"),
                        String::from("self-baseline$"),
                        String::from("justify-self-auto$"),
                        String::from("justify-self-start$"),
                        String::from("justify-self-end$"),
                        String::from("justify-self-center$"),
                        String::from("justify-self-stretch$"),
                        String::from("overflow-auto$"),
                        String::from("overflow-hidden$"),
                        String::from("overflow-clip$"),
                        String::from("overflow-visible$"),
                        String::from("overflow-scroll$"),
                        String::from("overflow-x-auto$"),
                        String::from("overflow-y-auto$"),
                        String::from("overflow-x-hidden$"),
                        String::from("overflow-y-hidden$"),
                        String::from("overflow-x-clip$"),
                        String::from("overflow-y-clip$"),
                        String::from("overflow-x-visible$"),
                        String::from("overflow-y-visible$"),
                        String::from("overflow-x-scroll$"),
                        String::from("overflow-y-scroll$"),
                        String::from("overscroll-auto$"),
                        String::from("overscroll-contain$"),
                        String::from("overscroll-none$"),
                        String::from("overscroll-y-auto$"),
                        String::from("overscroll-y-contain$"),
                        String::from("overscroll-y-none$"),
                        String::from("overscroll-x-auto$"),
                        String::from("overscroll-x-contain$"),
                        String::from("overscroll-x-none$"),
                        String::from("scroll-auto$"),
                        String::from("scroll-smooth$"),
                        String::from("truncate$"),
                        String::from("overflow-ellipsis$"),
                        String::from("text-ellipsis$"),
                        String::from("text-clip$"),
                        String::from("hyphens-none$"),
                        String::from("hyphens-manual$"),
                        String::from("hyphens-auto$"),
                        String::from("whitespace-normal$"),
                        String::from("whitespace-nowrap$"),
                        String::from("whitespace-pre$"),
                        String::from("whitespace-pre-line$"),
                        String::from("whitespace-pre-wrap$"),
                        String::from("whitespace-break-spaces$"),
                        String::from("text-wrap$"),
                        String::from("text-nowrap$"),
                        String::from("text-balance$"),
                        String::from("text-pretty$"),
                        String::from("break-normal$"),
                        String::from("break-words$"),
                        String::from("break-all$"),
                        String::from("break-keep$"),
                        String::from("rounded$"),
                        String::from("rounded-"),
                        String::from("rounded-s$"),
                        String::from("rounded-s-"),
                        String::from("rounded-e$"),
                        String::from("rounded-e-"),
                        String::from("rounded-t$"),
                        String::from("rounded-t-"),
                        String::from("rounded-r$"),
                        String::from("rounded-r-"),
                        String::from("rounded-b$"),
                        String::from("rounded-b-"),
                        String::from("rounded-l$"),
                        String::from("rounded-l-"),
                        String::from("rounded-ss$"),
                        String::from("rounded-ss-"),
                        String::from("rounded-se$"),
                        String::from("rounded-se-"),
                        String::from("rounded-ee$"),
                        String::from("rounded-ee-"),
                        String::from("rounded-es$"),
                        String::from("rounded-es-"),
                        String::from("rounded-tl$"),
                        String::from("rounded-tl-"),
                        String::from("rounded-tr$"),
                        String::from("rounded-tr-"),
                        String::from("rounded-br$"),
                        String::from("rounded-br-"),
                        String::from("rounded-bl$"),
                        String::from("rounded-bl-"),
                        String::from("border$"),
                        String::from("border-"),
                        String::from("border-x$"),
                        String::from("border-x-"),
                        String::from("border-y$"),
                        String::from("border-y-"),
                        String::from("border-s$"),
                        String::from("border-s-"),
                        String::from("border-e$"),
                        String::from("border-e-"),
                        String::from("border-t$"),
                        String::from("border-t-"),
                        String::from("border-r$"),
                        String::from("border-r-"),
                        String::from("border-b$"),
                        String::from("border-b-"),
                        String::from("border-l$"),
                        String::from("border-l-"),
                        String::from("border-solid$"),
                        String::from("border-dashed$"),
                        String::from("border-dotted$"),
                        String::from("border-double$"),
                        String::from("border-hidden$"),
                        String::from("border-none$"),
                        String::from("border-opacity-"),
                        String::from("bg-"),
                        String::from("bg-opacity-"),
                        String::from("from-"),
                        String::from("via-"),
                        String::from("to-"),
                        String::from("decoration-slice$"),
                        String::from("decoration-clone$"),
                        String::from("box-decoration-slice$"),
                        String::from("box-decoration-clone$"),
                        String::from("bg-fixed$"),
                        String::from("bg-local$"),
                        String::from("bg-scroll$"),
                        String::from("bg-clip-border$"),
                        String::from("bg-clip-padding$"),
                        String::from("bg-clip-content$"),
                        String::from("bg-clip-text$"),
                        String::from("bg-repeat$"),
                        String::from("bg-no-repeat$"),
                        String::from("bg-repeat-x$"),
                        String::from("bg-repeat-y$"),
                        String::from("bg-repeat-round$"),
                        String::from("bg-repeat-space$"),
                        String::from("bg-origin-border$"),
                        String::from("bg-origin-padding$"),
                        String::from("bg-origin-content$"),
                        String::from("fill-"),
                        String::from("stroke-"),
                        String::from("object-contain$"),
                        String::from("object-cover$"),
                        String::from("object-fill$"),
                        String::from("object-none$"),
                        String::from("object-scale-down$"),
                        String::from("object-"),
                        String::from("p-"),
                        String::from("px-"),
                        String::from("py-"),
                        String::from("ps-"),
                        String::from("pe-"),
                        String::from("pt-"),
                        String::from("pr-"),
                        String::from("pb-"),
                        String::from("pl-"),
                        String::from("text-left$"),
                        String::from("text-center$"),
                        String::from("text-right$"),
                        String::from("text-justify$"),
                        String::from("text-start$"),
                        String::from("text-end$"),
                        String::from("indent-"),
                        String::from("align-baseline$"),
                        String::from("align-top$"),
                        String::from("align-middle$"),
                        String::from("align-bottom$"),
                        String::from("align-text-top$"),
                        String::from("align-text-bottom$"),
                        String::from("align-sub$"),
                        String::from("align-super$"),
                        String::from("align-"),
                        String::from("font-"),
                        String::from("text-"),
                        String::from("uppercase$"),
                        String::from("lowercase$"),
                        String::from("capitalize$"),
                        String::from("normal-case$"),
                        String::from("italic$"),
                        String::from("not-italic$"),
                        String::from("normal-nums$"),
                        String::from("ordinal$"),
                        String::from("slashed-zero$"),
                        String::from("lining-nums$"),
                        String::from("oldstyle-nums$"),
                        String::from("proportional-nums$"),
                        String::from("tabular-nums$"),
                        String::from("diagonal-fractions$"),
                        String::from("stacked-fractions$"),
                        String::from("leading-"),
                        String::from("tracking-"),
                        String::from("text-opacity-"),
                        String::from("underline$"),
                        String::from("overline$"),
                        String::from("line-through$"),
                        String::from("no-underline$"),
                        String::from("decoration-"),
                        String::from("decoration-solid$"),
                        String::from("decoration-double$"),
                        String::from("decoration-dotted$"),
                        String::from("decoration-dashed$"),
                        String::from("decoration-wavy$"),
                        String::from("underline-offset-"),
                        String::from("antialiased$"),
                        String::from("subpixel-antialiased$"),
                        String::from("placeholder-"),
                        String::from("placeholder-opacity-"),
                        String::from("caret-"),
                        String::from("accent-"),
                        String::from("opacity-"),
                        String::from("bg-blend-normal$"),
                        String::from("bg-blend-multiply$"),
                        String::from("bg-blend-screen$"),
                        String::from("bg-blend-overlay$"),
                        String::from("bg-blend-darken$"),
                        String::from("bg-blend-lighten$"),
                        String::from("bg-blend-color-dodge$"),
                        String::from("bg-blend-color-burn$"),
                        String::from("bg-blend-hard-light$"),
                        String::from("bg-blend-soft-light$"),
                        String::from("bg-blend-difference$"),
                        String::from("bg-blend-exclusion$"),
                        String::from("bg-blend-hue$"),
                        String::from("bg-blend-saturation$"),
                        String::from("bg-blend-color$"),
                        String::from("bg-blend-luminosity$"),
                        String::from("mix-blend-normal$"),
                        String::from("mix-blend-multiply$"),
                        String::from("mix-blend-screen$"),
                        String::from("mix-blend-overlay$"),
                        String::from("mix-blend-darken$"),
                        String::from("mix-blend-lighten$"),
                        String::from("mix-blend-color-dodge$"),
                        String::from("mix-blend-color-burn$"),
                        String::from("mix-blend-hard-light$"),
                        String::from("mix-blend-soft-light$"),
                        String::from("mix-blend-difference$"),
                        String::from("mix-blend-exclusion$"),
                        String::from("mix-blend-hue$"),
                        String::from("mix-blend-saturation$"),
                        String::from("mix-blend-color$"),
                        String::from("mix-blend-luminosity$"),
                        String::from("mix-blend-plus-lighter$"),
                        String::from("shadow$"),
                        String::from("shadow-"),
                        String::from("outline-none$"),
                        String::from("outline$"),
                        String::from("outline-dashed$"),
                        String::from("outline-dotted$"),
                        String::from("outline-double$"),
                        String::from("outline-offset-"),
                        String::from("ring$"),
                        String::from("ring-"),
                        String::from("ring-inset$"),
                        String::from("ring-opacity-"),
                        String::from("ring-offset-"),
                        String::from("blur$"),
                        String::from("blur-"),
                        String::from("brightness-"),
                        String::from("contrast-"),
                        String::from("drop-shadow$"),
                        String::from("drop-shadow-"),
                        String::from("grayscale$"),
                        String::from("grayscale-"),
                        String::from("hue-rotate-"),
                        String::from("invert$"),
                        String::from("invert-"),
                        String::from("saturate-"),
                        String::from("sepia$"),
                        String::from("sepia-"),
                        String::from("filter$"),
                        String::from("filter-none$"),
                        String::from("backdrop-blur$"),
                        String::from("backdrop-blur-"),
                        String::from("backdrop-brightness-"),
                        String::from("backdrop-contrast-"),
                        String::from("backdrop-grayscale$"),
                        String::from("backdrop-grayscale-"),
                        String::from("backdrop-hue-rotate-"),
                        String::from("backdrop-invert$"),
                        String::from("backdrop-invert-"),
                        String::from("backdrop-opacity-"),
                        String::from("backdrop-saturate-"),
                        String::from("backdrop-sepia$"),
                        String::from("backdrop-sepia-"),
                        String::from("backdrop-filter$"),
                        String::from("backdrop-filter-none$"),
                        String::from("transition$"),
                        String::from("transition-"),
                        String::from("delay-"),
                        String::from("duration-"),
                        String::from("ease-"),
                        String::from("will-change-"),
                        String::from("content-"),
                        String::from("forced-color-adjust-auto$"),
                        String::from("forced-color-adjust-none$"),
                    ],
                },
            ]
            // TAILWIND-PRESET-END
        }
    }
}
