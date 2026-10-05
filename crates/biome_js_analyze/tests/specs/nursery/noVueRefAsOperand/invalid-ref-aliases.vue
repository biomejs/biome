<!-- should generate diagnostics -->
<script lang="ts">
import { type Ref, ref, toRefs, useTemplateRef } from 'vue'

const count = ref(0)
const countAlias = count
const nestedAlias = countAlias
const parenthesizedAlias = (count)
countAlias++
if (nestedAlias) {}
parenthesizedAlias++
if ((countAlias)) {}

const assertedProducer = ref(0) as Ref<number>
assertedProducer++
const satisfiesProducer = ref(0) satisfies Ref<number>
satisfiesProducer++
const typeAssertedProducer = <Ref<number>>ref(0)
typeAssertedProducer++

const assertedAlias = count as Ref<number>
assertedAlias++
const nonNullAlias = count!
nonNullAlias++
if (count as unknown) {}

const templateRef = useTemplateRef('element')
if (templateRef) {}

const [defaultedModel = null, modelModifiers] = defineModel()
if (defaultedModel) {}

const refs = toRefs({ value: 0 })
refs.value++
refs['value']++
((refs)).value++
`${refs.value}`
const valueRef = refs.value
valueRef++
const parenthesizedValueRef = (refs).value
parenthesizedValueRef++

const refsAlias = refs
const { value: destructuredRef } = refsAlias
destructuredRef++

const { ...restRefs } = refs
restRefs.value++
</script>
