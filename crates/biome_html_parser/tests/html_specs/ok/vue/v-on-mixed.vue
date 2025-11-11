<template>
	<!-- Long form -->
	<button v-on:click="onClick">Click</button>
	<form v-on:submit.prevent.stop="onSubmit"></form>
	<div v-on:mouseenter.once="onMouseEnterOnce"></div>
	<input v-on:keyup.enter.exact="onEnter" />

	<!-- Shorthand -->
	<button @click="onClickShorthand">Click</button>
	<a @click.prevent="onLinkClick" href="#link">Link</a>
	<input @keydown.esc.stop="onEsc" />

	<!-- Dynamic arguments (long form and shorthand) -->
	<div v-on:[evt]="onDynamicEvent"></div>
	<div @[evt]="onDynamicEventShort"></div>

	<!-- Dynamic args with modifiers -->
	<div v-on:[evt].passive.once="onPassiveOnce"></div>
	<div @[evt].capture.self="onCapturedSelf"></div>

	<!-- More mixed examples -->
	<input @keyup.left="onLeftKey" />
	<button v-on:contextmenu.prevent="onContextMenu">Context</button>
	<component v-on:focus.capture="onFocusCapture"></component>
</template>