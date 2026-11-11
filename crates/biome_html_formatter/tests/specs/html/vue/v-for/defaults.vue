<template>
	<div v-for="{ value = gt('v-for default') } in rows">{{ value }}</div>
	<div v-for="{ a=1,b=[2,3] } in rows">{{ a }}</div>
	<div v-for="( item=fallback , index ) of items">{{ item }}</div>
	<div v-for='{ text = "quoted" } in rows'>{{ text }}</div>
	<div v-for='{ text = `say "hi"` } in rows'>{{ text }}</div>
	<div v-for="{ key = 'k'in obj } in rows">{{ key }}</div>
</template>
