<!-- should generate diagnostics -->
<div v-for:arg="item in items"></div>

<div v-for.mod="item in items"></div>

<div v-for></div>

<div v-for=""></div>

<div v-for="(, index) in items"></div>

<div v-for="(item, { key }) in items"></div>

<MyItem v-for="item in items"></MyItem>

<div is="MyItem" v-for="item in items"></div>

<div v-for="item in items" :key="foo"></div>

<template v-for="item in items">
  <MyItem />
</template>

<template v-for="item in items">
  <div v-for="child in other" :key="child.id"></div>
</template>
