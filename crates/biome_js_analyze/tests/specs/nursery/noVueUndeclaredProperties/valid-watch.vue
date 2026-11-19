<!-- should not generate diagnostics -->
<script>
export default {
  data() {
    return { count: 0, items: [], user: { name: "" } };
  },
  methods: {
    onCount() {},
    onItems() {},
  },
  watch: {
    count: "onCount",
    items: ["onCount", { handler: "onItems", deep: true }],
    "user.name": { handler: "onCount", immediate: true },
    "$route.query"() {},
  },
  expose: ["count", "onCount"],
};
</script>
