use crate::JsonRuleAction;
use biome_analyze::{context::RuleContext, declare_source_rule, Ast, Rule, RuleAction};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_json_factory::make::{json_member_list, token};
use biome_json_syntax::{JsonMember, JsonMemberList, T};
use biome_rowan::{AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeSet;

declare_source_rule! {
    /// Sorts the keys of a JSON object in natural order
    ///
    /// ## Examples
    ///
    /// ```json,expect_diff
    /// {
    ///     "vase": "fancy",
    ///     "nested": {
    ///         "omega": "bar",
    ///         "alpha": "foo"
    ///     }
    /// }
    /// ```
    pub UseSortedKeys {
        version: "1.9.0",
        name: "useSortedKeys",
        language: "json",
    }
}

#[derive(Eq, PartialEq)]
pub struct MemberKey {
    node: JsonMember,
}

impl Ord for MemberKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort keys using natural ordering
        natord::compare(
            &self.node.name().unwrap().to_trimmed_string(),
            &other.node.name().unwrap().to_trimmed_string(),
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
            .map(|node| node.node.syntax().text_range_with_trivia().start());
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
            rule_action_category!(),
            Applicability::Always,
            markup! {
                "They keys of the current object can be sorted."
            },
            mutation,
        ))
    }
}
