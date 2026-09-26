/* should not generate diagnostics */

<script>
export default {
  computed: {
    fullName: {
      get() {
        return this.firstName + ' ' + this.lastName
      },
      set(value) {
        const names = value.split(' ')
        this.firstName = names[0]
        this.lastName = names[names.length - 1]
        this.history.push(value)
      },
    },
  },
  methods: {
    rename(value) {
      this.firstName = value
      this.history.push(value)
    },
  },
}
</script>
