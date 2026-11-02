<script lang="ts">
/* should generate diagnostics */
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    a: {
      type: 'String',
      default: 10,
    } as PropOptions<string>,
    b: 'Number' as any,
    c: {
      type: 'Object' as unknown as PropType<Foo>,
    },
  },
});
</script>
