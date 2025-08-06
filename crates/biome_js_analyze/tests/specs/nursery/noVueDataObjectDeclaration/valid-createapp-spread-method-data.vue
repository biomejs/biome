<script>
// createApp with spread + method `data`
import { createApp } from 'vue';

createApp({
	...data,
	data() {
		return {
			foo: 'bar'
		};
	}
}).mount('#app');
</script>
