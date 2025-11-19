{{#if showTitle}}
  <h1>{{title}}</h1>
{{/if}}

{{#each items as |item index|}}
  <li>{{item.name}}</li>
{{/each}}

{{#with user as |u|}}
  <p>{{u.name}}</p>
{{/with}}
