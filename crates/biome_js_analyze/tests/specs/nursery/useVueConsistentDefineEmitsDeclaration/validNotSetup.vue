<!-- should not generate diagnostics -->
<script lang="ts">
export default {
	emits: ["change"],
	setup() {
		defineEmits(["change"]);
	},
};
</script>
