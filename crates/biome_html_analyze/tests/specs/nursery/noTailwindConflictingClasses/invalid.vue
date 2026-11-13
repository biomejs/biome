<!-- should generate diagnostics -->
<template>
	<div class="flex grid"></div>
	<div class="hover:flex hover:grid"></div>
	<div :class="'p-2 p-4'"></div>
	<div :class="{ 'flex grid': isActive }"></div>
</template>
