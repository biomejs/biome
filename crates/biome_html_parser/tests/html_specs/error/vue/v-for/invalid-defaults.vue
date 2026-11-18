<template>
	<div v-for="{ value = } in rows" />
	<div v-for="item = in rows" />
	<div v-for="{ value = (x in y) } in rows" />
	<div v-for="{ value = 'x in y' } in rows" />
	<div v-for="{ ...rest = fallback } in rows" />
	<div v-for="{ value = 1 /* , } in rows" />
	<div v-for="{ value = /* , }" />
	<p>after</p>
</template>
