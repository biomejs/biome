<!-- should not generate diagnostics -->
<script setup>
function setOptions() {
  defineOptions();
}

if (import.meta.env.DEV) {
  defineOptions({ props: {} });
}
</script>
