<script setup>
/* should not generate diagnostics */
const guards = {
  beforeRouteEnter() {
    this.method();
  },
};
</script>
