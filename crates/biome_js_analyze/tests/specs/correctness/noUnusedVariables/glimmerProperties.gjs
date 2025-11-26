/* should not generate diagnostics - properties used in template */
export default class MyComponent {
  title = "Hello";
  count = 0;
  isActive = true;

  <template>
    <div>
      <h1>{{this.title}}</h1>
      <p>Count: {{this.count}}</p>
      <p>Active: {{this.isActive}}</p>
    </div>
  </template>
}
