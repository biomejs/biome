<!-- should not generate diagnostics -->
<script setup>
import { ref, watch } from 'vue';

const foo = ref(0);

watch(foo, (newVal, oldVal) => {
	console.log('new: %s, old: %s', newVal, oldVal);
});
</script>
