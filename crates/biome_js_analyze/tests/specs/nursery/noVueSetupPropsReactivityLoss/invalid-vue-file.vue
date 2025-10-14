<template>
  <div>{{ count }}</div>
</template>

<script>
export default {
  setup(props) {
    const { count } = props;
    return { count };
  }
}
</script>
