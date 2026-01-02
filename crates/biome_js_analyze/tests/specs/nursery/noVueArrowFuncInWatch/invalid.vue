<script>
/* should generate diagnostics */
export default {
  watch: {
    foo: (val, oldVal) => {
      console.log('new: %s, old: %s', val, oldVal)
    },
    bar: {
      handler: (val, oldVal) => {
        console.log('new: %s, old: %s', val, oldVal)
      }
    },
    short: (val, oldVal) => console.log('new: %s, old: %s', val, oldVal),
    qux: async (val, oldVal) => console.log('new: %s, old: %s', val, oldVal),
    empty: () => { },
    single: x => { /* body */ },
		comments: x /* comment */ => { /* body */ },
  }
}
</script>
