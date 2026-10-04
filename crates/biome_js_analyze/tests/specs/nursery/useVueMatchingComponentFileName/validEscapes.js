/* should not generate diagnostics */
import { defineComponent } from "vue";

defineComponent({
	name: "validEscapes",
});

defineComponent({
	name: `validE\x73capes`,
});
