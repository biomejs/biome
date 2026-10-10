use super::sfc;
use crate::{
    Access, BlockKind, BranchKind, Certainty, ComponentKind, EffectiveValue, Namespace, Origin,
    RefSite, Resolution, RootShape, SymbolKind, Tri,
};

const LIST: &str = r#"<script setup lang="ts">
import Card from './Card.vue'
const props = defineProps<{ items: Item[]; title: string }>()
const emit = defineEmits<{ select: [id: number] }>()
</script>

<template>
  <Card v-for="item in items" :key="item.id" @click="emit('select', item.id)">
    {{ title }}
  </Card>
</template>
"#;

#[test]
fn worked_example() {
    insta::assert_snapshot!(sfc(LIST).to_string());
}

#[test]
fn blocks_and_template_component() {
    let model = sfc(LIST);
    let kinds: Vec<_> = model.blocks().map(|block| block.kind()).collect();
    assert_eq!(kinds, [BlockKind::Script, BlockKind::Template]);
    let script = model.blocks().next().unwrap();
    assert!(script.is_setup());
    assert_eq!(script.lang(), Some("ts"));
    assert!(model.template_component().is_some());
}

#[test]
fn template_identifiers_resolve_through_every_scope() {
    let model = sfc(LIST);
    let resolved: Vec<_> = model
        .references()
        .filter(|reference| reference.site() == RefSite::Identifier)
        .map(|reference| {
            (
                reference.name().to_string(),
                reference.symbol().map(|symbol| symbol.kind()),
            )
        })
        .collect();
    assert!(resolved.contains(&("items".into(), Some(SymbolKind::Prop))));
    assert!(resolved.contains(&("item".into(), Some(SymbolKind::VForAlias))));
    assert!(resolved.contains(&("emit".into(), Some(SymbolKind::Local))));
    assert!(resolved.contains(&("title".into(), Some(SymbolKind::Prop))));
}

#[test]
fn emit_call_is_a_use_of_the_event() {
    let model = sfc(LIST);
    let event = model
        .references()
        .find(|reference| reference.site() == RefSite::EventName)
        .expect("an event reference");
    assert_eq!(event.name(), "select");
    assert_eq!(event.call_args(), Some(1));
    assert_eq!(event.symbol().unwrap().kind(), SymbolKind::Emit);
    let component = model.template_component().unwrap();
    let emit = component.emits().next().unwrap();
    assert_eq!(emit.uses().count(), 1);
}

#[test]
fn unused_and_undeclared_names() {
    let model = sfc(r#"<script setup>
defineProps(['used', 'unused'])
</script>
<template><p>{{ used }} {{ missing }} {{ Math.max(1, 2) }}</p></template>
"#);
    let component = model.template_component().unwrap();
    let uses: Vec<_> = component
        .props()
        .map(|prop| (prop.name().to_string(), prop.uses().count()))
        .collect();
    assert_eq!(uses, [("used".into(), 1), ("unused".into(), 0)]);
    let missing = model
        .references()
        .find(|reference| reference.name() == "missing")
        .unwrap();
    assert_eq!(missing.resolution(), Resolution::Unresolved);
    let math = model
        .references()
        .find(|reference| reference.name() == "Math")
        .unwrap();
    assert_eq!(math.resolution(), Resolution::Global);
}

#[test]
fn unreadable_props_make_missing_names_unknowable() {
    let model = sfc(r#"<script setup lang="ts">
import type { Props } from './props'
defineProps<Props>()
</script>
<template><p>{{ anything }}</p></template>
"#);
    let reference = model
        .references()
        .find(|reference| reference.name() == "anything")
        .unwrap();
    assert_eq!(reference.resolution(), Resolution::Unknowable);
}

#[test]
fn props_object_members_and_mutation() {
    let model = sfc(r#"<script setup>
const props = defineProps(['list', 'count'])
props.list.push(1)
</script>
<template>
  <button @click="props.count++">{{ props.list.length }}</button>
  <input v-model="count">
</template>
"#);
    let component = model.template_component().unwrap();
    let list = component
        .props()
        .find(|prop| prop.name() == "list")
        .unwrap();
    let accesses: Vec<_> = list.uses().map(|reference| reference.access()).collect();
    assert!(accesses.contains(&Access::Mutate));
    assert!(accesses.contains(&Access::Read));
    let count = component
        .props()
        .find(|prop| prop.name() == "count")
        .unwrap();
    let accesses: Vec<_> = count.uses().map(|reference| reference.access()).collect();
    assert_eq!(accesses, [Access::Write, Access::Write]);
}

#[test]
fn options_api_template_sees_instance_only() {
    let model = sfc(r#"<script>
import helper from './helper'
export default {
  name: 'MyList',
  props: { title: String },
  data() { return { open: false } },
  computed: { label() { return this.title + this.open } },
  methods: { toggle() { this.open = !this.open; this.$emit('toggled') } },
  emits: ['toggled'],
}
</script>
<template><h1 @click="toggle">{{ label }} {{ helper }}</h1></template>
"#);
    let component = model.template_component().unwrap();
    assert_eq!(component.name(), Some("MyList"));
    let helper = model
        .references()
        .find(|reference| reference.name() == "helper")
        .unwrap();
    assert_eq!(helper.resolution(), Resolution::Unresolved);
    let open = component.data().next().unwrap();
    let accesses: Vec<_> = open.uses().map(|reference| reference.access()).collect();
    assert_eq!(accesses, [Access::Read, Access::Write, Access::Read]);
    assert_eq!(component.emits().next().unwrap().uses().count(), 1);
    let toggle = component.methods().next().unwrap();
    assert_eq!(toggle.uses().count(), 1);
}

#[test]
fn both_script_blocks() {
    let model = sfc(r#"<script>
let nextId = 0
export default { name: 'Both', inheritAttrs: false, props: ['a'], directives: { focus: {} } }
</script>
<script setup>
const id = nextId++
defineProps(['b'])
</script>
<template><p v-focus>{{ id }} {{ nextId }} {{ a }} {{ b }}</p></template>
"#);
    // The setup block defines the component that renders the template. The
    // default export of the plain block stays a component of its own tree.
    assert_eq!(model.components().count(), 2);
    let component = model.template_component().unwrap();
    assert_eq!(component.kind(), ComponentKind::Setup);
    assert_eq!(component.name(), Some("Both"));
    assert_eq!(component.inherit_attrs(), Tri::No);
    assert!(component.has_macro_conflict());
    let props: Vec<_> = component
        .props()
        .map(|prop| prop.name().to_string())
        .collect();
    assert_eq!(props, ["b"]);
    let plain = component.plain_block_component().unwrap();
    assert!(!plain.has_template());
    let props: Vec<_> = plain.props().map(|prop| prop.name().to_string()).collect();
    assert_eq!(props, ["a"]);

    for name in ["id", "nextId"] {
        let reference = model
            .references()
            .find(|reference| reference.name() == name)
            .unwrap();
        assert_eq!(reference.symbol().unwrap().kind(), SymbolKind::Local);
    }
    // The template sees the props of both blocks.
    for name in ["a", "b"] {
        let reference = model
            .references()
            .find(|reference| reference.name() == name)
            .unwrap();
        assert_eq!(reference.symbol().unwrap().kind(), SymbolKind::Prop);
    }
    let directive = model
        .references()
        .find(|reference| reference.namespace() == Namespace::Directive)
        .unwrap();
    assert_eq!(
        directive.symbol().unwrap().kind(),
        SymbolKind::DirectiveRegistration
    );
}

#[test]
fn every_declaration_of_a_component_is_in_its_own_tree() {
    let model = sfc(r#"<script>
export default { components: { Card: {} } }
</script>
<script setup>
defineProps(['b'])
</script>
<template><Card ref="card" /></template>
"#);
    let component = model.template_component().unwrap();
    let setup = component.snippet().unwrap().id();
    assert!(component.declarations().count() > 0);
    for declaration in component.declarations() {
        assert_eq!(
            declaration.snippet().map(|snippet| snippet.id()),
            Some(setup)
        );
    }
}

#[test]
fn external_script_makes_missing_names_unknowable() {
    let model = sfc(r#"<template><Card v-focus>{{ title }}</Card></template>
<script src="./component.js"></script>
"#);
    assert_eq!(model.components().count(), 0);
    let resolutions: Vec<_> = model
        .references()
        .map(|reference| (reference.name().to_string(), reference.resolution()))
        .collect();
    assert_eq!(
        resolutions,
        [
            ("title".into(), Resolution::Unknowable),
            ("Card".into(), Resolution::Unknowable),
            ("v-focus".into(), Resolution::Unknowable),
        ]
    );
}

#[test]
fn components_and_directives() {
    let model = sfc(r#"<script setup lang="ts">
import MyCard from './MyCard.vue'
import type { Shape } from './shape'
import * as Ui from './ui'
const vFocus = {}
</script>
<template>
  <my-card v-focus v-other />
  <Ui.Button /><Shape /><Missing /><Transition />
</template>
"#);
    let by_tag: Vec<_> = model
        .references()
        .filter(|reference| reference.namespace() == Namespace::Component)
        .map(|reference| (reference.name().to_string(), reference.resolution()))
        .collect();
    assert!(matches!(by_tag[0], (ref name, Resolution::Symbol(_)) if name == "my-card"));
    assert!(matches!(by_tag[1], (ref name, Resolution::Symbol(_)) if name == "Ui.Button"));
    assert_eq!(by_tag[2], ("Shape".into(), Resolution::Unresolved));
    assert_eq!(by_tag[3], ("Missing".into(), Resolution::Unresolved));
    assert_eq!(by_tag[4], ("Transition".into(), Resolution::Global));
    let directives: Vec<_> = model
        .references()
        .filter(|reference| reference.namespace() == Namespace::Directive)
        .map(|reference| (reference.name().to_string(), reference.symbol().is_some()))
        .collect();
    assert_eq!(
        directives,
        [("v-focus".into(), true), ("v-other".into(), false)]
    );
}

#[test]
fn conditional_chains_and_operands() {
    let model = sfc(r#"<template>
  <p v-if="a || (b && c)">1</p>
  <p v-else-if="!a">2</p>
  <p v-else>3</p>
  <p v-if="d">4</p>
</template>
"#);
    let chains: Vec<_> = model.chains().collect();
    assert_eq!(chains.len(), 2);
    let kinds: Vec<_> = chains[0].branches().map(|branch| branch.kind).collect();
    assert_eq!(
        kinds,
        [BranchKind::If, BranchKind::ElseIf, BranchKind::Else]
    );
    let first = chains[0].branches().next().unwrap().condition.unwrap();
    assert_eq!(first.root(), &RootShape::Logical);
    let groups: Vec<_> = first
        .operands()
        .iter()
        .map(|operand| operand.or_group())
        .collect();
    assert_eq!(groups, [0, 1, 1]);
    let second = chains[0].branches().nth(1).unwrap().condition.unwrap();
    assert_eq!(second.root(), &RootShape::Not);
    assert_eq!(model.roots().count(), 4);
}

#[test]
fn effective_values_and_classes() {
    let model = sfc(r#"<template>
  <button type="button" :disabled="true" :title="label" class="a b"
    :class="['c', { d: true, e: on }, on ? 'f' : 'g', other]" />
</template>
"#);
    let button = model
        .elements()
        .find(|element| element.tag() == "button")
        .unwrap();
    assert_eq!(
        button.effective("type"),
        EffectiveValue::Static("button".into())
    );
    assert_eq!(button.effective("disabled"), EffectiveValue::Bool(true));
    assert!(matches!(
        button.effective("title"),
        EffectiveValue::Dynamic(_)
    ));
    assert_eq!(button.effective("id"), EffectiveValue::Absent);
    let classes: Vec<_> = button
        .classes()
        .map(|class| (class.name().to_string(), class.certainty()))
        .collect();
    assert_eq!(
        classes,
        [
            ("a".into(), Certainty::Always),
            ("b".into(), Certainty::Always),
            ("c".into(), Certainty::Always),
            ("d".into(), Certainty::Always),
            ("e".into(), Certainty::Conditional),
            ("f".into(), Certainty::Conditional),
            ("g".into(), Certainty::Conditional),
        ]
    );
    assert!(button.has_open_classes());
}

#[test]
fn equal_expressions_have_equal_fingerprints() {
    let model = sfc(r#"<template>
  <Field :value="user.name" @update:value="user.name = $event" />
  <Field :value="user.name" @update:value="v => user.age = v" />
</template>
"#);
    let fields: Vec<_> = model
        .elements()
        .filter(|element| element.tag() == "Field")
        .collect();
    let pair = |index: usize| {
        let attrs: Vec<_> = fields[index].attrs().collect();
        (
            attrs[0].value_snippet().unwrap(),
            attrs[1].value_snippet().unwrap(),
        )
    };
    let (bind, on) = pair(0);
    assert_eq!(
        on.root(),
        &RootShape::Assignment {
            target: bind.fingerprint(),
            from_event: true
        }
    );
    let (bind, on) = pair(1);
    assert!(matches!(
        on.root(),
        RootShape::Assignment { target, from_event: true } if *target != bind.fingerprint()
    ));
}

#[test]
fn slot_params_template_refs_and_slots() {
    let model = sfc(r#"<script setup lang="ts">
import { ref, useTemplateRef } from 'vue'
import List from './List.vue'
const input = ref(null)
const box = useTemplateRef('box')
defineSlots<{ header(): any }>()
</script>
<template>
  <List v-slot="{ row }"><span>{{ row.id }}</span></List>
  <input ref="input"><div ref="box" /><b ref="orphan" />
  <slot name="header" /><slot /><p>{{ $refs.orphan }}</p>
</template>
"#);
    let row = model
        .references()
        .find(|reference| reference.name() == "row")
        .unwrap();
    assert_eq!(row.symbol().unwrap().kind(), SymbolKind::SlotParam);
    let refs: Vec<_> = model
        .symbols()
        .filter(|symbol| symbol.kind() == SymbolKind::TemplateRef)
        .map(|symbol| (symbol.name().to_string(), symbol.uses().count()))
        .collect();
    assert_eq!(
        refs,
        [("input".into(), 0), ("box".into(), 1), ("orphan".into(), 1)]
    );
    let input = model
        .symbols()
        .find(|symbol| symbol.name() == "input" && symbol.kind() == SymbolKind::Local)
        .unwrap();
    assert_eq!(input.origin(), Origin::Ref);
    let slots: Vec<_> = model
        .references()
        .filter(|reference| reference.namespace() == Namespace::Slot)
        .map(|reference| (reference.name().to_string(), reference.resolution()))
        .collect();
    assert!(matches!(slots[0], (ref name, Resolution::Symbol(_)) if name == "header"));
    assert_eq!(slots[1], ("default".into(), Resolution::Unresolved));
}

#[test]
fn ranges_are_local_to_the_declaring_tree() {
    let code = "<template><p /></template>\n<script setup>\ndefineProps(['a'])\n</script>\n";
    let model = sfc(code);
    let prop = model.template_component().unwrap().props().next().unwrap();
    let snippet = prop.snippet().expect("declared in a snippet");
    let host = prop.host_range();
    assert_eq!(&code[host], "'a'");
    assert_eq!(prop.range() + snippet.offset(), host);
    assert!(u32::from(prop.range().start()) < u32::from(host.start()));
}
