<script>
export default {
  ...foo(),
  props: {
    ...foo(),
    foo: String
  },
  computed: {
    ...mapGetters({
      test: 'getTest'
    }),
    bar: {
      get () {}
    }
  },
  data: {
    ...foo(),
    dat: null
  },
  methods: {
    ...foo(),
    test () {}
  },
  data () {
    return {
      ...dat
    }
  }
}
</script>