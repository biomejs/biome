<!-- should generate diagnostics -->
<script lang="ts">
export default {
  props: {
    user: Object,
    settings: { type: Object, default: () => ({}) },
    items: [Array, String],
  },
};
</script>
