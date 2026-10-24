<script>
/* should generate diagnostics */
export default {
  props: {
    foo: String,
  },
};
</script>

<script setup lang="ts">
const { bar } = defineProps<{ bar?: string }>();
</script>
