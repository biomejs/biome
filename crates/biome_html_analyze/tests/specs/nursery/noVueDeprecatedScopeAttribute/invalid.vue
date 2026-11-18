<!-- should generate diagnostics -->
<template>
  <LinkList>
    <template scope="{ a }">
      <a />
    </template>
  </LinkList>
  <LinkList>
    <template slot="name" scope="props">
      {{ props.title }}
    </template>
  </LinkList>
  <LinkList>
    <template scope>
      <a />
    </template>
  </LinkList>
  <LinkList>
    <template v-if="show" scope='props' />
  </LinkList>
</template>
