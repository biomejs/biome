<!-- should not generate diagnostics -->
<script>
import { shared, sharedProps, type } from "./shared";

export default {
  ...shared,
  props: {
    ...sharedProps,
    constructor: String,
    union: [String, Number],
    typed: { type: String },
    shorthandType: { type, required: true },
    functionType: { type: function Title() {} },
    computedKey: { ["type"]: String },
    validated: { validator: (value) => value > 0 },
    computedValidator: { ["validator"]: (value) => value > 0 },
    shorthand,
    variable: sharedProps.variable,
    call: makeProp(),
  },
};
</script>
