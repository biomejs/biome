use crate::semantic_services::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_rowan::{AstNode, TextRange};
use rome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsObjectBindingPatternMember,
    JsArrayBindingPatternElementList, JsForVariableDeclaration, JsIdentifierAssignment,
    JsIdentifierBinding, JsObjectBindingPatternPropertyList, JsVariableDeclaration,
    JsVariableDeclarator, JsVariableDeclaratorList,
};

declare_rule! {
    /// Prevents from having `const` variables being re-assigned.
    ///
    /// Trying to assign a value to a `const` will cause an `TypeError` when the code is executed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a = 1;
    /// a = 4;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = 2;
    /// a += 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = 1;
    /// ++a;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = 1, b = 2;
    ///
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = 10;
    /// let b = 10;
    /// b = 20;
    /// ```
    ///
    pub(crate) NoConstAssign {
        version: "1.0.0",
        name: "noConstAssign",
        recommended: true,
    }
}

impl Rule for NoConstAssign {
    type Query = Semantic<JsIdentifierAssignment>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        let declared_binding = model.binding(node)?;

        if let Some(possible_declarator) = declared_binding.syntax().ancestors().find(|node| {
            !AnyJsObjectBindingPatternMember::can_cast(node.kind())
                && !JsObjectBindingPatternPropertyList::can_cast(node.kind())
                && !AnyJsArrayBindingPatternElement::can_cast(node.kind())
                && !JsArrayBindingPatternElementList::can_cast(node.kind())
                && !JsIdentifierBinding::can_cast(node.kind())
        }) {
            if JsVariableDeclarator::can_cast(possible_declarator.kind()) {
                let possible_declaration = possible_declarator.parent()?;
                if let Some(js_for_variable_declaration) =
                    JsForVariableDeclaration::cast_ref(&possible_declaration)
                {
                    if js_for_variable_declaration.is_const() {
                        return Some(declared_binding.syntax().text_trimmed_range());
                    }
                } else if let Some(js_variable_declaration) =
                    JsVariableDeclaratorList::cast_ref(&possible_declaration)
                        .and_then(|declaration| declaration.syntax().parent())
                        .and_then(JsVariableDeclaration::cast)
                {
                    if js_variable_declaration.is_const() {
                        return Some(declared_binding.syntax().text_trimmed_range());
                    }
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let name = node.name_token().ok()?;
        let name = name.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {"Can't assign "<Emphasis>{name}</Emphasis>" because it's a constant"},
            )
            .detail(
                state,
                markup! {"This is where the variable is defined as constant"},
            ),
        )
    }
}
