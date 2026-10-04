<script>
/* should generate diagnostics */
const scroll = null;
export default {
  "emits": {
    click: null,
    keydown(key) {
      return typeof key === "string";
    },
    "mouseup": (event) => true,
    scroll,
  },
};
</script>
