<!-- should generate diagnostics -->
<template>
  <!-- `ignore` matches whole event names only -->
  <VueComponent @customEventName="onEvent" />
  <!-- `ignoreTags` matches tag names exactly -->
  <IgnoredComponentTwo @otherEvent="onEvent" />
</template>
