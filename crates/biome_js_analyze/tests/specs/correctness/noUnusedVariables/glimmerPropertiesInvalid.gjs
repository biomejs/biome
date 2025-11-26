// unusedProperty is never used - should warn
export default class MyComponent {
  title = "Hello";
  count = 0;
  unusedProperty = "never used";

  <template>
    <div>
      <h1>{{this.title}}</h1>
      <p>Count: {{this.count}}</p>
    </div>
  </template>
}
