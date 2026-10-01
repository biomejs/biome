<!-- should generate diagnostics -->
<template>
    <div><li /></div>
    <ul><div><li>Indirect child</li></div></ul>
    <div><template v-if="visible"><li>Conditional item</li></template></div>
    <div><template v-for="item in items" :key="item"><li>{{ item }}</li></template></div>
    <div><transition><li v-if="visible">Transition item</li></transition></div>
    <div><keep-alive><li>Cached item</li></keep-alive></div>
    <div><slot><li>Slot fallback item</li></slot></div>
    <div>
        <template v-if="a"><li>First</li></template>
        <template v-else-if="b"><li>Second</li></template>
        <template v-else><li>Third</li></template>
    </div>
</template>
