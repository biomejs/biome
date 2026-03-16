<!-- should not generate diagnostics -->
<template>
  <v-list-item
    v-for="(rate, index) in playbackRate.availablePlaybackRates.value"
    :key="index"
    :value="rate"
    @click="setRate(rate)"
  >
    <v-list-item-title>{{ formatRate(rate) }}</v-list-item-title>
  </v-list-item>
</template>
