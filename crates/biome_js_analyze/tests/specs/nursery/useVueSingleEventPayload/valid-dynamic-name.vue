<!-- should not generate diagnostics -->
<script>
export default {
  methods: {
    onChange(eventName) {
      this.$emit(eventName, value1, value2);
    },
  },
};
</script>
