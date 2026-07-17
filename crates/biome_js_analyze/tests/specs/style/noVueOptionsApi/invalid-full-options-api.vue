<script>
// should generate diagnostics
// Full Options API component with multiple properties - all should be flagged
export default {
  name: 'MyComponent',
  props: ['message'],
  data() {
    return { count: 0 }
  },
  computed: {
    doubleCount() {
      return this.count * 2
    }
  },
  watch: {
    count(newVal) {
      console.log('changed')
    }
  },
  methods: {
    increment() {
      this.count++
    }
  },
  mounted() {
    console.log('mounted')
  }
}
</script>

