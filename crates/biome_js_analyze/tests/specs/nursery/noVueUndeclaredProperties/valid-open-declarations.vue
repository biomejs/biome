<!-- should not generate diagnostics -->
<script>
import { mapState } from "vuex";
import shared from "./shared";

export default {
  mixins: [shared],
  computed: {
    ...mapState(["user"]),
  },
  methods: {
    greet() {
      return this.user.name + this.fromMixin;
    },
  },
};
</script>

<template>
  <p>{{ user }} {{ fromMixin }}</p>
</template>
