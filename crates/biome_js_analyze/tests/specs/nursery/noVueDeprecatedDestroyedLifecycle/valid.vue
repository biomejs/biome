<!-- should not generate diagnostics -->
<script>
const beforeDestroy = () => {};
const destroyed = () => {};
const others = {};

export default {
  beforeMount() {},
  mounted() {},
  beforeUnmount: beforeDestroy,
  unmounted: destroyed,
  [beforeDestroy]() {},
  [destroyed]() {},
  ...others,
  methods: {
    destroyed() {},
  },
  data() {
    return {
      beforeDestroy: 1,
    };
  },
};
</script>
