<!-- should not generate diagnostics -->
<script>
import { computed, ref } from "vue";
import { store } from "./store";

const moduleState = { count: 0 };

export default {
  setup() {
    const foo = useFoo();
    const items = ref([]);
    const getter = () => foo.value;

    const test0 = computed(getter);
    const test1 = computed(() => foo.firstName + foo.lastName);
    const test2 = computed(() => [...items.value].reverse());
    const test3 = computed(() => {
      const local = { list: [] };
      local.list.push(foo.value);
      local.count = 1;
      function helper() {}
      helper.name = "";
      class Local {}
      Local.name = "";
      return local;
    });
    const test4 = computed({
      get: () => foo.firstName,
      set: (value) => {
        foo.firstName = value;
        items.value.push(value);
      },
    });
    const test5 = computed(() => {
      return () => {
        foo.count++;
      };
    });
    const test6 = computed(() => store.items.reverse());
    const test7 = computed(() => moduleState.count++);
    const test8 = computed(() => utils.reverse(foo.array));
    const test9 = computed(() => Object.assign({}, foo.data, { extra: "value" }));
    const test10 = computed(() => (console.log("a"), true));
    const test11 = computed((previous) => {
      previous.push(1);
      return previous;
    });
    const test12 = computed(() => {
      return {
        reset() {
          foo.count = 0;
        },
      };
    });

    const test13 = computed(() => {
      const out = {};
      out[foo.id] = true;
      return out;
    });
    const test14 = computed(() =>
      debounce(() => {
        foo.saved = true;
      }),
    );

    function update() {
      foo.count++;
      items.value.reverse();
    }

    return { test0, test1, test2, test3, test4, test5, test6, test7, test8, test9, test10, test11, test12, test13, test14, update };
  },
};
</script>
