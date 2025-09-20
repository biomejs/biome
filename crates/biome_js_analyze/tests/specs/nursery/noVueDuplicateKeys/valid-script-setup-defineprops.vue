<script setup>
defineProps({
  foo: String,
  bar: Number
})

const baz = ref('hello')
const qux = computed(() => 'world')

function handleClick() {
  console.log('clicked')
}
</script>