<!-- should not generate diagnostics -->
<template>
  <input @focus="onFocus" />
  <input @focusIn="onFocus" />
  <input @[eventName]="onFocus" />
  <input v-on:[eventName]="onFocus" />
  <input focusin="onFocus" />
  <input :focusin="onFocus" />
  <input v-bind:focusin="onFocus" />
  <a @click="onClick"></a>
  <a @click.stop="onClick"></a>
  <button @click="onClick"></button>
  <MyButtonGroup @click="onClick" />
  <mybutton @click="onClick"></mybutton>
  <input @keyup.enter="onKeyup" />
  <textarea @keyup.ctrl.enter="onKeyup"></textarea>
  <Foo.Baz @select="onSelect" />
  <MyInput @update:model-value="onUpdate" />
  <div v-model="value"></div>
</template>
