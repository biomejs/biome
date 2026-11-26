<!-- should generate diagnostics -->
<template>
  <input @focusin="onFocus" />
  <input v-on:focusin="onFocus" />
  <input @focusin.once="onFocus" />
  <form @submit="save"></form>
  <div v-on="handlers"></div>
  <a @click.prevent="onClick"></a>
  <a v-on:click.stop.prevent="onClick"></a>
  <MyButton @click="onClick" />
  <MyButton v-on:click="onClick"></MyButton>
  <input @keyup.ctrl.enter="onKeyup" />
  <Foo.Bar @select="onSelect" />
  <MyInput @update:modelValue="onUpdate" />
  <input @focusin />
  <input @focusin = "onFocus" />
</template>
