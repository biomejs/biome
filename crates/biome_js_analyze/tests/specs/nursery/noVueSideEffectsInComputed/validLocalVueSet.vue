/* should not generate diagnostics */

<script>
export default {
  computed: {
    // A local object that happens to own a `set` method is not Vue.
    localVue() {
      const Vue = {
        set(target, key, value) {
          return value
        },
      }
      return Vue.set({}, 'foo', 'bar')
    },
  },
}
</script>
