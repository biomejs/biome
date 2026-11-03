<!-- should generate diagnostics -->
<script setup lang="ts">
const events = ["notify"];
const prefix = "update";
let count = 0;
function validate(payload: string) {
	return payload.length > 0;
}

const emit = defineEmits({
	notify: validate,
	[`${prefix}:value`]: (value: string) => events.includes(value),
	submit: () => {
		count += 1;
		return true;
	},
});
</script>
