//! Generated file, do not edit by hand, see `xtask/codegen`

pub mod a11y;
pub mod complexity;
pub mod correctness;
pub mod nursery;
pub mod style;
pub mod suspicious;
::biome_analyze::declare_category! { pub Lint { kind : Lint , language : :: biome_css_syntax :: CssLanguage , groups : [self :: a11y :: A11y , self :: complexity :: Complexity , self :: correctness :: Correctness , self :: nursery :: Nursery , self :: style :: Style , self :: suspicious :: Suspicious ,] } }
