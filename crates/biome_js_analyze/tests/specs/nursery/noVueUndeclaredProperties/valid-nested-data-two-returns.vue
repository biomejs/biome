<!-- should not generate diagnostics -->
<script>
export default {
  props: ["guest"],
  data() {
    if (this.guest) {
      return { user: { name: "" } };
    }
    return { user: { email: "" } };
  },
  methods: {
    read() {
      return this.user.name + this.user.email;
    },
  },
};
</script>
