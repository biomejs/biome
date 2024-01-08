use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule};
use biome_css_syntax::CssColor;

declare_rule! {
    /// Noop rule
    pub(crate) Noop {
        version: "next",
        name: "noop",
    }
}

impl Rule for Noop {
    type Query = Ast<CssColor>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Option<Self::State> {
        None
    }
}
