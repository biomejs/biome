<!-- should not generate diagnostics -->
<script setup lang="ts">
const a = defineEmits(["change", "update"]);

const b = defineEmits({
	change: (id) => typeof id === "number",
	update: null,
});

const c = defineEmits();
</script>
