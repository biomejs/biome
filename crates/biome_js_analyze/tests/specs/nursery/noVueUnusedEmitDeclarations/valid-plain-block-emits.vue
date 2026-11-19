<!-- should not generate diagnostics -->
<script>
export default {
  methods: {
    go() {
      this.$emit("foo");
    },
  },
};
</script>

<script setup>
defineEmits(["foo"]);
</script>
