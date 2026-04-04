<script>
/* should not generate diagnostics */
// new Vue with method shorthand `data`
new Vue({
	...data,
	data() {
		return {
			foo: 'bar'
		};
	}
});
</script>
