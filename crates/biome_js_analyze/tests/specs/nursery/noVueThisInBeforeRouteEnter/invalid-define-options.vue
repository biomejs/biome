<script setup>
/* should generate diagnostics */
defineOptions({
  beforeRouteEnter(to, from, next) {
    this.load();
    next();
  },
});
</script>
