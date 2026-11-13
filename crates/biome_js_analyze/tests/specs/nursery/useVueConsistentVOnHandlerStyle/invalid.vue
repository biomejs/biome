<!-- should generate diagnostics -->
<template>
  <button @click="foo()" />
  <button v-on:click="foo()" />
  <button @click="() => foo()" />
  <button @click="count++" />
  <button @click="() => count++" />
  <button @click="(a, b) => foo(a, b)" />
  <button @click="foo( )" />
  <button @click=" beforeSpace()" />
  <button @click='afterSpace() ' />
  <button @click="fn();" />
  <button @click="fn();;" />
  <button @click="{ fn(); }" />
  <button @click="{(fn());;;}" />
  <button @click="foo(123)" />
  <button @click="foo.bar()" />
  <button @click="foo?.()" />
  <button @click="foo();foo();" />
  <button @click="{}" />
  <button @click="foo() /* comment */" />
  <button @click="function () { foo() }" />
  <button @click="function* () { foo() }" />
  <button @click="async () => foo()" />
  <template v-for="e in list">
    <button @click="e()" />
    <button @click="() => e()" />
    <button @click="handler(e)" />
    <button @click="handlers[e]()" />
    <button @click="e.foo()" />
    <button @click="handler($event, e)" />
    <button @click="(e) => handler(e, 1)" />
    <button @click="select(e), close()" />
  </template>
</template>
