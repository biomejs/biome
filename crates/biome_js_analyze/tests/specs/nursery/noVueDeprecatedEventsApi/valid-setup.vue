<!-- should not generate diagnostics -->
<script setup>
const instance = getCurrentInstance();
instance.$on("start", handler);
</script>
