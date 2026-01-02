<script>
/* should not generate diagnostics */
// Pure Composition API with setup() is valid
export default {
  setup() {
    const count = ref(0)
    const increment = () => count.value++
    return { count, increment }
  }
}
</script>

