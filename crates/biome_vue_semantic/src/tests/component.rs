use super::js_model;
use crate::{
    ComponentKind, DefaultSource, DefaultSources, Namespace, OpenReason, PropForm, PropType, Tri,
};
use biome_languages::JsFileSource;

fn setup_ts() -> JsFileSource {
    JsFileSource::ts().with_embedding_kind(*JsFileSource::vue_setup().as_embedding_kind())
}

#[test]
fn plain_file_without_vue_has_no_components() {
    let model = js_model(
        "export default { props: ['a'] }; defineComponent({});",
        JsFileSource::js_module(),
    );
    assert_eq!(model.components().count(), 0);
}

#[test]
fn define_component_in_js_file() {
    let model = js_model(
        r#"import { defineComponent } from "vue";
export const A = defineComponent({
  name: "MyButton",
  inheritAttrs: false,
  props: { label: String, size: { type: [Number, String], required: true, default: 1 } },
  emits: ["click"],
  data() { return { count: 0, user: { name: "" } }; },
  computed: { double() { return this.count * 2; }, full: { get() {}, set(v) {} } },
  methods: { reset(a, ...rest) {} },
  mounted() {},
});"#,
        JsFileSource::js_module(),
    );
    insta::assert_snapshot!(model.to_string());
}

#[test]
fn create_app_and_vue_global() {
    let model = js_model(
        "Vue.createApp({ props: ['a', 'b'], methods: { go() {} } });",
        JsFileSource::js_module(),
    );
    let component = model.components().next().expect("a component");
    assert_eq!(component.kind(), ComponentKind::CreateApp);
    let props: Vec<_> = component
        .props()
        .map(|prop| prop.name().to_string())
        .collect();
    assert_eq!(props, ["a", "b"]);
    assert_eq!(
        component.props().next().unwrap().form(),
        Some(PropForm::ArrayString)
    );
}

#[test]
fn options_export_only_in_vue_files() {
    let code = "export default { props: { a: Boolean } }";
    assert_eq!(js_model(code, JsFileSource::vue()).components().count(), 1);
    assert_eq!(
        js_model(code, JsFileSource::js_module())
            .components()
            .count(),
        0
    );
}

#[test]
fn every_prop_form_has_the_same_shape() {
    let options = js_model(
        "export default { props: { enabled: { type: Boolean, default: true } } }",
        JsFileSource::vue(),
    );
    let with_defaults = js_model(
        "withDefaults(defineProps<{ enabled?: boolean }>(), { enabled: true })",
        setup_ts(),
    );
    let destructured = js_model(
        "const { enabled = true } = defineProps<{ enabled?: boolean }>()",
        setup_ts(),
    );
    let expected = [
        DefaultSource::Inline,
        DefaultSource::WithDefaults,
        DefaultSource::Destructure,
    ];
    for (model, source) in [options, with_defaults, destructured].iter().zip(expected) {
        let component = model.components().next().expect("a component");
        let prop = component.props().next().expect("a prop");
        assert_eq!(prop.name(), "enabled");
        assert!(prop.types() == PropType::Boolean);
        let defaults: Vec<_> = prop.defaults().collect();
        assert_eq!(defaults.len(), 1);
        assert_eq!(defaults[0].source(), source);
    }
}

#[test]
fn external_default_sources_follow_the_define_props_call() {
    for (code, expected) in [
        ("defineProps<{ a?: string }>()", DefaultSources::empty()),
        (
            "const props = defineProps<{ a?: string }>()",
            DefaultSources::empty(),
        ),
        (
            "withDefaults(defineProps<{ a?: string }>(), {})",
            DefaultSource::WithDefaults.into(),
        ),
        (
            "const { a } = defineProps<{ a?: string }>()",
            DefaultSource::Destructure.into(),
        ),
        (
            "const { a } = withDefaults(defineProps<{ a?: string }>(), {})",
            DefaultSource::WithDefaults | DefaultSource::Destructure,
        ),
    ] {
        let model = js_model(code, setup_ts());
        let component = model.components().next().expect("a component");
        let prop = component.props().next().expect("a prop");
        assert_eq!(prop.external_default_sources(), expected, "`{code}`");
    }

    let model = js_model(
        "export default { props: { a: { type: String } } }",
        JsFileSource::vue(),
    );
    let component = model.components().next().expect("a component");
    let prop = component.props().next().expect("a prop");
    assert!(prop.external_default_sources().is_empty());
}

#[test]
fn unreadable_with_defaults_leaves_the_defaults_incomplete() {
    for (code, complete) in [
        (
            "withDefaults(defineProps<{ a?: string }>(), { a: '' })",
            true,
        ),
        (
            "withDefaults(defineProps<{ a?: string }>(), { a: '' } as const)",
            true,
        ),
        ("withDefaults(defineProps<{ a?: string }>())", true),
        ("const { a = '' } = defineProps<{ a?: string }>()", true),
        (
            "withDefaults(defineProps<{ a?: string }>(), defaults)",
            false,
        ),
        (
            "withDefaults(defineProps<{ a?: string }>(), makeDefaults())",
            false,
        ),
        (
            "withDefaults(defineProps<{ a?: string }>(), { ...defaults })",
            false,
        ),
        (
            "withDefaults(defineProps<{ a?: string }>(), { [key]: '' })",
            false,
        ),
    ] {
        let model = js_model(code, setup_ts());
        let component = model.components().next().expect("a component");
        let prop = component.props().next().expect("a prop");
        assert_eq!(prop.has_complete_defaults(), complete, "`{code}`");
    }
}

#[test]
fn unreadable_prop_options_leave_the_defaults_incomplete() {
    for (code, complete) in [
        ("export default { props: { a: { type: String } } }", true),
        ("export default { props: { a: { ...shared } } }", false),
        ("export default { props: { a: { [key]: 1 } } }", false),
    ] {
        let model = js_model(code, JsFileSource::vue());
        let component = model.components().next().expect("a component");
        let prop = component.props().next().expect("a prop");
        assert_eq!(prop.has_complete_defaults(), complete, "`{code}`");
    }
}

#[test]
fn script_setup_macros() {
    let model = js_model(
        r#"import { ref, computed, onMounted, watch } from "vue";
interface Props { title: string; items?: string[] }
const props = withDefaults(defineProps<Props>(), { items: () => [] });
const emit = defineEmits<{ change: [id: number]; (e: "close" | "open"): void }>();
const model = defineModel<string>("value");
defineSlots<{ default(props: {}): any; header: any }>();
defineOptions({ name: "MyList", inheritAttrs: false });
const count = ref(0);
const double = computed(() => count.value * 2);
onMounted(() => {});
watch(count, () => {});
"#,
        setup_ts(),
    );
    insta::assert_snapshot!(model.to_string());
}

#[test]
fn mixins_option_range() {
    let source = "export default { name: 'A', mixins: [m] }";
    let model = js_model(source, JsFileSource::vue());
    let component = model.components().next().unwrap();
    let range = component.mixins_range().unwrap();
    assert_eq!(&source[range], "mixins: [m]");

    let model = js_model("export default { extends: m }", JsFileSource::vue());
    let component = model.components().next().unwrap();
    assert_eq!(component.mixins_range(), None);
}

#[test]
fn unreadable_declarations_open_the_namespace() {
    let spread = js_model(
        "export default { props: { ...shared, a: String }, mixins: [m] }",
        JsFileSource::vue(),
    );
    let component = spread.components().next().unwrap();
    assert!(
        component
            .open_reasons(Namespace::Binding)
            .contains(OpenReason::Spread)
    );
    assert!(
        component
            .open_reasons(Namespace::Emit)
            .contains(OpenReason::Extends)
    );

    let imported_type = js_model(
        r#"import type { Props } from "./props"; defineProps<Props>()"#,
        setup_ts(),
    );
    let component = imported_type.components().next().unwrap();
    assert!(
        component
            .open_reasons(Namespace::Binding)
            .contains(OpenReason::UnresolvedType)
    );
    assert!(component.is_complete(Namespace::Emit));
}

#[test]
fn setup_function_return_members() {
    let model = js_model(
        r#"import { defineComponent, onMounted } from "vue";
defineComponent({
  props: ["a"],
  setup(props, { emit }) {
    onMounted(() => {});
    const count = 1;
    return { count, reset() {}, label: "x" };
  },
});"#,
        JsFileSource::js_module(),
    );
    let component = model.components().next().unwrap();
    let names: Vec<_> = component
        .setup_bindings()
        .map(|symbol| symbol.name().to_string())
        .collect();
    assert_eq!(names, ["count", "reset", "label"]);
    assert_eq!(component.hooks().count(), 1);
    assert_eq!(component.props().next().unwrap().required(), Tri::No);
}

#[test]
fn vue_2_lifecycle_option_names() {
    let model = js_model(
        r#"import { defineComponent } from "vue";
defineComponent({
  beforeDestroy() {},
  destroyed: () => {},
});"#,
        JsFileSource::js_module(),
    );
    let component = model.components().next().unwrap();
    let names: Vec<_> = component
        .hooks()
        .map(|symbol| symbol.name().to_string())
        .collect();
    assert_eq!(names, ["beforeDestroy", "destroyed"]);
}

#[test]
fn script_mode_file_with_vue_global() {
    let model = js_model(
        "Vue.createApp({ props: ['key'] });",
        JsFileSource::js_script(),
    );
    let component = model.components().next().expect("a component");
    assert_eq!(component.props().next().unwrap().name(), "key");
}

#[test]
fn component_call_inside_script_setup() {
    let model = js_model(
        r#"import { defineComponent } from "vue";
defineProps(["outer"]);
const Child = defineComponent({ props: ["inner"] });"#,
        JsFileSource::vue_setup(),
    );
    let kinds: Vec<_> = model
        .components()
        .map(|component| {
            (
                component.kind(),
                component
                    .props()
                    .map(|prop| prop.name().to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (ComponentKind::Setup, vec!["outer".to_string()]),
            (ComponentKind::DefineComponent, vec!["inner".to_string()]),
        ]
    );
}

#[test]
fn non_literal_declarations_open_the_namespace() {
    for code in [
        "export default { props: shared }",
        "export default { data() { return state } }",
        "export default { data }",
        "export default { computed: { [key]() {} } }",
        "export default { methods: { [key]() {} } }",
        "export default { setup() { return toRefs(state) } }",
        "export default { setup: sharedSetup }",
        "export default { setup }",
    ] {
        let model = js_model(code, JsFileSource::vue());
        let component = model.components().next().expect("a component");
        assert!(
            component
                .open_reasons(Namespace::Binding)
                .contains(OpenReason::NonLiteral),
            "`{code}` should leave the bindings open"
        );
    }
    let closed = js_model(
        "export default { props: { title: String }, data() { return { a: 1 } }, setup() { return { b: 2 } } }",
        JsFileSource::vue(),
    );
    assert!(
        closed
            .components()
            .next()
            .unwrap()
            .is_complete(Namespace::Binding)
    );
}

#[test]
fn shorthand_prop_is_a_prop_of_unknown_type() {
    let model = js_model("export default { props: { size } }", JsFileSource::vue());
    let component = model.components().next().unwrap();
    let prop = component.props().next().expect("a prop");
    assert_eq!(prop.name(), "size");
    assert!(prop.types() == PropType::Unknown);
    assert!(component.is_complete(Namespace::Binding));
}

#[test]
fn inline_function_prop_names_no_type() {
    for code in [
        "export default { props: { title() {} } }",
        "export default { props: { title: () => {} } }",
    ] {
        let model = js_model(code, JsFileSource::vue());
        let component = model.components().next().unwrap();
        let prop = component.props().next().expect("a prop");
        assert_eq!(prop.name(), "title", "`{code}`");
        assert!(prop.types().is_empty(), "`{code}` should name no type");
        assert!(component.is_complete(Namespace::Binding), "`{code}`");
    }
}

#[test]
fn type_option_held_by_a_variable_is_unknown() {
    for code in [
        "export default { props: { title: { type } } }",
        "export default { props: { title: { type: function Title() {} } } }",
    ] {
        let model = js_model(code, JsFileSource::vue());
        let component = model.components().next().unwrap();
        let prop = component.props().next().expect("a prop");
        assert!(prop.types() == PropType::Unknown, "`{code}`");
    }
}

#[test]
fn define_model_options_describe_its_prop() {
    let model = js_model(
        r#"const a = defineModel();
const b = defineModel("b", { type: Boolean, required: true });
const c = defineModel<string>("c");
const d = defineModel(Number);"#,
        setup_ts(),
    );
    let component = model.components().next().unwrap();
    let models: Vec<_> = component.models().collect();
    assert_eq!(models[0].name(), "modelValue");
    assert_eq!(models[0].form(), None);
    assert!(models[0].types().is_empty());
    assert_eq!(models[1].name(), "b");
    assert_eq!(models[1].form(), Some(PropForm::Options));
    assert!(models[1].types() == PropType::Boolean);
    assert_eq!(models[1].required(), Tri::Yes);
    assert_eq!(models[2].form(), Some(PropForm::TypeMember));
    assert!(models[2].types() == PropType::String);
    assert_eq!(models[3].form(), Some(PropForm::Constructor));
    assert!(models[3].types() == PropType::Number);
}

#[test]
fn runtime_type_range_covers_the_type_as_written() {
    let code = "defineProps({ a: Object as PropType<A>, b: { type: (Array), default: null }, c: { default: 1 }, d: String })";
    let model = js_model(code, setup_ts());
    let component = model.components().next().unwrap();
    let types: Vec<_> = component
        .props()
        .map(|prop| prop.runtime_type_range().map(|range| &code[range]))
        .collect();
    assert_eq!(
        types,
        [
            Some("Object as PropType<A>"),
            Some("(Array)"),
            None,
            Some("String")
        ]
    );

    let model = js_model("defineProps<{ a: object }>()", setup_ts());
    let prop = model.components().next().unwrap().props().next().unwrap();
    assert_eq!(prop.runtime_type_range(), None);
}

#[test]
fn types_the_model_cannot_list_open_the_namespace() {
    for code in [
        "interface Base { a: string } interface Props extends Base { b: string } defineProps<Props>()",
        "defineProps<{ [key: string]: string }>()",
        "type A = B; type B = A; defineProps<A>()",
        "defineProps<A & B>()",
    ] {
        let model = js_model(code, setup_ts());
        let component = model.components().next().expect("a component");
        assert!(
            component
                .open_reasons(Namespace::Binding)
                .contains(OpenReason::UnresolvedType),
            "`{code}` should leave the bindings open"
        );
    }
}

#[test]
fn method_signature_in_props_type_is_a_function_prop() {
    let model = js_model("defineProps<{ onClose(): void }>()", setup_ts());
    let component = model.components().next().unwrap();
    let prop = component.props().next().expect("a prop");
    assert_eq!(prop.name(), "onClose");
    assert!(prop.types() == PropType::Function);
    assert!(component.is_complete(Namespace::Binding));
}

#[test]
fn slots_option_declares_the_members_of_its_slots_type() {
    let plain_ts = JsFileSource::ts().with_embedding_kind(*JsFileSource::vue().as_embedding_kind());
    let model = js_model(
        r#"import { defineComponent, SlotsType } from "vue";
type Slots = { default: { msg: string }; "item-row"(props: {}): any };
export default defineComponent({ slots: Object as SlotsType<Slots> });"#,
        plain_ts,
    );
    let component = model.components().next().expect("a component");
    let slots: Vec<_> = component
        .slots()
        .map(|slot| slot.name().to_string())
        .collect();
    assert_eq!(slots, ["default", "item-row"]);
    assert!(component.is_complete(Namespace::Slot));

    for code in [
        "export default { slots: Object }",
        "export default { slots: Object as SlotsType<Imported> }",
        "export default { slots: Object as Other<{ a: any }> }",
        "export default { slots: Object as SlotsType<{ [name: string]: any }> }",
        "export default { slots: Object as SlotsType<{ [SLOT_NAME]: any }> }",
    ] {
        let model = js_model(code, plain_ts);
        let component = model.components().next().expect("a component");
        assert!(
            component
                .open_reasons(Namespace::Slot)
                .contains(OpenReason::UnresolvedType),
            "`{code}` should leave the slots open"
        );
    }
}

/// Returns the name of each emit of each component, the number of its uses,
/// and whether those are all of its uses.
fn emit_uses(code: &str, source: JsFileSource) -> Vec<(String, usize, bool)> {
    js_model(code, source)
        .components()
        .flat_map(|component| {
            component
                .emits()
                .map(|emit| {
                    (
                        emit.name().to_string(),
                        emit.uses().count(),
                        emit.has_complete_uses(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn emit_calls_are_uses_of_the_event() {
    let uses = emit_uses(
        r#"import { defineComponent } from "vue";
defineComponent({
  emits: ["a", "b", "c", "d"],
  setup(props, context) { context.emit("a"); },
  methods: { go() { this.$emit("b"); other.$emit("c"); } },
});
defineComponent({
  emits: ["a", "b"],
  setup(props, { emit: send }) { send?.("a"); },
});"#,
        JsFileSource::js_module(),
    );
    assert_eq!(
        uses,
        [
            ("a".into(), 1, true),
            ("b".into(), 1, true),
            ("c".into(), 0, true),
            ("d".into(), 0, true),
            ("a".into(), 1, true),
            ("b".into(), 0, true),
        ]
    );
}

#[test]
fn emit_function_used_without_a_call_opens_the_uses() {
    for setup in [
        "setup(props, { emit }) { return { notify: enabled ? emit : noop }; }",
        "setup(props, { emit }) { register(() => emit); }",
        "setup(props, { emit }) { register(emit); }",
        "setup(props, context) { register(() => context); }",
        "setup(props, context) { register(context.emit); }",
        "mounted() { register(() => this.$emit); }",
        "mounted() { const vm = this; vm.$emit('a'); }",
        "mounted() { this.$emit(name); }",
        "setup: sharedSetup",
    ] {
        let code = format!(
            r#"import {{ defineComponent }} from "vue";
defineComponent({{ emits: ["a"], {setup} }});"#
        );
        assert_eq!(
            emit_uses(&code, JsFileSource::js_module()),
            [("a".into(), 0, false)],
            "{setup}"
        );
    }
}

#[test]
fn template_option_opens_the_uses() {
    let uses = emit_uses(
        r#"import { defineComponent } from "vue";
defineComponent({ emits: ["a"], template: `<button @click="$emit('a')" />` });"#,
        JsFileSource::js_module(),
    );
    assert_eq!(uses, [("a".into(), 0, false)]);
}

#[test]
fn script_block_read_without_its_file_has_open_uses() {
    let uses = emit_uses(r#"defineEmits(["a"])"#, setup_ts());
    assert_eq!(uses, [("a".into(), 0, false)]);
    let uses = emit_uses(r#"export default { emits: ["a"] }"#, JsFileSource::vue());
    assert_eq!(uses, [("a".into(), 0, false)]);
}

#[test]
fn data_function_range_covers_each_function_form() {
    for (code, function) in [
        (
            "export default { data() { return {}; } }",
            "data() { return {}; }",
        ),
        (
            "export default { data: function () { return {}; } }",
            "function () { return {}; }",
        ),
        ("export default { data: (vm) => ({}) }", "(vm) => ({})"),
    ] {
        let model = js_model(code, JsFileSource::vue());
        let component = model.components().next().expect("a component");
        let range = component.data_function_range().expect("a data function");
        assert_eq!(&code[range], function);
    }
}

#[test]
fn data_function_range_is_absent_without_a_function() {
    for code in [
        "export default { data: { a: 1 } }",
        "export default { data: createData }",
        "export default { computed: {} }",
    ] {
        let model = js_model(code, JsFileSource::vue());
        let component = model.components().next().expect("a component");
        assert_eq!(component.data_function_range(), None, "{code}");
    }
}

#[test]
fn render_option_range_covers_the_member() {
    let code = "export default { name: 'A', render() { return null; } }";
    let model = js_model(code, JsFileSource::vue());
    let component = model.components().next().expect("a component");
    let range = component.render_range().expect("a render option");
    assert_eq!(&code[range], "render() { return null; }");

    let model = js_model("export default { name: 'A' }", JsFileSource::vue());
    let component = model.components().next().expect("a component");
    assert_eq!(component.render_range(), None);
}

#[test]
fn model_option_with_an_object_value_is_recorded() {
    let code = r#"import { defineComponent, createApp } from "vue";
defineComponent({ model: { prop: "checked", event: "change" } });
createApp({ model: ({}) });
defineComponent({ model() { return {}; } });
const model = {};
defineComponent({ model });
"#;
    let model = js_model(code, JsFileSource::js_module());
    let ranges: Vec<_> = model
        .components()
        .map(|component| component.model_option_range().map(|range| &code[range]))
        .collect();
    assert_eq!(
        ranges,
        [
            Some(r#"model: { prop: "checked", event: "change" }"#),
            Some("model: ({})"),
            None,
            None,
        ]
    );
}

#[test]
fn model_option_in_define_options_is_recorded() {
    let code = "defineOptions({ model: { prop: \"checked\" } });\n";
    let model = js_model(code, setup_ts());
    let component = model.components().next().unwrap();
    assert_eq!(
        component.model_option_range().map(|range| &code[range]),
        Some("model: { prop: \"checked\" }")
    );
}
