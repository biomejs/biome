use super::sfc;
use crate::{
    Access, Assignability, BlockKind, BranchKind, Certainty, ComponentKind, EffectiveValue,
    MacroOption, Namespace, Origin, RefSite, Resolution, RootShape, SymbolKind, Tri,
};
use biome_rowan::{TextRange, TextSize};

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

/// Renders the references of a model, one per line.
fn references(code: &str) -> String {
    sfc(code)
        .to_string()
        .lines()
        .filter(|line| line.starts_with("ref "))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn v_if_does_not_see_v_for_aliases_of_its_element() {
    insta::assert_snapshot!(references(
        r#"<script setup>
defineProps(['items', 'item'])
</script>
<template>
  <li v-for="item in items" v-if="item.visible" :key="item.id">{{ item.name }}</li>
  <p v-if="items" />
  <li v-else-if="entry" v-for="entry in items">{{ entry }}</li>
</template>
"#
    ));
}

#[test]
fn v_for_default_values_see_the_v_for_aliases_but_not_slot_params() {
    insta::assert_snapshot!(references(
        r#"<script setup>
import List from './List.vue'
defineProps(['rows', 'fallback', 'id'])
</script>
<template>
  <li v-for="({ id = fallback }, index = id) in rows">{{ index }}</li>
  <List v-for="{ row = fallback } in rows" v-slot="{ fallback }">{{ fallback }}</List>
</template>
"#
    ));
}

#[test]
fn slot_params_are_visible_to_the_slot_content_only() {
    insta::assert_snapshot!(references(
        r#"<script setup>
import List from './List.vue'
defineProps(['lists', 'row'])
</script>
<template>
  <List v-for="list in lists" v-slot="{ row }" v-if="row" :items="list" :selected="row">
    {{ row }} {{ list }}
  </List>
</template>
"#
    ));
}

#[test]
fn snippet_lookup_follows_template_scopes() {
    let code = r#"<script setup>
import List from './List.vue'
defineProps(['lists', 'row'])
</script>
<template>
  <List v-for="list in lists" v-slot="{ row }" :selected="row">{{ row }}</List>
</template>
"#;
    let model = sfc(code);
    // Resolves `name` in the snippet whose text is `text`, and returns the
    // kind of the symbol found.
    let lookup = |text: &str, name: &str| {
        model
            .snippets()
            .find(|snippet| &code[snippet.host_range()] == text)?
            .lookup(name)
            .map(|symbol| symbol.kind())
    };
    // The iterable of `v-for` does not see its variables.
    assert_eq!(lookup("lists", "list"), None);
    // The other directives of the element see its `v-for` variables, but not
    // its slot parameters.
    assert_eq!(lookup("row", "list"), Some(SymbolKind::VForAlias));
    assert_eq!(lookup("row", "row"), Some(SymbolKind::Prop));
    // The slot content sees both.
    assert_eq!(lookup(" row ", "row"), Some(SymbolKind::SlotParam));
    assert_eq!(lookup(" row ", "list"), Some(SymbolKind::VForAlias));
    assert_eq!(lookup(" row ", "unknown"), None);
    // A script block resolves its own names.
    let script = model.blocks().next().unwrap().snippet().unwrap();
    assert_eq!(script.lookup("List"), None);
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

/// Returns the name of each emit of the template component, the number of
/// its uses, and whether those are all of its uses.
fn emit_uses(code: &str) -> Vec<(String, usize, bool)> {
    sfc(code)
        .template_component()
        .unwrap()
        .emits()
        .map(|emit| {
            (
                emit.name().to_string(),
                emit.uses().count(),
                emit.has_complete_uses(),
            )
        })
        .collect()
}

#[test]
fn this_in_template_is_the_component_instance() {
    let model = sfc(r#"<script>
export default { props: ['title'], emits: ['a', 'b'] }
</script>
<template><button @click="this.$emit('a')">{{ this.title }}</button></template>
"#);
    let component = model.template_component().unwrap();
    let uses: Vec<_> = component
        .emits()
        .map(|emit| (emit.name().to_string(), emit.uses().count()))
        .collect();
    assert_eq!(uses, [("a".into(), 1), ("b".into(), 0)]);
    assert!(component.emits().all(|emit| emit.has_complete_uses()));
    assert_eq!(component.props().next().unwrap().uses().count(), 1);
}

#[test]
fn emit_function_passed_on_by_the_template_opens_the_uses() {
    let uses = emit_uses(
        r#"<script setup>
const emit = defineEmits(['a'])
</script>
<template><Child :notify="emit" /></template>
"#,
    );
    assert_eq!(uses, [("a".into(), 0, false)]);
    let uses = emit_uses(
        r#"<script>
export default { emits: ['a'] }
</script>
<template><Child :notify="$emit" /></template>
"#,
    );
    assert_eq!(uses, [("a".into(), 0, false)]);
}

#[test]
fn template_the_model_cannot_read_opens_the_uses() {
    for template in [
        r#"<template src="./template.html"></template>"#,
        r#"<template lang="pug">button(@click="$emit('a')")</template>"#,
    ] {
        let code = format!("<script>\nexport default {{ emits: ['a'] }}\n</script>\n{template}\n");
        assert_eq!(emit_uses(&code), [("a".into(), 0, false)], "{template}");
    }
    let uses = emit_uses(
        "<script>\nexport default { emits: ['a'] }\n</script>\n<template lang=\"html\"><p /></template>\n",
    );
    assert_eq!(uses, [("a".into(), 0, true)]);
}

#[test]
fn plain_block_emits_the_events_of_the_setup_block() {
    let model = sfc(r#"<script>
export default { methods: { go() { this.$emit('a') } } }
</script>
<script setup>
defineEmits(['a', 'b'])
</script>
"#);
    let setup = model.template_component().unwrap();
    let plain = setup.plain_block_component().unwrap();
    assert_eq!(plain.setup_block_component(), Some(setup.clone()));
    let uses: Vec<_> = setup
        .emits()
        .map(|emit| (emit.name().to_string(), emit.uses().count()))
        .collect();
    assert_eq!(uses, [("a".into(), 1), ("b".into(), 0)]);
    assert!(setup.emits().all(|emit| emit.has_complete_uses()));

    let uses = emit_uses(
        r#"<script>
export default { methods: { go(name) { this.$emit(name) } } }
</script>
<script setup>
defineEmits(['a'])
</script>
"#,
    );
    assert_eq!(uses, [("a".into(), 0, false)]);

    let uses = emit_uses(
        r#"<script>
export default { template: `<button @click="$emit('a')" />` }
</script>
<script setup>
defineEmits(['a'])
</script>
"#,
    );
    assert_eq!(uses, [("a".into(), 0, false)]);
}

#[test]
fn plain_block_emits_option() {
    let model = sfc(r#"<script>
export default { emits: ['change'] }
</script>
<script setup>
defineProps(['a'])
</script>
"#);
    let component = model.template_component().unwrap();
    assert_eq!(component.macro_options(), MacroOption::Emits);
}

#[test]
fn plain_block_emits_events_the_setup_block_may_declare() {
    let model = sfc(r#"<script lang="ts">
export default { methods: { go() { this.$emit('a') } } }
</script>
<script setup lang="ts">
import type { Events } from './events'
defineEmits<Events>()
</script>
"#);
    let event = model
        .references()
        .find(|reference| reference.site() == RefSite::EventName)
        .unwrap();
    assert_eq!(event.resolution(), Resolution::Unknowable);
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
    assert_eq!(component.macro_options(), MacroOption::Props);
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
fn object_class_key_ranges_cover_the_name() {
    let code = r#"<template>
  <div :class="{ a: x, 'b': x, ['c']: x, [`d`]: x, [('e')]: x }" />
</template>
"#;
    let model = sfc(code);
    let div = model
        .elements()
        .find(|element| element.tag() == "div")
        .unwrap();
    let classes: Vec<_> = div
        .classes()
        .map(|class| (class.name().to_string(), &code[class.host_range()]))
        .collect();
    assert_eq!(
        classes,
        [
            ("a".into(), "a"),
            ("b".into(), "b"),
            ("c".into(), "c"),
            ("d".into(), "d"),
            ("e".into(), "e"),
        ]
    );
    assert!(!div.has_open_classes());
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
fn assignability_of_directive_values() {
    let code = r#"<script setup lang="ts">
const props = defineProps<{ a: any; b: any }>()
</script>
<template>
  <input v-model="a" />
  <input v-model="(a)" />
  <input v-model="a.b[0]" />
  <input v-model="a as string" />
  <input v-model="a!" />
  <input v-model="a?.b" />
  <input v-model="a?.b.c" />
  <input v-model="a?.().b" />
  <input v-model="(a?.b).c" />
  <input v-model="(a?.b)[0].c" />
  <input v-model="(null).a" />
  <input v-model="a + b" />
  <input v-model="a()" />
  <input v-model="a?.b()" />
  <input v-model="(a, b)" />
</template>
"#;
    let model = sfc(code);
    let values: Vec<_> = model
        .elements()
        .filter(|element| element.tag() == "input")
        .filter_map(|element| element.attrs().next()?.value_snippet())
        .map(|snippet| (&code[snippet.host_range()], snippet.assignability()))
        .collect();
    assert_eq!(
        values,
        [
            ("a", Assignability::Assignable),
            ("(a)", Assignability::Assignable),
            ("a.b[0]", Assignability::Assignable),
            ("a as string", Assignability::Assignable),
            ("a!", Assignability::Assignable),
            ("a?.b", Assignability::OptionalChain),
            ("a?.b.c", Assignability::OptionalChain),
            ("a?.().b", Assignability::OptionalChain),
            ("(a?.b).c", Assignability::MaybeNullObject),
            ("(a?.b)[0].c", Assignability::MaybeNullObject),
            ("(null).a", Assignability::MaybeNullObject),
            ("a + b", Assignability::NotAssignable),
            ("a()", Assignability::NotAssignable),
            ("a?.b()", Assignability::NotAssignable),
            ("(a, b)", Assignability::NotAssignable),
        ]
    );
}

#[test]
fn compound_assignments_do_not_store_the_event() {
    let model = sfc(r#"<template>
  <Field @update:value="count += $event" />
  <Field @update:value="v => count += v" />
</template>
"#);
    let shapes: Vec<_> = model
        .elements()
        .filter(|element| element.tag() == "Field")
        .map(|element| {
            element
                .attrs()
                .next()
                .unwrap()
                .value_snippet()
                .unwrap()
                .root()
                .clone()
        })
        .collect();
    assert!(matches!(
        shapes[0],
        RootShape::Assignment {
            from_event: false,
            ..
        }
    ));
    assert_eq!(shapes[1], RootShape::Function { params: 1 });
}

#[test]
fn typed_arrow_parameters_store_the_event() {
    let model = sfc(r#"<script setup lang="ts">
const count = 0
</script>
<template>
  <Field @update:value="(v: number) => count = v" />
  <Field @update:value="(v = 0) => count = v" />
</template>
"#);
    let shapes: Vec<_> = model
        .elements()
        .filter(|element| element.tag() == "Field")
        .map(|element| {
            element
                .attrs()
                .next()
                .unwrap()
                .value_snippet()
                .unwrap()
                .root()
                .clone()
        })
        .collect();
    assert!(matches!(
        shapes[0],
        RootShape::Assignment {
            from_event: true,
            ..
        }
    ));
    assert_eq!(shapes[1], RootShape::Function { params: 1 });
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

#[test]
fn mutating_method_call_points_at_the_member_it_changes() {
    let code = "<script setup lang=\"ts\">\nconst props = defineProps(['list'])\nprops.list.push(1);\n(props.list!).sort();\n</script>\n";
    let model = sfc(code);
    let list = model.template_component().unwrap().props().next().unwrap();
    let mutations: Vec<_> = list
        .uses()
        .filter(|reference| reference.access() == Access::Mutate)
        .map(|reference| &code[reference.host_range()])
        .collect();
    assert_eq!(mutations, ["list", "list"]);
}

#[test]
fn write_through_computed_member_is_a_write() {
    let code = "<script setup>\nconst props = defineProps(['list'])\nprops.list[index].done = true;\nprops.list[index].push(1);\nprops.list[index];\n</script>\n";
    let model = sfc(code);
    let list = model.template_component().unwrap().props().next().unwrap();
    let uses: Vec<_> = list
        .uses()
        .map(|reference| {
            (
                &code[reference.host_range()],
                reference.access(),
                reference.has_open_path(),
            )
        })
        .collect();
    assert_eq!(
        uses,
        [
            ("list", Access::Write, true),
            ("list", Access::Mutate, true),
            ("list", Access::Read, true),
        ]
    );
}

#[test]
fn v_model_through_computed_member_is_a_write() {
    let model = sfc(r#"<script setup>
defineProps(['list'])
</script>
<template>
  <input v-model="list[0].name">
</template>
"#);
    let list = model.template_component().unwrap().props().next().unwrap();
    let accesses: Vec<_> = list.uses().map(|reference| reference.access()).collect();
    assert_eq!(accesses, [Access::Write]);
}

#[test]
fn object_assign_target_is_a_mutation() {
    let model = sfc(r#"<script setup>
const props = defineProps(['user'])
Object.assign(props.user, { name: 'Alice' });
Object.assign({}, props.user);
</script>
"#);
    let user = model.template_component().unwrap().props().next().unwrap();
    let accesses: Vec<_> = user.uses().map(|reference| reference.access()).collect();
    assert_eq!(accesses, [Access::Mutate, Access::Escape]);
}

#[test]
fn use_template_ref_accepts_a_template_literal() {
    let model = sfc(r#"<script setup>
import { useTemplateRef } from 'vue'
const a = useTemplateRef(`a`)
const b = useTemplateRef(tag`b`)
</script>
<template><input ref="a"><input ref="b"></template>
"#);
    let refs: Vec<_> = model
        .symbols()
        .filter(|symbol| symbol.kind() == SymbolKind::TemplateRef)
        .map(|symbol| (symbol.name().to_string(), symbol.uses().count()))
        .collect();
    assert_eq!(refs, [("a".into(), 1), ("b".into(), 0)]);
}

#[test]
fn template_refs_have_incomplete_uses_when_any_ref_may_be_read() {
    let cases = [
        // No script defines a component.
        r#"<template><input ref="a">{{ $refs[key] }}</template>"#,
        r#"<script src="./a.js"></script><template><input ref="a"></template>"#,
        // A component is defined.
        r#"<script setup>
import { useTemplateRef } from 'vue'
const a = useTemplateRef(name)
</script>
<template><input ref="a"></template>"#,
        r#"<script>
export default { mounted() { for (const key in this.$refs) {} } }
</script>
<template><input ref="a"></template>"#,
        r#"<script src="./a.js"></script>
<script setup>
const count = 0
</script>
<template><input ref="a"></template>"#,
    ];
    for code in cases {
        let model = sfc(code);
        let template_ref = model
            .symbols()
            .find(|symbol| symbol.kind() == SymbolKind::TemplateRef)
            .unwrap();
        assert!(!template_ref.has_complete_uses(), "{code}");
    }
}

#[test]
fn template_refs_have_complete_uses_when_read_by_name() {
    let model = sfc(r#"<template><input ref="a"><input ref="b">{{ $refs.a }}</template>"#);
    for template_ref in model
        .symbols()
        .filter(|symbol| symbol.kind() == SymbolKind::TemplateRef)
    {
        assert!(template_ref.has_complete_uses());
    }
}

#[test]
fn string_values_and_comments() {
    let code = r#"<template>
  <div :a="'foo'" :b="`foo`" :c="'it\'s'" :d="'a\\b'" :e="'\n'" :f="'\x41'" :g="'\0'" :h="`${x}`" :i="'foo' /* note */" :j="x" />
</template>
"#;
    let model = sfc(code);
    let div = model
        .elements()
        .find(|element| element.tag() == "div")
        .unwrap();
    let values: Vec<_> = div
        .attrs()
        .map(|attr| {
            let snippet = attr.value_snippet().unwrap();
            (
                attr.static_arg().unwrap().to_string(),
                snippet.string_value().map(|value| value.into_owned()),
                snippet.has_comments(),
            )
        })
        .collect();
    let expected: Vec<_> = [
        ("a", Some("foo"), false),
        ("b", Some("foo"), false),
        ("c", Some("it's"), false),
        ("d", Some("a\\b"), false),
        ("e", None, false),
        ("f", None, false),
        ("g", None, false),
        ("h", None, false),
        ("i", Some("foo"), true),
        ("j", None, false),
    ]
    .into_iter()
    .map(|(arg, value, comments)| (arg.to_string(), value.map(str::to_string), comments))
    .collect();
    assert_eq!(values, expected);
}

#[test]
fn attr_at_finds_the_attribute_with_the_exact_range() {
    let code = "<template>\n  <div id=\"x\" :title=\"'y'\" />\n</template>\n";
    let model = sfc(code);
    let start = code.find(":title").unwrap();
    let end = start + ":title=\"'y'\"".len();
    let range = TextRange::new(TextSize::from(start as u32), TextSize::from(end as u32));
    let attr = model.attr_at(range).unwrap();
    assert_eq!(attr.static_arg(), Some("title"));
    assert!(
        model
            .attr_at(TextRange::at(range.start(), 1.into()))
            .is_none()
    );
}
