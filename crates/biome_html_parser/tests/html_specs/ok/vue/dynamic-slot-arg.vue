<template
  v-for="episode in episodes"
  #[`${episode.id}-cell`]="{ row }"
  :key="episode.id"
>
  ...
</template>

<template
  v-for="episode in episodes"
  v-slot:[`${episode.id}-cell`]="{ row }"
  :key="episode.id"
>
  ...
</template>

<template
  v-for="episode in episodes"
  #[episode.id]="{ row }"
  :key="episode.id"
>
  ...
</template>
