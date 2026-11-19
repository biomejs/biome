<!-- should generate diagnostics -->
<script setup lang="ts">
import { computed, ref, reactive } from "vue";

const count = ref(0);
const user = reactive({ name: "" });
const items = ref<number[]>([]);

const doubled = computed(() => {
  count.value++;
  return count.value * 2;
});
const sorted = computed(() => items.value.sort());
computed(() => {
  user.name = "";
  return user;
});
const withObject = computed({
  get() {
    Object.assign(user, { name: "lorem" });
    return user.name;
  },
  set(value: string) {
    user.name = value;
  },
});
const doubledFunction = computed(function () {
  delete user.name;
  return 1;
});
const nonNull = computed(() => items.value!.push(1));
</script>
