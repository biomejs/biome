use bpaf::Bpaf;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, VisitNode};
use rome_js_syntax::{
    AnyTsType, TsDefaultTypeClause, TsIntersectionTypeElementList, TsParenthesizedType,
    TsReturnTypeAnnotation, TsThisParameter, TsTypeAnnotation, TsTypeArgumentList, TsTypeParameter,
    TsUnionTypeVariantList,
};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::{AstNode, SyntaxNode, SyntaxNodeCast};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_rule! {
    ///
    /// Disallow void type outside of generic or return types. like [no-invalid-void-type](https://typescript-eslint.io/rules/no-invalid-void-type/)
    ///
    /// void in TypeScript refers to a function return that is meant to be ignored. Attempting to use a void type outside of a return type or generic type argument is often a sign of programmer error. void can also be misleading for other developers even if used correctly.
    ///
    /// > The void type means cannot be mixed with any other types, other than never, which accepts all types. If you think you need this then you probably want the undefined type instead.
    ///
    /// ## Examples
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// type PossibleValues = number | void;
    /// type MorePossibleValues = string | ((number & any) | (string | void));
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function logSomething(thing: void) {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// interface Interface {
    ///     prop: void;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let foo: void;
    /// let bar = 1 as unknown as void;
    /// let baz = 1 as unknown as void | string;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// function foo(): void {};
    /// ```
    ///
    /// ## Options
    ///
    /// ### allowInGenericTypeArguments
    ///
    /// This option lets you control if void can be used as a valid value for generic type parameters.
    ///
    /// This option is `true` by default.
    ///
    /// The following patterns are considered warnings with `{ allowInGenericTypeArguments: false }`:
    ///
    /// ```ts
    /// logAndReturn<void>(undefined);
    ///
    /// let voidPromise: Promise<void> = new Promise<void>(() => {});
    /// let voidMap: Map<string, void> = new Map<string, void>();
    /// ```
    ///
    /// ### allowAsThisParameter
    ///
    ///
    ///
    ///
    /// This option allows specifying a this parameter of a function to be void when set to true. This pattern can be useful to explicitly label function types that do not use a this argument. See the TypeScript docs for more information.
    /// This option is `false` by default.
    ///
    /// The following patterns are considered warnings with `{ allowAsThisParameter: false }` but valid with `{ allowAsThisParameter: true }`:
    ///
    /// ```ts
    /// function doThing(this: void) {}
    /// class Example {
    ///     static helper(this: void) {}
    ///     callback(this: void) {}
    /// }
    /// ```
    ///
    pub(crate) NoConfusingVoidType {
        version: "1.0.0",
        name: "noConfusingVoidType",
        recommended: false,
    }
}

/// rule options for `noConfusingVoidType`
#[derive(Debug, Clone, Deserialize, Serialize, Bpaf)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoConfusingVoidTypeOptions {
    /// This option lets you control if void can be used as a valid value for generic type parameters.
    allow_in_generic_type_arguments: bool,
    /// This option allows specifying a this parameter of a function to be void when set to true. This pattern can be useful to explicitly label function types that do not use a this argument.
    allow_as_this_parameter: bool,
}

impl NoConfusingVoidTypeOptions {
    const KNOWN_KEYS: &'static [&'static str] =
        &["allowInGenericTypeArguments", "allowAsThisParameter"];
}

impl Default for NoConfusingVoidTypeOptions {
    fn default() -> Self {
        Self {
            allow_in_generic_type_arguments: true,
            allow_as_this_parameter: false,
        }
    }
}

type Language = <AnyTsType as AstNode>::Language;

// We only focus on union type
pub enum VoidTypeIn {
    Union,
    Unknown,
}

impl Rule for NoConfusingVoidType {
    type Query = Ast<AnyTsType>;

    type State = VoidTypeIn;

    type Signals = Option<Self::State>;

    type Options = NoConfusingVoidTypeOptions;

    fn run(ctx: &rome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if let AnyTsType::TsVoidType(node) = node {
            let result = node_in(node.syntax(), &ctx.options());
            return result;
        }

        None
    }
    fn diagnostic(
        ctx: &rome_analyze::context::RuleContext<Self>,
        state: &Self::State,
    ) -> Option<rome_analyze::RuleDiagnostic> {
        let options = ctx.options();
        let node = ctx.query();
        return Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {{match_message(state, options)}},
        ));
    }
}

fn node_in(
    node: &SyntaxNode<Language>,
    options: &NoConfusingVoidTypeOptions,
) -> Option<VoidTypeIn> {
    let parent = node.parent()?;

    // (string | void)
    if let Some(n) = parent.cast_ref::<TsParenthesizedType>() {
        return node_in(n.syntax(), options);
    }

    // string | void
    if let Some(n) = parent.cast_ref::<TsUnionTypeVariantList>() {
        return node_in(n.syntax(), options);
    }

    // string & void
    if let Some(n) = parent.cast_ref::<TsIntersectionTypeElementList>() {
        return node_in(n.syntax(), options);
    }

    // arg: void
    if let Some(n) = parent.cast_ref::<TsTypeAnnotation>() {
        return node_in(n.syntax(), options);
    }

    // fn<T = void>() {}
    // T = void
    if let Some(n) = parent.cast_ref::<TsDefaultTypeClause>() {
        return node_in(n.syntax(), options);
    }

    // string | void or string & void
    if let Some(n) = parent.cast_ref::<AnyTsType>() {
        return match n {
            AnyTsType::TsUnionType(_) => Some(VoidTypeIn::Union),
            AnyTsType::TsIntersectionType(_) => Some(VoidTypeIn::Unknown),
            _ => None,
        };
    }

    // function fn(this: void) {}
    if parent.cast_ref::<TsThisParameter>().is_some() {
        if options.allow_as_this_parameter {
            return None;
        }

        return Some(VoidTypeIn::Unknown);
    }

    // fn(): void;
    if parent.cast_ref::<TsReturnTypeAnnotation>().is_some() {
        return None;
    }

    // fn<T = void>() {} or Promise<void>
    if parent.cast_ref::<TsTypeParameter>().is_some()
        || parent.cast_ref::<TsTypeArgumentList>().is_some()
    {
        if options.allow_in_generic_type_arguments {
            return None;
        }
        return Some(VoidTypeIn::Unknown);
    }

    Some(VoidTypeIn::Unknown)
}

fn match_message(node: &VoidTypeIn, options: &NoConfusingVoidTypeOptions) -> String {
    if matches!(node, VoidTypeIn::Union) {
        return "void is not valid as a constituent in a union type".to_string();
    }

    format!(
        "void is only valid as a return type{}",
        if options.allow_in_generic_type_arguments {
            " or a type argument in generic type"
        } else {
            ""
        }
    )
}

impl VisitJsonNode for NoConfusingVoidTypeOptions {}
impl VisitNode<JsonLanguage> for NoConfusingVoidTypeOptions {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &Self::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "allowInGenericTypeArguments" => {
                self.allow_in_generic_type_arguments = self
                    .map_to_boolean(&value, &name, diagnostics)
                    .unwrap_or(self.allow_in_generic_type_arguments);
            }
            "allowAsThisParameter" => {
                self.allow_as_this_parameter = self
                    .map_to_boolean(&value, &name, diagnostics)
                    .unwrap_or(self.allow_as_this_parameter);
            }
            _ => {}
        }

        Some(())
    }
}
