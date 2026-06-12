import { defineComponent, createApp } from "vue";

/* Invalid: single-word name "Header" (not ignored, not builtin) */
defineComponent({
	name: "Header"
});

/* Invalid: single-word name "Widget" */
export default {
	name: "Widget"
};

createApp({ name: "Foo" })
