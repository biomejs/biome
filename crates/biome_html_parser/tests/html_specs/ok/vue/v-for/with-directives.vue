<template v-for="item in items">
	<div :key="item.id">{{ item.name }}</div>
</template>
<component v-for="(item, index) in items" :is="item.component" :key="index" />
<div v-if="enabled" v-for="item in items" :key="item.id">{{ item.name }}</div>
