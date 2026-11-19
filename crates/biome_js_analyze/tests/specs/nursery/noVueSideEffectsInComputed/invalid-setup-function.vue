<!-- should generate diagnostics -->
<script>
import { computed, reactive } from "vue";

export default {
  setup() {
    const foo = useFoo();
    const nested = { inner: {} };
    const arr = reactive([]);
    function fn() {}
    class Klass {}

    const test1 = computed(() => {
      foo.firstName = "lorem";
      nested.inner.value = "lorem";
      return foo.firstName;
    });
    const test2 = computed(() => {
      foo.count += 2;
      foo.count++;
      return foo.count;
    });
    const test3 = computed(() => foo.something.reverse());
    const test4 = computed(() => arr.reverse());
    const test5 = computed({
      get: () => foo.list.sort(),
      set: (value) => {
        foo.list = value;
      },
    });
    const test6 = computed(() => {
      fn.name = "";
      Klass.name = "";
      return true;
    });
    const test7 = computed(() => (foo.a = "", true));
    const test8 = computed(() => Object.assign(foo.data, { extra: "value" }));

    return { test1, test2, test3, test4, test5, test6, test7, test8 };
  },
};
</script>
