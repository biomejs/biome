<script setup>
import { computed } from 'vue'

let counter = 0
let flag = false

const assigned = computed(() => {
  counter = 1
  return counter
})
const updated = computed(() => {
  counter++
  return counter
})
const toggled = computed(() => {
  flag = !flag
  return flag
})
const local = computed(() => {
  // Declared in the getter, so this write is the getter's own.
  let own = 0
  own++
  return own
})
</script>
