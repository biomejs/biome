<script setup lang="ts">
/* should not generate diagnostics */
export interface SomePropInterface {
  someProp?: false | string;
  str?: "foo" | "bar";
  num?: 1 | 2;
  list?: string[];
  obj?: { foo: string };
  callback?: () => void;
  alias?: MaybeString;
}

type MaybeString = string | number;

const { alias = 1 } = withDefaults(defineProps<SomePropInterface>(), {
  someProp: false,
  str: "foo",
  num: 1,
  list: () => ["a"],
  obj: () => ({ foo: "foo" }),
  callback: () => {},
});
</script>
