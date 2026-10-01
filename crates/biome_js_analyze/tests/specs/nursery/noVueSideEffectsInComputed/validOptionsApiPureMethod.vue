/* should not generate diagnostics */

<script>
export default {
  data() {
    return { items: [2, 1] }
  },
  methods: {
    // A component method may share a name with an array mutator without being one.
    sort() {
      return [...this.items].sort()
    },
    push(item) {
      return [...this.items, item]
    },
  },
  computed: {
    sortedItems() {
      return this.sort()
    },
    withExtra() {
      return this.push(3)
    },
  },
}
</script>
