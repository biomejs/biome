<!-- should generate diagnostics -->
<script setup lang="ts">
type Props = {
  name: string;
  age?: number;
};
withDefaults(defineProps<Props>(), {
  name: "World",
});
</script>
