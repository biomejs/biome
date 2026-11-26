<!-- should generate diagnostics -->
<script setup lang="ts">
interface Base {
  size?: number;
}
interface Props extends Base {
  title: string;
}
const props = withDefaults(defineProps<Props>(), {
  title: "Title",
});
</script>
