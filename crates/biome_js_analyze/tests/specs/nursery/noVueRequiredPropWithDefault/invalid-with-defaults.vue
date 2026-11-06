<!-- should generate diagnostics -->
<script setup lang="ts">
interface Props {
  name: string;
  "na::me": string | number;
  readonly title: string;
  foo(): void;
  age?: number;
}
const props = withDefaults(defineProps<Props>(), {
  name: "World",
  "na::me": "World",
  title: "Title",
  foo() {},
});
</script>
