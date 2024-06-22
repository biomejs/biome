use crate::JsonRuleAction;
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, RefactorKind, Rule,
    RuleAction,
};
use biome_console::markup;
use biome_json_factory::make::{json_member_list, token};
use biome_json_syntax::{JsonMember, JsonMemberList, T};
use biome_rowan::{AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeSet;

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding stylelint rule (if any):
    ///
    pub UseSortedKeys {
        version: "next",
        name: "useSortedKeys",
        language: "json",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

#[derive(Eq, PartialEq)]
pub struct MemberKey {
    node: JsonMember,
}

impl Ord for MemberKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort imports using natural ordering
        natord::compare(
            &self.node.name().unwrap().text(),
            &other.node.name().unwrap().text(),
        )
    }
}

impl PartialOrd for MemberKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Members(pub BTreeSet<MemberKey>);

impl Members {
    /// Returns true if the nodes in the group are already sorted in the file
    fn is_sorted(&self) -> bool {
        // The imports are sorted if the text position of each node in the `BTreeMap`
        // (sorted in natural order) is higher than the previous item in
        // the sequence
        let mut iter = self
            .0
            .iter()
            .map(|node| node.node.syntax().text_range().start());
        let mut previous_start = iter.next().unwrap_or_default();
        iter.all(|start| {
            let is_sorted = previous_start < start;
            previous_start = start;
            is_sorted
        })
    }

    fn to_sorted_node(&self) -> JsonMemberList {
        let items = self.0.iter().map(|key| key.node.clone().detach());

        let separator_count = items.len().saturating_sub(1);

        let mut separators = Vec::new();

        for (index, _) in self.0.iter().enumerate() {
            if index != separator_count {
                separators.push(token(T![,]))
            }
        }

        json_member_list(items, separators)
    }
}

impl Rule for UseSortedKeys {
    type Query = Ast<JsonMemberList>;
    type State = Members;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if node.is_empty() {
            return None;
        }

        let state = node
            .iter()
            .filter_map(|node| {
                let node = node.ok()?;
                Some(MemberKey { node })
            })
            .collect::<BTreeSet<_>>();

        let state = Members(state);

        if !state.is_sorted() {
            Some(state)
        } else {
            None
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsonRuleAction> {
        let list = state.to_sorted_node();
        let mut mutation = ctx.root().begin();
        let node = ctx.query().clone();
        mutation.replace_node(node, list);

        Some(RuleAction::new(
            ActionCategory::Refactor(RefactorKind::Other(Cow::Borrowed("useSortedKeys"))),
            ctx.metadata().applicability(),
            markup! {
                "They keys of the current object can be sorted."
            },
            mutation,
        ))
    }
}
