<template>
	<!-- Complex mixed directives on one element -->
	<complex-widget
		v-if="isReady && items.length > 0"
		v-for="(item, index) in items"
		:key="item.id"
		v-show="showWidget"
		v-model:[activeProp].trim.number="current"
		v-bind:title="title"
		:data-id="item.id"
		v-bind:[dynamicAttr].camel="dynamicValue"
		:class="[{ active: isActive }, cls]"
		:style="{ color: color, '--x': x }"
		v-on:click="onClick(item, index)"
		@keydown.enter.exact.prevent.stop="onEnter"
		v-on:[eventName].once.passive="onDynamic"
		@[rawEvt].capture.self="onRaw"
		v-on:focus.capture="onFocus"
	>
		<template #default="{ slotProp }">
			<span>{{ slotProp }}</span>
		</template>
		<template #footer>
			<button @click="onFooterClick">OK</button>
		</template>
	</complex-widget>
</template>