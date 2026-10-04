<template>
	<div :class="{ a: b } // note
"></div>
	<Comp #default="{ item } // note
"></Comp>
	<div v-if="a && b // note
"></div>
	<Comp #default="{ someVeryLongPropertyName, anotherVeryLongPropertyName, yetAnotherOne }"></Comp>
	<div v-for="item in someVeryLongListOfItemsName || anotherVeryLongListOfItemsName || []">{{ item }}</div>
	<div v-if="someVeryLongConditionName | anotherVeryLongConditionName | yetAnotherCondition"></div>
</template>
