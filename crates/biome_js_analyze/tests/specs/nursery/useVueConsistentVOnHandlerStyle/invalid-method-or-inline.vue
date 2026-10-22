<!-- should generate diagnostics -->
<template>
  <button @click="() => foo()" />
  <button @click="(a, b) => foo(a, b)" />
  <button @click="a => foo(a)" />
  <button @click="() => { foo() }" />
  <button @click="() => foo() /* comment */" />
  <button @click="() => handler(foo)" />
  <button @click="() => foo?.()" />
  <button @click="() => count++" />
  <button @click="() => { count++ }" />
  <button @click="(a, b) => foo(b, a)" />
  <button @click="(a, b) => foo(...a, b)" />
  <button @click="(...a) => foo(a)" />
  <button @click="foo()" />
  <template v-for="e in list">
    <button @click="() => e()" />
    <button @click="(a) => e(a)" />
    <button @click="() => handler(e)" />
    <button @click="() => handlers[e]()" />
    <button @click="() => e.foo()" />
    <button @click="() => { handler(e); done(); }" />
  </template>
</template>
