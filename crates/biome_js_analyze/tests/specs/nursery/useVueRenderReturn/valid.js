/* should not generate diagnostics */
import { defineComponent, h } from "vue";

defineComponent({
	...foo,
	render() {
		return {};
	},
});

defineComponent({
	foo() {},
});

defineComponent({
	foo: {},
});

defineComponent({
	render: foo,
});

defineComponent({
	render: () => null,
});

defineComponent({
	render: () => {
		return h("div");
	},
});

defineComponent({
	render() {
		const foo = function () {};
		return foo;
	},
});

defineComponent({
	render() {
		if (a) {
			if (b) {
			}
			if (c) {
				return true;
			} else {
				return foo;
			}
		} else {
			return foo;
		}
	},
});

defineComponent({
	render() {
		if (a) {
			return `<div>a</div>`;
		} else {
			return `<span>a</span>`;
		}
	},
});

defineComponent({
	render() {
		const options = [];
		this.matches.forEach(function (match) {
			options.push(match);
		});
		return h("div", options);
	},
});

defineComponent({
	render() {
		switch (this.type) {
			case "a":
				return h("div");
			case "b":
				return h("span");
			default:
				return h("p");
		}
	},
});

defineComponent({
	render() {
		if (!this.ready) {
			throw new Error("Not ready");
		}
		return h("div");
	},
});

defineComponent({
	render() {
		try {
			return h("div");
		} catch (error) {
			return h("span", String(error));
		}
	},
});

// Not a Vue component.
const options = {
	render() {},
};
