<!-- should not generate diagnostics -->
<template>
  <MyComponent v-slot="{ data }">{{ data }}</MyComponent>
  <MyComponent v-slot:default="{ data }">{{ data }}</MyComponent>
  <MyComponent #default="{ data }">{{ data }}</MyComponent>
  <MyComponent v-slot="props" />
  <my-component v-slot="{ data }">{{ data }}</my-component>
  <foo v-slot="{ data }">{{ data }}</foo>
  <component :is="current" v-slot="{ data }">{{ data }}</component>
  <div is="vue:MyComponent" v-slot="{ data }">{{ data }}</div>
  <Ui.Card v-slot="{ data }">{{ data }}</Ui.Card>

  <MyComponent>
    <template v-slot>default</template>
  </MyComponent>
  <MyComponent>
    <template #default>default</template>
    <template #named>named</template>
  </MyComponent>
  <MyComponent>
    <template v-slot:named="{ data }">{{ data }}</template>
    <template #named-self-closing />
  </MyComponent>
  <MyComponent>
    <template #one>one</template>
    <template #two>two</template>
    <template #[one]>one</template>
    <template #[two]>two</template>
  </MyComponent>
  <MyComponent>
    <template v-if="c1" #one>foo</template>
    <template v-else-if="c2" #one>bar</template>
    <!-- a comment between branches -->
    <template v-else #one>baz</template>
  </MyComponent>
  <MyComponent>
    <template v-for="(key, value) in xxxx" #[key]>{{ value }}</template>
  </MyComponent>
  <MyComponent>
    <template v-for="(key, value) in xxxx" #[key]>{{ value }}</template>
    <template v-for="(key, value) in yyyy" #[key]>{{ value }}</template>
  </MyComponent>
  <MyComponent>
    <template #[key]>{{ value }}</template>
    <template v-for="(key, value) in yyyy" #[key]>{{ value }}</template>
  </MyComponent>
  <MyComponent>
    <OtherComponent v-slot="{ data }">{{ data }}</OtherComponent>
  </MyComponent>
  <svg>
    <MyComponent v-slot="slotProps">
      <MyChildComponent :thing="slotProps.thing" />
    </MyComponent>
  </svg>
</template>
