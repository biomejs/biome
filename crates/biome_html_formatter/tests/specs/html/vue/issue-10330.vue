<template>
<div>
<span>
<div>
<v-btn v-if="store.state.user" variant="text" to="/my-rooms">{{
	$t("nav.my-rooms")
}}</v-btn>
</div>
</span>
</div>
</template>

<template>
	<v-btn v-if="store.state.user" variant="text" to="/my-rooms">{{ $t("nav.my-rooms") }}</v-btn>
</template>

<template>
	<v-btn>{{
		value
	}}</v-btn>
</template>

<template>
	<span>{{ value }}<em aria-label="a very long label that forces the adjacent sibling to wrap">suffix</em></span>
</template>

<template>
	<span>{{ value }}<img alt="a very long label that forces the adjacent sibling to wrap" /></span>
</template>
