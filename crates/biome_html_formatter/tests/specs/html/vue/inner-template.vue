<template>
	<UModal>
		<!--
		because the children of this inner template already has a newline,
		the multiline formatting should be preserved
		-->
		<template #body>
			<p>{{ store.options.text }}</p>
		</template>
	</UModal>
</template>

<template>
	<UModal>
		<!--
		because the children of this inner template does not have a newline,
		the children should NOT be multiline formatted
		-->
		<template #body><p>foo</p></template>
	</UModal>
</template>
