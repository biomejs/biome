<!-- should not generate diagnostics -->
<template>
  <div :id="'comment'/*comment*/" />
  <div :id="'comment'//comment
  " />
</template>
