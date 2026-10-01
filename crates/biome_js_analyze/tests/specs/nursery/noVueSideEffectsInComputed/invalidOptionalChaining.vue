<script>
export default {
  computed: {
    optionalCall() {
      return this?.items?.reverse?.()
    },
    parenthesizedObject() {
      return (this?.items)?.reverse?.()
    },
  },
}
</script>
