<!-- should generate diagnostics -->
<ul>
  <li v-for="item in items">{{ item }}</li>
</ul>

<div class="wrapper" v-for="(item, i) in items" data-id="foo"></div>

<template v-for="item in items">
  <li>{{ item }}</li>
</template>
