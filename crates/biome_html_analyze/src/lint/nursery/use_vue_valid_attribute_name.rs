use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyVueDirectiveArgument, HtmlAttribute, HtmlAttributeList, HtmlSyntaxToken, VueDirective,
    VueDirectiveArgument, VueVBindShorthandDirective, element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::use_vue_valid_attribute_name::UseVueValidAttributeNameOptions;
use biome_unicode_table::{Dispatch, lookup_byte};

use crate::vue::is_component;

declare_lint_rule! {
    /// Require valid attribute names in Vue templates.
    ///
    /// An attribute name must start with a letter, an underscore (`_`), or a colon (`:`).
    /// The rest of the name can only contain letters, digits, hyphens (`-`), underscores,
    /// periods (`.`), and colons. Names such as `0abc` or `-def` break these rules.
    /// Vue sets attributes from JavaScript when it renders a template, and browsers can
    /// throw an error when asked to set an attribute with an invalid name.
    ///
    /// The rule checks plain attributes, such as `<p foo>`, and the names bound with
    /// `v-bind`, such as `<p v-bind:foo="value">` or `<p :foo="value">`.
    /// Dynamic names, such as `<p :[name]="value">`, aren't checked because their value
    /// is only known when the template runs.
    ///
    /// Attributes on components, such as `<MyComponent 0abc="value">`, aren't checked
    /// either, because Vue passes them to the component as props instead of setting
    /// them on an HTML element.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <p 0abc></p>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <p -def></p>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <p v-bind:0abc="value"></p>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <p :0abc="value"></p>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <p foo></p>
    ///   <p foo-bar="value"></p>
    ///   <p _foo-bar></p>
    ///   <p :foo-bar="value"></p>
    ///   <p :[name]="value"></p>
    ///   <MyComponent 0abc="value" />
    ///   <MyComponent :0abc="value" />
    /// </template>
    /// ```
    ///
    pub UseVueValidAttributeName {
        version: "next",
        name: "useVueValidAttributeName",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-attribute-name").same()],
    }
}

declare_node_union! {
    pub AnyVueNamedAttribute = HtmlAttribute | VueDirective | VueVBindShorthandDirective
}

impl Rule for UseVueValidAttributeName {
    type Query = Ast<AnyVueNamedAttribute>;
    /// The token holding the invalid attribute name.
    type State = HtmlSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = UseVueValidAttributeNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }

        let node = ctx.query();
        let name = match node {
            AnyVueNamedAttribute::HtmlAttribute(attribute) => {
                attribute.name().ok()?.value_token().ok()?
            }
            AnyVueNamedAttribute::VueDirective(directive) => {
                if !directive.is_binding() {
                    return None;
                }
                static_argument_token(directive.arg()?)?
            }
            AnyVueNamedAttribute::VueVBindShorthandDirective(directive) => {
                static_argument_token(directive.arg().ok()?)?
            }
        };
        if is_valid_attribute_name(name.text_trimmed()) {
            return None;
        }

        let element = node
            .parent::<HtmlAttributeList>()?
            .parent::<AnyHtmlTagElement>()?;
        if is_component(&element) {
            return None;
        }

        Some(name)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, name: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                name.text_trimmed_range(),
                markup! {
                    "The attribute name "<Emphasis>{name.text_trimmed()}</Emphasis>" isn't valid."
                },
            )
            .note(markup! {
                "Browsers can throw an error when Vue sets an attribute with an invalid name."
            })
            .note(markup! {
                "Start the name with a letter, an underscore, or a colon, and use only letters, digits, hyphens, underscores, periods, and colons after it."
            }),
        )
    }
}

/// Returns the name token of a static directive argument, such as `foo` in `:foo`.
///
/// Returns `None` for missing and dynamic arguments, such as `:[foo]`.
fn static_argument_token(argument: VueDirectiveArgument) -> Option<HtmlSyntaxToken> {
    match argument.arg()? {
        AnyVueDirectiveArgument::VueStaticArgument(argument) => argument.name_token().ok(),
        _ => None,
    }
}

/// Returns `true` if `name` matches the ASCII subset of the XML
/// [`Name`](https://www.w3.org/TR/xml/#NT-Name) production, which browsers use to
/// validate attribute names set from JavaScript.
///
/// Non-ASCII bytes are accepted because the HTML parser already reports non-ASCII
/// characters in Vue attribute names and directive arguments.
fn is_valid_attribute_name(name: &str) -> bool {
    let mut bytes = name.bytes().map(lookup_byte);
    bytes.next().is_some_and(is_name_start)
        && bytes.all(|dispatch| {
            is_name_start(dispatch)
                || matches!(
                    dispatch,
                    Dispatch::MIN | Dispatch::PRD | Dispatch::ZER | Dispatch::DIG
                )
        })
}

#[inline]
fn is_name_start(dispatch: Dispatch) -> bool {
    matches!(dispatch, Dispatch::IDT | Dispatch::COL | Dispatch::UNI)
}

#[cfg(test)]
mod tests {
    use super::is_valid_attribute_name;

    #[test]
    fn accepts_valid_names() {
        for name in ["foo", "_foo-bar", "foo.bar", "quux-.9", "xlink:href", ":foo"] {
            assert!(is_valid_attribute_name(name), "`{name}` should be valid");
        }
    }

    #[test]
    fn rejects_invalid_names() {
        for name in ["", "0abc", "-def", ".foo", "!ghi", "a$b", "a#b", "a&b"] {
            assert!(!is_valid_attribute_name(name), "`{name}` should be invalid");
        }
    }
}
