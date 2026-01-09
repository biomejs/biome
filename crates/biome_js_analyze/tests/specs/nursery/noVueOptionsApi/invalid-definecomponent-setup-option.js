// should generate diagnostics
// defineComponent with setup option inside object is still Options API style
// Use defineComponent with function signature instead for Vapor Mode
import { defineComponent, ref } from "vue";

export default defineComponent({
  setup() {
    const count = ref(0);
    return { count };
  }
});
