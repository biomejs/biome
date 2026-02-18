<script>
/* should not generate diagnostics */
// createApp with function `data`
import { createApp } from 'vue';

createApp({
	data: function () {
		return {
			foo: 'bar'
		};
	}
}).mount('#app');
</script>
