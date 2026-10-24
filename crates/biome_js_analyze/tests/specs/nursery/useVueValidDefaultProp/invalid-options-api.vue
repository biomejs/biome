<script>
/* should generate diagnostics */
export default {
  props: {
    a: { type: [Number, String], default: {} },
    b: { type: [Number, Object], default: {} },
    c: { type: Number, default: "" },
    d: { type: Number, default: false },
    e: { type: Number, default: [] },
    f: { type: String, default: 2 },
    g: { type: String, default: 1n },
    h: { type: Boolean, default: "" },
    i: { type: Object, default: "" },
    j: { type: Object, default: {} },
    k: { type: Object, default: [] },
    l: { type: Array, default: 55 },
    m: { type: Array, default: [] },
    n: { type: Function, default: 1 },
    o: { type: String, default: Number?.() },
    p: { type: String, default: /regex/ },
    q: { type: Number, default: `template` },
    "quoted": { type: Object, default: "" },
    ["computed"]: { type: Object, default: "" },
    r: { type: Date, default: "2020-01-01" },
    s: { type: String, default: ("parenthesized", 1) },
  },
};
</script>
