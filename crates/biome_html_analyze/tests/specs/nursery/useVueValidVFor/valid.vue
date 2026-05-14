<!-- should not generate diagnostics -->
<div v-for="item in items"></div>

<div v-for="({ id }, index) in items" :key="id"></div>

<MyItem v-for="item in items" :key="item.id" />

<div :is="componentName" v-for="item in items" :key="item.id"></div>

<template v-for="item in items" :key="item.id">
  <MyItem />
</template>

<template v-for="item in items">
  <div :key="item.id"></div>
</template>

<template v-for="item in items">
  <div v-for="child in item.children" :key="child.id"></div>
</template>

<template v-for="item in items">
  <div class="nested" v-for="child in item.children" :key="child.id"></div>
</template>

<template v-for="item in items">
  <div></div>
</template>
