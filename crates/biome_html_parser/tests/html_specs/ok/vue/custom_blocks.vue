<template>
	<div class="a">markup</div>
</template>

<template lang="pug">
	.test
		#foo
</template>

<docs>
This block is prose, and it may mention a `<my-component>` without closing it.
</docs>

<i18n>
{ "en": { "hello": "Hello" } }
</i18n>

<custom lang="javascript">
const foo = "</"
</custom>

<div>
	This is custom, not a div.
</div>
