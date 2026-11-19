<!-- should generate diagnostics -->
<script lang="ts">
import type { Slot } from "vue";

export default {
  render() {
    this.$slots.default!(foo, bar);
    (this.$slots.default as Slot)(foo, bar);
    (this.$slots as Record<string, Slot>).default(foo, bar);

    const item = this.$slots.item!;
    item(foo, bar);
  },
};
</script>
