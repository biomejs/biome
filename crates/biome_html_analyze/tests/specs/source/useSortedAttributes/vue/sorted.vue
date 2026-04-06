<p class="flex" dir="auto" id="hello" v-show="ok" v-foo:bar.baz v-text="msg"></p>

<div
  dir="auto"
  spellcheck="true"
  tabindex="-1"
  :is="tabs[currentTab]"
  v-for="item in items"
  v-if="awesome"
  v-once
  :key="item.id"
  ref="my-ref"
  v-slot:default
  v-model="countModel"
  :class="{ red: isRed }"
  v-bind:src="'/path/to/images/' + fileName"
  v-on:click="doThis"
  @scroll.passive="onScroll"
  v-text="msg"
></div>

<!-- with v-bind="object" -->
<div
  v-once
  ref="my-ref"
  v-slot:default
  v-on:click="doThis"

  v-bind="{ id: someProp, 'other-attr': otherProp }"
  
  dir="auto"
  spellcheck="true"
  tabindex="-1"
  :is="tabs[currentTab]"
  v-for="item in items"
  v-if="awesome"
  :key="item.id"
  v-model="countModel"
  :class="{ red: isRed }"
  v-bind:src="'/path/to/images/' + fileName"
  @scroll.passive="onScroll"
  v-text="msg"
></div>

<!-- sort alphabetically within the same category  -->
<div v-else v-else-if="type === 'B'" v-if="awesome" v-show="ok">
</div>

<div v-once v-pre></div>

<div :key="item.id" ref="my-ref"></div>

<div v-slot:default v-slot:footer #header v-slot="{ x, y }" #[dynamicSlotName]></div>

<div v-slot:default v-slot:footer #header #[dynamicSlotName] v-slot="{ x, y }"></div>

<div v-mycustomdirectivea v-mycustomdirectiveb ></div>

<div :propa="foo" v-bind:propb="foo" v-bind:propc="foo"></div>

<div @click.stop="doThis" v-on:click="doThis" @click.prevent="doThis" @scroll.passive="onScroll"></div>

<div v-html="html" v-text="msg"></div>
