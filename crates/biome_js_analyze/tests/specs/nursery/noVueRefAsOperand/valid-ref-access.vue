/* should not generate diagnostics */

<script>
import { ref } from 'vue'

const count = ref(0)

if (count.value) {}
switch (count.value) {}

var foo = -count.value
var foo = +count.value

count.value++
count.value--

count.value + 1
1 - count.value

count.value || other
count.value && other

var foo = count.value ? x : y
</script>
