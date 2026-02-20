<template>
  <div class="dynamic-args">
    <input :[dynamicProp]="value" @[dynamicEvent]="handler" />
    
    <button :[typeProp]="buttonType" @[clickEvent].prevent="onClick" @[hoverEvent].stop="onHover">
      {{ buttonText }}
    </button>
    
    <div :[classProp]="dynamicClass" :[styleProp]="dynamicStyle" @[focusEvent].capture="onFocus">
      Dynamic content
    </div>
    
    <ul>
      <li v-for="item in items" :key="item.id"
          :[item.attr]="item.value"
          @[item.event].prevent.stop="item.handler"
          @[item.event2].once="item.handler2">
        {{ item.label }}
      </li>
    </ul>
    
    <form @[submitEvent].prevent="onSubmit">
      <input v-for="field in fields" :key="field.name"
             :[field.prop]="field.value"
             :[field.model]="field.data"
             @[field.input]="field.onInput"
             @[field.blur]="field.onBlur"
             @[field.focus].prevent="field.onFocus" />
      
      <select :[selectProp]="selectValue" @[changeEvent]="onChange">
        <option v-for="opt in options" :key="opt.value"
                :[opt.prop]="opt.value"
                :[opt.selected]="opt.isSelected">
          {{ opt.label }}
        </option>
      </select>
    </form>
    
    <table @[clickEvent].stop="onTableClick">
      <tr v-for="row in rows" :key="row.id"
          :[row.attr]="row.value"
          @[row.event].prevent="row.handler"
          @[row.event2].capture.once="row.handler2">
        <td v-for="cell in row.cells" :key="cell.id"
            :[cell.prop]="cell.value"
            @[cell.event].stop.prevent="cell.handler">
          {{ cell.content }}
        </td>
      </tr>
    </table>
    
    <nav @[navEvent].prevent="onNav">
      <a v-for="link in links" :key="link.id"
         :[link.href]="link.url"
         :[link.target]="link.targetVal"
         @[link.click].prevent.stop="link.onClick"
         @[link.mouseenter].once="link.onEnter">
        {{ link.text }}
      </a>
    </nav>
    
    <modal :[modalProp]="modalValue"
           @[modalShow]="onShow"
           @[modalHide].once="onHide"
           @[modalClose].prevent.stop="onClose">
      <header :[headerProp]="headerValue" @[headerEvent]="onHeaderEvent">
        <h2>{{ modalTitle }}</h2>
      </header>
      <section :[contentProp]="contentValue" @[contentEvent].capture="onContentEvent">
        <p>{{ modalContent }}</p>
      </section>
      <footer :[footerProp]="footerValue" @[footerEvent].stop="onFooterEvent">
        <button :[actionProp]="actionValue" @[actionEvent].prevent="onAction">Close</button>
      </footer>
    </modal>
    
    <card v-for="card in cards" :key="card.id"
          :[card.titleProp]="card.title"
          :[card.imageProp]="card.image"
          :[card.classProp]="card.class"
          @[card.click].prevent.stop.once="card.onClick"
          @[card.hover].capture="card.onHover">
      <template #[card.slotName]>
        {{ card.slotContent }}
      </template>
    </card>
    
    <tabs :[activeTabProp]="activeTab" @[tabChange]="onTabChange">
      <tab v-for="tab in tabs" :key="tab.id"
           :[tab.nameProp]="tab.name"
           :[tab.disabledProp]="tab.disabled"
           @[tab.select].prevent="tab.onSelect"
           @[tab.deselect].once="tab.onDeselect">
        {{ tab.content }}
      </tab>
    </tabs>
    
    <dropdown :[visibleProp]="dropdownVisible"
              @[toggleEvent].prevent="toggleDropdown"
              @[selectEvent].stop="onSelect"
              @[outsideEvent].capture.once="onOutsideClick">
      <item v-for="item in dropdownItems" :key="item.id"
            :[item.valueProp]="item.value"
            :[item.disabledProp]="item.disabled"
            @[item.click].prevent.stop="item.onClick"
            @[item.hover]="item.onHover">
        {{ item.label }}
      </item>
    </dropdown>
    
    <tooltip :[tooltipProp]="tooltipContent"
             :[positionProp]="tooltipPosition"
             @[showEvent].once="onTooltipShow"
             @[hideEvent].prevent="onTooltipHide">
      <span @[triggerEvent]="onTrigger">Hover me</span>
    </tooltip>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const dynamicProp = ref('value')
const dynamicEvent = ref('input')
const value = ref('test')
const handler = () => {}

const typeProp = ref('type')
const buttonType = ref('button')
const clickEvent = ref('click')
const hoverEvent = ref('mouseenter')
const buttonText = ref('Click me')
const onClick = () => {}
const onHover = () => {}

const classProp = ref('class')
const styleProp = ref('style')
const dynamicClass = ref('dynamic')
const dynamicStyle = ref({ color: 'red' })
const focusEvent = ref('focus')
const onFocus = () => {}

const items = ref([
  { id: 1, attr: 'data-foo', value: 'bar', event: 'click', event2: 'dblclick', handler: () => {}, handler2: () => {}, label: 'Item 1' },
  { id: 2, attr: 'data-baz', value: 'qux', event: 'mouseover', event2: 'mouseout', handler: () => {}, handler2: () => {}, label: 'Item 2' }
])

const submitEvent = ref('submit')
const onSubmit = () => {}

const fields = ref([
  { name: 'email', prop: 'type', value: 'email', model: 'v-model', data: '', input: 'input', onInput: () => {}, blur: 'blur', onBlur: () => {}, focus: 'focus', onFocus: () => {} },
  { name: 'password', prop: 'type', value: 'password', model: 'v-model', data: '', input: 'input', onInput: () => {}, blur: 'blur', onBlur: () => {}, focus: 'focus', onFocus: () => {} }
])

const selectProp = ref('value')
const selectValue = ref('')
const changeEvent = ref('change')
const onChange = () => {}

const options = ref([
  { value: 'a', prop: 'value', selected: 'selected', isSelected: false, label: 'Option A' },
  { value: 'b', prop: 'value', selected: 'selected', isSelected: true, label: 'Option B' }
])

const clickEvent = ref('click')
const onTableClick = () => {}

const rows = ref([
  { id: 1, attr: 'data-row', value: '1', event: 'click', handler: () => {}, event2: 'dblclick', handler2: () => {}, cells: [
    { id: 'c1', prop: 'data-cell', value: '1-1', event: 'click', handler: () => {}, content: 'Cell 1' },
    { id: 'c2', prop: 'data-cell', value: '1-2', event: 'click', handler: () => {}, content: 'Cell 2' }
  ]}
])

const navEvent = ref('click')
const onNav = () => {}

const links = ref([
  { id: 1, href: 'href', url: '/', target: 'target', targetVal: '_self', click: 'click', onClick: () => {}, mouseenter: 'mouseenter', onEnter: () => {}, text: 'Home' },
  { id: 2, href: 'href', url: '/about', target: 'target', targetVal: '_blank', click: 'click', onClick: () => {}, mouseenter: 'mouseenter', onEnter: () => {}, text: 'About' }
])

const modalProp = ref('show')
const modalValue = ref(true)
const modalShow = ref('show')
const modalHide = ref('hide')
const modalClose = ref('close')
const onShow = () => {}
const onHide = () => {}
const onClose = () => {}
const modalTitle = ref('Modal Title')
const modalContent = ref('Modal content here')

const headerProp = ref('class')
const headerValue = ref('modal-header')
const headerEvent = ref('click')
const onHeaderEvent = () => {}

const contentProp = ref('class')
const contentValue = ref('modal-content')
const contentEvent = ref('scroll')
const onContentEvent = () => {}

const footerProp = ref('class')
const footerValue = ref('modal-footer')
const footerEvent = ref('click')
const onFooterEvent = () => {}

const actionProp = ref('type')
const actionValue = ref('button')
const actionEvent = ref('click')
const onAction = () => {}

const cards = ref([
  { id: 1, titleProp: 'title', title: 'Card 1', imageProp: 'image', image: '/img1.jpg', classProp: 'class', class: 'card', click: 'click', onClick: () => {}, hover: 'mouseenter', onHover: () => {}, slotName: 'default', slotContent: 'Card content' }
])

const activeTabProp = ref('value')
const activeTab = ref('tab1')
const tabChange = ref('change')
const onTabChange = () => {}

const tabs = ref([
  { id: 't1', nameProp: 'name', name: 'tab1', disabledProp: 'disabled', disabled: false, select: 'select', onSelect: () => {}, deselect: 'deselect', onDeselect: () => {}, content: 'Tab 1 content' }
])

const visibleProp = ref('visible')
const dropdownVisible = ref(false)
const toggleEvent = ref('toggle')
const selectEvent = ref('select')
const outsideEvent = ref('outside-click')
const toggleDropdown = () => {}
const onSelect = () => {}
const onOutsideClick = () => {}

const dropdownItems = ref([
  { id: 'i1', valueProp: 'value', value: 'v1', disabledProp: 'disabled', disabled: false, click: 'click', onClick: () => {}, hover: 'hover', onHover: () => {}, label: 'Item 1' }
])

const tooltipProp = ref('content')
const tooltipContent = ref('Tooltip text')
const positionProp = ref('position')
const tooltipPosition = ref('top')
const showEvent = ref('show')
const hideEvent = ref('hide')
const onTooltipShow = () => {}
const onTooltipHide = () => {}
const triggerEvent = ref('mouseenter')
const onTrigger = () => {}
</script>

<style scoped>
.dynamic-args { padding: 20px; }
</style>
