<!-- should not generate diagnostics -->
<template>
<div class="bg-(--brand)"></div>
<div :class="['w-(--width)', { '[color:var(--ink)]': active }]"></div>
</template>
