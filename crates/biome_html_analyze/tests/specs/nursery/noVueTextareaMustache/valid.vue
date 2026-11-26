<!-- should not generate diagnostics -->
<template>
  <textarea v-model="text"></textarea>
  <textarea :value="text"></textarea>
  <textarea>Plain text</textarea>
  <textarea>{{ never closed</textarea>
  <textarea></textarea>
  <textarea />
  <Textarea>{{ text }}</Textarea>
  <TEXTAREA>{{ text }}</TEXTAREA>
  <textarea v-pre>{{ text }}</textarea>
  <div v-pre>
    <textarea>{{ text }}</textarea>
  </div>
  <div>{{ text }}</div>
</template>
