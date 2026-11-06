<!-- should not generate diagnostics -->
<script setup>
defineOptions({
  inheritAttrs: [1, 2].some((item) => item > 1),
  data() {
    const count = 0;
    return { count };
  },
});
</script>
