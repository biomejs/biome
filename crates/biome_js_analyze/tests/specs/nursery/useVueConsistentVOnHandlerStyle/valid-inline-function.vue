<!-- should not generate diagnostics -->
<template>
  <button @click="() => foo()" />
  <button @click="() => count++" />
  <button @click="function () { foo() }" />
  <button @click="form.submit" />
</template>
