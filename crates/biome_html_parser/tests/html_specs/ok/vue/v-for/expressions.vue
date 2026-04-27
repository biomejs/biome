<div v-for="item in items.list">{{ item }}</div>
<div v-for="item in getItems()">{{ item }}</div>
<div v-for="item in items[index]">{{ item }}</div>
<div v-for="item in items.filter(item => item.visible)">{{ item }}</div>
<div v-for="item in condition ? visibleItems : hiddenItems">{{ item }}</div>
<div v-for="item in items ?? fallbackItems">{{ item }}</div>
