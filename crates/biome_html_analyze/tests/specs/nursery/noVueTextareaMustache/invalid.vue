<!-- should generate diagnostics -->
<template>
  <div><textarea>{{ text }}</textarea></div>
  <textarea v-model="text">{{ text }}</textarea>
  <textarea>{{ first }} and {{ second }}</textarea>
  <textarea>
    Hello, {{ name }}!
  </textarea>
  <textarea>{{ unclosed }} and {{ never closed</textarea>
</template>
