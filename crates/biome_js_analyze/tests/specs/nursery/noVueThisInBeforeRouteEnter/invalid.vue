<script>
/* should generate diagnostics */
export default {
  data() {
    return { greeting: "Hello" };
  },
  async beforeRouteEnter(to, from, next = this.fallback) {
    this.method();
    this.attribute = this.method();
    if (this.value === 42) {
    }
    if (true) {
      this.nested();
    }
    await Promise.resolve().then(() => this.inArrow());
    next();
  },
};
</script>
