use super::js_model;
use crate::{ComponentKind, DefaultSource, Namespace, OpenReasons, PropForm, PropType, Tri};
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
        assert!(prop.types().is_only(PropType::Boolean));
        let defaults: Vec<_> = prop.defaults().collect();
        assert_eq!(defaults.len(), 1);
        assert_eq!(defaults[0].source(), source);
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
fn unreadable_declarations_open_the_namespace() {
    let spread = js_model(
        "export default { props: { ...shared, a: String }, mixins: [m] }",
        JsFileSource::vue(),
    );
    let component = spread.components().next().unwrap();
    assert!(
        component
            .open_reasons(Namespace::Binding)
            .contains(OpenReasons::SPREAD)
    );
    assert!(
        component
            .open_reasons(Namespace::Emit)
            .contains(OpenReasons::EXTENDS)
    );

    let imported_type = js_model(
        r#"import type { Props } from "./props"; defineProps<Props>()"#,
        setup_ts(),
    );
    let component = imported_type.components().next().unwrap();
    assert!(
        component
            .open_reasons(Namespace::Binding)
            .contains(OpenReasons::UNRESOLVED_TYPE)
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
        "export default { props: { title() {} } }",
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
                .contains(OpenReasons::NON_LITERAL),
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
    assert!(prop.types().is_only(PropType::Unknown));
    assert!(component.is_complete(Namespace::Binding));
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
                .contains(OpenReasons::UNRESOLVED_TYPE),
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
    assert!(prop.types().is_only(PropType::Function));
    assert!(component.is_complete(Namespace::Binding));
}
