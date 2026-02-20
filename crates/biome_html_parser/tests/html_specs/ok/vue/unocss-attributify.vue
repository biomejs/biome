<template>
	<!--
		The `hover:bg` attribute is syntax for UnoCSS's Attributify preset: https://unocss.dev/presets/attributify
		We need to make sure that the `hover:bg` attribute is parsed as a single attribute, and not split into two attributes `hover` and `:bg`.
	-->
  <button
    type = "button"
    hover:bg="gray-400 opacity-20"
  >
    +
  </button>
</template>
