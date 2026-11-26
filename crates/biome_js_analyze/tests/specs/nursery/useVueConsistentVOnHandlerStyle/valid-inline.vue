<!-- should not generate diagnostics -->
<template>
  <button @click="foo()" />
  <button @click="foo($event)" />
  <button @click="count++" />
  <button @click="foo(); bar()" />
  <button @click="form.submit" />
</template>
