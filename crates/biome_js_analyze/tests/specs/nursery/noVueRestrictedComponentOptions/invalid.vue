<!-- should generate diagnostics -->
<script>
const attached = () => {};
const name = "label";

export default {
  init() {},
  beforeCompile: function () {},
  attached,
  "detached": () => {},
  ["ready"]() {},
  props: ({
    size: Number,
    label: {
      type: String,
      twoWay: true,
    },
    [name]: {
      type: String,
      twoWay: true,
    },
  }),
  foo: {
    ...other,
    value: 1,
  },
  computed: {
    full: {
      get() {},
      set(value) {},
    },
  },
};
</script>
