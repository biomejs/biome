//! Finds Vue components by matching the syntax that can define one, independently of the Vue
//! semantic model. Rules should query the model in [`crate::services::vue`] instead.

use crate::frameworks::vue::vue_call::is_vue_api_reference;
use crate::services::semantic::Semantic;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, JsCallExpression, JsExportDefaultExpressionClause, JsModule,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstSeparatedList, declare_node_union};

/// VueComponentQuery is a query type that can be used to find Vue components.
/// It can match any potential Vue component.
pub type VueComponentQuery = Semantic<AnyPotentialVueComponent>;

declare_node_union! {
    pub AnyPotentialVueComponent = JsExportDefaultExpressionClause
        | JsCallExpression | JsModule
}

/// An abstraction over multiple ways to define a vue component.
/// Provides a list of declarations for a component.
#[derive(Debug)]
pub enum AnyVueComponent {
    /// Options API style Vue component.
    /// ```html
    /// <script> export default { props: [ ... ], data: { ... }, ... }; </script>
    /// ```
    OptionsApi(VueOptionsApiComponent),
    /// A component may be defined right in the createApp call.
    /// ```js
    /// createApp({ props: [ ... ], ... });
    /// ```
    CreateApp(VueCreateApp),
    /// Define Component is one of the ways to define a Vue component.
    /// ```js
    /// defineComponent((...) => { ... }, { props: [ ... ], ... });
    /// defineComponent({ props: [ ... ], ... });
    /// ```
    DefineComponent(VueDefineComponent),
    /// Setup component is a Vue 3 component that uses the Composition API.
    /// ```html
    /// <script setup> defineProps({ ... }); const someData = { ... }; </script>
    /// ```
    Setup,
}

impl AnyVueComponent {
    pub fn from_potential_component(
        potential_component: &AnyPotentialVueComponent,
        model: &SemanticModel,
        source: &JsFileSource,
    ) -> Option<Self> {
        match potential_component {
            AnyPotentialVueComponent::JsModule(_) => {
                let embedding_kind = source.as_embedding_kind();
                if !embedding_kind.is_vue_setup() {
                    return None;
                }
                Some(Self::Setup)
            }
            AnyPotentialVueComponent::JsExportDefaultExpressionClause(
                default_expression_clause,
            ) => {
                if !source.as_embedding_kind().is_vue() {
                    return None;
                }
                if let Some(call_expression) = default_expression_clause
                    .expression()
                    .ok()?
                    .as_js_call_expression()
                {
                    // export default defineComponent({ ... });
                    let callee = call_expression
                        .callee()
                        .ok()
                        .and_then(|callee| callee.inner_expression())?;

                    if is_vue_api_reference(&callee, model, "defineComponent") {
                        // ignore defineComponent calls here, they get flagged separately
                        return None;
                    }
                }
                Some(Self::OptionsApi(VueOptionsApiComponent {
                    default_expression_clause: default_expression_clause.clone(),
                }))
            }
            AnyPotentialVueComponent::JsCallExpression(call_expression) => {
                let callee = call_expression
                    .callee()
                    .ok()
                    .and_then(|callee| callee.inner_expression())?;
                if is_vue_api_reference(&callee, model, "defineComponent") {
                    Some(Self::DefineComponent(VueDefineComponent {
                        call_expression: call_expression.clone(),
                    }))
                } else if is_vue_api_reference(&callee, model, "createApp") {
                    Some(Self::CreateApp(VueCreateApp {
                        call_expression: call_expression.clone(),
                    }))
                } else {
                    None
                }
            }
        }
    }
}

/// Options API style Vue component.
/// ```html
/// <script> export default { props: [ ... ], data: { ... }, ... }; </script>
/// ```
#[derive(Debug)]
pub struct VueOptionsApiComponent {
    default_expression_clause: JsExportDefaultExpressionClause,
}

/// A component may be defined right in the createApp call.
/// ```js
/// createApp({ props: [ ... ], ... });
/// ```
#[derive(Debug)]
pub struct VueCreateApp {
    call_expression: JsCallExpression,
}

/// Define Component is one of the ways to define a Vue component.
/// ```js
/// defineComponent((...) => { ... }, { props: [ ... ], ... });
/// defineComponent({ props: [ ... ], ... });
/// ```
#[derive(Debug)]
pub struct VueDefineComponent {
    call_expression: JsCallExpression,
}

impl VueOptionsApiBasedComponent for VueOptionsApiComponent {
    fn definition_expression(&self) -> Option<AnyJsExpression> {
        self.default_expression_clause.expression().ok()
    }
}

impl VueOptionsApiBasedComponent for VueCreateApp {
    fn definition_expression(&self) -> Option<AnyJsExpression> {
        let args = self
            .call_expression
            .arguments()
            .ok()
            .map(|arguments| arguments.args())?;

        // createApp({ props: [...] });
        let first_argument = args.first()?.ok()?;
        first_argument.as_any_js_expression().cloned()
    }
}

impl VueOptionsApiBasedComponent for VueDefineComponent {
    fn definition_expression(&self) -> Option<AnyJsExpression> {
        let args = self
            .call_expression
            .arguments()
            .ok()
            .map(|arguments| arguments.args())?;

        // defineComponent({ props: [...] });
        // defineComponent(setup, { props: [...] });
        let last_argument = args.last()?.ok()?;
        last_argument.as_any_js_expression().cloned()
    }
}

pub trait VueOptionsApiBasedComponent {
    /// Returns the potential object expression that defines the component.
    /// Examples:
    /// ```js
    /// defineComponent(
    ///     { props: [...] } // this
    /// );
    /// createApp(
    ///     { props: [...] } // this
    /// );
    /// export default { props: [...] }; // this
    /// ```
    fn definition_expression(&self) -> Option<AnyJsExpression>;
}
