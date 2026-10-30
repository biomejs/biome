use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsCallArgument, AnyJsExportClause, AnyJsExpression,
    AnyJsLiteralExpression, AnyJsModuleItem, AnyJsObjectMember, AnyJsObjectMemberName, AnyJsRoot,
    AnyJsTemplateElement, JsArrayExpression, JsCallExpression, JsIdentifierExpression, JsImport,
    JsObjectExpression, JsReferenceIdentifier, JsShorthandPropertyObjectMember, JsSyntaxKind,
    JsSyntaxNode, JsTemplateExpression, JsVariableDeclarator, binding_ext::AnyJsBindingDeclaration,
    inner_string_text, numbers::parse_js_number_with_single_rounding, try_unescape_js_string,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, Text, TextRange};
use rustc_hash::{FxHashMap, FxHashSet};

/// A value of a framework configuration, evaluated without running the
/// configuration.
///
/// Literals, arrays, objects, and `const` variables declared in the same file
/// are evaluated. A call to an imported function is kept as a [ConfigCall],
/// because loaders recognize integrations by the package they come from.
/// Everything else is [ConfigValue::Unknown].
#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue {
    String {
        value: Text,
        range: TextRange,
    },
    Number(f64),
    Boolean(bool),

    /// The elements of an array. An unknown element can stand for any
    /// number of elements, such as the spread of an unknown value, so the
    /// positions after it are unknown.
    Array(Vec<Self>),
    Object(ConfigObject),
    Call(ConfigCall),

    /// A value that can only be known by running the configuration.
    Unknown,
}

/// The value that properties of unknown values have.
static UNKNOWN: ConfigValue = ConfigValue::Unknown;

impl ConfigValue {
    /// Evaluates the default export of `root`, such as
    /// `export default defineConfig({ ... })`.
    ///
    /// Returns [ConfigValue::Unknown] when the file has no `export default`,
    /// for example when it assigns `module.exports`.
    pub fn from_default_export(root: &AnyJsRoot, model: &SemanticModel) -> Self {
        let items = match root {
            AnyJsRoot::JsModule(module) => module.items(),
            AnyJsRoot::TsDeclarationModule(module) => module.items(),
            _ => return Self::Unknown,
        };
        let expression = items.iter().find_map(|item| {
            let AnyJsModuleItem::JsExport(export) = item else {
                return None;
            };
            let AnyJsExportClause::JsExportDefaultExpressionClause(clause) =
                export.export_clause().ok()?
            else {
                return None;
            };
            clause.expression().ok()
        });

        match expression {
            Some(expression) => Evaluator::new(model).evaluate(&expression),
            None => Self::Unknown,
        }
    }

    /// Returns an object without properties, the options of a framework
    /// without configuration.
    pub fn new_empty_object() -> Self {
        Self::Object(ConfigObject {
            entries: Vec::new(),
        })
    }

    /// Returns the property `name` of this object.
    ///
    /// An unknown value and the result of a call have unknown properties, so
    /// this returns [ConfigValue::Unknown] for them. See [ConfigObject::get]
    /// for objects. Returns `None` for other values.
    pub fn get(&self, name: &str) -> Option<&Self> {
        match self {
            Self::Object(object) => object.get(name),
            Self::Call(_) | Self::Unknown => Some(&UNKNOWN),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String { value, .. } => Some(value.text()),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Self]> {
        match self {
            Self::Array(elements) => Some(elements),
            _ => None,
        }
    }

    pub fn as_call(&self) -> Option<&ConfigCall> {
        match self {
            Self::Call(call) => Some(call),
            _ => None,
        }
    }

    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// An object of a framework configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigObject {
    /// Properties in source order. `None` stands for properties that can't
    /// be known, such as the spread of an unknown value or a computed
    /// property name.
    entries: Vec<(Option<Text>, ConfigValue)>,
}

impl ConfigObject {
    /// Returns the value of the property `name`.
    ///
    /// When the object has unknown properties after the last property
    /// `name`, they may replace it, so this returns [ConfigValue::Unknown].
    /// Returns `None` when the object doesn't have the property.
    pub fn get(&self, name: &str) -> Option<&ConfigValue> {
        self.entries
            .iter()
            .rev()
            .find_map(|(key, value)| match key {
                Some(key) if key.text() == name => Some(value),
                Some(_) => None,
                None => Some(&UNKNOWN),
            })
    }
}

/// A call to a function imported from another module, such as
/// `starlight({ ... })` after `import starlight from "@astrojs/starlight"`.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigCall {
    /// The specifier of the import, such as `@astrojs/starlight`.
    pub specifier: Text,

    /// The name of the imported export, `default` for a default import.
    pub export: Text,

    /// The arguments of the call. Like in [ConfigValue::Array], an unknown
    /// argument can stand for any number of arguments.
    pub arguments: Vec<ConfigValue>,
}

impl ConfigCall {
    /// Returns the package that the function comes from, or `None` when the
    /// specifier isn't a package name, such as `./plugin.js`.
    pub fn package_name(&self) -> Option<&str> {
        package_name(self.specifier.text())
    }

    /// Returns the first argument, or an empty object when there is none.
    pub fn options(&self) -> ConfigValue {
        self.arguments
            .first()
            .cloned()
            .unwrap_or_else(ConfigValue::new_empty_object)
    }
}

/// Returns the package that `specifier` imports from, such as `astro` for
/// `astro/config`.
///
/// Returns `None` for a relative or absolute path, a subpath import of the
/// importing package (`#name`), and a specifier with a protocol
/// (`node:path`).
pub fn package_name(specifier: &str) -> Option<&str> {
    if specifier.starts_with(['.', '/', '#']) {
        return None;
    }
    let mut separators = specifier.match_indices('/').map(|(index, _)| index);
    let end = if specifier.starts_with('@') {
        separators.nth(1)
    } else {
        separators.next()
    };
    let name = end.map_or(specifier, |end| &specifier[..end]);
    (!name.is_empty() && !name.contains(':')).then_some(name)
}

/// The maximum nesting of values that [Evaluator] evaluates. Deeper values
/// are unknown, so that the evaluation can't overflow the stack.
const MAX_DEPTH: usize = 64;

/// The maximum number of expressions that [Evaluator] evaluates for one
/// configuration. Once it's reached, every other value is unknown.
///
/// Each reference to a variable evaluates its initializer again, so a few
/// variables that refer to each other twice could create values of any size.
const MAX_EVALUATIONS: usize = 10_000;

struct Evaluator<'a> {
    model: &'a SemanticModel,

    /// The `const` declarations being evaluated. A declaration that refers
    /// to itself is unknown.
    pending: FxHashSet<TextRange>,

    depth: usize,

    /// How many more expressions can be evaluated.
    budget: usize,

    /// Whether the `const` declaration at each range is plain, or `None`
    /// while that's being computed. See [Self::is_plain_variable].
    plain_variables: FxHashMap<TextRange, Option<bool>>,

    /// How many [Self::is_plain_variable] calls are in progress.
    plain_depth: usize,
}

impl<'a> Evaluator<'a> {
    fn new(model: &'a SemanticModel) -> Self {
        Self {
            model,
            pending: FxHashSet::default(),
            depth: 0,
            budget: MAX_EVALUATIONS,
            plain_variables: FxHashMap::default(),
            plain_depth: 0,
        }
    }

    fn evaluate(&mut self, expression: &AnyJsExpression) -> ConfigValue {
        if self.depth >= MAX_DEPTH || self.budget == 0 {
            return ConfigValue::Unknown;
        }
        self.budget -= 1;
        self.depth += 1;
        let value = self.evaluate_expression(expression.clone().omit_parentheses());
        self.depth -= 1;
        value
    }

    fn evaluate_expression(&mut self, expression: AnyJsExpression) -> ConfigValue {
        let value = match expression {
            AnyJsExpression::AnyJsLiteralExpression(literal) => evaluate_literal(&literal),
            AnyJsExpression::JsTemplateExpression(template) => evaluate_template(&template),
            AnyJsExpression::JsArrayExpression(array) => Some(self.evaluate_array(&array)),
            AnyJsExpression::JsObjectExpression(object) => Some(self.evaluate_object(&object)),
            AnyJsExpression::JsIdentifierExpression(identifier) => identifier
                .name()
                .ok()
                .and_then(|name| self.evaluate_reference(&name)),
            AnyJsExpression::JsCallExpression(call) => self.evaluate_call(&call),
            AnyJsExpression::TsAsExpression(expression) => expression
                .expression()
                .ok()
                .map(|expression| self.evaluate(&expression)),
            AnyJsExpression::TsSatisfiesExpression(expression) => expression
                .expression()
                .ok()
                .map(|expression| self.evaluate(&expression)),
            _ => None,
        };
        value.unwrap_or(ConfigValue::Unknown)
    }

    fn evaluate_array(&mut self, array: &JsArrayExpression) -> ConfigValue {
        let mut elements = Vec::new();
        for element in array.elements().iter() {
            match element {
                Ok(AnyJsArrayElement::AnyJsExpression(expression)) => {
                    elements.push(self.evaluate(&expression));
                }
                Ok(AnyJsArrayElement::JsSpread(spread)) => {
                    match spread.argument().map(|argument| self.evaluate(&argument)) {
                        Ok(ConfigValue::Array(spread)) => elements.extend(spread),
                        _ => elements.push(ConfigValue::Unknown),
                    }
                }
                // A hole is `undefined`, a value that loaders don't read.
                Ok(AnyJsArrayElement::JsArrayHole(_)) | Err(_) => {
                    elements.push(ConfigValue::Unknown);
                }
            }
        }
        ConfigValue::Array(elements)
    }

    fn evaluate_object(&mut self, object: &JsObjectExpression) -> ConfigValue {
        let mut entries = Vec::new();
        for member in object.members().iter() {
            let entry = match &member {
                Ok(AnyJsObjectMember::JsPropertyObjectMember(property)) => {
                    let name = property.name().ok();
                    let key = name.as_ref().and_then(property_name);
                    // `__proto__: value` sets the prototype, whose properties
                    // the object then has.
                    let is_prototype = name.is_some_and(|name| {
                        name.as_js_literal_member_name().is_some()
                            && key.as_ref().is_some_and(|key| key.text() == "__proto__")
                    });
                    if is_prototype {
                        (None, ConfigValue::Unknown)
                    } else {
                        let value = property
                            .value()
                            .map_or(ConfigValue::Unknown, |value| self.evaluate(&value));
                        (key, value)
                    }
                }
                Ok(AnyJsObjectMember::JsShorthandPropertyObjectMember(property)) => {
                    let reference = property.name().ok();
                    let key = reference
                        .as_ref()
                        .and_then(|reference| reference.name().ok())
                        .filter(|name| !name.contains('\\'))
                        .map(Text::from);
                    let value = reference
                        .and_then(|reference| self.evaluate_reference(&reference))
                        .unwrap_or(ConfigValue::Unknown);
                    (key, value)
                }
                Ok(AnyJsObjectMember::JsSpread(spread)) => {
                    match spread.argument().map(|argument| self.evaluate(&argument)) {
                        Ok(ConfigValue::Object(spread)) => entries.extend(spread.entries),
                        _ => entries.push((None, ConfigValue::Unknown)),
                    }
                    continue;
                }
                Ok(AnyJsObjectMember::JsMethodObjectMember(method)) => (
                    method.name().ok().as_ref().and_then(property_name),
                    ConfigValue::Unknown,
                ),
                Ok(AnyJsObjectMember::JsGetterObjectMember(getter)) => (
                    getter.name().ok().as_ref().and_then(property_name),
                    ConfigValue::Unknown,
                ),
                Ok(AnyJsObjectMember::JsSetterObjectMember(setter)) => (
                    setter.name().ok().as_ref().and_then(property_name),
                    ConfigValue::Unknown,
                ),
                _ => (None, ConfigValue::Unknown),
            };
            entries.push(entry);
        }
        ConfigValue::Object(ConfigObject { entries })
    }

    /// Evaluates the initializer of the `const` variable that `reference`
    /// refers to.
    ///
    /// Returns `None` when the file may change the value of the variable,
    /// because only the initializer is evaluated. See
    /// [Self::is_plain_variable].
    fn evaluate_reference(&mut self, reference: &JsReferenceIdentifier) -> Option<ConfigValue> {
        let binding = self.model.binding(reference)?;
        let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) =
            binding.tree().declaration()?
        else {
            return None;
        };
        if !self.is_plain_variable(&declarator) {
            return None;
        }
        let initializer = declarator.initializer()?.expression().ok()?;

        let range = declarator.syntax().text_trimmed_range();
        if !self.pending.insert(range) {
            return None;
        }
        let value = self.evaluate(&initializer);
        self.pending.remove(&range);
        Some(value)
    }

    /// Returns whether `declarator` declares a `const` variable that the
    /// file only reads to build the default export, so that its value is
    /// its initializer.
    ///
    /// The variable must have references, and each reference must be part of
    /// the default export, or of the initializer of another such variable,
    /// through literals, spreads, and arguments of imported functions.
    /// Imported functions are assumed not to change their arguments. Any
    /// other use, such as `value.push(item)`, `change(value)`, or
    /// `[value].forEach(change)`, may change the value.
    ///
    /// Variables nested deeper than [MAX_DEPTH] initializers aren't plain.
    fn is_plain_variable(&mut self, declarator: &JsVariableDeclarator) -> bool {
        let range = declarator.syntax().text_trimmed_range();
        match self.plain_variables.get(&range) {
            Some(Some(is_plain)) => return *is_plain,
            // The variable refers to itself through other variables.
            Some(None) => return false,
            None => {}
        }
        if self.plain_depth >= MAX_DEPTH {
            return false;
        }
        self.plain_variables.insert(range, None);
        self.plain_depth += 1;

        // A destructuring pattern binds a part of the initializer, which
        // isn't followed.
        let binding = declarator
            .id()
            .ok()
            .and_then(|id| id.as_any_js_binding().cloned())
            .and_then(|binding| {
                self.model
                    .as_binding_by_range(binding.syntax().text_trimmed_range())
            });
        let is_plain = declarator
            .declaration()
            .is_some_and(|declaration| declaration.is_const())
            && binding.is_some_and(|binding| {
                let mut references = binding.all_references().peekable();
                references.peek().is_some()
                    && references.all(|reference| self.is_plain_read(&reference.syntax()))
            });

        self.plain_depth -= 1;
        self.plain_variables.insert(range, Some(is_plain));
        is_plain
    }

    /// Returns whether `reference` reads a variable to build the default
    /// export. See [Self::is_plain_variable].
    fn is_plain_read(&mut self, reference: &JsSyntaxNode) -> bool {
        let Some(mut node) = reference.parent() else {
            return false;
        };
        if !JsIdentifierExpression::can_cast(node.kind())
            && !JsShorthandPropertyObjectMember::can_cast(node.kind())
        {
            return false;
        }
        while let Some(parent) = node.parent() {
            match parent.kind() {
                JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
                | JsSyntaxKind::TS_AS_EXPRESSION
                | JsSyntaxKind::TS_SATISFIES_EXPRESSION
                | JsSyntaxKind::JS_SPREAD
                | JsSyntaxKind::JS_ARRAY_ELEMENT_LIST
                | JsSyntaxKind::JS_ARRAY_EXPRESSION
                | JsSyntaxKind::JS_OBJECT_MEMBER_LIST
                | JsSyntaxKind::JS_OBJECT_EXPRESSION
                | JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER
                | JsSyntaxKind::JS_CALL_ARGUMENT_LIST => node = parent,
                JsSyntaxKind::JS_CALL_ARGUMENTS => {
                    let Some(call) = parent.parent().and_then(JsCallExpression::cast) else {
                        return false;
                    };
                    if self.imported_callee(&call).is_none() {
                        return false;
                    }
                    node = call.into_syntax();
                }
                JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => return true,
                JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                    return parent
                        .parent()
                        .and_then(JsVariableDeclarator::cast)
                        .is_some_and(|declarator| self.is_plain_variable(&declarator));
                }
                _ => return false,
            }
        }
        false
    }

    fn evaluate_call(&mut self, call: &JsCallExpression) -> Option<ConfigValue> {
        let (specifier, export) = self.imported_callee(call)?;
        let arguments = call
            .arguments()
            .ok()?
            .args()
            .iter()
            .map(|argument| match argument {
                Ok(AnyJsCallArgument::AnyJsExpression(expression)) => self.evaluate(&expression),
                _ => ConfigValue::Unknown,
            })
            .collect();

        Some(ConfigValue::Call(ConfigCall {
            specifier,
            export,
            arguments,
        }))
    }

    /// Returns the import specifier and the export name of the function
    /// that `call` calls, or `None` when the function isn't imported.
    fn imported_callee(&self, call: &JsCallExpression) -> Option<(Text, Text)> {
        let callee = call.callee().ok()?.omit_parentheses();
        let callee = callee.as_js_identifier_expression()?;
        let binding = self.model.binding(&callee.name().ok()?)?;
        let declaration = binding.tree().declaration()?;
        let export: Text = match &declaration {
            AnyJsBindingDeclaration::JsDefaultImportSpecifier(_) => Text::new_static("default"),
            AnyJsBindingDeclaration::JsNamedImportSpecifier(specifier) => {
                specifier.name().ok()?.inner_string_text().ok()?.into()
            }
            AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_) => binding
                .tree()
                .name_token()
                .ok()?
                .token_text_trimmed()
                .into(),
            _ => return None,
        };
        let specifier = declaration
            .syntax()
            .ancestors()
            .find_map(JsImport::cast)?
            .source_text()
            .ok()?;
        // Escape sequences in names and specifiers aren't decoded.
        if export.text().contains('\\') || specifier.contains('\\') {
            return None;
        }
        Some((specifier.into(), export))
    }
}

/// Returns the name of a property, or `None` when the name can't be known
/// exactly, such as `[key]`, or a number that doesn't name itself, such as
/// `0x10`, which names the property `16`.
fn property_name(name: &AnyJsObjectMemberName) -> Option<Text> {
    match name {
        AnyJsObjectMemberName::JsLiteralMemberName(name) => {
            let token = name.value().ok()?;
            let text = token.token_text_trimmed();
            match token.kind() {
                JsSyntaxKind::JS_STRING_LITERAL => {
                    try_unescape_js_string(inner_string_text(&token))
                }
                // A number names a property with its shortest decimal form.
                // Integers up to 15 digits without leading zeros are already
                // in that form.
                JsSyntaxKind::JS_NUMBER_LITERAL => (text.text().len() <= 15
                    && (text.text() == "0" || !text.starts_with('0'))
                    && text.bytes().all(|byte| byte.is_ascii_digit()))
                .then(|| text.into()),
                _ => (!text.contains('\\')).then(|| text.into()),
            }
        }
        AnyJsObjectMemberName::JsComputedMemberName(name) => {
            match name.expression().ok()?.omit_parentheses() {
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsStringLiteralExpression(string),
                ) => try_unescape_js_string(string.inner_string_text().ok()?),
                _ => None,
            }
        }
        AnyJsObjectMemberName::JsMetavariable(_) => None,
    }
}

fn evaluate_literal(literal: &AnyJsLiteralExpression) -> Option<ConfigValue> {
    match literal {
        AnyJsLiteralExpression::JsStringLiteralExpression(string) => Some(ConfigValue::String {
            value: try_unescape_js_string(string.inner_string_text().ok()?)?,
            range: string.syntax().text_trimmed_range(),
        }),
        AnyJsLiteralExpression::JsNumberLiteralExpression(number) => {
            parse_js_number_with_single_rounding(number.value_token().ok()?.text_trimmed())
                .map(ConfigValue::Number)
        }
        AnyJsLiteralExpression::JsBooleanLiteralExpression(boolean) => Some(ConfigValue::Boolean(
            boolean.value_token().ok()?.text_trimmed() == "true",
        )),
        _ => None,
    }
}

/// Evaluates a template literal without substitutions, such as
/// `` `./src/styles.css` ``.
fn evaluate_template(template: &JsTemplateExpression) -> Option<ConfigValue> {
    if template.tag().is_some() {
        return None;
    }
    let mut value = String::new();
    for element in template.elements().iter() {
        let AnyJsTemplateElement::JsTemplateChunkElement(chunk) = element else {
            return None;
        };
        let chunk = chunk.template_chunk_token().ok()?;
        // Escape sequences aren't decoded, and a template turns each line
        // ending into `\n`, so a chunk with either is unknown.
        if chunk.text_trimmed().contains(['\\', '\r']) {
            return None;
        }
        value.push_str(chunk.text_trimmed());
    }
    Some(ConfigValue::String {
        value: value.into(),
        range: template.syntax().text_trimmed_range(),
    })
}
