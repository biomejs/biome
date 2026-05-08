use crate::css_module_info::traverse::CssClassStep;
use crate::module_graph::ModuleInfoKind;
use crate::{CssModuleInfo, HtmlModuleInfo, JsModuleInfo};

macro_rules! impl_update_always {
    ($($ty:ty),* $(,)?) => {
        $(
            // SAFETY: Unconditional swap — always reports "changed". Correct for
            // types stored behind `#[no_eq]` where Salsa skips equality checks.
            unsafe impl salsa::Update for $ty {
                unsafe fn maybe_update(old_pointer: *mut Self, new_value: Self) -> bool {
                    unsafe { *old_pointer = new_value };
                    true
                }
            }
        )*
    };
}

impl_update_always!(
    JsModuleInfo,
    CssModuleInfo,
    HtmlModuleInfo,
    ModuleInfoKind,
    CssClassStep,
);
