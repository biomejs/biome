<!-- should generate diagnostics -->
<script>
export default {
  props: {
    a: [String, Boolean],
    b: { type: [String, Boolean] },
    c: [String, Number, Boolean],
    d: { type: [String, Boolean, Number] },
    e: [Number, Boolean],
    f: [
      String,
      Number,
      Boolean,
    ],
    g: [ String, Boolean ],
    h: ([String, Boolean]),
  },
};
</script>
