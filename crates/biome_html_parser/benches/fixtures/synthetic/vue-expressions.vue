<template>
  <div class="expression-heavy">
    <h1>{{ pageTitle || 'Default Title' }} - {{ pageSubtitle }}</h1>
    <p>Welcome {{ user ? user.name : 'Guest' }}! You have {{ notifications.length }} new {{ notifications.length === 1 ? 'notification' : 'notifications' }}.</p>
    
    <div class="stats">
      <span>Total: {{ items.reduce((sum, item) => sum + item.value, 0) }}</span>
      <span>Average: {{ items.length ? items.reduce((sum, item) => sum + item.value, 0) / items.length : 0 }}</span>
      <span>Max: {{ Math.max(...items.map(i => i.value)) }}</span>
      <span>Min: {{ Math.min(...items.map(i => i.value)) }}</span>
    </div>
    
    <ul>
      <li v-for="item in items" :key="item.id">
        {{ item.name }} - {{ item.description || 'No description' }} ({{ item.status === 'active' ? 'Active' : item.status === 'pending' ? 'Pending' : 'Inactive' }})
        <span v-if="item.tags && item.tags.length">Tags: {{ item.tags.join(', ') }}</span>
        <span v-else>No tags</span>
        Price: ${{ item.price ? item.price.toFixed(2) : '0.00' }}
        Discount: {{ item.discount ? (item.price * (1 - item.discount)).toFixed(2) : item.price ? item.price.toFixed(2) : '0.00' }}
      </li>
    </ul>
    
    <div class="computed">
      <p>Full Name: {{ user ? `${user.firstName} ${user.lastName}` : 'Anonymous' }}</p>
      <p>Formatted Date: {{ new Date(timestamp).toLocaleDateString() }}</p>
      <p>Uppercase: {{ message ? message.toUpperCase() : '' }}</p>
      <p>Reversed: {{ message ? message.split('').reverse().join('') : '' }}</p>
      <p>Filtered: {{ items.filter(i => i.active).map(i => i.name).join(', ') }}</p>
    </div>
    
    <form>
      <label>Name: {{ formData.name || 'Empty' }}</label>
      <label>Email: {{ formData.email ? formData.email.toLowerCase() : 'Not provided' }}</label>
      <label>Age: {{ formData.birthDate ? new Date().getFullYear() - new Date(formData.birthDate).getFullYear() : 'Unknown' }}</label>
      <label>Country: {{ formData.country || 'Not selected' }} ({{ formData.country ? getCountryCode(formData.country) : 'N/A' }})</label>
    </form>
    
    <table>
      <tr v-for="row in tableData" :key="row.id">
        <td>{{ row.index + 1 }}</td>
        <td>{{ row.name }}</td>
        <td>{{ row.values.reduce((a, b) => a + b, 0) }}</td>
        <td>{{ row.values.length ? (row.values.reduce((a, b) => a + b, 0) / row.values.length).toFixed(2) : '0.00' }}</td>
        <td>{{ row.enabled ? 'Yes' : 'No' }}</td>
        <td>{{ row.metadata ? JSON.stringify(row.metadata) : '{}' }}</td>
      </tr>
    </table>
    
    <div class="nested">
      <p>Level 1: {{ data.level1 ? data.level1.value : 'N/A' }}</p>
      <p>Level 2: {{ data.level1 && data.level1.level2 ? data.level1.level2.value : 'N/A' }}</p>
      <p>Level 3: {{ data.level1 && data.level1.level2 && data.level1.level2.level3 ? data.level1.level2.level3.value : 'N/A' }}</p>
      <p>Array access: {{ arr && arr[0] ? arr[0].name : 'None' }}</p>
      <p>Method call: {{ getValue() || 'Default' }}</p>
      <p>Computed prop: {{ computedValue }}</p>
    </div>
    
    <footer>
      <span>{{ footer.left }}</span>
      <span>{{ footer.center }}</span>
      <span>{{ footer.right }}</span>
      <span>Rendered at: {{ new Date().toISOString() }}</span>
    </footer>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const pageTitle = ref('Dashboard')
const pageSubtitle = ref('Overview')
const user = ref({ name: 'John', firstName: 'John', lastName: 'Doe' })
const notifications = ref([1, 2, 3])
const items = ref([
  { id: 1, name: 'Item 1', value: 10, status: 'active', tags: ['a', 'b'], price: 99.99, discount: 0.1, active: true },
  { id: 2, name: 'Item 2', value: 20, status: 'pending', tags: [], price: 49.99, active: false },
  { id: 3, name: 'Item 3', value: 30, status: 'inactive', price: 29.99, active: true }
])
const timestamp = ref(Date.now())
const message = ref('Hello World')
const formData = ref({ name: 'Test', email: 'TEST@EXAMPLE.COM', birthDate: '1990-01-01', country: 'USA' })
const tableData = ref([
  { id: 1, index: 0, name: 'Row 1', values: [1, 2, 3], enabled: true, metadata: { key: 'value' } },
  { id: 2, index: 1, name: 'Row 2', values: [4, 5, 6], enabled: false }
])
const data = ref({ level1: { value: 1, level2: { value: 2, level3: { value: 3 } } } })
const arr = ref([{ name: 'First' }])
const footer = ref({ left: 'Left', center: 'Center', right: 'Right' })

const getCountryCode = (country) => country.slice(0, 2).toUpperCase()
const getValue = () => 'computed'
const computedValue = computed(() => 'value')
</script>

<style scoped>
.expression-heavy { padding: 20px; }
.stats span { margin-right: 20px; }
</style>
