<script setup>
const message = ref('Hello Vue 3!')
const count = ref(0)

const doubleCount = computed(() => count.value * 2)

function increment() {
  count.value++
}
</script>