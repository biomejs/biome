<script setup lang="ts">
/* should generate diagnostics */
interface Props {
  foo?: boolean;
  bar?: true | false;
  baz?: (boolean);
}

const props = withDefaults(defineProps<Props>(), {
  foo: true,
  bar: false,
  baz() {
    return true;
  },
});
</script>
