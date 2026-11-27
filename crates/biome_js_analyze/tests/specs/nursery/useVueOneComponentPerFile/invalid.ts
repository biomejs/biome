/* should generate diagnostics */
import { defineComponent as define } from "vue";

export const A = define({ name: "A" } as const);
export const B = define({ name: "B" } satisfies object);
