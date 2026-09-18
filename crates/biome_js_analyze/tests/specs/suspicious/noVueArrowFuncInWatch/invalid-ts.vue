<script lang="ts">
/* should generate diagnostics */
export default {
  watch: {
    foo: (val: string, oldVal: string) => {
      console.log('new: %s, old: %s', val, oldVal)
    },
    bar: {
      handler: async (val: number, oldVal: number) => {
        console.log('new: %s, old: %s', val, oldVal)
      }
    }
  }
}
</script>
