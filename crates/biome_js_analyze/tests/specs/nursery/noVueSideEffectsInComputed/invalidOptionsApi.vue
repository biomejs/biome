<script>
export default {
  computed: {
    assign() {
      this.firstName = 'lorem'
      return this.firstName + ' ' + this.lastName
    },
    compound() {
      this.count += 2
      return this.count
    },
    update() {
      this.count++
      --this.count
      return this.count
    },
    remove() {
      delete this.entries.first
      return this.entries
    },
    mutatingMethod() {
      return this.items.reverse()
    },
    nestedChain() {
      const pushed = this.outer.items.push('example')
      return pushed
    },
    computedMember() {
      this.items[index] = other[index]
      return this.items
    },
    deepMutatingMethod() {
      return this.items.keys.sort()
    },
    computedMethodName() {
      return this.items['reverse']()
    },
    assignOntoComponent() {
      return Object.assign(this, { flag: true })
    },
  },
}
</script>
