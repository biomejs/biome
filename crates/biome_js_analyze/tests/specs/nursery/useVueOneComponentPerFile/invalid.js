/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

createApp({});
createApp({});

defineComponent({});
defineComponent(() => () => null);
defineComponent((props) => () => null, { props: ["title"] });

Vue.defineComponent({ name: "FromGlobal" });

defineComponent((props) => () => null, sharedOptions);
