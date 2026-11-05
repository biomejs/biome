<!-- should not generate diagnostics -->
<script>
export default {
  beforeCreate() {},
  mounted() {},
  twoWay: true,
  props: {
    label: { type: String, required: true },
    other: ["twoWay"],
  },
  spread: {
    ...bar,
  },
  qux: {
    ...bar,
    baz,
  },
  computed: {
    full() {},
  },
  data() {
    return { init: 1, ready: 2 };
  },
  methods: {
    init() {},
  },
};
</script>
