<!-- should generate diagnostics -->
<script setup lang="ts">
type Emits = {
  change: [value: string];
};

const emit = defineEmits<Emits>();

emit("close");
</script>
