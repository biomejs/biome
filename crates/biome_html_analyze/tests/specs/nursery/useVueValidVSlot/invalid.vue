<!-- should generate diagnostics -->
<template>
  <div v-slot="{ data }">{{ data }}</div>
  <div v-slot:named="{ data }">{{ data }}</div>
  <div><template v-slot></template></div>
  <template v-slot:named></template>

  <MyComponent v-slot:named="{ data }">{{ data }}</MyComponent>
  <MyComponent #named />
  <MyComponent v-slot="{ data }">
    {{ data }}
    <template #named></template>
  </MyComponent>

  <MyComponent v-slot="{ data }" #default="other">{{ data }}</MyComponent>
  <MyComponent>
    <template v-slot:one v-slot:two></template>
  </MyComponent>

  <MyComponent>
    <template v-slot:one>1st</template>
    <template v-slot:one>2nd</template>
  </MyComponent>
  <MyComponent>
    <template v-slot:[one]>1st</template>
    <template v-slot:[one]>2nd</template>
  </MyComponent>
  <MyComponent>
    <template v-slot>1st</template>
    <template v-slot:default>2nd</template>
  </MyComponent>
  <MyComponent>
    <template v-if="c1" v-slot:one>1st</template>
    <template v-slot:one>2nd</template>
  </MyComponent>
  <MyComponent>
    <template v-if="c1" v-slot:one>1st</template>
    <template v-else v-slot:one>2nd</template>
    <template v-if="c2" v-slot:one>3rd</template>
  </MyComponent>
  <MyComponent>
    <template v-for="(key, value) in definition" v-slot:one>{{ value }}</template>
  </MyComponent>
  <MyComponent>
    <template v-for="item in items" #key>{{ item }}</template>
  </MyComponent>

  <MyComponent v-slot.foo="{ data }">{{ data }}</MyComponent>
  <MyComponent>
    <template v-slot.foo></template>
  </MyComponent>
  <MyComponent>
    <template #foo.bar.baz></template>
  </MyComponent>

  <MyComponent v-slot>content</MyComponent>
  <MyComponent v-slot="">content</MyComponent>
  <MyComponent #default="  ">content</MyComponent>
</template>
