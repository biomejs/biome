/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

defineComponent({
	name: "Foo",
});

defineComponent(() => {}, {
	name: "Foo",
});

defineComponent({
	name: `Foo`,
});

createApp({
	name: "Foo",
});

app.component("Foo", {
	template: "<div />",
});

app.component("Bar", defineComponent({}));

app.component(("Bar"), ({}));
