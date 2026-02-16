<script>
/* should not generate diagnostics */
// const app = createApp + app.component with function `data` (dot access)
import { createApp } from 'vue';

const app = createApp(App);
app.component('some-comp', {
	data: function () {
		return {
			foo: 'bar'
		};
	}
});
</script>
