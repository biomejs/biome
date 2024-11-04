use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsTemplateElement, JsTemplateExpression};
use biome_rowan::{AstNode, AstNodeList};

declare_lint_rule! {
    /// Disallow unnecessary `String.raw` function in template string literals without any escape sequence.
    ///
    /// `String.raw` is useless when contains a raw string without any escape-like sequence.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// String.raw`a`;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// String.raw`a ${v}`;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// String.raw`\n ${a}`;
    /// ```
    ///
    /// ```js
    /// String.raw`\n`;
    /// ```
    pub NoUselessStringRaw {
        version: "1.9.4",
        name: "noUselessStringRaw",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoUselessStringRaw {
    type Query = Ast<JsTemplateExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let tag = node.tag()?;
        let tag = tag.as_js_static_member_expression()?;

        let object = tag.object().ok()?;
        let object_expr = object.as_js_identifier_expression()?;
        let object_name = object_expr.name().ok()?.value_token().ok()?;
        let object_name = object_name.text_trimmed();

        let member = tag.member().ok()?;
        let member_name = member.as_js_name()?.value_token().ok()?;
        let member_name = member_name.text_trimmed();

        if object_name != "String" || member_name != "raw" {
            return None;
        }

        if can_remove_string_raw(node) {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "String.raw is useless when the raw string doesn't contain any escape sequence."
                },
            )
            .note(markup! {
                "Remove the String.raw call beacause it's useless here, String.raw can deal with string which contains escape sequence like \\n, \\t, \\r, \\\\, \\\", \\\'."
            }),
        )
    }
}

fn can_remove_string_raw(node: &JsTemplateExpression) -> bool {
    !node.elements().iter().any(|element| {
        match element {
            AnyJsTemplateElement::JsTemplateElement(_) => false,
            AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                match chunk.template_chunk_token() {
                    Ok(token) => token.text().contains('\\'),
                    Err(_) => {
                        // if found an error, return `true` means `String.raw` can't remove
                        true
                    }
                }
            }
        }
    })
}
