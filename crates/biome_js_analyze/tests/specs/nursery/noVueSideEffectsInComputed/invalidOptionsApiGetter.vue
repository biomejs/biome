<script>
export default {
  computed: {
    pair: {
      get() {
        this.firstName = 'lorem'
        return this.firstName + ' ' + this.lastName
      },
      set(value) {
        this.firstName = value
      },
    },
    assigned: function () {
      return this.items.reverse()
    },
  },
}
</script>
