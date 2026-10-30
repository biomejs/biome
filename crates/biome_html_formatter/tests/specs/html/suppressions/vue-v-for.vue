<template>
	<!-- biome-ignore format: keep as written -->
	<li v-for='item   in   ["a","b"]' :key='item'>{{ item }}</li>
	<!-- biome-ignore format: keep as written -->
	<li v-for='x in [`say "hi"`]'>{{ x }}</li>
	<ul v-for='group   in   groups'>
		<!-- biome-ignore format: keep as written -->
		<li v-for='item   in   group.items'>{{ item }}</li>
		<li v-for='item   in   group.items'>{{ item }}</li>
	</ul>
	<li v-for='item   in   ["a","b"]'>{{ item }}</li>
</template>
