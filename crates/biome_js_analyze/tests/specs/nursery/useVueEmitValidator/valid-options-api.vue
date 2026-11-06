<script>
/* should not generate diagnostics */
import { isNumber } from "./validators";

function shorthand() {}

export default {
  emits: {
    arrow: (payload) => typeof payload === "object",
    noParameters: () => {},
    method(payload) {
      return typeof payload === "object";
    },
    emptyMethod() {},
    functionExpression: function (payload) {
      return true;
    },
    reference: isNumber,
    shorthand,
  },
};
</script>
