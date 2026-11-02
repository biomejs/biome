<!-- should generate diagnostics -->
<script>
export default {
  data() {
    return {
      count: 0,
      items: [],
      user: { name: "" },
    };
  },
  methods: {
    onCount() {},
  },
  watch: {
    count: "onCount",
    total: "onCount",
    "user.nmae": "onCount",
    "user.name": "onNameChange",
    items: ["onCount", { handler: "onItemsChange", deep: true }],
  },
  expose: ["count", "reset"],
};
</script>
