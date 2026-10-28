use crate::services::vue::VueSemantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, HtmlSyntaxKind, T, VueDirective, VueModifier,
    element_ext::AnyHtmlTagElement,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstNodeList, TextRange, TextSize};
use biome_rule_options::use_vue_valid_v_model::UseVueValidVModelOptions;
use biome_string_case::StrLikeExtension;
use biome_vue_semantic::{Assignability, RefBase, Reference, RootShape, Snippet, SymbolKind};

declare_lint_rule! {
    /// Enforce valid `v-model` directives.
    ///
    /// `v-model` keeps a form field and a variable in sync: when the user
    /// types, Vue writes the new value back to the variable. This only works
    /// in some places, and only with a value that Vue can write to.
    ///
    /// This rule reports `v-model` directives that:
    ///
    /// - are placed on an element other than `<input>`, `<select>`,
    ///   `<textarea>`, or a component, such as `<div v-model="foo">`;
    /// - are placed on a file input, such as `<input type="file" v-model="foo">`;
    ///   the browser doesn't let code set the file of a file input;
    /// - have an argument on a native element, such as `<input v-model:foo="bar">`;
    ///   only components accept an argument;
    /// - have a modifier other than `.lazy`, `.number`, or `.trim` on a native
    ///   element, such as `<input v-model.foo="bar">`;
    /// - have no value, such as `<input v-model>`;
    /// - have a value that can't be assigned to, such as `<input v-model="foo + bar">`;
    /// - read their value with optional chaining (`?.`), such as `<input v-model="foo?.bar">`;
    /// - read a property of a value that may be `null` or `undefined`, such as
    ///   `<input v-model="(foo?.bar).baz">`;
    /// - write to a variable of a `v-for` or `v-slot`, such as
    ///   `<input v-for="item in items" v-model="item">`; a new value for these
    ///   variables wouldn't reach the list or the slot data they were read from.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-model="foo"></div>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <input v-model:foo="bar" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <input v-model />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <input v-model="foo + bar" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <input v-model="foo?.bar" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <div v-for="todo in todos">
    ///     <input v-model="todo" />
    ///   </div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <input v-model="foo" />
    ///   <input v-model.lazy="foo" />
    ///   <textarea v-model="foo"></textarea>
    ///   <MyComponent v-model="foo" />
    ///   <MyComponent v-model:propName.modifier="foo" />
    ///   <div v-for="todo in todos">
    ///     <input v-model="todo.name" />
    ///   </div>
    /// </template>
    /// ```
    ///
    pub UseVueValidVModel {
        version: "next",
        name: "useVueValidVModel",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-model").same()],
    }
}

pub enum ViolationKind {
    UnsupportedElement(AnyHtmlTagElement),
    FileInput,
    Argument,
    Modifier(VueModifier),
    MissingValue,
    NotAssignable,
    OptionalChain,
    MaybeNullObject,
    /// The value is a variable of a `v-for` or `v-slot`.
    LoopOrSlotVariable(Reference),
}

impl Rule for UseVueValidVModel {
    type Query = VueSemantic<VueDirective>;
    type State = ViolationKind;
    type Signals = Box<[Self::State]>;
    type Options = UseVueValidVModelOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let directive = ctx.query();
        if !directive.is_two_way_binding() {
            return Box::default();
        }
        let Some(element) = enclosing_tag_element(directive) else {
            return Box::default();
        };

        let mut violations = Vec::new();
        let tag_kind = element.tag_name_kind();
        let is_form_field = tag_kind.is_some_and(|kind| FORM_FIELDS.contains(kind));
        let is_component = is_component(&element, tag_kind);
        if !is_form_field && (!is_component || is_unsupported_component(&element)) {
            violations.push(ViolationKind::UnsupportedElement(element.clone()));
        }
        if tag_kind == Some(T![input]) && is_file_input(&element) {
            violations.push(ViolationKind::FileInput);
        }
        if !is_component {
            if directive.arg().is_some() {
                violations.push(ViolationKind::Argument);
            }
            violations.extend(
                directive
                    .modifiers()
                    .iter()
                    .filter(|modifier| {
                        modifier
                            .modifier_token()
                            .is_ok_and(|token| !NATIVE_MODIFIERS.contains(&token.text_trimmed()))
                    })
                    .map(ViolationKind::Modifier),
            );
        }

        if has_empty_value(directive) {
            violations.push(ViolationKind::MissingValue);
        } else if let Some(snippet) =
            ctx.model()
                .element_at(directive.range())
                .and_then(|element| {
                    element
                        .attrs()
                        .find(|attr| attr.range() == directive.range())?
                        .value_snippet()
                })
        {
            violations.extend(value_violation(&snippet));
        }

        violations.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let directive = ctx.query();
        let diagnostic = match state {
            ViolationKind::UnsupportedElement(element) => {
                let tag = element.tag_name()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    directive.range(),
                    markup! {
                        "The "<Emphasis>"v-model"</Emphasis>" directive can't be used on "<Emphasis>"<"{tag.text()}">"</Emphasis>" elements."
                    },
                )
                .note(markup! {
                    "Vue can only keep "<Emphasis>"<input>"</Emphasis>", "<Emphasis>"<select>"</Emphasis>", "<Emphasis>"<textarea>"</Emphasis>", and components in sync with a variable."
                })
                .note(markup! {
                    "Move the directive to a form field or a component, or bind the value with "<Emphasis>"v-bind"</Emphasis>" and listen for changes with "<Emphasis>"v-on"</Emphasis>"."
                })
            }
            ViolationKind::FileInput => RuleDiagnostic::new(
                rule_category!(),
                directive.range(),
                markup! {
                    "The "<Emphasis>"v-model"</Emphasis>" directive can't be used on file inputs."
                },
            )
            .note(markup! {
                "Browsers don't let code set the file of a file input, so Vue can't keep it in sync with a variable."
            })
            .note(markup! {
                "Read the selected files in a "<Emphasis>"change"</Emphasis>" event listener instead, e.g. "<Emphasis>"@change=\"onFileChange\""</Emphasis>"."
            }),
            ViolationKind::Argument => RuleDiagnostic::new(
                rule_category!(),
                directive.arg()?.range(),
                markup! {
                    "The "<Emphasis>"v-model"</Emphasis>" directive doesn't accept an argument on native elements."
                },
            )
            .note(markup! {
                "Only components accept an argument, which names the prop to keep in sync."
            })
            .note(markup! {
                "Remove the argument."
            }),
            ViolationKind::Modifier(modifier) => RuleDiagnostic::new(
                rule_category!(),
                modifier.range(),
                markup! {
                    "The "<Emphasis>"v-model"</Emphasis>" directive doesn't support this modifier on native elements."
                },
            )
            .note(markup! {
                "On native elements, "<Emphasis>"v-model"</Emphasis>" only supports the "<Emphasis>"lazy"</Emphasis>", "<Emphasis>"number"</Emphasis>", and "<Emphasis>"trim"</Emphasis>" modifiers."
            })
            .note(markup! {
                "Remove or correct the modifier."
            }),
            ViolationKind::MissingValue => RuleDiagnostic::new(
                rule_category!(),
                directive.range(),
                markup! {
                    "This "<Emphasis>"v-model"</Emphasis>" directive is missing a value."
                },
            )
            .note(markup! {
                "The value names the variable that Vue keeps in sync with the field."
            })
            .note(markup! {
                "Add a value to the directive, e.g. "<Emphasis>"v-model=\"message\""</Emphasis>"."
            }),
            ViolationKind::NotAssignable => RuleDiagnostic::new(
                rule_category!(),
                value_range(directive)?,
                markup! {
                    "The value of this "<Emphasis>"v-model"</Emphasis>" directive can't be assigned to."
                },
            )
            .note(markup! {
                "Vue writes the new value of the field back to this expression, so it must be a variable or a property, such as "<Emphasis>"foo"</Emphasis>" or "<Emphasis>"foo.bar"</Emphasis>"."
            })
            .note(markup! {
                "Replace the expression with the variable or property to keep in sync."
            }),
            ViolationKind::OptionalChain => RuleDiagnostic::new(
                rule_category!(),
                value_range(directive)?,
                markup! {
                    "The value of this "<Emphasis>"v-model"</Emphasis>" directive uses optional chaining."
                },
            )
            .note(markup! {
                "Vue writes the new value of the field back to this expression, and a property read with "<Emphasis>"?."</Emphasis>" can't be assigned to."
            })
            .note(markup! {
                "Replace "<Emphasis>"?."</Emphasis>" with "<Emphasis>"."</Emphasis>", and make sure the object exists before the field is shown, e.g. with "<Emphasis>"v-if"</Emphasis>"."
            }),
            ViolationKind::MaybeNullObject => RuleDiagnostic::new(
                rule_category!(),
                value_range(directive)?,
                markup! {
                    "The value of this "<Emphasis>"v-model"</Emphasis>" directive sets a property of a value that may be "<Emphasis>"null"</Emphasis>" or "<Emphasis>"undefined"</Emphasis>"."
                },
            )
            .note(markup! {
                "Setting a property of "<Emphasis>"null"</Emphasis>" or "<Emphasis>"undefined"</Emphasis>" throws an error when the field changes."
            })
            .note(markup! {
                "Bind to a property of an object that always exists, and make sure it exists before the field is shown, e.g. with "<Emphasis>"v-if"</Emphasis>"."
            }),
            ViolationKind::LoopOrSlotVariable(reference) => {
                let name = reference.name();
                let is_slot = reference
                    .symbol()
                    .is_some_and(|symbol| symbol.kind() == SymbolKind::SlotParam);
                let source = if is_slot { "v-slot" } else { "v-for" };
                RuleDiagnostic::new(
                    rule_category!(),
                    reference.host_range(),
                    markup! {
                        "This "<Emphasis>"v-model"</Emphasis>" directive writes to "<Emphasis>{name}</Emphasis>", a variable of "<Emphasis>{source}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Vue doesn't allow assigning to the variables of "<Emphasis>{source}</Emphasis>", because the new value wouldn't reach the data they were read from."
                })
                .note(markup! {
                    "Bind to a property of the variable instead, e.g. "<Emphasis>{name}".name"</Emphasis>", or to the original data."
                })
            }
        };
        Some(diagnostic)
    }
}

const FORM_FIELDS: TokenSet<HtmlSyntaxKind> = token_set![T![input], T![select], T![textarea]];

const NATIVE_MODIFIERS: [&str; 3] = ["lazy", "number", "trim"];

/// Built-in components and elements that `v-model` can't keep in sync with a
/// variable, in both their kebab-case and PascalCase spellings.
const UNSUPPORTED_COMPONENTS: [&str; 6] = [
    "keep-alive",
    "keepalive",
    "slot",
    "transition",
    "transition-group",
    "transitiongroup",
];

fn enclosing_tag_element(directive: &VueDirective) -> Option<AnyHtmlTagElement> {
    directive
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyHtmlTagElement::cast)
}

/// Returns `true` when Vue renders `element` as a component: its name isn't a
/// known HTML or SVG tag, or it has an `is` attribute.
fn is_component(element: &AnyHtmlTagElement, tag_kind: Option<HtmlSyntaxKind>) -> bool {
    tag_kind.is_none_or(|kind| kind == HtmlSyntaxKind::HTML_UNKNOWN_TAG)
        || element.find_attribute_or_vue_binding("is").is_some()
}

fn is_unsupported_component(element: &AnyHtmlTagElement) -> bool {
    element.tag_name().is_some_and(|name| {
        UNSUPPORTED_COMPONENTS
            .binary_search_by(|unsupported| unsupported.cmp_ignore_ascii_case(name.text()))
            .is_ok()
    })
}

fn is_file_input(element: &AnyHtmlTagElement) -> bool {
    element
        .find_attribute_by_name("type")
        .and_then(|attribute| attribute.as_html_attribute()?.html_string())
        .and_then(|string| string.inner_string_text().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("file"))
}

/// Returns `true` when the directive has no value, or a value with only
/// whitespace.
fn has_empty_value(directive: &VueDirective) -> bool {
    let Some(initializer) = directive.initializer() else {
        return true;
    };
    match initializer.value() {
        Ok(AnyHtmlAttributeInitializer::HtmlString(string)) => string
            .inner_string_text()
            .is_ok_and(|text| text.trim().is_empty()),
        _ => false,
    }
}

/// Returns the range of the directive's value, without its quotes and the
/// whitespace around it.
fn value_range(directive: &VueDirective) -> Option<TextRange> {
    let initializer = directive.initializer()?.value().ok()?;
    let string = initializer.as_html_string()?;
    let range = string.inner_string_range().ok()?;
    let text = string.inner_string_text().ok()?;
    let text = text.text();
    let start = text.len() - text.trim_start().len();
    let end = text.trim_end().len();
    Some(TextRange::new(
        range.start() + TextSize::from(start as u32),
        range.start() + TextSize::from(end as u32),
    ))
}

/// Returns the problem with the value of a `v-model`, if any.
///
/// A value that failed to parse is not reported: the parser already reports it.
fn value_violation(snippet: &Snippet) -> Option<ViolationKind> {
    match snippet.assignability() {
        Assignability::NotAssignable if *snippet.root() == RootShape::Invalid => None,
        Assignability::NotAssignable => Some(ViolationKind::NotAssignable),
        Assignability::OptionalChain => Some(ViolationKind::OptionalChain),
        Assignability::MaybeNullObject => Some(ViolationKind::MaybeNullObject),
        Assignability::Assignable if *snippet.root() == RootShape::Identifier => snippet
            .references()
            .find(|reference| {
                reference.base() == RefBase::Bare
                    && reference.symbol().is_some_and(|symbol| {
                        matches!(symbol.kind(), SymbolKind::VForAlias | SymbolKind::SlotParam)
                    })
            })
            .map(ViolationKind::LoopOrSlotVariable),
        Assignability::Assignable => None,
    }
}

#[cfg(test)]
mod tests {
    use super::UNSUPPORTED_COMPONENTS;
    use biome_string_case::StrLikeExtension;
    use std::cmp::Ordering;

    #[test]
    fn unsupported_components_are_sorted() {
        for pair in UNSUPPORTED_COMPONENTS.windows(2) {
            assert_eq!(
                pair[0].cmp_ignore_ascii_case(pair[1]),
                Ordering::Less,
                "{} should come after {}",
                pair[0],
                pair[1]
            );
        }
    }
}
