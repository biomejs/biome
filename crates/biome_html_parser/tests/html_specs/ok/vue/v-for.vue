<template>
	<ul>
		<li v-for="(item, index) in items" :key="item.id">{{ item.label }} - {{ index }}</li>
	</ul>
	<div v-for="n in 5">{{ n }}</div>
	<div v-for="(value, key) in object">{{ key }}: {{ value }}</div>
	<div v-for="(foo, i) of foos" v-if="i % 2 === 0">{{ foo }}</div>
	<!-- basic array iteration -->
	<ul>
		<li v-for="item in items">{{ item }}</li>
	</ul>

	<!-- with index and key binding -->
	<ul>
		<li v-for="(item, index) in items" :key="index">{{ index }} - {{ item }}</li>
	</ul>

	<!-- iterate object values -->
	<div v-for="(value, key) in obj">{{ key }}: {{ value }}</div>

	<!-- iterate object with value, key, index and a computed key -->
	<div v-for="(value, key, index) in obj" :key="key + '-' + index">{{ index }}: {{ key }}</div>

	<!-- v-for on a template wrapper -->
	<template v-for="(item, i) in list">
		<li :key="i">{{ item }}</li>
	</template>

	<!-- numeric range iteration -->
	<component v-for="n in 10" :key="n" :prop="n" />

	<!-- nested v-for -->
	<ul>
		<li v-for="(items, group) in groups" :key="group">
			<span v-for="(item, i) in items" :key="i">{{ item }}</span>
		</li>
	</ul>

	<!-- combined with other directives -->
	<div v-for="x in xs" v-if="x.visible" :data-i="x.id">{{ x.label }}</div>
	<div v-else>fallback</div>

	<!-- destructuring-ish style (parser tolerance) -->
	<div v-for="(user, i) in users" :key="user.id">{{ user.name }}</div>
</template>
