<script setup>
/* should not generate diagnostics */
import { ref, watch } from 'vue';

const foo = ref(0);

watch(foo, (new, old) => {
	console.log('new: %s, old: %s', new, old);
});
</script>
