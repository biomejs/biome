<script>
export default {
  props: ['foo'],
  computed: {
    foo () {}
  },
  data () {
    return {
      foo: null
    }
  },
  methods: {
    foo () {}
  },
  setup () {
    const foo = ref(1)

    return {
      foo
    }
  }
}
</script>