import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';
import { on } from '@ember/modifier';

export default class Counter extends Component {
  @tracked count = 0;

  increment = () => {
    this.count++;
  };

  <template>
    <div class="counter" ...attributes>
      <h1>{{@title}}</h1>

      <p>Current count: <strong>{{this.count}}</strong></p>

      {{#if @showButtons}}
        <button {{on "click" this.increment}}>
          Increment
        </button>
      {{/if}}

      {{#each @items as |item index|}}
        <li>{{index}}: {{item.name}}</li>
      {{/each}}
    </div>
  </template>
}
