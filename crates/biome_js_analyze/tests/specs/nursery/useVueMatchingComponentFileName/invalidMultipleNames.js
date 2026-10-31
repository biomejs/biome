/* should generate diagnostics */
import { defineComponent } from "vue";

app.component("Foo", {
	name: "Bar",
});

app.component("Foo", defineComponent({
	name: "Bar",
}));
