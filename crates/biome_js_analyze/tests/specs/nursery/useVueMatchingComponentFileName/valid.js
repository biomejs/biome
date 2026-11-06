/* should not generate diagnostics */
import { createApp, defineComponent } from "vue";
import Foo from "./Foo.vue";

defineComponent({
	name: "valid",
});

defineComponent({
	name: `valid`,
});

createApp({
	name: "valid",
});

app.component("valid", {
	name: "valid",
});

// The name isn't a static string.
defineComponent({
	name: someName,
});

defineComponent({
	name: `${prefix}Foo`,
});

defineComponent({
	name: tag`Foo`,
});

app.component(someName, {});

// The component is defined elsewhere.
app.component("Foo", Foo);

// These calls don't register components.
app.component("Foo");
app.component("Foo", {}, extra);
app.nested.component("Foo", {});
component("Foo", {});

// The object isn't a component definition outside `.vue`, `.jsx`, and `.tsx` files.
export default {
	name: "Foo",
};
