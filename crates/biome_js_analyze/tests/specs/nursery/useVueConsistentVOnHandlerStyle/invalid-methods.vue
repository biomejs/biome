<!-- should generate diagnostics -->
<script>
export default {
  methods: {
    noParams() {},
    oneParam(a) {},
    rest(...a) {},
    withDefault(a = 42) {},
    arrowTwo: (a, b) => {},
    shorthand,
  },
};
</script>
<template>
  <button @click="noParams()" />
  <button @click="oneParam()" />
  <button @click="rest()" />
  <button @click="withDefault()" />
  <button @click="(a, b) => arrowTwo(a, b)" />
  <button @click="(a) => oneParam(a)" />
  <button @click="(a) => arrowTwo(a)" />
  <button @click="(a) => rest(a)" />
  <button @click="(a, b) => shorthand(a, b)" />
  <button @click="shorthand()" />
</template>
