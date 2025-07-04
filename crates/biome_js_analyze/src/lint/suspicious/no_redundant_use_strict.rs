use crate::JsRuleAction;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::js_directive;
use biome_js_syntax::{
    AnyJsClass, JsDirective, JsDirectiveList, JsFileSource, JsFunctionBody, JsModule, JsScript,
    JsSyntaxKind, JsSyntaxToken,
};
use biome_rule_options::no_redundant_use_strict::NoRedundantUseStrictOptions;

use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, declare_node_union};

declare_lint_rule! {
 /// Prevents from having redundant `"use strict"`.
 ///
 /// The directive `"use strict"` **isn't** needed in `.mjs` files, or in `.js` files inside projects where the `package.json` defines library as module:
 ///
 ///
 /// ```json,ignore
 /// {
 ///    "type": "module"
 /// }
 /// ```
 ///
 /// Instead, `.cjs` files are considered "scripts" and the directive `"use strict"` is accepted and advised.
 ///
 /// ## Examples
 ///
 /// ### Invalid
 /// ```cjs,expect_diagnostic
 /// "use strict";
 /// function foo() {
 ///  	"use strict";
 /// }
 /// ```
 /// ```cjs,expect_diagnostic
 /// "use strict";
 /// "use strict";
 ///
 /// function foo() {
 ///
 /// }
 /// ```
 /// ```cjs,expect_diagnostic
 /// function foo() {
 /// "use strict";
 /// "use strict";
 /// }
 /// ```
 /// ```cjs,expect_diagnostic
 /// class C1 {
 /// 	test() {
 /// 		"use strict";
 /// 	}
 /// }
 /// ```
 /// ```cjs,expect_diagnostic
 /// const C2 = class {
 /// 	test() {
 /// 		"use strict";
 /// 	}
 /// };
 ///
 /// ```
 /// ### Valid
 /// ```cjs
 /// function foo() {
 ///
 /// }
 ///```
 /// ```cjs
 ///  function foo() {
 ///     "use strict";
 /// }
 /// function bar() {
 ///     "use strict";
 /// }
 ///```
 ///

 pub NoRedundantUseStrict {
        version: "1.0.0",
        name: "noRedundantUseStrict",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! { AnyNodeWithDirectives = JsFunctionBody | JsScript }
impl AnyNodeWithDirectives {
    fn directives(&self) -> JsDirectiveList {
        match self {
            Self::JsFunctionBody(node) => node.directives(),
            Self::JsScript(script) => script.directives(),
        }
    }

    const fn is_script(&self) -> bool {
        matches!(self, Self::JsScript(_))
    }
}
declare_node_union! { pub AnyJsStrictModeNode = AnyJsClass | JsModule | JsDirective  }

impl Rule for NoRedundantUseStrict {
    type Query = Ast<JsDirective>;
    type State = AnyJsStrictModeNode;
    type Signals = Option<Self::State>;
    type Options = NoRedundantUseStrictOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.inner_string_text().ok()? != "use strict" {
            return None;
        }
        let file_source = ctx.source_type::<JsFileSource>();
        let mut outer_most: Option<AnyJsStrictModeNode> = None;
        let root = ctx.root();
        match root {
            biome_js_syntax::AnyJsRoot::JsModule(js_module) => outer_most = Some(js_module.into()),
            _ => {
                for n in node.syntax().ancestors() {
                    match AnyNodeWithDirectives::try_cast(n) {
                        Ok(parent) => {
                            let directives_len = parent.directives().len();
                            for (index, directive) in parent.directives().into_iter().enumerate() {
                                let directive_text = directive.inner_string_text().ok()?;

                                if directive_text == "use strict" {
                                    // if we are analysing a commonjs file, we ignore the first directive that we have at the top, because it's not redundant
                                    if index + 1 == directives_len
                                        && parent.is_script()
                                        && file_source.is_script()
                                        && outer_most.is_none()
                                    {
                                        break;
                                    }
                                    outer_most = Some(directive.into());
                                    break; // continue with next parent
                                }
                            }
                        }
                        Err(n) => {
                            if let Some(module_or_class) = AnyJsClass::cast(n) {
                                outer_most = Some(module_or_class.into());
                            }
                        }
                    }
                }
            }
        }

        if let Some(outer_most) = outer_most {
            // skip itself
            if outer_most.syntax() == node.syntax() {
                return None;
            }
            return Some(outer_most);
        }

        None
    }
    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Redundant "<Emphasis>"use strict"</Emphasis>" directive."
            },
        );

        match state {
            AnyJsStrictModeNode::AnyJsClass(js_class) =>  diag = diag.detail(
                js_class.range(),
                markup! {"All parts of a class's body are already in strict mode."},
            ) ,
            AnyJsStrictModeNode::JsModule(_js_module) => diag= diag.note(
                markup! {"The entire contents of "<Emphasis>"JavaScript modules"</Emphasis>" are automatically in strict mode, with no statement needed to initiate it."},
            ),
            AnyJsStrictModeNode::JsDirective(js_directive) => diag= diag.detail(
                js_directive.range(),
                markup! {"This outer "<Emphasis>"use strict"</Emphasis>" directive already enables strict mode."},
            ),
        }

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let value_token = node.value_token().ok()?;
        let new_node = js_directive(JsSyntaxToken::new_detached(
            JsSyntaxKind::JSX_TEXT_LITERAL,
            "",
            [],
            [],
        ))
        .build()
        .with_leading_trivia_pieces(value_token.leading_trivia().pieces())?;

        mutation.replace_node_discard_trivia(node.clone(), new_node);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the redundant "<Emphasis>"use strict"</Emphasis>" directive." }
                .to_owned(),
            mutation,
        ))
    }
}
