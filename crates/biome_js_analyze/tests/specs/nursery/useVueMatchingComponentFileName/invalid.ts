/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
	name: "Foo",
	props: {
		foo: String as PropType<string>,
	},
});
