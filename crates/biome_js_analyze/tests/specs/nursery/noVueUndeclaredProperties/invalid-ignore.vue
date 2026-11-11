<!-- should generate diagnostics -->
<script>
export default {
  methods: {
    greet() {
      return this.i18n.translate("hello") + this.other;
    },
  },
};
</script>
