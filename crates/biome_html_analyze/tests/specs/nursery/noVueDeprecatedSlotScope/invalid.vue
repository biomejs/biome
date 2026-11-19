<!-- should generate diagnostics -->
<template>
  <LinkList>
    <template slot-scope="{a}"><a /></template>
  </LinkList>
  <LinkList>
    <template slot-scope><a /></template>
  </LinkList>
  <LinkList>
    <a slot-scope="{a}" />
  </LinkList>
  <LinkList>
    <span class="item" slot-scope="props">{{ props.title }}</span>
  </LinkList>
  <link-list>
    <template slot-scope="props">{{ props.title }}</template>
    <template #header>Header</template>
    <div slot="footer">Footer</div>
  </link-list>
  <LinkList>
    <template v-if="ready" slot-scope="props">{{ props.title }}</template>
    <template v-else slot-scope="props">{{ props.title }}</template>
  </LinkList>
</template>
