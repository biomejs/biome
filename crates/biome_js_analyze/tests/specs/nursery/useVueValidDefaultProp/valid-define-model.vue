<script setup lang="ts">
/* should not generate diagnostics */
const a = defineModel({ type: Boolean });
const b = defineModel("b", { type: String, required: true });
const c = defineModel("c", { type: Boolean, default: false });
const d = defineModel("d", { type: Number, default: 0 });
const e = defineModel("e", { type: Array, default: () => [] });
const f = defineModel("f", { type: Object, default: () => ({}) });
const g = defineModel<string>("g", { default: "hello" });
const h = defineModel<boolean>("h", { default: false });
const i = defineModel<string>("i", { required: true });
const j = defineModel<string>("j", { type: Number, default: 0 });
</script>
