/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

createApp({
  components: {
    Child: defineComponent({
      methods: {
        onClick() {
          this.$emit("click");
        },
      },
    }),
  },
  methods: {
    onKeydown() {
      this.$emit("keydown");
    },
  },
});
