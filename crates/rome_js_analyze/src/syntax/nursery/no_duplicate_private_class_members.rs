use std::collections::{HashMap, HashSet};

use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};

use rome_diagnostics::category;
use rome_js_syntax::{AnyJsClassMember, JsClassMemberList, TextRange};
use rome_rowan::AstNode;

declare_rule! {
    /// Catch a `SyntaxError` when defining duplicate private class members.
    ///
    /// ## Examples
    ///
    /// ```js
    /// class A {
    ///   #foo;
    ///   #foo;
    //  }
    /// ```
    pub(crate) NoDuplicatePrivateClassMembers {
        version: "1.0.0",
        name: "noDuplicatePrivateClassMembers",
        recommended: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MemberType {
    Normal,
    Getter,
    Setter,
}

impl Rule for NoDuplicatePrivateClassMembers {
    type Query = Ast<JsClassMemberList>;
    type State = (String, TextRange);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut defined_members: HashMap<String, HashSet<MemberType>> = HashMap::new();

        let node = ctx.query();
        node.into_iter()
            .filter_map(|member| {
                let member_name = member
                    .name()
                    .ok()??
                    .as_js_private_class_member_name()?
                    .text();
                let member_type = match member {
                    AnyJsClassMember::JsGetterClassMember(_) => MemberType::Getter,
                    AnyJsClassMember::JsMethodClassMember(_) => MemberType::Normal,
                    AnyJsClassMember::JsPropertyClassMember(_) => MemberType::Normal,
                    AnyJsClassMember::JsSetterClassMember(_) => MemberType::Setter,
                    _ => return None,
                };

                if let Some(stored_members) = defined_members.get_mut(&member_name) {
                    if stored_members.contains(&MemberType::Normal)
                        || stored_members.contains(&member_type)
                        || member_type == MemberType::Normal
                    {
                        return Some((member_name, member.range()));
                    } else {
                        stored_members.insert(member_type);
                    }
                } else {
                    defined_members.insert(member_name, HashSet::from([member_type]));
                }

                None
            })
            .collect()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (member_name, range) = state;
        let diagnostic = RuleDiagnostic::new(
            category!("parse/noDuplicatePrivateClassMembers"),
            range,
            format!("Duplicate private class member {:?}", member_name),
        );

        Some(diagnostic)
    }
}
