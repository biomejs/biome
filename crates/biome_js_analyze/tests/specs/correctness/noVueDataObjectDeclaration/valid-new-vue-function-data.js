/* should not generate diagnostics */
// new Vue with function `data`
new Vue({
	data: function () {
		return {
			foo: 'bar'
		};
	}
});
