<!-- should not generate diagnostics -->
<script>
export default {
  methods: {
    reset() {
      this.$emit("update:modelValue", "");
    },
  },
};
</script>

<script setup>
defineModel();
</script>
