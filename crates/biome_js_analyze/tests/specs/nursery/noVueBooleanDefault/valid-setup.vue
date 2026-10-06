<script setup lang="ts">
/* should not generate diagnostics */
interface Props {
  foo?: boolean;
  bar?: boolean | string;
  baz?: string;
}

const { baz = "baz" } = withDefaults(defineProps<Props>(), {
  bar: "bar",
});
</script>
