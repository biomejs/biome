<div v-for="{ id, tags: [primary, secondary } ] in members">{{ id }}</div>
<div v-for="[foo, profile: { username }] in members">{{ foo }}</div>
<div v-for="([first, { id, profile: { username } }], index in groupedMembers">{{ index }}</div>
