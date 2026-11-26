<script setup>
/* should generate diagnostics */
const a = defineModel({ type: Boolean, default: "hello" });
const b = defineModel("count", { type: Number, default: false });
const c = defineModel("list", { type: Array, default: [] });
const d = defineModel("options", { type: Object, default: {} });
const e = defineModel("label", { type: String, default: () => 123 });
</script>
