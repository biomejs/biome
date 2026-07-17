<div v-for='item in items'>{{ item }}</div>
<div v-for='item of items'>{{ item }}</div>
<div v-for='(item, index) of items'>{{ index }}: {{ item }}</div>
