<!-- should generate diagnostics -->
<script>
export default {
  props: ["name"],
  setup(props) {
    console.log(props.name, props.title);
    return { greeting: "hello" };
  },
};
</script>

<template>
  <p>{{ greeting }} {{ greting }}</p>
</template>
