<!-- should not generate diagnostics -->
<template>
  <button @click="foo"></button>
  <button @click="foo" :click="foo"></button>
  <button @click.ctrl="foo"></button>
  <button @click.exact="foo"></button>
  <button v-on:click="foo"></button>
  <button v-on:click.ctrl="foo"></button>
  <button v-on:click.exact="foo"></button>
  <button @click="foo" @click.stop="bar"></button>
  <button @click="foo" @click.prevent="bar" @click.stop="baz"></button>
  <button @click.prevent="foo" @click.stop="bar"></button>
  <button @click.exact="foo" @click.ctrl="bar"></button>
  <button @click.exact="foo" @click.ctrl.exact="bar" @click.ctrl.shift="baz"></button>
  <button @click.ctrl.exact="foo" @click.ctrl.shift="bar"></button>
  <button @click="foo" @focus="foo"></button>
  <button @click="foo" @click="foo"></button>
  <button @click="foo"></button><button @click.ctrl="foo"></button>
  <button v-on:click.exact="foo" v-on:click.ctrl="foo"></button>
  <a @mouseenter="showTooltip" @mouseenter.once="attachTooltip"></a>
  <input @keypress.exact="foo" @keypress.esc.exact="bar" @keypress.ctrl="baz">
  <input @keypress.a="foo" @keypress.b="bar" @keypress.a.b="baz">
  <input @keypress.shift="foo" @keypress.ctrl="bar">
  <input
    @keypress.27="foo"
    @keypress.27.middle="bar"
  >
  <input
    @keydown.a.b.c="abc"
    @keydown.a="a"
    @keydown.b="b"
    @keydown.c="c"
  >
  <input
    @keydown.a.b="ab"
    @keydown.a="a"
    @keydown.b="b"
    @keydown.c="c"
    @keydown.a.c="ac"
  >
  <input
    @keydown.enter="foo"
    @keydown.shift.tab="bar"
  />
  <input
    @keydown.enter="foo"
    @keydown.shift.tab.prevent="bar"
  />
  <input
    @keydown.ctrl.enter.exact="save"
    @keydown.stop
  />
  <button @[click]="foo"></button>
  <button @[foo]="foo" @[bar].ctrl="bar"></button>
  <button @[click]="foo" @click.ctrl="bar"></button>
  <button v-on="listeners" @click.ctrl="bar"></button>
  <UiButton @click="foo" @click.ctrl="bar" />
  <ui-button @click="foo" @click.ctrl="bar"></ui-button>
  <Foo.Bar @click="foo" @click.ctrl="bar" />
  <component :is="view" @click="foo" @click.ctrl="bar" />
  <card @click="foo" @click.ctrl="bar"></card>
  <div is="vue:my-component" @click="foo" @click.ctrl="bar"></div>
  <input @keydown.enter.exact="foo" @keydown.enter.shift="bar" />
  <div @click="foo" @click.ctrl.right="bar"></div>
  <div @click.right="foo" @click.ctrl="bar"></div>
  <div @click="foo" @click.ctrl.middle="bar"></div>
</template>
