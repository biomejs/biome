<!-- should not generate diagnostics -->
<script setup>
const events = ["notify"];

function setup() {
	return defineEmits(events);
}

const result = (events.length, defineEmits(events));
</script>
