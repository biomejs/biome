use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsLiteralExpression, AnyJsObjectMember,
    AnyJsObjectMemberName, AnyJsTemplateElement, JsExpressionTemplateRoot, JsLiteralMemberName,
    JsObjectExpression, JsStringLiteralExpression, JsSyntaxKind, JsTemplateChunkElement,
    JsTemplateExpression, T,
};
use biome_languages::JsFileSource;
use biome_parser::{TokenSet, token_set};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, TextRange, TextSize, declare_node_union,
};
use biome_rule_options::use_vue_separate_static_class::UseVueSeparateStaticClassOptions;

declare_lint_rule! {
    /// Require static class names to be in a separate `class` attribute.
    ///
    /// In Vue, the `:class` binding (also written `v-bind:class`) is for classes that are
    /// computed while the app runs, such as a class that is only added when a condition is true.
    /// A class name that never changes doesn't need a binding: putting it in the plain `class`
    /// attribute keeps the classes that always apply apart from the ones that change, and makes
    /// the template easier to read.
    ///
    /// Vue merges the `class` attribute and the `:class` binding, so an element can use both.
    ///
    /// :::note
    /// This rule reads the template of `.vue` files, which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`. Without that setting, the rule reports nothing.
    /// :::
    ///
    /// This rule reports class names in a `:class` binding that always apply:
    /// - a string, such as `:class="'header'"`;
    /// - a string in an array, such as `:class="['header', theme]"`;
    /// - an object key whose value is `true`, such as `:class="{ header: true }"`;
    /// - a part of a template literal that is separated by spaces from every
    ///   `${}` placeholder, such as `header` in ``:class="`header ${theme}`"``.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div :class="'header'"></div>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div :class="['header', theme]"></div>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div :class="{ header: true, active: isActive }"></div>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div :class="`header ${theme}`"></div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div class="header"></div>
    ///   <div class="header" :class="theme"></div>
    ///   <div class="header" :class="{ active: isActive }"></div>
    ///   <div :class="`header-${theme}`"></div>
    /// </template>
    /// ```
    pub UseVueSeparateStaticClass {
        version: "next",
        name: "useVueSeparateStaticClass",
        language: "js",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("prefer-separate-static-class").same()],
    }
}

declare_node_union! {
    /// A part of a `:class` binding that names classes which always apply.
    pub AnyStaticClass = JsStringLiteralExpression | JsTemplateChunkElement | JsLiteralMemberName
}

impl Rule for UseVueSeparateStaticClass {
    type Query = Ast<JsExpressionTemplateRoot>;
    type State = AnyStaticClass;
    type Signals = Box<[Self::State]>;
    type Options = UseVueSeparateStaticClassOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let embedding_kind = ctx.source_type::<JsFileSource>().as_embedding_kind();
        if !embedding_kind.is_vue() || !embedding_kind.is_class_attribute() {
            return Box::default();
        }
        let mut static_classes = Vec::new();
        if let Some(expression) = ctx.query().expression() {
            collect_static_classes(&expression, &mut static_classes);
        }
        static_classes.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (class_names, range) = match state {
            AnyStaticClass::JsStringLiteralExpression(string) => {
                (string.inner_string_text().ok()?, string.range())
            }
            AnyStaticClass::JsTemplateChunkElement(chunk) => {
                // Highlight the class names without the spaces that separate
                // them from the placeholders.
                let token = chunk.template_chunk_token().ok()?;
                let text = token.text_trimmed();
                let start = token.text_trimmed_range().start();
                let range = TextRange::new(
                    start + TextSize::of(&text[..text.len() - text.trim_start().len()]),
                    start + TextSize::of(text.trim_end()),
                );
                (token.token_text_trimmed(), range)
            }
            AnyStaticClass::JsLiteralMemberName(name) => (name.name().ok()?, name.range()),
        };
        let class_names = ClassNames(class_names.text());
        let (noun, verb) = if class_names.has_several() {
            ("classes", "are")
        } else {
            ("class", "is")
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The static "{noun}" "<Emphasis>{class_names}</Emphasis>" "{verb}" in a "<Emphasis>":class"</Emphasis>" binding."
                },
            )
            .note(markup! {
                "Classes that always apply don't need a binding. Mixing them with the classes that change makes the template harder to read."
            })
            .note(markup! {
                "Move the static classes to the "<Emphasis>"class"</Emphasis>" attribute of the element."
            }),
        )
    }
}

/// Pushes the parts of `expression` that name classes which always apply.
///
/// Only the outer expression, array items, and object keys are inspected,
/// because a class inside a condition or a function call doesn't always apply.
fn collect_static_classes(expression: &AnyJsExpression, static_classes: &mut Vec<AnyStaticClass>) {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string),
        ) => collect_static_string(string, static_classes),
        AnyJsExpression::JsTemplateExpression(template) => {
            collect_static_template_chunks(&template, static_classes);
        }
        AnyJsExpression::JsArrayExpression(array) => {
            for element in array.elements().iter().flatten() {
                if let AnyJsArrayElement::AnyJsExpression(element) = element {
                    collect_static_classes(&element, static_classes);
                }
            }
        }
        AnyJsExpression::JsObjectExpression(object) => {
            collect_static_object_keys(&object, static_classes);
        }
        _ => {}
    }
}

/// Pushes `string` unless it is empty or only holds whitespace, because it
/// then names no class.
fn collect_static_string(
    string: JsStringLiteralExpression,
    static_classes: &mut Vec<AnyStaticClass>,
) {
    if string
        .inner_string_text()
        .is_ok_and(|text| !text.trim().is_empty())
    {
        static_classes.push(string.into());
    }
}

/// Pushes the chunks of an untagged template literal whose classes are not
/// joined to a `${}` placeholder.
///
/// In `` `a ${b}-c d` ``, the chunk `a ` always applies `a`, but the chunk
/// `-c d` is skipped as a whole, because `-c` is part of a class name built
/// from `b`.
fn collect_static_template_chunks(
    template: &JsTemplateExpression,
    static_classes: &mut Vec<AnyStaticClass>,
) {
    if template.tag().is_some() {
        return;
    }
    let elements = template.elements();
    let mut follows_placeholder = false;
    let mut elements = elements.iter().peekable();
    while let Some(element) = elements.next() {
        let chunk = match element {
            AnyJsTemplateElement::JsTemplateChunkElement(chunk) => chunk,
            AnyJsTemplateElement::JsTemplateElement(_) => {
                follows_placeholder = true;
                continue;
            }
        };
        let precedes_placeholder = elements.peek().is_some();
        let Ok(token) = chunk.template_chunk_token() else {
            continue;
        };
        let text = token.text_trimmed();
        let is_static = !text.trim().is_empty()
            && (!follows_placeholder || text.starts_with(char::is_whitespace))
            && (!precedes_placeholder || text.ends_with(char::is_whitespace));
        if is_static {
            static_classes.push(chunk.into());
        }
    }
}

/// Pushes the keys of `object` whose value is `true`.
///
/// A computed key is included when it is a string, as in `{ ['a']: true }`.
fn collect_static_object_keys(
    object: &JsObjectExpression,
    static_classes: &mut Vec<AnyStaticClass>,
) {
    for member in object.members().iter().flatten() {
        let AnyJsObjectMember::JsPropertyObjectMember(property) = member else {
            continue;
        };
        let is_always_applied = property.value().is_ok_and(|value| {
            matches!(
                value.omit_parentheses(),
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsBooleanLiteralExpression(boolean),
                ) if boolean.value_token().is_ok_and(|token| token.kind() == T![true])
            )
        });
        if !is_always_applied {
            continue;
        }
        match property.name() {
            Ok(AnyJsObjectMemberName::JsLiteralMemberName(name)) => {
                if name
                    .value()
                    .is_ok_and(|token| CLASS_NAME_KEY_KINDS.contains(token.kind()))
                    && name.name().is_ok_and(|text| !text.trim().is_empty())
                {
                    static_classes.push(name.into());
                }
            }
            Ok(AnyJsObjectMemberName::JsComputedMemberName(name)) => {
                if let Ok(expression) = name.expression() {
                    match expression.omit_parentheses() {
                        AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(string),
                        ) => collect_static_string(string, static_classes),
                        AnyJsExpression::JsTemplateExpression(template)
                            if template
                                .elements()
                                .iter()
                                .all(|element| element.as_js_template_element().is_none()) =>
                        {
                            collect_static_template_chunks(&template, static_classes);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

/// Writes class names separated by single spaces, so a list written across
/// several lines fits in one diagnostic line.
struct ClassNames<'a>(&'a str);

impl ClassNames<'_> {
    fn has_several(&self) -> bool {
        self.0.split_whitespace().nth(1).is_some()
    }
}

impl biome_console::fmt::Display for ClassNames<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        for (index, class_name) in self.0.split_whitespace().enumerate() {
            if index > 0 {
                fmt.write_str(" ")?;
            }
            fmt.write_str(class_name)?;
        }
        Ok(())
    }
}

/// The tokens of an object key that names a class: `{ a: true }` and
/// `{ 'a': true }`, but not `{ 1: true }`.
const CLASS_NAME_KEY_KINDS: TokenSet<JsSyntaxKind> =
    token_set![T![ident], JsSyntaxKind::JS_STRING_LITERAL];
