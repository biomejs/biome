<template>
	<div v-for="{ value = gt('v-for default') } in rows" />
	<div v-for="[first = 1, second = getSecond()] in pairs" />
	<div v-for="{ id, label: name = fallback(id, 'none') } in users" />
	<div v-for="({ size = { width: 1, height: [2, 3] } }, index = 0) in boxes" />
	<div v-for="item = defaultItem of items" />
	<div v-for="{ a=1,b=[2,3] } in rows" />
	<div v-for="{ text = ')]}' } in rows" />
	<div v-for='{ text = "in" } in rows' />
	<div v-for="{ text = `${prefix}-in` } in rows" />
	<div v-for="{ flag = ready ? yes : no } in rows" />
	<div v-for="{ size = of.length, key = 'k'in obj } in rows" />
	<div v-for="{ re = /,/.test(text), other } in rows" />
	<div v-for="{ re = /[)\]}]/, half = total / 2, other } in rows" />
	<div v-for="{ a = 1 /* , } */, b } in rows" />
</template>
