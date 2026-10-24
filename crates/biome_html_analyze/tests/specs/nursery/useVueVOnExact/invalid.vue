<!-- should generate diagnostics -->
<template>
  <button
    @click="foo"
    @click.ctrl="bar"
  ></button>
  <button
    v-on:click="foo"
    v-on:click.ctrl="bar"
  ></button>
  <button
    @click="foo"
    @click.ctrl.stop="bar"
  ></button>
  <button
    @click.prevent="foo"
    @click.ctrl="bar"
  ></button>
  <input
    @keydown.prevent="foo"
    @keydown.ctrl="bar"
  />
  <input
    @keydown.ctrl.enter.exact="save"
    @keydown.stop="foo"
  />
  <button
    @click.exact="foo"
    @click.ctrl="bar"
    @click.ctrl.shift="baz"
  ></button>
  <button
    @click="foo"
    @click.ctrl="bar"
    @click.ctrl.shift="baz"
  ></button>
  <button
    @click.alt.shift="foo"
    @click.alt.ctrl.shift="bar"
  ></button>
  <input
    @keypress.27="foo"
    @keypress.27.shift="bar"
  >
  <input
    @keypress.exact="foo"
    @keypress.esc="bar"
    @keypress.ctrl="baz"
  >
  <button
    @[foo]="foo"
    @[foo].ctrl="bar"
  ></button>
  <svg @click="foo" @click.meta="bar"></svg>
  <input
    @keydown.enter="foo"
    @keydown.enter.shift.exact="bar"
  />
  <button is="fancy-button" @click="foo" @click.ctrl="bar"></button>
  <button :is="kind" @click="foo" @click.ctrl="bar"></button>
  <div @click.right="foo" @contextmenu.ctrl="bar"></div>
  <div @mousedown="foo" @mousedown.ctrl.right="bar"></div>
</template>
