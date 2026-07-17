<div v-if="ok" v-for=" item   in   items " :key="item.id" @click="select(item)">{{ item }}</div>
<template v-for="( item , index ) in items" :key="index">
  <span>{{ item }}</span>
</template>
