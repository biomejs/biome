<!-- should not generate diagnostics -->
<script>
import other from "./other";

export default {
  mixins: [other],
  computed: {
    fromMember() {
      return this.fromMixin;
    },
  },
  methods: {
    run() {
      this.fromMember();
    },
  },
};
</script>
