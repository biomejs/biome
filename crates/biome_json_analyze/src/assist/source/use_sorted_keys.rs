use crate::JsonRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, context::RuleContext, declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make::{json_member_list, token};
use biome_json_syntax::{JsonMember, JsonMemberList, JsonObjectValue, T, TextRange};
use biome_rowan::{AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt};
use biome_string_case::StrLikeExtension;
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
        fix_kind: FixKind::Safe,
    }
}

#[derive(Debug)]
pub struct MemberKey {
    node: JsonMember,
}
impl Eq for MemberKey {}
impl PartialEq for MemberKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Ord for MemberKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Keep the order for elements that cannot be compared
        let Ok(self_name) = self.node.name().and_then(|name| name.inner_string_text()) else {
            return Ordering::Equal;
        };
        let Ok(other_name) = other.node.name().and_then(|name| name.inner_string_text()) else {
            return Ordering::Equal;
        };
        self_name.text().ascii_nat_cmp(other_name.text())
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

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("assist/source/useSortedKeys"),
            Self::text_range(ctx, state),
            markup! {
                "The keys are not sorted."
            },
        ))
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query()
            .syntax()
            .ancestors()
            .find_map(JsonObjectValue::cast)
            .map(|node| node.range())
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsonRuleAction> {
        let list = state.to_sorted_node();
        let mut mutation = ctx.root().begin();
        let node = ctx.query().clone();
        mutation.replace_node(node, list);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "They keys of the current object can be sorted."
            },
            mutation,
        ))
    }
}
