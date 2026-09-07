<!-- should not generate diagnostics -->
<template>
  <!-- biome-ignore lint/a11y/noHeaderScope: intentional scope attribute -->
  <div scope="col"></div>

  <!-- biome-ignore lint/a11y/noHeaderScope: intentional scope binding -->
  <div :scope="'row'"></div>
</template>
