<!-- should generate diagnostics -->
<script setup lang="ts">
interface Props {
  prop1?: string;
  prop2?: string;
  prop3?: string;
}
const props = withDefaults(defineProps<Props>(), { prop1: "a" });
</script>
