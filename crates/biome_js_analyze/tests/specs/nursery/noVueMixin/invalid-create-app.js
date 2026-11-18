/* should generate diagnostics */
import { createApp } from "vue";
import counterMixin from "./counterMixin";

createApp({
  mixins: [counterMixin],
}).mount("#app");
