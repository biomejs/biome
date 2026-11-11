<!-- should generate diagnostics -->
<script>
export default {
  data() {
    return {
      user: { name: "", address: { city: "" } },
    };
  },
  methods: {
    read() {
      return this.user.nmae + this.user.address.zip;
    },
  },
};
</script>

<template>
  <p>{{ user.age }} {{ user.address.city.length }} {{ user.address.country.code }}</p>
</template>
