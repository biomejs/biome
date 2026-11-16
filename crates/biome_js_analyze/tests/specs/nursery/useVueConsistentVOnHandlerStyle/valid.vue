<!-- should not generate diagnostics -->
<script setup>
import Item from "./Item.vue";
</script>
<template>
  <button @click="foo" />
  <button v-on:click="foo" />
  <button @click="form.submit" />
  <button @click="handlers[name]" />
  <button :click="foo()" />
  <button @[event]="foo" />
  <button v-on="{ click: foo }" />
  <template v-for="e in list">
    <button @click="e" />
    <button @click="() => foo(e)" />
    <button @click="(a) => foo(a, e)" />
  </template>
  <Item v-slot="{ item }">
    <button @click="() => remove(item)" />
  </Item>
</template>
