<script>
/* should generate diagnostics */
export default {
  beforeRouteEnter: function (to, from, next) {
    this.method();
    next();
  },
};
</script>
