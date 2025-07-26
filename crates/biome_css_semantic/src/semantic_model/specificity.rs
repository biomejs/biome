use crate::semantic_model::model::Specificity;

use biome_css_syntax::{
    AnyCssCompoundSelector, AnyCssPseudoClass, AnyCssRelativeSelector, AnyCssSelector,
    AnyCssSimpleSelector, AnyCssSubSelector, CssComplexSelector, CssCompoundSelector,
    CssPseudoClassSelector,
};

use biome_rowan::{AstNodeList, AstSeparatedList};

const ID_SPECIFICITY: Specificity = Specificity(1, 0, 0);
const CLASS_SPECIFICITY: Specificity = Specificity(0, 1, 0);
const TYPE_SPECIFICITY: Specificity = Specificity(0, 0, 1);
const ZERO_SPECIFICITY: Specificity = Specificity(0, 0, 0);

fn evaluate_any_simple_selector(selector: &AnyCssSimpleSelector) -> Specificity {
    match selector {
        AnyCssSimpleSelector::CssTypeSelector(_) => TYPE_SPECIFICITY,
        AnyCssSimpleSelector::CssUniversalSelector(_) => ZERO_SPECIFICITY,
    }
}

/// See https://developer.mozilla.org/en-US/docs/Web/CSS/Specificity#the_is_not_has_and_css_nesting_exceptions
const fn evaluate_pseudo_function_selector(name: &str) -> Option<Specificity> {
    match name.as_bytes() {
        b"where" => None,
        b"is" | b"not" | b"has" | b"matches" => Some(ZERO_SPECIFICITY),
        _ => Some(CLASS_SPECIFICITY),
    }
}

fn evaluate_any_pseudo_class(class: &AnyCssPseudoClass) -> Specificity {
    // https://www.w3.org/TR/selectors-4/#specificity-rules
    match class {
        AnyCssPseudoClass::CssBogusPseudoClass(_) => ZERO_SPECIFICITY,
        AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(selector) => {
            CLASS_SPECIFICITY
                + selector
                    .selector()
                    .map_or(ZERO_SPECIFICITY, |s| evaluate_any_compound_selector(&s))
        }
        AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(selector_list) => {
            let list_max = selector_list
                .compound_selectors()
                .iter()
                .map(|s| s.map_or(ZERO_SPECIFICITY, |s| evaluate_any_compound_selector(&s)))
                .reduce(|acc, e| acc.max(e))
                .unwrap_or(ZERO_SPECIFICITY);

            CLASS_SPECIFICITY + list_max
        }
        AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(_) => CLASS_SPECIFICITY,
        AnyCssPseudoClass::CssPseudoClassFunctionNth(_) => CLASS_SPECIFICITY,
        AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(selector_list) => {
            if let Some(base) = selector_list
                .name()
                .ok()
                .and_then(|name| name.value_token().ok())
                .and_then(|name| evaluate_pseudo_function_selector(name.text_trimmed()))
            {
                let list_max = selector_list
                    .relative_selectors()
                    .iter()
                    .map(|relative_selector| {
                        relative_selector
                            .map_or(ZERO_SPECIFICITY, |s| evaluate_any_relative_selector(&s))
                    })
                    .reduce(|acc, e| acc.max(e))
                    .unwrap_or(ZERO_SPECIFICITY);
                base + list_max
            } else {
                ZERO_SPECIFICITY
            }
        }
        AnyCssPseudoClass::CssPseudoClassFunctionSelector(s) => {
            if let Some(base) = s
                .name()
                .ok()
                .and_then(|name| name.value_token().ok())
                .and_then(|name| evaluate_pseudo_function_selector(name.text_trimmed()))
            {
                base + s.selector().map_or(ZERO_SPECIFICITY, |selector| {
                    evaluate_any_selector(&selector)
                })
            } else {
                ZERO_SPECIFICITY
            }
        }
        AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(selector_list) => {
            if let Some(base) = selector_list
                .name()
                .ok()
                .and_then(|name| name.value_token().ok())
                .and_then(|name| evaluate_pseudo_function_selector(name.text_trimmed()))
            {
                let list_max = selector_list
                    .selectors()
                    .iter()
                    .map(|selector| {
                        selector.map_or(ZERO_SPECIFICITY, |s| evaluate_any_selector(&s))
                    })
                    .reduce(|acc, e| acc.max(e))
                    .unwrap_or(ZERO_SPECIFICITY);
                base + list_max
            } else {
                ZERO_SPECIFICITY
            }
        }
        AnyCssPseudoClass::CssPseudoClassFunctionValueList(_) => CLASS_SPECIFICITY,
        AnyCssPseudoClass::CssPseudoClassIdentifier(_) => CLASS_SPECIFICITY,
        &AnyCssPseudoClass::CssPseudoClassFunctionCustomIdentifierList(_) => CLASS_SPECIFICITY,
    }
}

fn evaluate_pseudo_selector(selector: &CssPseudoClassSelector) -> Specificity {
    match selector.class() {
        Ok(any_pseudo_class) => evaluate_any_pseudo_class(&any_pseudo_class),
        Err(_) => ZERO_SPECIFICITY,
    }
}

fn evaluate_any_subselector(selector: &AnyCssSubSelector) -> Specificity {
    // https://www.w3.org/TR/selectors-4/#typedef-subclass-selector
    match selector {
        AnyCssSubSelector::CssIdSelector(_) => ID_SPECIFICITY,
        AnyCssSubSelector::CssClassSelector(_) => CLASS_SPECIFICITY,
        AnyCssSubSelector::CssAttributeSelector(_) => CLASS_SPECIFICITY,
        AnyCssSubSelector::CssPseudoClassSelector(s) => evaluate_pseudo_selector(s),
        AnyCssSubSelector::CssPseudoElementSelector(_) => TYPE_SPECIFICITY,
        AnyCssSubSelector::CssBogusSubSelector(_) => ZERO_SPECIFICITY,
    }
}

pub fn evaluate_compound_selector(selector: &CssCompoundSelector) -> Specificity {
    let nested_specificity = ZERO_SPECIFICITY; // TODO: Implement this

    let simple_specificity = selector
        .simple_selector()
        .map_or(ZERO_SPECIFICITY, |s| evaluate_any_simple_selector(&s));
    let subselector_specificity = selector
        .sub_selectors()
        .iter()
        .map(|s| evaluate_any_subselector(&s))
        .reduce(|acc, e| acc + e)
        .unwrap_or(ZERO_SPECIFICITY);

    nested_specificity + simple_specificity + subselector_specificity
}

fn evaluate_any_compound_selector(selector: &AnyCssCompoundSelector) -> Specificity {
    match selector {
        AnyCssCompoundSelector::CssBogusSelector(_) => ZERO_SPECIFICITY,
        AnyCssCompoundSelector::CssCompoundSelector(s) => evaluate_compound_selector(s),
    }
}

pub fn evaluate_complex_selector(selector: &CssComplexSelector) -> Specificity {
    let left_specificity = selector
        .left()
        .map_or(ZERO_SPECIFICITY, |s| evaluate_any_selector(&s));
    let right_specificity = selector
        .right()
        .map_or(ZERO_SPECIFICITY, |s| evaluate_any_selector(&s));

    left_specificity + right_specificity
}

pub fn evaluate_any_selector(selector: &AnyCssSelector) -> Specificity {
    match selector {
        AnyCssSelector::CssCompoundSelector(s) => evaluate_compound_selector(s),
        AnyCssSelector::CssComplexSelector(s) => evaluate_complex_selector(s),
        AnyCssSelector::CssBogusSelector(_) => ZERO_SPECIFICITY,
        AnyCssSelector::CssMetavariable(_) => {
            // TODO: Implement this
            ZERO_SPECIFICITY
        }
    }
}

fn evaluate_any_relative_selector(selector: &AnyCssRelativeSelector) -> Specificity {
    match selector {
        AnyCssRelativeSelector::CssBogusSelector(_) => ZERO_SPECIFICITY,
        AnyCssRelativeSelector::CssRelativeSelector(s) => s
            .selector()
            .map_or(ZERO_SPECIFICITY, |s| evaluate_any_selector(&s)),
    }
}
