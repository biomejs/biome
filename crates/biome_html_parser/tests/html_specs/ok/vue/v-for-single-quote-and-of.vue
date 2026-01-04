<template>
	<!-- single-quoted with of and tuple -->
	<div v-for='(item, i) of items'>{{ item }} - {{ i }}</div>

	<!-- single-quoted basic of -->
	<ul>
		<li v-for='item of list'>{{ item }}</li>
	</ul>

	<!-- ensure plain attributes named in/of aren’t treated specially -->
	<div in of data-in="foo" data-of="bar">plain content in of</div>

	<!-- ensure text content containing keywords is not special -->
	<p>Words: in, of, inside, outside</p>

	<!-- mixed: real v-for followed by normal attr 'of' -->
	<div v-for='(v, k, idx) of obj' of="not-a-directive">{{ k }}: {{ v }} ({{ idx }})</div>
</template>
