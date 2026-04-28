<div v-for="(item) in items">{{ item }}</div>
<div v-for="(item, index) in items">{{ index }}: {{ item }}</div>
<div v-for="(value, key) in object">{{ key }}: {{ value }}</div>
<div v-for="(value, key, index) in object">{{ index }}: {{ key }}: {{ value }}</div>
