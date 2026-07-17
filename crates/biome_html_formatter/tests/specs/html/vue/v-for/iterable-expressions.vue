<div v-for="item in items">{{ item }}</div>
<div v-for="item       in visibleItems">{{ item }}</div>
<div v-for="item in    fallbackItems">{{ item }}</div>
<div v-for="item in list.map((value) => value.id)">{{ item }}</div>
