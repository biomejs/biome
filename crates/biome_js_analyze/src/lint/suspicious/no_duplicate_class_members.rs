use biome_analyze::RuleSource;
use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsClassMemberName, JsClassMemberList, JsGetterClassMember, JsMethodClassMember,
    JsPropertyClassMember, JsSetterClassMember, JsStaticModifier, JsSyntaxList, TextRange,
};
use biome_rowan::{declare_node_union, AstNode};
use biome_rowan::{AstNodeList, TokenText};
use rustc_hash::{FxHashMap, FxHashSet};

declare_lint_rule! {
    /// Disallow duplicate class members.
    ///
    /// If there are declarations of the same name among class members,
    /// the last declaration overwrites other declarations silently.
    /// It can cause unexpected behaviours.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   bar() { }
    ///   bar() { }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   bar() { }
    ///   get bar() { }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   bar;
    ///   bar() { }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   static bar() { }
    ///   static bar() { }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class Foo {
    ///   bar() { }
    ///   qux() { }
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   set bar(value) { }
    ///   get bar() { }
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   bar;
    ///   qux;
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   bar;
    ///   qux() { }
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   static bar() { }
    ///   bar() { }
    /// }
    /// ```
    ///
    pub NoDuplicateClassMembers {
        version: "1.0.0",
        name: "noDuplicateClassMembers",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-dupe-class-members"),
            RuleSource::EslintTypeScript("no-dupe-class-members")
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

fn get_member_name(node: &AnyJsClassMemberName) -> Option<TokenText> {
    match node {
        AnyJsClassMemberName::JsLiteralMemberName(node) => node.name().ok(),
        _ => None,
    }
}

fn is_static_member(node: JsSyntaxList) -> bool {
    node.into_iter().any(|m| {
        if let biome_rowan::SyntaxSlot::Node(node) = m {
            JsStaticModifier::can_cast(node.kind())
        } else {
            false
        }
    })
}

declare_node_union! {
    pub AnyClassMemberDefinition = JsGetterClassMember | JsMethodClassMember | JsPropertyClassMember | JsSetterClassMember
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MemberType {
    Normal,
    Getter,
    Setter,
}

impl AnyClassMemberDefinition {
    fn name(&self) -> Option<AnyJsClassMemberName> {
        match self {
            AnyClassMemberDefinition::JsGetterClassMember(node) => node.name().ok(),
            AnyClassMemberDefinition::JsMethodClassMember(node) => node.name().ok(),
            AnyClassMemberDefinition::JsPropertyClassMember(node) => node.name().ok(),
            AnyClassMemberDefinition::JsSetterClassMember(node) => node.name().ok(),
        }
    }

    fn modifiers_list(&self) -> JsSyntaxList {
        match self {
            AnyClassMemberDefinition::JsGetterClassMember(node) => {
                node.modifiers().into_syntax_list()
            }
            AnyClassMemberDefinition::JsMethodClassMember(node) => {
                node.modifiers().into_syntax_list()
            }
            AnyClassMemberDefinition::JsPropertyClassMember(node) => {
                node.modifiers().into_syntax_list()
            }
            AnyClassMemberDefinition::JsSetterClassMember(node) => {
                node.modifiers().into_syntax_list()
            }
        }
    }

    fn range(&self) -> TextRange {
        match self {
            AnyClassMemberDefinition::JsGetterClassMember(node) => node.range(),
            AnyClassMemberDefinition::JsMethodClassMember(node) => node.range(),
            AnyClassMemberDefinition::JsPropertyClassMember(node) => node.range(),
            AnyClassMemberDefinition::JsSetterClassMember(node) => node.range(),
        }
    }

    fn member_type(&self) -> MemberType {
        match self {
            AnyClassMemberDefinition::JsGetterClassMember(_) => MemberType::Getter,
            AnyClassMemberDefinition::JsMethodClassMember(_) => MemberType::Normal,
            AnyClassMemberDefinition::JsPropertyClassMember(_) => MemberType::Normal,
            AnyClassMemberDefinition::JsSetterClassMember(_) => MemberType::Setter,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct MemberState {
    name: String,
    is_static: bool,
}

impl Rule for NoDuplicateClassMembers {
    type Query = Ast<JsClassMemberList>;
    type State = AnyClassMemberDefinition;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut defined_members: FxHashMap<MemberState, FxHashSet<MemberType>> =
            FxHashMap::default();

        let node = ctx.query();
        node.into_iter()
            .filter_map(|member| {
                let member = AnyClassMemberDefinition::cast(member.into_syntax())?;
                let member_name_node = member.name()?;
                let member_state = MemberState {
                    name: get_member_name(&member_name_node)?.to_string(),
                    is_static: is_static_member(member.modifiers_list()),
                };

                let member_type = member.member_type();
                if let Some(stored_members) = defined_members.get_mut(&member_state) {
                    if stored_members.contains(&MemberType::Normal)
                        || stored_members.contains(&member_type)
                        || member_type == MemberType::Normal
                    {
                        return Some(member);
                    } else {
                        stored_members.insert(member_type);
                    }
                } else {
                    defined_members
                        .entry(member_state)
                        .or_default()
                        .insert(member_type);
                }

                None
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range(),
            format!(
                "Duplicate class member name {:?}",
                get_member_name(&state.name()?)?.text()
            ),
        );

        Some(diagnostic)
    }
}
