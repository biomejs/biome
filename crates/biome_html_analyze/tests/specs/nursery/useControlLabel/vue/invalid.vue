<!-- should generate diagnostics -->
<template>
	<button></button>
	<menuitem />

	<!-- A dynamic value inside a hidden child does not name the control. -->
	<button><span aria-hidden="true">{{ label }}</span></button>
</template>
