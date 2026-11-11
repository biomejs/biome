<script setup lang="ts">
/* should not generate diagnostics */
interface Props {
  required: string;
  optional?: number;
  flag?: boolean;
}

withDefaults(defineProps<Props>(), {
  optional: 42,
});
</script>
