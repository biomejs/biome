use crate::rule_mover::AnalyzerMover;
use crate::version_services::Version;
use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make::{
    json_member, json_member_name, json_string_literal, json_string_value, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonRoot, T};
use biome_rowan::{AstNode, TriviaPieceKind, WalkEvent};
use rustc_hash::FxHashSet;

declare_migration! {
    pub(crate) StyleRules {
        version: "2.0.0",
        name: "styleRules",
    }
}

const STYLE_RULES_THAT_WERE_ERROR: [&str; 23] = [
    "useNumberNamespace",
    "noNonnullAssertion",
    "useAsConstAssertion",
    "noParameterAssign",
    "noInferrableTypes",
    "useNodejsImportProtocol",
    "useExportType",
    "useDefaultParameterLast",
    "noUnusedTemplateLiteral",
    "useExponentiationOperator",
    "useEnumInitializers",
    "useShorthandFunctionType",
    "useLiteralEnumMembers",
    "noUselessElse",
    "useNumericLiterals",
    "noCommaOperator",
    "useConst",
    "noArguments",
    "useSelfClosingElements",
    "useImportType",
    "useTemplate",
    "useSingleVarDeclarator",
    "useWhile",
];

impl Rule for StyleRules {
    type Query = Version<JsonRoot>;
    type State = FxHashSet<Box<str>>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut nodes = FxHashSet::default();
        for rule in STYLE_RULES_THAT_WERE_ERROR {
            nodes.insert(Box::from(rule));
        }

        let events = node.syntax().preorder();

        for event in events {
            match event {
                WalkEvent::Enter(node) => {
                    let Some(node) = JsonMember::cast(node) else {
                        continue;
                    };
                    let node_text = node
                        .name()
                        .ok()
                        .and_then(|node| node.inner_string_text().ok());

                    let Some(node_text) = node_text else {
                        continue;
                    };
                    if node_text == "style" {
                        let list = node
                            .value()
                            .ok()
                            .and_then(|n| n.as_json_object_value().cloned())
                            .map(|n| n.json_member_list());

                        let Some(list) = list else { continue };
                        for item in list {
                            let member = item
                                .ok()
                                .and_then(|n| n.name().ok())
                                .and_then(|n| n.inner_string_text().ok());
                            if let Some(node_text) = member {
                                nodes.remove(&Box::from(node_text.text()));
                            }
                        }
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }

        Some(nodes)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, _node: &Self::State) -> Option<RuleDiagnostic> {
        let root = _ctx.root();
        Some(RuleDiagnostic::new(
            category!("migrate"),
            root.range(),
            markup! {
                "Biome style rules aren't recommended anymore."
            }
                .to_owned(),
        ).note(markup!{
            "To avoid regressions with your code base, Biome will update the configuration file to maintain the compatibility with your current setup."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let mut rule_mover = AnalyzerMover::from_root(ctx.root());

        for rule_to_move in state {
            let member = json_member(
                json_member_name(
                    json_string_literal(rule_to_move.as_ref()).with_leading_trivia(vec![
                        (TriviaPieceKind::Newline, "\n"),
                        (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
                    ]),
                ),
                token(T![:]),
                AnyJsonValue::JsonStringValue(json_string_value(json_string_literal("error"))),
            );
            rule_mover.replace_rule(rule_to_move.as_ref(), member, "style");
        }

        let mutation = rule_mover.run_queries()?;

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Update the configuration to enable these rules."
            }
            .to_owned(),
            mutation,
        ))
    }
}
