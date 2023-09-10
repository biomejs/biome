use crate::transformers::ts_enum::TsEnum;
use biome_analyze::{GroupCategory, RegistryVisitor, RuleCategory, RuleGroup};
use biome_js_syntax::JsLanguage;

pub(crate) struct TransformationGroup;
pub(crate) struct TransformationCategory;

impl GroupCategory for TransformationCategory {
    type Language = JsLanguage;
    const CATEGORY: RuleCategory = RuleCategory::Transformation;

    fn record_groups<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
        registry.record_group::<TransformationGroup>()
    }
}

impl RuleGroup for TransformationGroup {
    type Language = JsLanguage;
    type Category = TransformationCategory;
    const NAME: &'static str = "transformations";

    fn record_rules<V: RegistryVisitor<Self::Language> + ?Sized>(registry: &mut V) {
        registry.record_rule::<TsEnum>();
    }
}

pub fn visit_transformation_registry<V: RegistryVisitor<JsLanguage>>(registry: &mut V) {
    registry.record_category::<TransformationCategory>();
}
