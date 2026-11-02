<script setup lang="ts">
/* should generate diagnostics */
interface Props {
  a?: string;
  b?: number;
  c?: boolean;
  d?: string[];
  e?: { foo: string };
  f?: string | number;
  g?: () => void;
}

withDefaults(defineProps<Props>(), {
  a: 42,
  b: "s",
  c: {},
  d: [],
  e: { foo: "foo" },
  f: [42],
  g: 10,
});
</script>
