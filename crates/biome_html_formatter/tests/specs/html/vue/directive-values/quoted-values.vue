<template>
	<div :bar="foo   + 1"></div>
	<div v-bind:title="a+b"></div>
	<div :bar='foo'></div>
	<div :bar="foo( 'x' )"></div>
	<div v-if="a&&b"></div>
	<div :class="{ active:isActive }"></div>
	<button @click="  item.select  "></button>
	<button @click="( ) =>  select( item )"></button>
	<Comp #default="{ item,index }"></Comp>
	<div :style="{ color: activeColor, fontSize: fontSize + 'px', backgroundColor: someVeryLongVariableName }"></div>
	<div v-if="someVeryLongConditionName && anotherVeryLongConditionName && yetAnotherCondition"></div>
	<div :title="isActive ? someVeryLongTranslationKeyName : anotherVeryLongTranslationKeyName"></div>
	<p>{{ "text"   + name }}</p>
	<p>{{ someVeryLongConditionName && anotherVeryLongConditionName && yetAnotherCondition }}</p>
</template>
