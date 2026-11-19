<!-- should not generate diagnostics -->
<script>
const events = ["notify"];

defineEmits(events);
defineEmits();

export default {
	setup() {
		defineEmits(events);
		defineEmits();
	},
};
</script>
