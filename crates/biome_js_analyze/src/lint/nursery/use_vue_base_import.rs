use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsImportClause, AnyJsImportLike, JsExportFromClause, JsExportNamedFromClause,
    JsModuleSource, JsSyntaxKind, JsSyntaxToken, inner_string_text,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, BatchMutationExt, TokenText, AstSeparatedList};
use biome_rule_options::use_vue_base_import::UseVueBaseImportOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce importing Vue's public entry point instead of internal Vue packages.
    ///
    /// The `@vue/runtime-dom`, `@vue/runtime-core`, `@vue/reactivity`, and `@vue/shared` packages are internal implementation packages. Their public exports should be imported from `vue` instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { computed } from "@vue/reactivity";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export * from "@vue/shared";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { computed } from "vue";
    /// import { internalOnly } from "@vue/reactivity";
    /// ```
    ///
    pub UseVueBaseImport {
        version: "2.5.13",
        name: "useVueBaseImport",
        language: "js",
        sources: &[RuleSource::EslintVueJs("prefer-import-from-vue").same()],
        recommended: true,
        domains: &[RuleDomain::Vue],
        fix_kind: FixKind::Safe,
    }
}

pub struct UseVueBaseImportState {
    fixable: bool,
}

impl Rule for UseVueBaseImport {
    type Query = Ast<AnyJsImportLike>;
    type State = UseVueBaseImportState;
    type Signals = Option<Self::State>;
    type Options = UseVueBaseImportOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let AnyJsImportLike::JsModuleSource(module_source) = ctx.query() else {
            return None;
        };

        let module_name_text = module_source.inner_string_text().ok()?;
        if !VUE_BASE_MODULES.contains(&module_name_text.text()) {
            return None;
        }

        match_import_source(ctx, module_source)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let module_name = module_name(ctx)?;
        let source = inner_string_text(&module_name);
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                module_name.text_trimmed_range(),
                markup! {
                    "Don't import the internal Vue package "<Emphasis>"'"{source.text()}"'"</Emphasis>"."
                },
            )
            .note(markup! {
                "The public "<Emphasis>"'vue'"</Emphasis>" entry point re-exports Vue's supported runtime APIs."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if !state.fixable {
            return None;
        }

        let module_name = module_name(ctx)?;
        let delimiter = module_name.text_trimmed().chars().next()?;
        let replacement_text = format!("{delimiter}vue{delimiter}");
        let replacement =
            JsSyntaxToken::new_detached(JsSyntaxKind::JS_STRING_LITERAL, &replacement_text, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token(module_name, replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Import "<Emphasis>"'vue'"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

const VUE_BASE_MODULES: &[&str] = &[
    "@vue/runtime-dom",
    "@vue/runtime-core",
    "@vue/reactivity",
    "@vue/shared",
];

const VUE_EXPORT_NAMES: &[&str] = &[
    "AllowedAttrs",
    "AllowedComponentProps",
    "AnchorHTMLAttributes",
    "App",
    "AppConfig",
    "AppContext",
    "AreaHTMLAttributes",
    "AriaAttributes",
    "AsyncComponentLoader",
    "AsyncComponentOptions",
    "Attrs",
    "AudioHTMLAttributes",
    "BaseHTMLAttributes",
    "BaseTransition",
    "BaseTransitionProps",
    "BaseTransitionPropsValidators",
    "BlockquoteHTMLAttributes",
    "ButtonHTMLAttributes",
    "CSSProperties",
    "CanvasHTMLAttributes",
    "ClassValue",
    "ColHTMLAttributes",
    "ColgroupHTMLAttributes",
    "Comment",
    "CompatVue",
    "Component",
    "ComponentCustomElementInterface",
    "ComponentCustomOptions",
    "ComponentCustomProperties",
    "ComponentCustomProps",
    "ComponentInjectOptions",
    "ComponentInstance",
    "ComponentInternalInstance",
    "ComponentObjectPropsOptions",
    "ComponentOptions",
    "ComponentOptionsBase",
    "ComponentOptionsMixin",
    "ComponentOptionsWithArrayProps",
    "ComponentOptionsWithObjectProps",
    "ComponentOptionsWithoutProps",
    "ComponentPropsOptions",
    "ComponentProvideOptions",
    "ComponentPublicInstance",
    "ComponentTypeEmits",
    "ComputedGetter",
    "ComputedOptions",
    "ComputedRef",
    "ComputedSetter",
    "ConcreteComponent",
    "CreateAppFunction",
    "CreateComponentPublicInstance",
    "CreateComponentPublicInstanceWithMixins",
    "CustomElementOptions",
    "CustomRefFactory",
    "DataHTMLAttributes",
    "DebuggerEvent",
    "DebuggerEventExtraInfo",
    "DebuggerOptions",
    "DeepReadonly",
    "DefineComponent",
    "DefineProps",
    "DefineSetupFnComponent",
    "DelHTMLAttributes",
    "DeprecationTypes",
    "DetailsHTMLAttributes",
    "DialogHTMLAttributes",
    "Directive",
    "DirectiveArguments",
    "DirectiveBinding",
    "DirectiveHook",
    "DirectiveModifiers",
    "EffectScheduler",
    "EffectScope",
    "ElementNamespace",
    "EmbedHTMLAttributes",
    "EmitFn",
    "EmitsOptions",
    "EmitsToProps",
    "ErrorCodes",
    "Events",
    "ExtractDefaultPropTypes",
    "ExtractPropTypes",
    "ExtractPublicPropTypes",
    "FieldsetHTMLAttributes",
    "FormHTMLAttributes",
    "Fragment",
    "FunctionDirective",
    "FunctionPlugin",
    "FunctionalComponent",
    "GlobalComponents",
    "GlobalDirectives",
    "HMRRuntime",
    "HTMLAttributes",
    "HtmlHTMLAttributes",
    "HydrationRenderer",
    "HydrationStrategy",
    "HydrationStrategyFactory",
    "IframeHTMLAttributes",
    "ImgHTMLAttributes",
    "InjectionKey",
    "InputAutoCompleteAttribute",
    "InputHTMLAttributes",
    "InputTypeHTMLAttribute",
    "InsHTMLAttributes",
    "IntrinsicElementAttributes",
    "KeepAlive",
    "KeepAliveProps",
    "KeygenHTMLAttributes",
    "LabelHTMLAttributes",
    "LegacyConfig",
    "LiHTMLAttributes",
    "LinkHTMLAttributes",
    "MapHTMLAttributes",
    "MaybeRef",
    "MaybeRefOrGetter",
    "MediaHTMLAttributes",
    "MenuHTMLAttributes",
    "MetaHTMLAttributes",
    "MeterHTMLAttributes",
    "MethodOptions",
    "ModelRef",
    "MultiWatchSources",
    "NativeElements",
    "ObjectDirective",
    "ObjectEmitsOptions",
    "ObjectHTMLAttributes",
    "ObjectPlugin",
    "OlHTMLAttributes",
    "OptgroupHTMLAttributes",
    "OptionHTMLAttributes",
    "OptionMergeFunction",
    "OutputHTMLAttributes",
    "ParamHTMLAttributes",
    "Plugin",
    "ProgressHTMLAttributes",
    "Prop",
    "PropType",
    "PublicProps",
    "QuoteHTMLAttributes",
    "Raw",
    "Reactive",
    "ReactiveEffect",
    "ReactiveEffectOptions",
    "ReactiveEffectRunner",
    "ReactiveFlags",
    "Ref",
    "RenderFunction",
    "Renderer",
    "RendererElement",
    "RendererNode",
    "RendererOptions",
    "ReservedProps",
    "RootHydrateFunction",
    "RootRenderFunction",
    "RuntimeCompilerOptions",
    "SVGAttributes",
    "ScriptHTMLAttributes",
    "SelectHTMLAttributes",
    "SetupContext",
    "ShallowReactive",
    "ShallowRef",
    "ShallowUnwrapRef",
    "ShortEmitsToObject",
    "Slot",
    "Slots",
    "SlotsType",
    "SourceHTMLAttributes",
    "Static",
    "StyleHTMLAttributes",
    "StyleValue",
    "Suspense",
    "SuspenseBoundary",
    "SuspenseProps",
    "TableHTMLAttributes",
    "TdHTMLAttributes",
    "Teleport",
    "TeleportProps",
    "TemplateRef",
    "Text",
    "TextareaHTMLAttributes",
    "ThHTMLAttributes",
    "TimeHTMLAttributes",
    "ToRef",
    "ToRefs",
    "TrackHTMLAttributes",
    "TrackOpTypes",
    "Transition",
    "TransitionGroup",
    "TransitionGroupProps",
    "TransitionHooks",
    "TransitionProps",
    "TransitionState",
    "TriggerOpTypes",
    "UnwrapNestedRefs",
    "UnwrapRef",
    "VNode",
    "VNodeArrayChildren",
    "VNodeChild",
    "VNodeNormalizedChildren",
    "VNodeProps",
    "VNodeRef",
    "VNodeTypes",
    "VideoHTMLAttributes",
    "VueElement",
    "VueElementConstructor",
    "WatchCallback",
    "WatchEffect",
    "WatchEffectOptions",
    "WatchHandle",
    "WatchOptions",
    "WatchOptionsBase",
    "WatchSource",
    "WatchStopHandle",
    "WebViewHTMLAttributes",
    "WritableComputedOptions",
    "WritableComputedRef",
    "callWithAsyncErrorHandling",
    "callWithErrorHandling",
    "camelize",
    "capitalize",
    "cloneVNode",
    "compile",
    "compileToFunction",
    "computed",
    "createApp",
    "createBaseVNode",
    "createBlock",
    "createCommentVNode",
    "createElementBlock",
    "createElementVNode",
    "createHydrationRenderer",
    "createRenderer",
    "createSSRApp",
    "createSlots",
    "createStaticVNode",
    "createTextVNode",
    "createVNode",
    "customRef",
    "defineAsyncComponent",
    "defineComponent",
    "defineCustomElement",
    "defineEmits",
    "defineExpose",
    "defineModel",
    "defineOptions",
    "defineProps",
    "defineSSRCustomElement",
    "defineSlots",
    "devtools",
    "effect",
    "effectScope",
    "getCurrentInstance",
    "getCurrentScope",
    "getCurrentWatcher",
    "getTransitionRawChildren",
    "guardReactiveProps",
    "h",
    "handleError",
    "hasInjectionContext",
    "hydrate",
    "hydrateOnIdle",
    "hydrateOnInteraction",
    "hydrateOnMediaQuery",
    "hydrateOnVisible",
    "initCustomFormatter",
    "inject",
    "isMemoSame",
    "isProxy",
    "isReactive",
    "isReadonly",
    "isRef",
    "isRuntimeOnly",
    "isShallow",
    "isVNode",
    "markRaw",
    "mergeProps",
    "nextTick",
    "nodeOps",
    "normalizeClass",
    "normalizeProps",
    "normalizeStyle",
    "onActivated",
    "onBeforeMount",
    "onBeforeUnmount",
    "onBeforeUpdate",
    "onDeactivated",
    "onErrorCaptured",
    "onMounted",
    "onRenderTracked",
    "onRenderTriggered",
    "onScopeDispose",
    "onServerPrefetch",
    "onUnmounted",
    "onUpdated",
    "onWatcherCleanup",
    "openBlock",
    "patchProp",
    "popScopeId",
    "provide",
    "proxyRefs",
    "pushScopeId",
    "queuePostFlushCb",
    "reactive",
    "readonly",
    "ref",
    "registerRuntimeCompiler",
    "render",
    "renderList",
    "renderSlot",
    "resolveComponent",
    "resolveDirective",
    "resolveDynamicComponent",
    "resolveTransitionHooks",
    "setBlockTracking",
    "setDevtoolsHook",
    "setTransitionHooks",
    "shallowReactive",
    "shallowReadonly",
    "shallowRef",
    "ssrContextKey",
    "stop",
    "toDisplayString",
    "toHandlerKey",
    "toHandlers",
    "toRaw",
    "toRef",
    "toRefs",
    "toValue",
    "transformVNodeArgs",
    "triggerRef",
    "unref",
    "useAttrs",
    "useCssModule",
    "useCssVars",
    "useHost",
    "useId",
    "useModel",
    "useSSRContext",
    "useShadowRoot",
    "useSlots",
    "useTemplateRef",
    "useTransitionState",
    "vModelCheckbox",
    "vModelDynamic",
    "vModelRadio",
    "vModelSelect",
    "vModelText",
    "vShow",
    "version",
    "warn",
    "watch",
    "watchEffect",
    "watchPostEffect",
    "watchSyncEffect",
    "withCtx",
    "withDefaults",
    "withDirectives",
    "withKeys",
    "withMemo",
    "withModifiers",
    "withScopeId",
];

fn match_import_source(
    ctx: &RuleContext<UseVueBaseImport>,
    module_source: &JsModuleSource,
) -> Option<UseVueBaseImportState> {
    if let Some(import_clause) = module_source.parent::<AnyJsImportClause>() {
        if ctx
            .source_type::<JsFileSource>()
            .language()
            .is_definition_file()
            && matches!(import_clause, AnyJsImportClause::JsImportBareClause(_))
        {
            return None;
        }

        return match &import_clause {
            AnyJsImportClause::JsImportBareClause(_) => {
                Some(UseVueBaseImportState { fixable: true })
            }
            AnyJsImportClause::JsImportNamedClause(_) => {
                let names = import_clause
                    .named_specifiers()?
                    .specifiers()
                    .iter()
                    .filter_map(|specifier| {
                        specifier
                            .ok()?
                            .imported_name()
                            .map(|name| name.token_text_trimmed())
                    });
                state_for_named_bindings(names)
            }
            AnyJsImportClause::JsImportCombinedClause(_)
            | AnyJsImportClause::JsImportDefaultClause(_)
            | AnyJsImportClause::JsImportNamespaceClause(_) => {
                Some(UseVueBaseImportState { fixable: false })
            }
        };
    }

    if let Some(export_clause) = module_source.parent::<JsExportNamedFromClause>() {
        let names = export_clause
            .specifiers()
            .iter()
            .filter_map(|specifier| specifier.ok()?.source_name().ok()?.inner_string_text().ok());

        return state_for_named_bindings(names);
    }

    module_source
        .parent::<JsExportFromClause>()
        .map(|_| UseVueBaseImportState { fixable: false })
}

fn module_name(ctx: &RuleContext<UseVueBaseImport>) -> Option<JsSyntaxToken> {
    let AnyJsImportLike::JsModuleSource(module_source) = ctx.query() else {
        return None;
    };

    module_source.value_token().ok()
}

fn state_for_named_bindings(
    names: impl IntoIterator<Item = TokenText>,
) -> Option<UseVueBaseImportState> {
    let mut has_names = false;
    let mut should_report = false;
    let mut fixable = true;

    for name in names {
        has_names = true;
        let is_vue_export = is_vue_export_name(name.text());
        should_report |= is_vue_export;
        fixable &= is_vue_export;
    }

    (!has_names || should_report).then_some(UseVueBaseImportState { fixable })
}

fn is_vue_export_name(name: &str) -> bool {
    VUE_EXPORT_NAMES.binary_search(&name).is_ok()
}

#[cfg(test)]
mod tests {
    use super::VUE_EXPORT_NAMES;

    #[test]
    fn vue_export_names_are_sorted() {
        assert!(VUE_EXPORT_NAMES.windows(2).all(|pair| pair[0] < pair[1]));
    }
}
