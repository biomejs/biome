<!-- should not generate diagnostics -->
<script setup>
const supported = ref(true);
const enabled = ref(false);
const count = ref(0);
const isActive = ref(false);

function toggleCaptions() {
	enabled.value = !enabled.value;
}

function handleClick() {
	count.value++;
}
</script>

<template>
  <!-- v-on shorthand with function call -->
  <button @click="toggleCaptions()">Toggle</button>
  
  <!-- v-on shorthand with function reference -->
  <button @click="handleClick">Click</button>
  
  <!-- v-bind shorthand with expression -->
  <button :disabled="!supported">Disabled</button>
  
  <!-- v-if directive -->
  <div v-if="count > 0">Count: {{ count }}</div>
  
  <!-- v-show directive -->
  <div v-show="isActive">Active</div>
</template>
