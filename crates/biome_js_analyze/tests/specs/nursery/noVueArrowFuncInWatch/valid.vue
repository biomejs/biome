<script>
/* should not generate diagnostics */
export default {
  watch: {
    a: function (val, oldVal) {
      console.log('new: %s, old: %s', val, oldVal)
    },
    b: 'someMethod',
    c: {
      handler: function (val, oldVal) { /* ... */ },
      deep: true
    },
    d: {
      handler: 'someMethod',
      immediate: true
    },
    e: [
      'handle1',
      function handle2 (val, oldVal) { /* ... */ },
      {
        handler: function handle3 (val, oldVal) { /* ... */ },
        /* ... */
      }
    ],
    'e.f': function (val, oldVal) { /* ... */ }
  }
}
</script>
