<!-- should not generate diagnostics -->
<script setup lang="ts">
interface Props {
  name?: string;
  age: number;
  get getter(): string;
  set getter(value: string);
  [computed]: string;
}
const computed = "computed";
const props = withDefaults(defineProps<Props>(), {
  name: "World",
  getter: "World",
  [computed]: "World",
});
</script>
