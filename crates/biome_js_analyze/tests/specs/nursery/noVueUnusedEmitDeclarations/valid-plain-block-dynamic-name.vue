<!-- should not generate diagnostics -->
<script>
export default {
  methods: {
    go(name) {
      this.$emit(name);
    },
  },
};
</script>

<script setup>
defineEmits(["foo"]);
</script>
