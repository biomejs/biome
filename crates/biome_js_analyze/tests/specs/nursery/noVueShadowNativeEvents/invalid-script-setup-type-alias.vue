<!-- should generate diagnostics -->
<script setup lang="ts">
type Emits = { click: [] };
interface OtherEmits {
  (e: "keydown"): void;
}
defineEmits<Emits>();
defineEmits<(OtherEmits)>();
</script>
