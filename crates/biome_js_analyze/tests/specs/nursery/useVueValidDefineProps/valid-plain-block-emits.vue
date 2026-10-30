<!-- should not generate diagnostics -->
<script>
export default {
  emits: ["change"],
};
</script>
<script setup>
defineProps({ msg: String });
</script>
