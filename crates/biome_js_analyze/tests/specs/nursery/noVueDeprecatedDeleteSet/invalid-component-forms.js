/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

defineComponent({
  mounted() {
    this.$set(obj, key, value);
  },
});

createApp({
  mounted() {
    this.$delete(obj, key);
  },
});

// `Vue` is not declared in this file, so it refers to the global Vue object.
Vue.set(obj, key, value);
Vue.delete(obj, key);
