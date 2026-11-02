<script setup lang="ts">
/* should generate diagnostics */
interface Props {
  foo?: number;
  bar?: string;
  baz?: boolean;
  qux: string;
  onClose?(): void;
}

withDefaults(defineProps<Props>(), {
  bar: "bar",
});
</script>
