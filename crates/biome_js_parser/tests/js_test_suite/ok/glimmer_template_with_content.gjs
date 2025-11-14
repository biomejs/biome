// Template with various content
const MyTemplate = <template>
  <div>
    <h1>Hello {{name}}</h1>
    <p>{{@description}}</p>
    {{#each items as |item|}}
      <li>{{item}}</li>
    {{/each}}
  </div>
</template>;
