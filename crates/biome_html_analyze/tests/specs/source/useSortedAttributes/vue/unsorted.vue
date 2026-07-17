<p v-text="msg" v-show="ok" dir="auto" v-foo:bar.baz id="hello" class="flex"></p>

<div 
  v-slot:default
  v-on:click="doThis"
  v-once
  ref="my-ref"
  :key="item.id"
  v-bind:src="'/path/to/images/' + fileName"
  spellcheck="true"
  v-for="item in items"
  tabindex="-1"
  @scroll.passive="onScroll"
  :is="tabs[currentTab]"
  v-text="msg"
  dir="auto"
  v-model="countModel"
  v-if="awesome"
  :class="{ red: isRed }"
></div>

<!-- with v-bind="object" -->
<div 
  v-slot:default
  v-on:click="doThis"
  v-once
  ref="my-ref"
  v-bind="{ id: someProp, 'other-attr': otherProp }"
  :key="item.id"
  v-bind:src="'/path/to/images/' + fileName"
  spellcheck="true"
  v-for="item in items"
  tabindex="-1"
  @scroll.passive="onScroll"
  :is="tabs[currentTab]"
  v-text="msg"
  dir="auto"
  v-model="countModel"
  v-if="awesome"
  :class="{ red: isRed }"
></div>

<!-- attributes within the same category  -->
<div v-if="awesome" v-else v-else-if="type === 'B'" v-show="ok">
</div>

<div v-pre v-once></div>

<div ref="my-ref" :key="item.id"></div>

<div v-slot="{ x, y }" v-slot:default v-slot:footer #header #[dynamicSlotName]></div>

<div v-slot:footer v-slot="{ x, y }" v-slot:default #header #[dynamicSlotName]></div>

<div v-slot:default #[dynamicSlotName] v-slot:footer v-slot="{ x, y }" #header></div>

<div v-mycustomdirectiveb v-mycustomdirectivea></div>

<div v-bind:propc="foo" v-bind:propb="foo" :propa="foo"></div>

<div @scroll.passive="onScroll" @click.stop="doThis" v-on:click="doThis" @click.prevent="doThis"></div>

<div v-text="msg" v-html="html"></div>
