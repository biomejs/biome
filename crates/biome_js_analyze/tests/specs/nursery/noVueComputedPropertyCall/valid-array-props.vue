<!-- should not generate diagnostics -->
<script>
export default {
  props: ["value"],
  computed: {
    fromProp() {
      return this.value;
    },
  },
  methods: {
    run() {
      this.fromProp();
    },
  },
};
</script>
