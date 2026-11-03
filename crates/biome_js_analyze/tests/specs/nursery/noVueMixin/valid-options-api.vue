<!-- should not generate diagnostics -->
<script>
export default {
  name: "Counter",
  data() {
    return { mixins: [] };
  },
  methods: {
    mixins() {},
  },
};
</script>
