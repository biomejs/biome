<script setup lang="ts">
/* should generate diagnostics */
type Emits = { click: [] };
interface OtherEmits {
  (e: "keydown"): void;
}
defineEmits<Emits>();
defineEmits<(OtherEmits)>();
</script>
