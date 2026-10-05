/* should not generate diagnostics */
function defineComponent(options) {
	return options;
}

defineComponent({
	name: "Foo",
});

defineOptions({
	name: "Foo",
});
