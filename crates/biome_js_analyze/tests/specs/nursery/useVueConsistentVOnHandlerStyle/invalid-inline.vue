<!-- should generate diagnostics -->
<template>
  <button @click="foo" />
  <button @click="() => foo()" />
  <button @click="(a) => foo(a)" />
  <button @click="(a, b) => foo(a, b)" />
  <button @click="() => count++" />
  <button @click="() => foo(a, b)" />
  <button @click="() => { foo() }" />
  <button @click="() => { count++; foo(); }" />
  <button @click="() => foo() /* comment */" />
  <button @click="() => foo" />
  <button @click="() => form.submit" />
  <button @click="() => (foo)" />
  <button @click="() => (() => go())" />
  <button @click="async () => foo()" />
  <button @click="function () { foo() }" />
</template>
