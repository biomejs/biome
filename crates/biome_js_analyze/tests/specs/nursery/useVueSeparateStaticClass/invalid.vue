<!-- should generate diagnostics -->
<template>
  <div v-bind:class="'static-class'" />
  <div :class="'static-class'" />
  <div :class="`static-class`" />
  <div :class="`form-item input ${isDisabled ? 'disabled' : ''}`" />
  <div :class="`${dynamicClass} static-class`" />
  <div :class="`${start} static-class ${end}`" />
  <div :class='"static-class"' />
  <div :class="('static-class')" />
  <div :class="['static-class']" />
  <div :class="{'static-class': true}" />
  <div :class="{foo: true}" />
  <div :class="{default: true}" />
  <div :class="{['static-class']: true}" />
  <div :class="{[`static-class`]: true}" />
  <div :class="['static-class', dynamicClass]" />
  <div :class="[dynamicClass, otherDynamicClass, 'static-class']" />
  <div :class="{'static-class': true, 'dynamic-class': foo}" />
  <div :class="[{'dynamic-class': foo, 'static-class': true}]" />
  <div :class="[dynamicClass, {staticClass: true}]" />
  <div :class="[['nested-static-class']]" />
  <div :class="['static-a static-b']" />
  <div :class="`a ${foo}-b c`" />
  <div
    class="other-class"
    other-attribute
    :class="[
      'static-class-a',
      {'static-class-b': true, 'dynamic-class-a': foo},
      dynamicClassB,
    ]" />
</template>
