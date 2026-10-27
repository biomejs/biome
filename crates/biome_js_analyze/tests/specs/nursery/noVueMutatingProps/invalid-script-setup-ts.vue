<!-- should generate diagnostics -->
<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    value?: string;
    items: string[];
  }>(),
  {
    value: "",
  },
);

function update() {
  props.value = "test";
  props.items!.push("item");
  (props.items as string[]).unshift("item");
}
</script>
