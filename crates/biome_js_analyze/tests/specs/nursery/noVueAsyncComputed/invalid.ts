/* should generate diagnostics */
import { computed } from "vue";

const nonNullCallee = computed(() => bar.then!(() => {}));
const withTypeArguments = computed<number>(async () => 1);
const typeAssertion = computed(() => (Promise as PromiseConstructor).resolve(1));
