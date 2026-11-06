/* should generate diagnostics */
import { createApp, defineComponent, h } from "vue";

defineComponent({
	render() {},
});

defineComponent({
	render: function () {
		if (foo) {
			return h("div", "hello");
		}
	},
});

defineComponent({
	render: function () {
		if (a) {
			return;
		}
		return h("div");
	},
});

defineComponent({
	render: function () {
		if (a) {
		} else {
			return h("div", "hello");
		}
	},
});

defineComponent({
	render: () => {
		h("div");
	},
});

defineComponent({
	render() {
		switch (this.type) {
			case "a":
				return h("div");
			case "b":
				return h("span");
		}
	},
});

defineComponent({
	render() {
		switch (this.type) {
			case "a":
				return h("div");
			case "b":
				break;
		}
	},
});

defineComponent({
	render() {
		switch (this.type) {
		}
	},
});

defineComponent({
	render() {
		switch (this.type) {
			case "a":
			case "b":
		}
	},
});

defineComponent({
	render() {
		try {
			return h("div");
		} catch (error) {
			console.error(error);
		}
	},
});

defineComponent({
	render() {
		for (const item of this.items) {
			return h("div", item);
		}
	},
});

defineComponent({
	render: (function () {}),
});

createApp({
	render() {
		h("div");
	},
});
