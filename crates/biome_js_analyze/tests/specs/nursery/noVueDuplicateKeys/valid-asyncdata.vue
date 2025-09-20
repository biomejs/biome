<script>
export default {
  asyncData() {
    return {
      foo: 'bar'
    }
  },
  data() {
    return {
      baz: 'qux'
    }
  },
  methods: {
    handleClick() {
      console.log('click');
    }
  }
}
</script>