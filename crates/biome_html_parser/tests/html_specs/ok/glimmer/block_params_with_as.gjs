<template>
  {{#each items as |item|}}
    {{item}}
  {{/each}}

  {{#let value as |v|}}
    {{v}}
  {{/let}}
</template>
