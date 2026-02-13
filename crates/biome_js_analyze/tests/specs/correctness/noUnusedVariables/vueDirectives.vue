<script setup>
const supported = ref(true);
const enabled = ref(false);
const count = ref(0);
const items = ref([1, 2, 3]);
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
  
  <!-- v-for directive -->
  <li v-for="item in items" :key="item">{{ item }}</li>
</template>
