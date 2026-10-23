<!-- should not generate diagnostics -->
<script lang="ts">
import shared from "./shared";

interface FormState {
  name: string;
}

export default {
  mixins: [shared],
  data() {
    return {
      form: {} as FormState,
      user: { name: "" },
    };
  },
  methods: {
    read() {
      return this.form.name + this.user.age;
    },
  },
};
</script>
