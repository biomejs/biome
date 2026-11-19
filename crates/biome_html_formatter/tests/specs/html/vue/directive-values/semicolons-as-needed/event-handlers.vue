<template>
  <div @click="[a].forEach(f)"></div>
  <div @loaded="(map = $event) && initMap()"></div>
  <div @click="count++;"></div>
  <div @click="handler;"></div>
  <div @click="a();[b].forEach(f)"></div>
  <div @click="const x = 1; f(x)"></div>
  <div @click="if (ok) save()"></div>
</template>
