<!-- should not generate diagnostics -->
<template>
    <li>Root item</li>
    <ul><li v-for="item in items" :key="item">{{ item }}</li></ul>
    <ol><li /></ol>
    <menu><li>Action</li></menu>
    <Li />
    <ul><template v-for="item in items" :key="item"><li>{{ item }}</li></template></ul>
    <ul><template v-if="visible"><li>Conditional item</li></template></ul>
    <ul><transition><li v-if="visible">Transition item</li></transition></ul>
    <ul><keep-alive><li>Cached item</li></keep-alive></ul>
    <ul><slot><li>Slot fallback item</li></slot></ul>
    <transition-group tag="ul"><li v-for="item in items" :key="item">{{ item }}</li></transition-group>
    <component is="ul"><li>Dynamic element child</li></component>
    <teleport to="#list"><li>Teleported item</li></teleport>
    <List><li>Component child</li></List>
    <TransitionGroup tag="ul"><li>Component child</li></TransitionGroup>
    <div><template><li>Native template item</li></template></div>
    <div><template v-if="visible"><template><li>Native template item</li></template></template></div>
</template>
