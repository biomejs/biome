/* should not generate diagnostics */
import { createApp } from "vue";

const another = function () {
  this.$on("start", (args) => {
    console.log("start");
  });
};

createApp({
  mounted() {
    this.$emit("start");
  },
});

const notAComponent = {
  mounted() {
    this.$on("start", handler);
  },
};
