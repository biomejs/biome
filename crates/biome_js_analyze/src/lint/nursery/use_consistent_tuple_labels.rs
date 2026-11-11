use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyTsTupleTypeElement, AnyTsType, TsTupleType};
use biome_rowan::AstNode;
use biome_rule_options::use_consistent_tuple_labels::UseConsistentTupleLabelsOptions;

declare_lint_rule! {
    /// Require either all or none of the elements of a tuple type to have labels.
    ///
    /// A tuple type describes an array with a fixed number of elements, where each position has its own type.
    /// Each element can have a label, such as `x` in `[x: number, y: number]`, that describes what the position means.
    /// When only some elements have labels, the elements without one look like they were forgotten,
    /// and the tuple is harder to read.
    ///
    /// This rule reports the elements without a label in a tuple type where at least one other element has a label.
    /// Tuple types where no element has a label are allowed.
    /// A rest element, such as `...rest: number[]`, can also have a label, and is checked the same way as the other elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// type Point = [x: number, number];
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type Route = [name: string, ...string[]];
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// type Point = [x: number, y: number];
    /// type Pair = [number, number];
    /// type Route = [name: string, ...segments: string[]];
    /// ```
    ///
    pub UseConsistentTupleLabels {
        version: "2.6.7",
        name: "useConsistentTupleLabels",
        language: "ts",
        sources: &[RuleSource::EslintUnicorn("consistent-tuple-labels").same()],
        recommended: false,
    }
}

impl Rule for UseConsistentTupleLabels {
    type Query = Ast<TsTupleType>;
    /// An element without a label.
    type State = AnyTsTupleTypeElement;
    type Signals = Box<[Self::State]>;
    type Options = UseConsistentTupleLabelsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut has_label = false;
        let mut unlabeled = Vec::new();
        for element in ctx.query().elements() {
            match element {
                Ok(AnyTsTupleTypeElement::TsNamedTupleTypeElement(_)) => has_label = true,
                Ok(AnyTsTupleTypeElement::AnyTsType(AnyTsType::TsBogusType(_))) | Err(_) => {
                    return Box::default();
                }
                Ok(element) => unlabeled.push(element),
            }
        }
        if has_label {
            unlabeled.into_boxed_slice()
        } else {
            Box::default()
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, element: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                element.range(),
                markup! {
                    "This tuple element has no label, but other elements of the same tuple do."
                },
            )
            .note(markup! {
                "Labeling only some elements of a tuple makes it unclear what the other positions mean."
            })
            .note(markup! {
                "Add a label to this element, or remove the labels from the other elements."
            }),
        )
    }
}
