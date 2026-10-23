<!-- should not generate diagnostics -->
<script>
export default {
  methods: {
    notify() {
      this.$emit("change");
    },
  },
};
</script>

<script setup>
defineEmits(["change"]);
</script>
