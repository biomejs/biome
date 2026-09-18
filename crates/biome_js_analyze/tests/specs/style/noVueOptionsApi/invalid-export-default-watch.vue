<script>
// should generate diagnostics
// Options API: watch is not supported in Vapor Mode
export default {
  watch: {
    count(newVal, oldVal) {
      console.log('count changed')
    }
  }
}
</script>

