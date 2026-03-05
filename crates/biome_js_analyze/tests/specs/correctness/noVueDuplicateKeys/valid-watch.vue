<template>
  <div>This is valid because this is how you declare watchers in Vue. The child keys of `watch` define which properties/variables to watch, and they are not declarations.</div>
</template>

<script>
	export default {
		name: 'QFormContainer',
		props: {
			formData: {
				type: Object,
				default: () => ({}),
				validator: (val) => true
			},
		},
		watch: {
			formData: {
				handler(newValue)
				{
					//do things
				},
				deep: true
			}
		}
	}
</script>
