<!-- should generate diagnostics -->
<script>
export default {
  props: ["value", "list"],
  methods: {
    update() {
      this.value = "test";
      this.list.splice(0, 1);
    },
  },
};
</script>
