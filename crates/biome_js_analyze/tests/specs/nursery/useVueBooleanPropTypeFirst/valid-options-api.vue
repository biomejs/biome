<!-- should not generate diagnostics -->
<script>
export default {
  props: {
    a: Boolean,
    b: String,
    c: [Boolean],
    d: [String],
    e: [Boolean, String],
    f: { type: Boolean },
    g: { type: String },
    h: { type: [Boolean] },
    i: { type: [String] },
    j: { type: [Boolean, String] },
    k: {},
    l: [String, Number],
  },
};
</script>
