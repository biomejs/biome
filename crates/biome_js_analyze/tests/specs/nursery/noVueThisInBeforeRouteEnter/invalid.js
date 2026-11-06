/* should generate diagnostics */
import { createApp, defineComponent } from "vue";

export const A = defineComponent({
  beforeRouteEnter() {
    this.a();
  },
});

export const B = defineComponent(() => {}, {
  "beforeRouteEnter": function () {
    this.b();
  },
});

export const C = defineComponent(({
  ["beforeRouteEnter"]() {
    this.c();
  },
}));

createApp({
  beforeRouteEnter() {
    this.d();
  },
});
