<!-- should not generate diagnostics -->
<script>
export default {
  computed: {
    foo: {
      get () {},
      set () {}
    }
  }
}
</script>