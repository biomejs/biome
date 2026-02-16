use crate::AnyHtmlAttributeInitializer;
use crate::static_value::StaticValue;

impl AnyHtmlAttributeInitializer {
    pub fn as_static_value(&self) -> Option<StaticValue> {
        match self {
            Self::HtmlAttributeSingleTextExpression(_) => None,
            Self::HtmlString(value) => Some(StaticValue::String(value.value_token().ok()?)),
        }
    }
}
