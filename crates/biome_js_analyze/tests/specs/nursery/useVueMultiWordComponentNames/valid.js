/* should not generate diagnostics */
import { defineComponent, createApp } from "vue";

/* Valid: ignored single-word name (default ignore) */
export default {
	name: "App"
};

/* Valid: kebab-case multi-word */
defineComponent({
	name: "my-component"
});

/* Valid: PascalCase multi-word */
defineComponent({
	name: "MyComponent"
});

/* Valid: Vue builtin component name (ignored) */
defineComponent({
	name: "Transition"
});

createApp({ name: "MyApp" })
